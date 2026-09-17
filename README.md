# design-everything

视觉设计核心 skill (design-core)。六阶段契约流程, 质量门由脚本判定 (pass/fail, 无评分), 状态落盘 design-brief.json, 知识库为单文件 SQLite FTS。运行时入口与完整契约见 [skill.md](./skill.md)。

## 架构

```
源层        data/tsv/*.tsv              唯一事实源, 人可审, diff 友好
   ↓ build_db.py
构建层      data/design_knowledge.db    FTS5 + CJK 二元分词, bm25; 构建产物, 不手工修改
   ↓ design.py
运行层      list/search/read/contrast/palette/export/gate/init
   ↓
契约层      design-brief.json           每阶段写入, 用户确认推进, 中断可恢复
   ↓
评测层      gate                        pass/fail + 缺失清单, 退出码 0/1
```

原则: 确定性问题 (对比度, 配色旋转, 导出, 检索) 交代码; 模糊性问题 (审美, 取舍, 提问) 交 LLM; 边界写进契约。

## CLI 接口

```
uv run python scripts/design.py list                        模块与条目数
uv run python scripts/design.py search "<词>" [--module M]  FTS 检索, bm25 排序, 无结果回退 LIKE
uv run python scripts/design.py read <module> [--limit N]   读取整模块
uv run python scripts/design.py contrast <fg> <bg> [--large]  WCAG 对比度, 正文 AA=4.5, 大字号 3.0
uv run python scripts/design.py palette <hex> [--scheme S]  analogous|complementary|split|tetradic, 输出 60/30/10 角色
uv run python scripts/design.py export <brief> --format F   css | tailwind | godot
uv run python scripts/design.py gate <brief> [--up-to S]    校验简报, 输出 JSON {pass, missing, contrast}, 退出码 0/1
uv run python scripts/design.py init [--out PATH]           生成空简报模板, 已存在则拒绝覆盖
```

运行回退链: `uv run python` 失败换 `py`, 再换 `python3`; CLI 全部失败时直接 Grep `data/tsv/<module>.tsv`。

## gate 契约

- 六阶段 analyze/propose/color/layout/elements/avoid 依次校验; `--up-to S` 只查到阶段 S, 与推进节奏一致
- color 阶段自动计算 primary/accent 对 background 的对比度, 未达 AA 判 fail
- 输出只有 pass/fail 加缺失清单, 无评分; gate 未通过禁止宣布完成

## 知识库

- 事实源 `data/tsv/` 共 18 模块 (ui, ux, game, scene, pose, screenplay, cinematography, manga_storyboard, video_editing, character_motion + literature_* 8 个), 实时计数用 `design.py list`
- 条目表头统一 title/content/tags/metadata; metadata 尽量带 source 与日期
- 修改知识: 改 TSV 后重跑 `uv run python scripts/build_db.py`, 永远不改 .db
- 中文检索经 CJK 二元分词, 双字词 (如"按钮") 可命中

## 目录

```
design-everything/
├── skill.md                 L2 入口: 公理, 六阶段契约, 命令与回退链, 反例
├── README.md                本文件
├── scripts/
│   ├── build_db.py          TSV -> design_knowledge.db, stdlib-only, 可重复构建
│   ├── design.py            单一 CLI, stdlib-only
│   └── README.md            工具链说明与退役记录
├── data/
│   ├── tsv/                 事实源 (18 模块)
│   ├── design_knowledge.db  构建产物 (gitignore)
│   └── evaluation_cases.md  golden case 规划
├── references/              L3 深度知识: styles, color, composition, ui, ux + 叙事域 8 篇
├── RULES.kdl                设计规则声明 (rust validator 已退役, 待后续接管)
├── .github/workflows/       CI: py 工具链冒烟 (构建库, 检索, 对比度, gate 反向用例)
└── .gitignore               忽略构建产物与运行态文件
```

历史遗留, 待 P0 清洁移出: doc/, templates/, examples/, style-guides/, specs/, 文学素材*.md, NOT_SORT.MD, LINK.md, OTHER_SKILLS.md, openmemory.md。

## 模块索引

- 视觉域 references (六阶段直接引用): styles, color, composition, ui, ux
- 视觉域 DB 模块 (可检索): ui, ux
- 叙事域 (拆分为独立 skill 的规划中, 数据仍可检索): game, manga_storyboard, video_editing, cinematography, screenplay, pose, scene, character_motion
- 文学素材库 literature_*: 独立 skill 规划中
- agent 域 ai_design.md 与 ai_agent.md: 不属本 skill, 不加载; modules.md 与 checklists.md 未挂接流程, 暂不使用

## 新鲜度

引用外部资料带来源与日期, 核验不了标注"未核验"; 本仓库声明与磁盘实现保持一致, 发现不一致以磁盘为准并报告。
