# 专业模块索引

本索引列出 design-everything skill 涵盖的所有专业模块。

---

## 模块分类

### Interface Design | 界面设计

| 模块 | slug | TSV 文件 | 关键词 |
|------|------|----------|--------|
| UI Components | `ui` | ui.tsv | ui, components, 按钮, 输入框, 卡片, 导航, 模态框, 表单 |
| UX Design | `ux` | ux.tsv | ux, 用户体验, 用户研究, 交互设计, 信息架构, 可用性, 设计原则 |

### Creative Design | 创意设计

| 模块 | slug | TSV 文件 | 关键词 |
|------|------|----------|--------|
| Game Design | `game` | game.tsv | game, 游戏设计, 游戏机制, 玩家心理, 数值设计, 心流, 游戏化 |
| Manga Storyboard | `manga_storyboard` | manga_storyboard.tsv | manga, 分镜, 漫画, 构图, 画格, 镜头语言, 叙事, 漫画分镜 |
| Video Editing | `video_editing` | video_editing.tsv | video, 剪辑, 蒙太奇, 节奏, 叙事, 镜头, 转场, 爱森斯坦 |
| Cinematography | `cinematography` | cinematography.tsv | cinematography, 镜头, 摄影, 构图, 光影, 运动, 景别, 角度 |
| Screenplay | `screenplay` | screenplay.tsv | screenplay, 剧本, 编剧, 人物, 对话, 叙事, 结构, 三幕式 |
| Character Pose | `pose` | pose.tsv | pose, 姿势, 人物, 动作, 动态, 人体, 重心, 运动 |
| Scene Design | `scene` | scene.tsv | scene, 场景, 环境, 氛围, 光影, 构图, 空间, 叙事 |
| Character Motion | `character_motion` | character_motion.tsv | motion, 动作, 动画, 运动, 表情, 迪士尼, 运动原理, 动画规律 |

---

## 数据库关系

```
┌─────────────┐       ┌─────────────┐       ┌─────────────┐
│ categories  │       │   modules   │       │ module_tags │
├─────────────┤       ├─────────────┤       ├─────────────┤
│ id (PK)     │←──┐   │ id (PK)     │←──┐   │ id (PK)     │
│ name        │   └───│ category_id │   └───│ module_id   │
│ slug        │       │ name        │       │ tag         │
│ description │       │ slug        │       └─────────────┘
└─────────────┘       │ keywords    │
                      │ tsv_file    │
                      │ item_count  │
                      └─────────────┘
```

---

## 模块详情

### UI Components (`ui`)

**内容规模**：~45 项

**核心主题**：
- 按钮设计（主要/次要/幽灵按钮）
- 输入框设计（文本框/选择器/开关）
- 卡片设计（信息卡/产品卡/用户卡）
- 导航设计（顶部导航/侧边导航/面包屑）
- 模态框设计
- 表单设计

**参考文档**：[references/ui.md](ui.md)

---

### UX Design (`ux`)

**内容规模**：~33 项

**核心主题**：
- 用户研究方法
- 可用性原则
- 交互设计模式
- 信息架构
- 设计原则（尼尔森可用性原则等）

**参考文档**：[references/ux.md](ux.md)

---

### Game Design (`game`)

**内容规模**：~34 项

**核心主题**：
- 游戏机制设计
- 心流理论
- 玩家心理
- 数值平衡
- 游戏化设计

**参考文档**：[references/game.md](game.md)

---

### Manga Storyboard (`manga_storyboard`)

**内容规模**：~33 项

**核心主题**：
- 分镜构图
- 画格设计
- 镜头语言
- 叙事技巧
- 视觉节奏

**参考文档**：[references/manga_storyboard.md](manga_storyboard.md)

---

### Video Editing (`video_editing`)

**内容规模**：~30 项

**核心主题**：
- 蒙太奇理论
- 剪辑手法
- 节奏控制
- 转场技巧
- 叙事剪辑

**参考文档**：[references/video_editing.md](video_editing.md)

---

### Cinematography (`cinematography`)

**内容规模**：~33 项

**核心主题**：
- 镜头语言
- 构图法则
- 光影设计
- 运动镜头
- 景别与角度

**参考文档**：[references/cinematography.md](cinematography.md)

---

### Screenplay (`screenplay`)

**内容规模**：~31 项

**核心主题**：
- 剧本结构（三幕式）
- 人物塑造
- 对话写作
- 叙事技巧
- 冲突设计

**参考文档**：[references/screenplay.md](screenplay.md)

---

### Character Pose (`pose`)

**内容规模**：~34 项

**核心主题**：
- 人体姿势
- 动态表现
- 重心原理
- 人体解剖
- 姿势设计

**参考文档**：[references/pose.md](pose.md)

---

### Scene Design (`scene`)

**内容规模**：~37 项

**核心主题**：
- 场景构成
- 光影氛围
- 空间设计
- 环境叙事
- 透视原理

**参考文档**：[references/scene.md](scene.md)

---

### Character Motion (`character_motion`)

**内容规模**：~41 项

**核心主题**：
- 动画原理（迪士尼12原则）
- 运动规律
- 表情动画
- 动作设计
- 时间控制

**参考文档**：[references/character_motion.md](character_motion.md)
