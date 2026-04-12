#!/usr/bin/env python3
"""
Design Everything - SQLite Index Manager
跨平台 SQLite 管理脚本 - 仅存储模块索引和关系数据
TSV 内容直接读取，不导入数据库
"""

import sqlite3
import os
from pathlib import Path
from typing import Optional, List, Dict, Any

SCRIPT_DIR = Path(__file__).parent.parent.resolve()
DB_PATH = SCRIPT_DIR / "data" / "modules_index.db"

class IndexDBManager:
    def __init__(self, db_path: Optional[str] = None):
        self.db_path = Path(db_path) if db_path else DB_PATH
        self._init_db()

    def _get_connection(self) -> sqlite3.Connection:
        conn = sqlite3.connect(str(self.db_path))
        conn.row_factory = sqlite3.Row
        return conn

    def _init_db(self):
        os.makedirs(self.db_path.parent, exist_ok=True)
        with self._get_connection() as conn:
            conn.executescript("""
                CREATE TABLE IF NOT EXISTS categories (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE,
                    slug TEXT NOT NULL UNIQUE,
                    description TEXT
                );

                CREATE TABLE IF NOT EXISTS modules (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE,
                    slug TEXT NOT NULL UNIQUE,
                    description TEXT,
                    category_id INTEGER,
                    keywords TEXT,
                    tsv_file TEXT,
                    item_count INTEGER DEFAULT 0,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (category_id) REFERENCES categories(id)
                );

                CREATE TABLE IF NOT EXISTS module_tags (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    module_id INTEGER NOT NULL,
                    tag TEXT NOT NULL,
                    FOREIGN KEY (module_id) REFERENCES modules(id)
                );

                CREATE INDEX IF NOT EXISTS idx_modules_slug ON modules(slug);
                CREATE INDEX IF NOT EXISTS idx_modules_category ON modules(category_id);
                CREATE INDEX IF NOT EXISTS idx_tags_module ON module_tags(module_id);
                CREATE INDEX IF NOT EXISTS idx_tags_tag ON module_tags(tag);
            """)
            conn.commit()

    def register_category(self, name: str, slug: str, description: str) -> int:
        with self._get_connection() as conn:
            cursor = conn.execute("""
                INSERT OR IGNORE INTO categories (name, slug, description)
                VALUES (?, ?, ?)
            """, (name, slug, description))
            return cursor.lastrowid

    def register_module(self, name: str, slug: str, description: str,
                       category_slug: str, keywords: str, tsv_file: str) -> int:
        with self._get_connection() as conn:
            cursor = conn.execute("""
                INSERT OR REPLACE INTO modules (name, slug, description, category_id, keywords, tsv_file, updated_at)
                VALUES (?, ?, ?,
                    (SELECT id FROM categories WHERE slug = ?),
                    ?, ?, CURRENT_TIMESTAMP)
            """, (name, slug, description, category_slug, keywords, tsv_file))
            return cursor.lastrowid

    def register_tag(self, module_slug: str, tag: str):
        with self._get_connection() as conn:
            conn.execute("""
                INSERT OR IGNORE INTO module_tags (module_id, tag)
                SELECT id, ? FROM modules WHERE slug = ?
            """, (tag, module_slug))

    def update_module_item_count(self, module_slug: str, count: int):
        with self._get_connection() as conn:
            conn.execute("""
                UPDATE modules SET item_count = ? WHERE slug = ?
            """, (count, module_slug))

    def search_modules(self, query: str) -> List[Dict[str, Any]]:
        with self._get_connection() as conn:
            cursor = conn.execute("""
                SELECT m.*, c.name as category_name
                FROM modules m
                LEFT JOIN categories c ON m.category_id = c.id
                WHERE m.name LIKE ? OR m.description LIKE ? OR m.keywords LIKE ?
                ORDER BY
                    CASE WHEN m.name LIKE ? THEN 0 ELSE 1 END,
                    m.name
            """, (f"%{query}%", f"%{query}%", f"%{query}%", f"{query}%"))
            return [dict(row) for row in cursor.fetchall()]

    def get_module_by_slug(self, slug: str) -> Optional[Dict[str, Any]]:
        with self._get_connection() as conn:
            cursor = conn.execute("""
                SELECT m.*, c.name as category_name
                FROM modules m
                LEFT JOIN categories c ON m.category_id = c.id
                WHERE m.slug = ?
            """, (slug,))
            row = cursor.fetchone()
            return dict(row) if row else None

    def get_all_modules(self) -> List[Dict[str, Any]]:
        with self._get_connection() as conn:
            cursor = conn.execute("""
                SELECT m.*, c.name as category_name
                FROM modules m
                LEFT JOIN categories c ON m.category_id = c.id
                ORDER BY c.name, m.name
            """)
            return [dict(row) for row in cursor.fetchall()]

    def get_all_categories(self) -> List[Dict[str, Any]]:
        with self._get_connection() as conn:
            cursor = conn.execute("SELECT * FROM categories ORDER BY name")
            return [dict(row) for row in cursor.fetchall()]

    def get_modules_by_category(self, category_slug: str) -> List[Dict[str, Any]]:
        with self._get_connection() as conn:
            cursor = conn.execute("""
                SELECT m.* FROM modules m
                JOIN categories c ON m.category_id = c.id
                WHERE c.slug = ?
                ORDER BY m.name
            """, (category_slug,))
            return [dict(row) for row in cursor.fetchall()]

    def get_module_stats(self) -> Dict[str, Any]:
        with self._get_connection() as conn:
            total_modules = conn.execute("SELECT COUNT(*) FROM modules").fetchone()[0]
            total_categories = conn.execute("SELECT COUNT(*) FROM categories").fetchone()[0]
            by_category = conn.execute("""
                SELECT c.name, COUNT(m.id) as count
                FROM categories c
                LEFT JOIN modules m ON c.id = m.category_id
                GROUP BY c.id
                ORDER BY count DESC
            """).fetchall()
            return {
                "total_modules": total_modules,
                "total_categories": total_categories,
                "by_category": [dict(row) for row in by_category]
            }


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Design Everything Index Manager")
    parser.add_argument("--init", action="store_true", help="Initialize database")
    parser.add_argument("--register-module", nargs=6, metavar=("NAME", "SLUG", "DESC", "CATEGORY", "KEYWORDS", "TSV"),
                       help="Register a module")
    parser.add_argument("--register-category", nargs=3, metavar=("NAME", "SLUG", "DESC"),
                       help="Register a category")
    parser.add_argument("--list", action="store_true", help="List all modules")
    parser.add_argument("--list-categories", action="store_true", help="List all categories")
    parser.add_argument("--list-by-category", metavar="CATEGORY", help="List modules by category")
    parser.add_argument("--search", metavar="QUERY", help="Search modules")
    parser.add_argument("--stats", action="store_true", help="Show statistics")

    args = parser.parse_args()
    db = IndexDBManager()

    if args.init:
        print(f"Database initialized: {db.db_path}")

    elif args.register_category:
        name, slug, desc = args.register_category
        db.register_category(name, slug, desc)
        print(f"Category '{name}' registered.")

    elif args.register_module:
        name, slug, desc, category, keywords, tsv = args.register_module
        db.register_module(name, slug, desc, category, keywords, tsv)
        print(f"Module '{name}' registered.")

    elif args.list:
        for m in db.get_all_modules():
            print(f"[{m['category_name']}] {m['name']} ({m['slug']}) - {m['item_count']} items")

    elif args.list_categories:
        for c in db.get_all_categories():
            print(f"{c['name']} ({c['slug']})")

    elif args.list_by_category:
        for m in db.get_modules_by_category(args.list_by_category):
            print(f"  {m['name']} ({m['slug']})")

    elif args.search:
        for m in db.search_modules(args.search):
            print(f"[{m['category_name']}] {m['name']}: {m['description']}")

    elif args.stats:
        stats = db.get_module_stats()
        print(f"Categories: {stats['total_categories']}")
        print(f"Modules: {stats['total_modules']}")
        for c in stats["by_category"]:
            print(f"  {c['name']}: {c['count']} modules")

    else:
        parser.print_help()


if __name__ == "__main__":
    main()
