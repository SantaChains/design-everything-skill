# scripts

py 工具链 (stdlib-only), design-everything 的 L4 层.

- build_db.py: data/tsv/*.tsv -> data/design_knowledge.db (FTS5 加 CJK 二元分词, bm25)
- design.py: 单一 CLI, 子命令 list/search/read/contrast/palette/export/gate/init

运行: uv run python scripts/design.py <子命令> (回退 py / python3)
重建库: uv run python scripts/build_db.py

2026-09-17: Rust 实现与旧 py 三件套 (tsv_reader/db_manager/index_manager) 按"砍掉一切冗余"退役, 已提交版本可从 git 历史恢复.
