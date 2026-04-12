#!/usr/bin/env python3
"""
Design Everything - TSV Reader
直接读取 TSV 内容文件，无需导入数据库
跨平台兼容：Windows / macOS / Linux
"""

import csv
import os
from pathlib import Path
from typing import Optional, List, Dict, Any

SCRIPT_DIR = Path(__file__).parent.parent.resolve()
TSV_DIR = SCRIPT_DIR / "data" / "tsv"

class TSVReader:
    def __init__(self, tsv_dir: Optional[str] = None):
        self.tsv_dir = Path(tsv_dir) if tsv_dir else TSV_DIR

    def read_tsv(self, filename: str) -> List[Dict[str, Any]]:
        filepath = self.tsv_dir / filename
        if not filepath.exists():
            raise FileNotFoundError(f"TSV file not found: {filepath}")

        items = []
        with open(filepath, "r", encoding="utf-8") as f:
            reader = csv.DictReader(f, delimiter="\t")
            for row in reader:
                items.append({
                    "title": row.get("title", ""),
                    "content": row.get("content", ""),
                    "tags": row.get("tags", ""),
                    "metadata": row.get("metadata", "")
                })
        return items

    def search_in_tsv(self, filename: str, query: str,
                      tags: Optional[List[str]] = None) -> List[Dict[str, Any]]:
        items = self.read_tsv(filename)
        results = []

        query_lower = query.lower() if query else ""
        for item in items:
            matched = False
            if query_lower:
                if query_lower in item["title"].lower() or query_lower in item["content"].lower():
                    matched = True
            elif tags:
                item_tags = [t.strip() for t in item["tags"].split(",")]
                if any(tag in item_tags for tag in tags):
                    matched = True
            else:
                matched = True

            if matched:
                results.append(item)

        return results

    def search_all(self, query: str, tags: Optional[List[str]] = None) -> Dict[str, List[Dict[str, Any]]]:
        results = {}
        for tsv_file in self.tsv_dir.glob("*.tsv"):
            module_results = self.search_in_tsv(tsv_file.name, query, tags)
            if module_results:
                module_slug = tsv_file.stem
                results[module_slug] = module_results
        return results

    def get_module_list(self) -> List[str]:
        return [f.stem for f in self.tsv_dir.glob("*.tsv")]

    def count_items(self, filename: str) -> int:
        try:
            items = self.read_tsv(filename)
            return len(items)
        except FileNotFoundError:
            return 0

    def get_all_counts(self) -> Dict[str, int]:
        counts = {}
        for tsv_file in self.tsv_dir.glob("*.tsv"):
            counts[tsv_file.stem] = self.count_items(tsv_file.name)
        return counts


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Design Everything TSV Reader")
    parser.add_argument("--read", metavar="MODULE", help="Read a specific TSV module")
    parser.add_argument("--search", nargs="+", metavar="QUERY", help="Search in all modules")
    parser.add_argument("--search-module", nargs=2, metavar=("MODULE", "QUERY"), help="Search in specific module")
    parser.add_argument("--list", action="store_true", help="List all available modules")
    parser.add_argument("--count", metavar="MODULE", help="Count items in a module")
    parser.add_argument("--counts", action="store_true", help="Count items in all modules")

    args = parser.parse_args()
    reader = TSVReader()

    if args.read:
        try:
            items = reader.read_tsv(f"{args.read}.tsv")
            print(f"\n=== {args.read.upper()} === ({len(items)} items)\n")
            for item in items:
                print(f"[{item['title']}]")
                print(f"  {item['content'][:200]}..." if len(item['content']) > 200 else f"  {item['content']}")
                print(f"  Tags: {item['tags']}\n")
        except FileNotFoundError:
            print(f"Module '{args.read}' not found.")

    elif args.search:
        query = " ".join(args.search)
        print(f"\nSearching: '{query}'\n")
        results = reader.search_all(query)
        for module, items in results.items():
            print(f"=== {module.upper()} ({len(items)} matches) ===")
            for item in items[:5]:
                print(f"  • {item['title']}")
            if len(items) > 5:
                print(f"  ... and {len(items) - 5} more")
            print()

    elif args.search_module:
        module, query = args.search_module
        items = reader.search_in_tsv(f"{module}.tsv", query)
        print(f"\n=== {module.upper()} - '{query}' ({len(items)} matches) ===\n")
        for item in items:
            print(f"[{item['title']}]")
            print(f"  {item['content']}")
            print(f"  Tags: {item['tags']}\n")

    elif args.list:
        modules = reader.get_module_list()
        print(f"\nAvailable modules ({len(modules)}):\n")
        for m in modules:
            count = reader.count_items(f"{m}.tsv")
            print(f"  {m}: {count} items")

    elif args.count:
        count = reader.count_items(f"{args.count}.tsv")
        print(f"{args.count}: {count} items")

    elif args.counts:
        counts = reader.get_all_counts()
        print(f"\nModule counts:\n")
        total = 0
        for module, count in counts.items():
            print(f"  {module}: {count}")
            total += count
        print(f"\nTotal: {total} items")

    else:
        parser.print_help()


if __name__ == "__main__":
    main()
