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
│   ├── design_knowledge.db  构建产物 (已入库, 免构建即用)
│   └── evaluation_cases.md  golden case 规划
├── references/              L3 深度知识: styles, color, composition, ui, ux + 叙事域 8 篇
├── RULES.kdl                设计规则声明 (rust validator 已退役, 待后续接管)
└── .gitignore               忽略编辑器缓存与备份 (.history, *.bak, *.log)
```

历史遗留 (doc/, docs/, specs/, templates/, examples/, style-guides/, 文学素材 md 与 kdl, NOT_SORT.MD, LINK.md, OTHER_SKILLS.md, openmemory.md, 内嵌 .trae 技能合集) 已于 2026-09-17 移出至 d:\ABASE\_Godot\design-archive, 旧内容可经 git 历史 (021750f 及之前) 追溯。

## 模块索引

- 视觉域 references (六阶段直接引用): styles, color, composition, ui, ux
- 视觉域 DB 模块 (可检索): ui, ux
- 叙事域 (拆分为独立 skill 的规划中, 数据仍可检索): game, manga_storyboard, video_editing, cinematography, screenplay, pose, scene, character_motion
- 文学素材库 literature_*: 独立 skill 规划中
- agent 域 ai_design.md 与 ai_agent.md: 不属本 skill, 不加载; modules.md 与 checklists.md 未挂接流程, 暂不使用

## 新鲜度

引用外部资料带来源与日期, 核验不了标注"未核验"; 本仓库声明与磁盘实现保持一致, 发现不一致以磁盘为准并报告。

## 路线图

总目标判据: 任意设计意图一句话进入, 经域路由、契约化阶段推进、确定性工具校验, 产出过 gate 的可验证交付物。以下为现状差距与打磨次序。

### 基线 (已达成)

- 视觉六阶段契约, design-brief.json 文件态, gate pass/fail 无评分
- py 单一 CLI 八子命令, FTS5 CJK 检索知识库, db 入库免构建
- 目录收敛至 skill 运行所需, 归档区 (d:\ABASE\_Godot\design-archive) 独立于仓库

### 阶段 1: 域扩展 (design-core 到 design-family)

- 叙事域 skill: 分镜/剪辑/摄影/剧本/姿势/场景/动画/游戏机制, 数据已在库, 缺 skill 封装与阶段映射; 剧本域需阶段变体 (人物弧光, 三幕节拍) 而非照搬视觉六阶段
- 文学素材库 skill: design-archive/literature 已有底子 (literature_library.kdl 与六篇研究文档)
- agent 域 skill: references/ai_design.md 与 ai_agent.md 迁入, 归 agent 开发
- 域间协作契约: S5 元素清单可触发跨域 skill 协作, 共享同一 gate 语义与 brief 结构

### 阶段 2: 工具与标准深化

- 对比度升级: APCA (WCAG 3 草案的可感知对比度算法) 与 WCAG 2.x 并列输出; 色盲模拟 (protan/deutan/tritan)
- 字体搭配: 知识模块 (TSV) 加搭配规则工具, 设计最高频痛点之一
- 网格与间距: 8pt 网格计算, 间距阶梯生成
- 色板扩展: 现有四 scheme 之外增加亮度分层与中性色阶梯
- golden case 评测落地: evaluation_cases.md 中规划 3-5 个冒烟用例, 任何 prompt 或脚本改动必跑, 防 prompt 回归
- 知识新鲜度硬约束: TSV metadata 强制 created_at 与 source 字段, 构建时计算 stale 标记, 超期条目不静默沿用
- references 与 TSV 去重: 单一事实源, 重复内容生成化或删除

### 阶段 3: 生态与自迭代

- RULES.kdl validator 以 py 接管 (需 mini KDL 解析或转格式), 恢复规则校验能力
- self-improving 经验回写落地: 定义经验入库格式 (可验证命令加来源日期), 定期审计, 不可验证即删除
- godot 生态联动: 与 godogen、godot-asset-forge 资产管线对接, AI 自动生成资产按本 skill 规范产出
- agent 适配指南: 豆包/workbuddy/traework/qoder 等按本规范产出资产
- 多语言: 触发词与检索关键词双语兼容

### 阶段 4: 万物设计判据

- 新领域接入 = TSV 模块 + 阶段映射 + 工具三件套, 不改核心, 域路由自动生效
- 跨域共性平移工具化: 平移模式库, 记录"域 A 原理到域 B 应用"的可验证条目 (公理六落地)
- 全链路验收: 模糊需求经 Inversion 提问、契约流程、工具校验, 到 gate 通过的交付物, 中间无人工补位环节

