# design-everything | 设计万花筒

> **核心理念**: 设计不是填空,是破局。好的设计让观者"哇"完之后还想回来研究为什么。

---

## 📖 项目概览

**design-everything** 是一个专业的设计指导 Skill,涵盖视觉设计、UI/UX、游戏设计、媒体设计等多个领域。

### 核心特性

- **🎭 Agent Identity**: 资深视觉设计师 + 用户体验专家
- **📊 四级加载系统**: L1-L4 渐进式加载,零 token 开销
- **⚠️ StateGraph 工作流**: 6 阶段强制流程,确保质量
- **📚 11 个专业模块**: 涵盖设计全领域
- **🔧 脚本工具**: TSV 数据库查询,零 token 执行

---

## 🎯 触发条件

**必须激活**:
- 视觉设计: 配色、风格、构图、排版、字体
- UI/UX: 界面设计、用户体验、交互设计、组件设计
- 游戏设计: 游戏机制、角色设计、场景设计
- 媒体设计: 分镜、剪辑、动效、视频、镜头语言
- 设计工具: Figma、Sketch、Photoshop、Blender

**不激活**:
- 仅提及"设计"但无具体意图
- 代码架构设计、软件设计模式、数据库设计
- 系统设计、分布式系统设计
- 抽象"设计理念"讨论无具体应用

---

## 📂 项目结构

```
design-everything/
├── skill.md                    # 核心技能定义(StateGraph 工作流)
├── openmemory.md               # 记忆技能框架
│
├── doc/                        # 📚 Skill 设计文档
│   ├── index.md                # 文档索引
│   ├── 00_核心概念与价值.md    # Skill 定义与本质
│   ├── 01_设计原则体系.md      # 黄金法则与设计原则
│   ├── 02_架构设计指南.md      # 标准分层架构
│   ├── 03_高级设计模式.md      # 设计模式与范式
│   ├── 04_实践指南与案例.md    # 最佳实践与案例
│   ├── 05_优秀资源与工具.md    # 资源仓库与工具
│   ├── agentDev.md             # Agent CLI 技术实现
│   └── structDev.md            # Agent 工作流架构
│
├── data/                       # 💾 数据文件
│   ├── tsv/                    # TSV 格式设计数据
│   │   ├── character_motion.tsv
│   │   ├── cinematography.tsv
│   │   ├── game.tsv
│   │   ├── manga_storyboard.tsv
│   │   ├── pose.tsv
│   │   ├── scene.tsv
│   │   ├── screenplay.tsv
│   │   ├── ui.tsv
│   │   ├── ux.tsv
│   │   └── video_editing.tsv
│   ├── evaluation_cases.md
│   └── modules_index.db
│
├── references/                 # 📖 参考文档(11 个专业模块)
│   ├── ai_agent.md             # AI Agent 设计
│   ├── ui.md                   # UI 组件设计
│   ├── ux.md                   # UX 设计
│   ├── game.md                 # 游戏设计
│   ├── manga_storyboard.md     # 漫画分镜
│   ├── video_editing.md        # 视频剪辑
│   ├── cinematography.md       # 镜头语言
│   ├── screenplay.md           # 剧本写作
│   ├── pose.md                 # 角色姿势
│   ├── scene.md                # 场景设计
│   ├── character_motion.md     # 角色动作
│   ├── color.md                # 色彩理论
│   ├── composition.md          # 构图原理
│   ├── styles.md               # 设计风格
│   ├── modules.md              # 模块索引
│   └── checklists.md           # 检查清单
│
├── templates/                  # 📋 设计模板
│   ├── color_palette.md        # 配色方案模板
│   ├── composition_layout.md   # 构图与布局模板
│   ├── storyboard.md           # 分镜故事板模板
│   └── game_design.md          # 游戏设计模板
│
├── examples/                   # 🎨 设计示例
│   ├── good/                   # 优秀案例
│   │   └── ui_ux_design.md
│   └── bad/                    # 反面教材
│       └── ui_ux_design.md
│
├── style-guides/               # 🎭 风格指南
│   └── design_styles.md        # 设计风格指南
│
├── scripts/                    # 🔧 脚本工具
│   ├── tsv_reader.py           # TSV 数据查询
│   ├── index_manager.py        # 模块索引管理
│   ├── db_manager.py           # 数据库管理
│   └── setup.py                # 初始化脚本
│
├── specs/                      # 📋 规格文档
│   ├── spec.md                 # 技术规格
│   ├── tasks.md                # 任务清单
│   └── checklist.md            # 检查清单
│
├── RULES.kdl                   # 项目规则
├── LINK.md                     # 相关链接
└── OTHER_SKILLS.md             # 其他技能
```

---

## ⚡ 快速开始

### 1. 触发技能

```
用户: "帮我设计一个科技感的网站配色方案"
AI: [自动激活 design-everything 技能]
```

### 2. StateGraph 工作流

技能将按顺序执行 6 个阶段:

```
ANALYZE (需求解析)
   ↓
PROPOSE (风格提案)
   ↓
COLOR (色彩方案)
   ↓
LAYOUT (构图建议)
   ↓
ELEMENTS (元素清单)
   ↓
AVOID (禁忌提醒)
```

### 3. 使用脚本工具

```bash
# 搜索所有模块
python scripts/tsv_reader.py --search "配色"

# 读取指定模块
python scripts/tsv_reader.py --read ui

# 搜索特定模块
python scripts/tsv_reader.py --search-module ui "按钮"

# 查看所有模块
python scripts/index_manager.py --list

# 按分类查看
python scripts/index_manager.py --list-by-category creative
```

---

## 📚 核心文档

### Skill 设计文档

| 文档 | 内容 | 阅读时间 |
|------|------|---------|
| [文档索引](./doc/index.md) | 完整文档导航 | 5 分钟 |
| [核心概念与价值](./doc/00_核心概念与价值.md) | Skill 定义、本质、价值 | 10 分钟 |
| [设计原则体系](./doc/01_设计原则体系.md) | 黄金法则、设计原则 | 30 分钟 |
| [架构设计指南](./doc/02_架构设计指南.md) | 标准分层架构 | 30 分钟 |
| [高级设计模式](./doc/03_高级设计模式.md) | 设计模式、范式 | 40 分钟 |
| [实践指南与案例](./doc/04_实践指南与案例.md) | 最佳实践、案例 | 30 分钟 |
| [优秀资源与工具](./doc/05_优秀资源与工具.md) | 资源仓库、工具 | 20 分钟 |

### 设计参考文档

| 模块 | 简介 |
|------|------|
| [AI Agent Design](./references/ai_agent.md) | Agent 架构、设计原则、工作流模式 |
| [UI Components](./references/ui.md) | 按钮、输入框、卡片、导航等 |
| [UX Design](./references/ux.md) | 用户研究、可用性、设计原则 |
| [Game Design](./references/game.md) | 游戏机制、心流理论、玩家心理 |
| [Manga Storyboard](./references/manga_storyboard.md) | 分镜、构图、叙事技巧 |
| [Video Editing](./references/video_editing.md) | 蒙太奇、剪辑手法、节奏控制 |
| [Cinematography](./references/cinematography.md) | 镜头语言、构图、光影运动 |
| [Screenplay](./references/screenplay.md) | 剧本结构、人物塑造、对话写作 |
| [Character Pose](./references/pose.md) | 人体姿势、动态、重心 |
| [Scene Design](./references/scene.md) | 场景构成、光影、氛围 |
| [Character Motion](./references/character_motion.md) | 动画原理、运动规律 |

---

## 🎨 设计资源

### 模板 Templates

| 文件 | 内容 |
|------|------|
| [color_palette.md](./templates/color_palette.md) | 配色方案模板(科技、自然、温暖、专业、复古风格) |
| [composition_layout.md](./templates/composition_layout.md) | 构图与布局模板(三分法、黄金分割、网格系统) |
| [storyboard.md](./templates/storyboard.md) | 分镜故事板模板(画格类型、景别、转场符号) |
| [game_design.md](./templates/game_design.md) | 游戏设计模板(核心循环、心流理论、数值平衡) |

### 示例 Examples

| 类型 | 内容 |
|------|------|
| [优秀案例](./examples/good/ui_ux_design.md) | UI/UX 设计优秀案例(按钮、表单、导航、卡片、空状态) |
| [反面教材](./examples/bad/ui_ux_design.md) | UI/UX 设计常见错误(层级混乱、信息过载、错误处理) |

### 风格指南 Style Guides

| 文件 | 内容 |
|------|------|
| [design_styles.md](./style-guides/design_styles.md) | 设计风格指南(极简、赛博朋克、蒸汽波、孟菲斯、浮世绘、玻璃态) |

---

## 🔧 技术架构

### 四级加载系统

| 层级 | 内容 | 大小限制 | 加载时机 | Token 成本 |
|------|------|----------|----------|-----------|
| **L1** | Frontmatter | ~150 词 | 始终常驻 | ~100 词 |
| **L2** | SKILL.md Body | <400 行 | Skill 触发时 | <5k 词 |
| **L3** | References/ | 按需 | 需要时加载 | 无上限 |
| **L4** | Scripts/ | 零 token | 执行时调用 | **0** |

### StateGraph 工作流

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   ANALYZE   │────→│   PROPOSE   │────→│    COLOR    │
│   需求解析   │     │   风格提案   │     │   色彩方案   │
└─────────────┘     └─────────────┘     └─────────────┘
                                               │
                                               ↓
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│    AVOID    │←────│  ELEMENTS   │←────│   LAYOUT    │
│   禁忌提醒   │     │   元素清单   │     │   构图建议   │
└─────────────┘     └─────────────┘     └─────────────┘
```

**Transition 条件**:
1. 当前阶段状态为 `completed`
2. quality_score >= 0.8 (80分)
3. 或用户明确批准

### Checkpoint 状态管理

- **自动保存**: 每个阶段完成后自动保存
- **手动标记**: 关键决策点手动创建检查点
- **时间旅行**: 支持回退到任意历史检查点
- **分支实验**: 从检查点创建分支进行 A/B 测试

---

## 🎯 使用场景

### 场景 1: 网站配色设计

```
用户: "帮我设计一个科技感的网站配色方案"

AI 执行流程:
1. ANALYZE: 提取关键词(科技感、网站)、情绪(未来、创新)、场景(Web)
2. PROPOSE: 提供 2-3 种风格选项(赛博朋克、极简科技、未来主义)
3. COLOR: 基于三色原则提供配色方案
4. LAYOUT: 推荐构图法则和视觉层级
5. ELEMENTS: 列出需要的设计元素
6. AVOID: 提醒应避免的设计选择
```

### 场景 2: UI 组件设计

```
用户: "设计一个登录表单"

AI 执行流程:
1. ANALYZE: 提取关键词(登录、表单)、情绪(信任、简洁)、场景(App/Web)
2. PROPOSE: 提供风格选项(极简、Material、Glassmorphism)
3. COLOR: 提供配色方案
4. LAYOUT: 推荐表单布局
5. ELEMENTS: 列出必需元素(输入框、按钮、链接等)
6. AVOID: 提醒常见错误(信息过载、错误处理不当等)
```

### 场景 3: 游戏机制设计

```
用户: "设计一个休闲游戏的核心循环"

AI 执行流程:
1. ANALYZE: 提取关键词(休闲游戏、核心循环)、情绪(轻松、成就感)
2. PROPOSE: 提供游戏类型选项(消除、跑酷、放置)
3. COLOR: 提供配色方案
4. LAYOUT: 设计游戏界面布局
5. ELEMENTS: 列出游戏元素(UI、角色、道具等)
6. AVOID: 提醒设计陷阱(过度复杂、付费墙等)
```

---

## 📊 核心设计原则

### 设计的本质

设计是**解决问题**与**创造意义**的交汇点。

### 设计的层级

```
表现层(视觉) → 行为层(交互) → 框架层(结构) → 范围层(内容) → 战略层(目标)
```

### AI 辅助设计原则

- 设计时: 明确"不是什么"比"是什么"更重要
- 创意时: 风格混搭打破定式
- 迭代时: 每次只改一个变量

### 自由度分级

| 自由度 | 适用场景 | 控制方式 |
|--------|----------|----------|
| **高** | 创意方向、风格混搭 | 文字引导 + 启发式 |
| **中** | 配色方案、构图建议 | 模板 + 参数 + 约束 |
| **低** | TSV 数据查询、格式转换 | 脚本执行 |
| **固定** | 安全检查、格式验证 | 强制规则 |

---

## 🔗 相关资源

### 内部资源

- [Skill 设计文档](./doc/index.md) - 完整的 Skill 设计指南
- [技术规格](./specs/spec.md) - 项目技术规格
- [任务清单](./specs/tasks.md) - 开发任务清单
- [检查清单](./specs/checklist.md) - 质量检查清单

### 外部资源

- [Anthropic - Building Effective Agents](https://www.anthropic.com/research/building-effective-agents)
- [OpenAI - Building Agents](https://platform.openai.com/docs/guides/agents)
- [alirezarezvani/claude-skills](https://github.com/alirezarezvani/claude-skills)
- [anthropics/skills](https://github.com/anthropics/skills)

---

## 📝 更新日志

### v2.0 (2026-04-13)
- 系统化整合 doc 文件夹文档
- 创建完整的 README.md 融合所有内容
- 更新项目结构反映实际架构
- 添加快速开始指南和使用场景

---

## 🤝 贡献指南

如发现问题或有改进建议:
1. 检查对应的文档或代码
2. 提出具体的改进建议
3. 保持项目的简洁性和系统性

---

> **设计是破局,不是填空。规则是用来打破的,但首先你要知道规则是什么。**
