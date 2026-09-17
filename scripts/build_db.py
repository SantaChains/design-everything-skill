"""构建 design_knowledge.db: data/tsv/*.tsv -> 单文件 SQLite (FTS5)。

stdlib-only, 可重复构建 (先删后建)。中文检索采用 CJK 二元分词预处理,
查询侧用同一变换, 保证 "按钮" 这类双字词可命中。
"""
from __future__ import annotations

import csv
import re
import sqlite3
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TSV_DIR = ROOT / "data" / "tsv"
DB_PATH = ROOT / "data" / "design_knowledge.db"

CJK = re.compile(r"[\u3400-\u9fff\uf900-\ufaff]+")

SCHEMA = """
CREATE TABLE modules (
  module_id   TEXT PRIMARY KEY,
  name        TEXT NOT NULL,
  entry_count INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE entries (
  id        INTEGER PRIMARY KEY,
  module_id TEXT NOT NULL REFERENCES modules(module_id),
  title     TEXT NOT NULL,
  content   TEXT NOT NULL,
  tags      TEXT NOT NULL DEFAULT '',
  metadata  TEXT NOT NULL DEFAULT ''
);
CREATE INDEX idx_entries_module ON entries(module_id);
CREATE VIRTUAL TABLE entries_fts USING fts5 (
  title, content, tags, ix,
  content='entries', content_rowid='id',
  tokenize='unicode61 remove_diacritics 2'
);
"""


def cjk_tokens(text: str) -> list[str]:
    """CJK 连续段切为重叠二元组; 单字段保留单字; 非CJK 按词保留。"""
    tokens: list[str] = []
    for part in re.split(r"([^\u3400-\u9fff\uf900-\ufaff]+)", text):
        if not part:
            continue
        if CJK.fullmatch(part):
            if len(part) == 1:
                tokens.append(part)
            else:
                tokens.extend(part[i:i + 2] for i in range(len(part) - 1))
        else:
            tokens.extend(w for w in re.split(r"[\s,.;:!?|，。；：！？、]+", part) if w)
    return tokens


def index_text(title: str, content: str, tags: str) -> str:
    """ix 列: 三字段合并后的分词流, 供 MATCH 使用。"""
    return " ".join(cjk_tokens(f"{title} {content} {tags}"))


def read_tsv(path: Path) -> list[dict]:
    with path.open(encoding="utf-8", newline="") as f:
        rows = list(csv.DictReader(f, delimiter="\t"))
    need = {"title", "content"}
    if not rows or not need.issubset(rows[0]):
        print(f"[skip] {path.name}: 缺少 title/content 列", file=sys.stderr)
        return []
    return rows


def build() -> int:
    tsvs = sorted(TSV_DIR.glob("*.tsv"))
    if not tsvs:
        print(f"未找到 TSV: {TSV_DIR}", file=sys.stderr)
        return 1

    DB_PATH.unlink(missing_ok=True)
    conn = sqlite3.connect(DB_PATH)
    conn.executescript(SCHEMA)

    total = 0
    for path in tsvs:
        module_id = path.stem
        rows = read_tsv(path)
        if not rows:
            continue
        conn.executemany(
            "INSERT INTO entries (module_id, title, content, tags, metadata) "
            "VALUES (?, ?, ?, ?, ?)",
            [(module_id, r["title"].strip(), r["content"].strip(),
              r.get("tags", "").strip(), r.get("metadata", "").strip()) for r in rows],
        )
        conn.execute(
            "INSERT INTO modules (module_id, name, entry_count) VALUES (?, ?, ?)",
            (module_id, module_id, len(rows)),
        )
        print(f"  {module_id}: {len(rows)} 条")
        total += len(rows)

    # FTS 逐行填充 (ix 为分词列)
    for row_id, title, content, tags in conn.execute(
        "SELECT id, title, content, tags FROM entries"
    ):
        conn.execute(
            "INSERT INTO entries_fts (rowid, title, content, tags, ix) VALUES (?, ?, ?, ?, ?)",
            (row_id, title, content, tags, index_text(title, content, tags)),
        )

    conn.commit()
    # FTS 与 entries 一一对应; 冒烟验证中文双字词可命中
    ok = conn.execute("SELECT count() FROM entries").fetchone()[0]
    hit = conn.execute(
        "SELECT rowid FROM entries_fts WHERE entries_fts MATCH '按钮' LIMIT 1"
    ).fetchone()
    conn.close()
    print(f"构建完成: {DB_PATH.name}  模块 {len(tsvs)}  条目 {total}  FTS {ok}  冒烟 {'ok' if hit else '未命中'}")
    return 0 if (ok == total and hit) else 1


if __name__ == "__main__":
    sys.exit(build())
