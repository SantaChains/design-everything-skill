---
name: design-everything
description: |
  视觉设计核心技能: 风格提案, 配色方案, 构图布局, UI 组件与设计禁忌.
  六阶段契约流程, 状态落盘 design-brief.json, 质量门由 scripts/design.py 判定 (pass/fail 加缺失清单, 无评分).
  知识库: 单文件 SQLite FTS, 检索命令 uv run python scripts/design.py search "<关键词>".
  Use when 用户提到 配色, 色彩, 风格, 构图, 排版, 字体, UI, 组件, 海报, Logo, 界面, 视觉, 或要求 color palette, layout, style proposal.
  不适用: 代码架构, 软件模式, 数据库设计, 系统设计; 游戏机制与分镜剧本等叙事域数据可在本库检索, 独立 skill 规划中.
---

# design-everything | design-core

## 核心公理

1. 诚实为先: 不知道就明说; 无验证器支撑的分数一律不输出; 文档声明与磁盘实现必须一致.
2. 可验证: 质量门由脚本判定, 输出 pass/fail 加缺失清单; LLM 负责判断与创意, 不负责算术与合规.
3. 程序式计算优先: 对比度, 配色旋转, 导出, 检索交脚本; 审美, 取舍, 提问交 LLM.
4. 渐进披露: 本文件是索引与契约, 深度知识在 references/, 按需读取.
5. 文件态自省: 全部阶段状态写入 design-brief.json, 不依赖上下文记忆.
6. 学识与开创 (激活时机: S2 提案, 风格混搭, 跨域需求): 符合对应领域经验, 充分调用可用工具; 以渊博学识和理念从精神具象出现象与物质; 善抓跨领域共性平移方法和大胆前卫开创; 最后合理取舍与融合, 砍掉一切冗余; 开创性方案仍须过 gate 并经用户确认, 不可验证处标注"未核验".

设计第一问 (S1 必答): 这个设计最不想传达什么? 知道要避免什么, 比知道要做什么更重要.

## 运行环境

- 知识库: `data/design_knowledge.db` (TSV 的构建产物, 永远不手工改库; 事实源是 `data/tsv/*.tsv`)
- 重建库: `uv run python scripts/build_db.py`
- CLI: `uv run python scripts/design.py <子命令>`
- 回退链: `uv run python` 失败换 `py`, 再换 `python3`; CLI 全部失败时, 直接读 `data/tsv/<module>.tsv` 并用 Grep 检索
- 库不存在或怀疑过期时, 先跑 build_db.py, 再检索

## 状态文件 design-brief.json

生命周期:
1. S1 开始时 `design.py init --out design-brief.json` 生成骨架 (已存在则跳过 init)
2. 每阶段末: 填入该阶段字段, 更新顶层 `stage`, 向用户展示该阶段数据并获确认
3. 用户确认后才能进入下一阶段; propose.selected 只能在用户明示选择后写入
4. 中断恢复: 新会话若工作目录存在 design-brief.json, 读取 `stage` 从该阶段继续, 不重做已完成阶段

```json
{
  "version": 1,
  "stage": "analyze|propose|color|layout|elements|avoid|complete",
  "analyze":   { "keywords": [], "emotion": "", "scene": "", "audience": "", "not_convey": "" },
  "propose":   { "options": [ { "name": "", "keywords": [], "color_features": "" } ], "selected": "" },
  "color": {
    "primary":   { "hex": "", "name": "", "emotion": "" },
    "secondary": { "hex": "", "name": "", "emotion": "" },
    "accent":    { "hex": "", "name": "", "emotion": "" },
    "background": "", "rationale": ""
  },
  "layout":   { "composition_rule": "", "visual_hierarchy": [], "description": "" },
  "elements": { "required": [], "recommended": [], "optional": [] },
  "avoid":    { "taboos": [ { "avoid": "", "reason": "", "alternative": "" } ] }
}
```

禁止: 跨阶段批量填写; 未确认即推进; 猜测字段值.

## 六阶段契约

每阶段四要素 — 写入字段, 动作, 禁止, gate 命令. gate 通过 (missing 为空) 才进下一阶段. 下文 gate 省略公共前缀: `design.py gate design-brief.json`.

### S1 ANALYZE 需求解析

- 写入: analyze.{keywords, emotion, scene, audience, not_convey}
- 动作: 从请求提取关键词与情绪与场景; 先问倒置问题 "最不想传达什么" 记入 not_convey; 信息不足用 AskUserQuestion, 一次只问一个核心要素, 确认后再问下一个
- 禁止: 猜测缺失字段; 一次抛出多个问题; 用户说"你看着办"时跳过 not_convey
- 失败: 用户拒绝现有解析 → 清空 analyze 重问
- gate: `design.py gate design-brief.json --up-to analyze`

### S2 PROPOSE 风格提案

- 写入: propose.options (2 到 3 个), 用户确认后写 propose.selected
- 动作: 每选项含 name, keywords, color_features, 代表案例; 风格参考 → references/styles.md; 跨库检索佐证 → `design.py search "<情绪或场景>"`
- 禁止: 只给一个选项; 编造不存在的代表案例
- 失败: 全部被拒 → 回退 S1 重新解析, 不在原选项上硬调
- gate: `--up-to propose`

### S3 COLOR 色彩方案

- 写入: color.{primary, secondary, accent}.{hex, name, emotion}, color.background, color.rationale
- 动作: 60/30/10 三色原则; 配色生成 → `design.py palette <基色HEX> --scheme analogous|complementary|split|tetradic`; 心理速查 → references/color.md; 色值组合必须实测 `design.py contrast <前景> <背景>` (正文 AA>=4.5, 大字号加 --large 阈值 3.0)
- 禁止: 心算或凭记忆判断对比度; 提交未过 AA 的组合 (用户明示豁免除外, 豁免记入 rationale)
- 失败: contrast fail → palette 换 scheme 或调明度饱和度后重测, 直到 pass
- gate: `--up-to color`

### S4 LAYOUT 构图建议

- 写入: layout.{composition_rule, visual_hierarchy, description}
- 动作: 法则速查 → references/composition.md (三分法/黄金分割/对角线/对称/留白); 层级按 焦点→次级→背景 表述; 给文字版布局描述
- 禁止: 构图与情绪矛盾 (例: 传达庄重却选对角线)
- 失败: 用户否 → 换法则重提, 不叠加修补
- gate: `--up-to layout`

### S5 ELEMENTS 元素清单

- 写入: elements.{required, recommended, optional}, 每元素含 name 与 purpose
- 动作: 组件知识检索 → `design.py search "<元素>" --module ui`; 按布局逐区推导所需元素
- 禁止: 列出说不出用途的装饰元素
- gate: `--up-to elements`

### S6 AVOID 禁忌清单

- 写入: avoid.taboos, 每条含 avoid, reason, alternative
- 动作: 从 analyze.not_convey 出发倒推禁忌; 覆盖 视觉冲突/文化禁忌/技术限制 三类
- 禁止: 无 reason 或无 alternative 的条目
- gate: `--up-to avoid` (missing 必须为空)

### COMPLETE 完成

- 动作: 全量 `design.py gate design-brief.json` 必须输出 pass=true; 可选导出 `design.py export design-brief.json --format css|tailwind|godot`; 有可验证新经验时追加到 data/tsv 对应模块 (metadata 带 source 与日期) 并重跑 build_db.py
- 禁止: gate 未 pass 就宣布完成; 输出任何形式的质量分数或百分比

## 输出格式

每阶段向用户输出三段: 该阶段写入 brief 的字段值摘要; gate 结果原文 (pass 或 missing 清单); 待确认项一句话. 最终输出: gate 全量结果加导出物路径. 不使用打分, 星级, 置信度.

## 知识库检索 (L4 零上下文)

```
design.py list                          全部模块与条目数
design.py search "<词>"                 跨库 FTS 检索, bm25 排序
design.py search "<词>" --module ui     限定模块
design.py read <module> --limit 10      读取整模块
```

命中后仅把需要的条目带入回复, 不整表粘贴. 文学素材 (literature_* 模块) 与叙事域模块可检索, 但属独立 skill 管辖, 引用时注明来源模块.

## 模块索引

- 视觉域 references (六阶段直接引用): styles, color, composition, ui, ux
- 视觉域 DB 模块 (data/tsv, 可检索): ui, ux
- 叙事域 (拆分进行中, 数据仍可检索): game, manga_storyboard, video_editing, cinematography, screenplay, pose, scene, character_motion
- 文学素材库: literature_* (独立 skill 规划中)
- agent 域: references/ai_design.md 与 ai_agent.md 不属本 skill, 不加载; modules.md 与 checklists.md 未挂接流程, 暂不使用

## 新鲜度约束

1. 引用外部资料必须带来源与日期; 核验不了的内容标注"未核验", 不沉默, 不编造.
2. 涉及工具或规范的最新要求, 先搜当年官方文档再回答.
3. 库内条目 metadata 中带 source= 的为有据条目; 无 source 的视为内部经验, 引用时如实说明.
4. 本文件各命令与磁盘实现一致; 发现不一致, 以磁盘为准并明确报告差异.

## 反例速查 (禁止行为)

- 错误: 未问用途直接给三色方案 → 正确: S1 先定 not_convey
- 错误: "质量分 0.9, 进入下一阶段" → 正确: 报 gate pass=true 与 missing=[]
- 错误: "这对色值对比度应该没问题" → 正确: design.py contrast 实测后引用输出
- 错误: 一次问五个问题 → 正确: 一次一个核心要素, 确认后再问
- 错误: 手工改 design_knowledge.db → 正确: 改 data/tsv 后重跑 build_db.py
