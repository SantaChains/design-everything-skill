
---
# rime

## 新绿匆匆
    name: "新绿匆匆"
    author: "简纯"
    # ==========================================
    # 基础背景层 - 深邃太空灰
    # ==========================================
    back_color: 0x1E2127
    border_color: 0x2D3139
    candidate_back_color: 0x1E2127
    candidate_border_color: 0x00000000
    candidate_shadow_color: 0x00000000
    # ==========================================
    # 阴影系统 - 柔和悬浮感
    # ==========================================
    shadow_color: 0x60000000
    hilited_shadow_color: 0x00000000
    hilited_candidate_shadow_color: 0x40000000
    # ==========================================
    # 输入编码区 - 极光蓝主题
    # ==========================================
    text_color: 0x89B4FA
    hilited_text_color: 0xA6D189
    hilited_back_color: 0x2A2E36
    hilited_mark_color: 0xF38BA8
    # ==========================================
    # 序号标签系统 - 层次分明
    # ==========================================
    label_color: 0x6C7086
    hilited_label_color: 0x1E1E2E
    hilited_candidate_label_color: 0x1E1E2E
    # ==========================================
    # 候选词列表 - 柔和护眼
    # ==========================================
    candidate_text_color: 0xCDD6F4
    comment_text_color: 0x6C7086
    # ==========================================
    # 选中候选词 - 极光渐变效果
    # ==========================================
    hilited_candidate_text_color: 0x1E1E2E
    hilited_comment_text_color: 0x45475A
    hilited_candidate_back_color: 0xA6E3A1
    hilited_candidate_border_color: 0x00000000
    # ==========================================
    # 翻页按钮 - 极光紫点缀
    # ==========================================
    prevpage_color: 0xCBA6F7
    nextpage_color: 0xCBA6F7

## 橘橙时
    name: "橙黄橘熟"
    author: "简纯"
    # ==========================================
    # 基础背景层 - 深邃太空灰
    # ==========================================
    back_color: 0x1E2127
    border_color: 0x3A3F47
    # 候选词背景
    candidate_back_color: 0x252830
    candidate_border_color: 0x2D3139
    candidate_shadow_color: 0x00000000
    # 悬停效果
    candidate_hover_back_color: 0x2D3139
    candidate_hover_text_color: 0xE8EDF3
    # ==========================================
    # 阴影系统 - 柔和悬浮感
    # ==========================================
    shadow_color: 0x40000000
    hilited_shadow_color: 0x00000000
    hilited_candidate_shadow_color: 0x30000000
    # ==========================================
    # 输入编码区 - 极光蓝主题
    # ==========================================
    text_color: 0x89B4FA
    hilited_text_color: 0xA6D189
    hilited_back_color: 0x2A2E36
    hilited_mark_color: 0xF38BA8
    # ==========================================
    # 序号标签系统 - 层次分明
    # ==========================================
    label_color: 0x5C6370
    hilited_label_color: 0x1E1E2E
    hilited_candidate_label_color: 0x1E2127
    # ==========================================
    # 候选词列表 - 柔和护眼
    # ==========================================
    candidate_text_color: 0xCDD6F4
    comment_text_color: 0x6C7086
    # ==========================================
    # 选中候选词 - 极光蓝高亮
    # ==========================================
    hilited_candidate_text_color: 0x1E2127
    hilited_comment_text_color: 0x3D4350
    hilited_candidate_back_color: 0x89B4FA
    hilited_candidate_border_color: 0xA6D189
    # ==========================================
    # 翻页按钮 - 极光紫点缀
    # ==========================================
    prevpage_color: 0xCBA6F7
    nextpage_color: 0xCBA6F7

---

# pwsh

## 配色配置接口

### Windows Terminal 颜色配置

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `background` | 终端背景色 | 主背景色 |
| `foreground` | 终端前景色 | 主文本色 |
| `cursorColor` | 光标颜色 | 高可见度色 |
| `selectionBackground` | 选中文本背景 | 半透明强调色 |
| `black` | ANSI黑色 | 基础色板 |
| `red` | ANSI红色 | 基础色板 |
| `green` | ANSI绿色 | 基础色板 |
| `yellow` | ANSI黄色 | 基础色板 |
| `blue` | ANSI蓝色 | 基础色板 |
| `purple` | ANSI紫色 | 基础色板 |
| `cyan` | ANSI青色 | 基础色板 |
| `white` | ANSI白色 | 基础色板 |
| `brightBlack` | ANSI亮黑色 | 亮色色板 |
| `brightRed` | ANSI亮红色 | 亮色色板 |
| `brightGreen` | ANSI亮绿色 | 亮色色板 |
| `brightYellow` | ANSI亮黄色 | 亮色色板 |
| `brightBlue` | ANSI亮蓝色 | 亮色色板 |
| `brightPurple` | ANSI亮紫色 | 亮色色板 |
| `brightCyan` | ANSI亮青色 | 亮色色板 |
| `brightWhite` | ANSI亮白色 | 亮色色板 |

### PowerShell 配置文件配置

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `$Host.UI.RawUI.BackgroundColor` | 控制台背景色 | 主背景色 |
| `$Host.UI.RawUI.ForegroundColor` | 控制台前景色 | 主文本色 |
| `$Host.UI.RawUI.CursorSize` | 光标大小 | 可见度调整 |
| `$PSStyle.Background.Black` | 背景黑色ANSI码 | 转义序列 |
| `$PSStyle.Background.Red` | 背景红色ANSI码 | 转义序列 |
| `$PSStyle.Background.Green` | 背景绿色ANSI码 | 转义序列 |
| `$PSStyle.Background.Yellow` | 背景黄色ANSI码 | 转义序列 |
| `$PSStyle.Background.Blue` | 背景蓝色ANSI码 | 转义序列 |
| `$PSStyle.Background.Magenta` | 背景洋红ANSI码 | 转义序列 |
| `$PSStyle.Background.Cyan` | 背景青色ANSI码 | 转义序列 |
| `$PSStyle.Background.White` | 背景白色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.Black` | 前景黑色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.Red` | 前景红色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.Green` | 前景绿色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.Yellow` | 前景黄色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.Blue` | 前景蓝色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.Magenta` | 前景洋红ANSI码 | 转义序列 |
| `$PSStyle.Foreground.Cyan` | 前景青色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.White` | 前景白色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.BrightBlack` | 前景亮黑色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.BrightRed` | 前景亮红色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.BrightGreen` | 前景亮绿色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.BrightYellow` | 前景亮黄色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.BrightBlue` | 前景亮蓝色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.BrightMagenta` | 前景亮洋红ANSI码 | 转义序列 |
| `$PSStyle.Foreground.BrightCyan` | 前景亮青色ANSI码 | 转义序列 |
| `$PSStyle.Foreground.BrightWhite` | 前景亮白色ANSI码 | 转义序列 |
| `$PSStyle.Reset` | 重置样式ANSI码 | 转义序列 |
| `$PSStyle.Blink` | 闪烁效果 | 文本样式 |
| `$PSStyle.Bold` | 粗体效果 | 文本样式 |
| `$PSStyle.Hidden` | 隐藏文本 | 文本样式 |
| `$PSStyle.Reverse` | 反色效果 | 文本样式 |
| `$PSStyle.Italic` | 斜体效果 | 文本样式 |
| `$PSStyle.Underline` | 下划线效果 | 文本样式 |
| `$PSStyle.Strikethrough` | 删除线效果 | 文本样式 |

### Windows Terminal 高级配置

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `colorScheme` | 引用配色方案名称 | 主题引用 |
| `useAcrylic` | 启用亚克力效果 | 透明效果 |
| `acrylicOpacity` | 亚克力不透明度 | 透明程度 |
| `backgroundImage` | 背景图片路径 | 图片背景 |
| `backgroundImageOpacity` | 背景图片不透明度 | 图片透明 |
| `backgroundImageStretchMode` | 背景图片拉伸模式 | 图片适配 |
| `backgroundImageAlignment` | 背景图片对齐 | 图片位置 |
| `fontFace` | 字体名称 | 字体设置 |
| `fontSize` | 字体大小 | 字体设置 |
| `fontWeight` | 字体粗细 | 字体设置 |
| `padding` | 内边距 | 布局间距 |
| `antialiasingMode` | 抗锯齿模式 | 渲染质量 |
| `cursorShape` | 光标形状 | 光标样式 |
| `cursorHeight` | 光标高度 | 光标样式 |
| `altGrAliasing` | AltGr键别名 | 键盘设置 |

### 颜色方案定义 (settings.json)

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `name` | 配色方案名称 | 标识符 |
| `colors` | 颜色数组 | 色板定义 |
| `colors[0]` | 黑色索引 | ANSI 0 |
| `colors[1]` | 红色索引 | ANSI 1 |
| `colors[2]` | 绿色索引 | ANSI 2 |
| `colors[3]` | 黄色索引 | ANSI 3 |
| `colors[4]` | 蓝色索引 | ANSI 4 |
| `colors[5]` | 紫色索引 | ANSI 5 |
| `colors[6]` | 青色索引 | ANSI 6 |
| `colors[7]` | 白色索引 | ANSI 7 |
| `colors[8]` | 亮黑色索引 | ANSI 8 |
| `colors[9]` | 亮红色索引 | ANSI 9 |
| `colors[10]` | 亮绿色索引 | ANSI 10 |
| `colors[11]` | 亮黄色索引 | ANSI 11 |
| `colors[12]` | 亮蓝色索引 | ANSI 12 |
| `colors[13]` | 亮紫色索引 | ANSI 13 |
| `colors[14]` | 亮青色索引 | ANSI 14 |
| `colors[15]` | 亮白色索引 | ANSI 15 |

### PowerShell 提示符颜色 (oh-my-posh等)

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `segment.background` | 段背景色 | 分段背景 |
| `segment.foreground` | 段前景色 | 分段文本 |
| `segment.leading_diamond` | 前导钻石符 | 装饰符号 |
| `segment.trailing_diamond` | 尾随钻石符 | 装饰符号 |
| `segment.template` | 显示模板 | 内容格式 |
| `segment.type` | 段类型 | 功能标识 |
| `palette` | 调色板定义 | 颜色变量 |
| `palette.<name>` | 自定义颜色变量 | 可复用色值 |
| `transient_prompt` | 临时提示符 | 简化显示 |
| `secondary_prompt` | 二级提示符 | 多行输入 |
| `valid_line` | 有效行样式 | 命令验证 |
| `error_line` | 错误行样式 | 错误提示 |

---
---

# nvim

## 配色配置接口

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `colorscheme <name>` | 加载指定配色方案 | 整体主题色调 |
| `highlight Normal` | 普通文本高亮组 | 编辑器主背景、前景色 |
| `highlight Comment` | 注释高亮组 | 柔和灰调，低饱和度 |
| `highlight Keyword` | 关键字高亮组 | 高饱和强调色 |
| `highlight String` | 字符串高亮组 | 暖色调 |
| `highlight Function` | 函数名高亮组 | 冷色调 |
| `highlight Identifier` | 标识符高亮组 | 中性色 |
| `highlight Statement` | 语句高亮组 | 醒目强调色 |
| `highlight Type` | 类型高亮组 | 区分度高的色调 |
| `guifg` | GUI前景色属性 | 文字颜色 |
| `guibg` | GUI背景色属性 | 背景颜色 |
| `gui` | 样式属性(bold/italic/underline) | 视觉强调 |
| `ctermfg` | 终端前景色 | 兼容终端的颜色索引 |
| `ctermbg` | 终端背景色 | 兼容终端的颜色索引 |
| `on_colors` | 颜色回调函数 | 自定义调色板 |
| `on_highlights` | 高亮组回调函数 | 细粒度样式控制 |

---


# vscode

## 配色配置接口

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `workbench.colorCustomizations` | 工作区UI颜色自定义 | 界面框架色调 |
| `editor.foreground` | 编辑器默认文字颜色 | 主文本色 |
| `editor.background` | 编辑器背景色 | 主背景色 |
| `editor.lineHighlightBackground` | 当前行高亮背景 | 轻微对比色 |
| `editorCursor.foreground` | 光标颜色 | 高可见度色 |
| `editor.selectionBackground` | 选中文本背景 | 半透明强调色 |
| `sideBar.background` | 侧边栏背景 | 与主背景区分 |
| `sideBar.foreground` | 侧边栏文字 | 次要文本色 |
| `statusBar.background` | 状态栏背景 | 主题强调色 |
| `activityBar.background` | 活动栏背景 | 深色或对比色 |
| `terminal.ansiBlack` | 终端黑色 | ANSI色板基础 |
| `terminal.ansiRed` | 终端红色 | 错误/警告 |
| `terminal.ansiGreen` | 终端绿色 | 成功/正常 |
| `terminal.ansiYellow` | 终端黄色 | 提示/注意 |
| `terminal.ansiBlue` | 终端蓝色 | 信息/链接 |
| `terminal.ansiMagenta` | 终端洋红 | 特殊标记 |
| `terminal.ansiCyan` | 终端青色 | 辅助信息 |
| `terminal.ansiWhite` | 终端白色 | 高亮文本 |
| `editor.tokenColorCustomizations` | 语法标记颜色自定义 | 代码语义着色 |
| `editor.tokenColorCustomizations.keywords` | 关键字颜色 | 语法核心强调 |
| `editor.tokenColorCustomizations.strings` | 字符串颜色 | 数据值色调 |
| `editor.tokenColorCustomizations.comments` | 注释颜色 | 淡化辅助色 |
| `editor.tokenColorCustomizations.numbers` | 数字颜色 | 常量值色调 |
| `editor.tokenColorCustomizations.types` | 类型颜色 | 类型定义强调 |
| `editor.tokenColorCustomizations.functions` | 函数颜色 | 函数调用标识 |
| `editor.tokenColorCustomizations.variables` | 变量颜色 | 变量引用标识 |
| `editor.tokenColorCustomizations.textMateRules` | TextMate规则数组 | 精确scope控制 |
| `textMateRules[].scope` | 作用域选择器 | 语法元素匹配 |
| `textMateRules[].settings.foreground` | 前景色 | 文字颜色 |
| `textMateRules[].settings.fontStyle` | 字体样式 | bold/italic/underline |

### textMateRules 常用scope

| scope | 说明 | 颜色方向 |
|-------|------|----------|
| `comment` / `comment.line` / `comment.block` | 注释 | 低饱和灰调 |
| `string` / `string.quoted` / `string.template` | 字符串 | 暖色调 |
| `keyword` / `keyword.control` / `keyword.operator` | 关键字/运算符 | 高饱和强调 |
| `storage.type` / `storage.modifier` | 存储类型/修饰符 | 类型相关色 |
| `entity.name.function` / `entity.name.method` | 函数/方法名 | 冷色调 |
| `entity.name.class` / `entity.name.type` | 类名/类型名 | 区分度高的色 |
| `entity.name.tag` | HTML/XML标签 | 标记语言色 |
| `entity.other.attribute-name` | 属性名 | 辅助色 |
| `variable` / `variable.language` / `variable.parameter` | 变量 | 中性色 |
| `constant` / `constant.numeric` / `constant.language` | 常量 | 常量值色调 |
| `support.function` / `support.class` / `support.type` | 内置支持 | 库/框架标识色 |
| `punctuation` / `punctuation.definition` | 标点符号 | 弱化色 |
| `meta.function` / `meta.class` / `meta.method` | 元信息 | 结构标识 |

---

# obsidian

## 配色配置接口

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--background-primary` | 主背景色 | 编辑器主色调 |
| `--background-primary-alt` | 主背景替代色 | 轻微变化区分 |
| `--background-secondary` | 次背景色 | 侧边栏/面板 |
| `--background-secondary-alt` | 次背景替代色 | 悬浮/激活态 |
| `--background-modifier-border` | 边框修饰背景 | 分隔线色调 |
| `--background-modifier-form-field` | 表单字段背景 | 输入区域 |
| `--text-normal` | 普通文本色 | 主文本高对比 |
| `--text-muted` | 淡化文本色 | 次要信息 |
| `--text-faint` | 微弱文本色 | 提示/占位符 |
| `--text-accent` | 强调文本色 | 链接/交互 |
| `--text-accent-hover` | 强调悬停色 | 交互反馈 |
| `--text-error` | 错误文本色 | 警告提示 |
| `--text-success` | 成功文本色 | 确认状态 |
| `--h1-color` | H1标题颜色 | 最高层级强调 |
| `--h2-color` | H2标题颜色 | 次级强调 |
| `--h3-color` | H3标题颜色 | 中等强调 |
| `--h4-color` | H4标题颜色 | 轻微强调 |
| `--h5-color` | H5标题颜色 | 淡化强调 |
| `--h6-color` | H6标题颜色 | 最淡强调 |
| `--code-background` | 代码块背景 | 深色或浅色区分 |
| `--code-normal` | 代码普通文本 | 代码主色调 |
| `--code-comment` | 代码注释 | 低饱和灰调 |
| `--code-keyword` | 代码关键字 | 语法强调 |
| `--code-string` | 代码字符串 | 数据值色调 |
| `--code-function` | 代码函数 | 调用标识 |
| `--code-number` | 代码数字 | 常量值 |
| `--code-operator` | 代码运算符 | 逻辑符号 |
| `--blockquote-border-color` | 引用边框色 | 左侧强调线 |
| `--blockquote-background` | 引用背景色 | 轻微区分 |
| `--callout-default` | 默认标注色 | 信息提示 |
| `--callout-info` | 信息标注色 | 蓝色系 |
| `--callout-warning` | 警告标注色 | 黄色/橙色系 |
| `--callout-error` | 错误标注色 | 红色系 |
| `--callout-success` | 成功标注色 | 绿色系 |
| `--interactive-accent` | 交互强调色 | 按钮/开关 |
| `--interactive-accent-hover` | 交互悬停色 | 悬停反馈 |
| `--scrollbar-bg` | 滚动条背景 | 轨道色调 |
| `--scrollbar-thumb-bg` | 滚动条滑块 | 可拖动标识 |
| `--graph-node` | 图谱节点 | 节点主色 |
| `--graph-line` | 图谱连线 | 关系线色 |
| `--graph-text` | 图谱文字 | 标签颜色 |
| `--canvas-background` | 画布背景 | 白板底色 |
| `--canvas-card` | 画布卡片 | 卡片背景 |
| `--canvas-line` | 画布连线 | 连接线色 |

### 背景修饰类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--background-modifier-hover` | 悬停状态背景 | 轻微提亮/变暗 |
| `--background-modifier-active` | 激活状态背景 | 中等强调色 |
| `--background-modifier-selected` | 选中状态背景 | 高亮强调色 |
| `--background-modifier-error` | 错误状态背景 | 红色系淡化 |
| `--background-modifier-success` | 成功状态背景 | 绿色系淡化 |
| `--background-modifier-message` | 消息提示背景 | 信息提示色 |

### 文本修饰类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--text-highlight-bg` | 文本高亮背景 | 荧光黄/标记色 |
| `--text-selection` | 选中文本色 | 半透明强调色 |
| `--text-on-accent` | 强调色上的文字 | 高对比白/黑 |
| `--text-warning` | 警告文本色 | 黄色/橙色系 |
| `--text-on-selection` | 选中区域文字色 | 高对比反色 |

### 链接类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--link-color` | 链接颜色 | 主题强调色 |
| `--link-color-hover` | 链接悬停色 | 高亮强调色 |
| `--link-decoration` | 链接装饰线 | 下划线/无 |
| `--link-decoration-hover` | 链接悬停装饰 | 下划线强调 |
| `--link-unresolved-color` | 未解析链接色 | 淡化强调色 |
| `--link-unresolved-decoration` | 未解析链接装饰 | 虚线/特殊标记 |
| `--link-external-color` | 外部链接色 | 区分于内部链接 |
| `--link-external-color-hover` | 外部链接悬停 | 高亮外部链接 |
| `--link-external-decoration` | 外部链接装饰 | 特殊标记 |

### 标签类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--tag-color` | 标签文字色 | 主题强调色 |
| `--tag-background` | 标签背景色 | 淡化强调色 |
| `--tag-color-hover` | 标签悬停文字 | 高亮强调色 |
| `--tag-background-hover` | 标签悬停背景 | 中等强调色 |
| `--tag-border-color` | 标签边框色 | 轻微强调色 |
| `--tag-border-width` | 标签边框宽度 | 细边框 |
| `--tag-padding-x` | 标签水平内边距 | 紧凑间距 |
| `--tag-padding-y` | 标签垂直内边距 | 紧凑间距 |
| `--tag-radius` | 标签圆角 | 小圆角 |

### 导航类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--nav-item-color` | 导航项文字色 | 次要文本色 |
| `--nav-item-color-hover` | 导航项悬停文字 | 主文本色 |
| `--nav-item-color-active` | 导航项激活文字 | 强调色 |
| `--nav-item-color-selected` | 导航项选中文字 | 高对比强调 |
| `--nav-item-background-hover` | 导航项悬停背景 | 轻微背景变化 |
| `--nav-item-background-active` | 导航项激活背景 | 中等背景强调 |
| `--nav-item-background-selected` | 导航项选中背景 | 高亮背景强调 |
| `--nav-indentation-guide-color` | 导航缩进线色 | 极淡分隔线 |
| `--nav-collapse-icon-color` | 折叠图标色 | 次要图标色 |
| `--nav-collapse-icon-color-collapsed` | 折叠后图标色 | 强调图标色 |

### 标签页类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--tab-text-color` | 标签页文字色 | 次要文本色 |
| `--tab-text-color-active` | 激活标签文字 | 主文本色 |
| `--tab-text-color-focused` | 聚焦标签文字 | 主文本强调 |
| `--tab-text-color-hover` | 悬停标签文字 | 主文本色 |
| `--tab-background` | 标签页背景 | 次背景色 |
| `--tab-background-active` | 激活标签背景 | 主背景色 |
| `--tab-background-hover` | 悬停标签背景 | 次背景变化 |
| `--tab-divider-color` | 标签分隔线色 | 边框色 |
| `--tab-outline-color` | 标签轮廓色 | 聚焦轮廓 |
| `--tab-stacked-shadow` | 堆叠标签阴影 | 深度阴影 |

### 列表类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--list-marker-color` | 列表标记色 | 强调色 |
| `--list-marker-color-collapsed` | 折叠列表标记 | 高亮强调色 |
| `--list-bullet-color` | 无序列表点色 | 次要强调色 |
| `--list-number-color` | 有序列表数字色 | 次要强调色 |
| `--list-indentation-guide-color` | 列表缩进线色 | 极淡分隔线 |
| `--list-indentation-guide-color-active` | 激活缩进线色 | 中等分隔线 |

### 表格类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--table-background` | 表格背景色 | 主背景色 |
| `--table-border-color` | 表格边框色 | 边框修饰色 |
| `--table-header-background` | 表头背景色 | 次背景强调 |
| `--table-header-background-hover` | 表头悬停背景 | 中等背景变化 |
| `--table-row-even-background` | 偶数行背景 | 轻微背景变化 |
| `--table-row-odd-background` | 奇数行背景 | 主背景色 |
| `--table-row-background-hover` | 行悬停背景 | 中等背景变化 |
| `--table-cell-padding-x` | 单元格水平内边距 | 紧凑间距 |
| `--table-cell-padding-y` | 单元格垂直内边距 | 紧凑间距 |

### 模态框/弹窗类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--modal-background` | 模态框背景 | 主背景色 |
| `--modal-border-color` | 模态框边框 | 边框修饰色 |
| `--modal-border-width` | 模态框边框宽 | 细边框 |
| `--modal-border-radius` | 模态框圆角 | 中等圆角 |
| `--modal-shadow` | 模态框阴影 | 深度阴影 |
| `--modal-width` | 模态框宽度 | 固定宽度 |
| `--modal-height` | 模态框高度 | 自适应高度 |
| `--prompt-background` | 命令面板背景 | 主背景色 |
| `--prompt-border-color` | 命令面板边框 | 边框修饰色 |
| `--prompt-shadow` | 命令面板阴影 | 悬浮阴影 |
| `--tooltip-background` | 提示框背景 | 次背景强调 |
| `--tooltip-color` | 提示框文字 | 主文本色 |
| `--tooltip-border-color` | 提示框边框 | 边框修饰色 |
| `--tooltip-border-width` | 提示框边框宽 | 细边框 |
| `--tooltip-radius` | 提示框圆角 | 小圆角 |

### Ribbon工具栏类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--ribbon-background` | Ribbon背景色 | 次背景色 |
| `--ribbon-background-collapsed` | 折叠Ribbon背景 | 次背景强调 |
| `--ribbon-width` | Ribbon宽度 | 固定窄宽 |
| `--ribbon-padding` | Ribbon内边距 | 紧凑间距 |
| `--ribbon-icon-color` | Ribbon图标色 | 次要图标色 |
| `--ribbon-icon-color-hover` | Ribbon图标悬停 | 主图标色 |
| `--ribbon-icon-color-active` | Ribbon图标激活 | 强调图标色 |
| `--ribbon-icon-size` | Ribbon图标大小 | 标准图标 |
| `--ribbon-icon-stroke-width` | Ribbon图标线宽 | 细线宽 |

### 搜索类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--search-result-background` | 搜索结果背景 | 主背景色 |
| `--search-result-background-hover` | 搜索结果悬停 | 次背景变化 |
| `--search-result-foreground` | 搜索结果文字 | 主文本色 |
| `--search-result-highlight-bg` | 搜索高亮背景 | 荧光标记色 |
| `--search-result-highlight-fg` | 搜索高亮文字 | 高对比色 |
| `--search-icon-color` | 搜索图标色 | 次要图标色 |
| `--search-clear-button-color` | 清除按钮色 | 次要图标色 |
| `--search-clear-button-color-hover` | 清除按钮悬停 | 警告色 |

### 状态/反馈类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--status-bar-background` | 状态栏背景 | 次背景色 |
| `--status-bar-foreground` | 状态栏文字 | 次要文本色 |
| `--status-bar-border-color` | 状态栏边框 | 边框修饰色 |
| `--status-bar-item-color` | 状态栏项颜色 | 次要文本色 |
| `--status-bar-item-color-hover` | 状态栏项悬停 | 主文本色 |
| `--status-bar-item-background-hover` | 状态栏项悬停背景 | 次背景变化 |
| `--notice-background` | 通知背景 | 次背景强调 |
| `--notice-color` | 通知文字 | 主文本色 |
| `--notice-border-color` | 通知边框 | 边框修饰色 |
| `--notice-border-radius` | 通知圆角 | 中等圆角 |
| `--notice-shadow` | 通知阴影 | 悬浮阴影 |

### 输入/表单类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--input-background` | 输入框背景 | 次背景色 |
| `--input-foreground` | 输入框文字 | 主文本色 |
| `--input-border-color` | 输入框边框 | 边框修饰色 |
| `--input-border-width` | 输入框边框宽 | 细边框 |
| `--input-border-radius` | 输入框圆角 | 小圆角 |
| `--input-shadow` | 输入框阴影 | 内阴影 |
| `--input-shadow-hover` | 输入框悬停阴影 | 外阴影 |
| `--input-shadow-focus` | 输入框聚焦阴影 | 强调阴影 |
| `--input-placeholder-color` | 占位符颜色 | 极淡文本色 |
| `--input-disabled-background` | 禁用输入背景 | 次背景淡化 |
| `--input-disabled-foreground` | 禁用输入文字 | 极淡文本色 |
| `--input-disabled-border-color` | 禁用输入边框 | 边框淡化 |
| `--checkbox-color` | 复选框颜色 | 强调色 |
| `--checkbox-color-hover` | 复选框悬停 | 高亮强调色 |
| `--checkbox-border-color` | 复选框边框 | 边框修饰色 |
| `--checkbox-border-color-hover` | 复选框悬停边框 | 强调边框色 |
| `--slider-thumb-border-color` | 滑块边框色 | 边框修饰色 |
| `--slider-track-background` | 滑块轨道背景 | 次背景色 |

### 菜单类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--menu-background` | 菜单背景 | 主背景色 |
| `--menu-foreground` | 菜单文字 | 主文本色 |
| `--menu-border-color` | 菜单边框 | 边框修饰色 |
| `--menu-border-width` | 菜单边框宽 | 细边框 |
| `--menu-border-radius` | 菜单圆角 | 中等圆角 |
| `--menu-shadow` | 菜单阴影 | 悬浮阴影 |
| `--menu-item-color` | 菜单项颜色 | 主文本色 |
| `--menu-item-color-hover` | 菜单项悬停文字 | 主文本强调 |
| `--menu-item-color-disabled` | 菜单项禁用文字 | 极淡文本色 |
| `--menu-item-background-hover` | 菜单项悬停背景 | 次背景变化 |
| `--menu-item-background-active` | 菜单项激活背景 | 中等背景强调 |
| `--menu-separator-color` | 菜单分隔线色 | 边框修饰色 |
| `--menu-icon-color` | 菜单图标色 | 次要图标色 |
| `--menu-icon-color-hover` | 菜单图标悬停 | 主图标色 |

### 分割线类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--divider-color` | 分割线颜色 | 边框修饰色 |
| `--divider-color-hover` | 分割线悬停色 | 中等边框色 |
| `--divider-width` | 分割线宽度 | 细线宽 |
| `--divider-vertical-width` | 垂直分割线宽 | 细线宽 |
| `--divider-horizontal-height` | 水平分割线高 | 细线高 |

### 阴影类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--shadow-color` | 阴影颜色 | 半透明黑色 |
| `--shadow-color-ambient` | 环境阴影 | 极淡阴影 |
| `--shadow-color-umbra` | 本影颜色 | 中等阴影 |
| `--shadow-color-penumbra` | 半影颜色 | 轻微阴影 |
| `--shadow-s` | 小阴影 | 轻微悬浮 |
| `--shadow-m` | 中等阴影 | 标准悬浮 |
| `--shadow-l` | 大阴影 | 深度悬浮 |
| `--shadow-xl` | 超大阴影 | 模态阴影 |

### 字体类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--font-interface` | 界面字体 | 无衬线字体 |
| `--font-text` | 正文字体 | 阅读字体 |
| `--font-monospace` | 等宽字体 | 代码字体 |
| `--font-heading` | 标题字体 | 强调字体 |
| `--font-ui-smaller` | 超小UI字体 | 极小号 |
| `--font-ui-small` | 小UI字体 | 小号 |
| `--font-ui-medium` | 中UI字体 | 中号 |
| `--font-ui-large` | 大UI字体 | 大号 |
| `--font-text-size` | 正文字号 | 标准字号 |
| `--font-line-height` | 行高 | 标准行高 |
| `--font-heading-size` | 标题字号 | 层级递减 |

### 圆角类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--radius-s` | 小圆角 | 轻微圆角 |
| `--radius-m` | 中圆角 | 标准圆角 |
| `--radius-l` | 大圆角 | 明显圆角 |
| `--radius-xl` | 超大圆角 | 卡片圆角 |

### 间距类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--size-0` | 零间距 | 无间距 |
| `--size-1` | 超小间距 | 4px |
| `--size-2` | 小间距 | 8px |
| `--size-3` | 中间距 | 12px |
| `--size-4` | 标准间距 | 16px |
| `--size-5` | 大间距 | 20px |
| `--size-6` | 超大间距 | 24px |
| `--size-7` | 特大间距 | 32px |
| `--size-8` | 巨大间距 | 48px |

### 动画/过渡类

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `--anim-duration-none` | 无动画 | 即时 |
| `--anim-duration-fast` | 快速动画 | 100ms |
| `--anim-duration-moderate` | 中速动画 | 200ms |
| `--anim-duration-slow` | 慢速动画 | 300ms |
| `--anim-motion-smooth` | 平滑过渡 | ease |
| `--anim-motion-delay` | 延迟过渡 | ease-out |
| `--anim-motion-jumpy` | 跳跃过渡 | cubic-bezier |
| `--anim-motion-swing` | 摆动过渡 | cubic-bezier |


---

# IDEA

## 配色配置接口

### 编辑器颜色

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `CARET_ROW_COLOR` | 当前行背景色 | 轻微高亮 |
| `CARET_COLOR` | 光标颜色 | 高可见度 |
| `SELECTION_BACKGROUND` | 选中文本背景 | 半透明强调色 |
| `SELECTION_FOREGROUND` | 选中文本前景 | 高对比色 |
| `LINE_NUMBERS_COLOR` | 行号颜色 | 次要文本色 |
| `LINE_NUMBERS_BACKGROUND` | 行号背景 | 与编辑器区分 |
| `GUTTER_BACKGROUND` | 装订线背景 | 次背景色 |
| `RIGHT_MARGIN_COLOR` | 右边距线色 | 淡分隔线 |
| `WHITESPACES` | 空白字符颜色 | 极淡提示色 |
| `INDENT_GUIDE` | 缩进引导线 | 淡分隔线 |
| `SELECTED_INDENT_GUIDE` | 选中缩进线 | 中等强调 |

### 代码高亮 - 基础

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `TEXT` | 普通文本 | 主文本色 |
| `DEFAULT_IDENTIFIER` | 默认标识符 | 中性色 |
| `DEFAULT_CONSTANT` | 常量 | 强调色 |
| `DEFAULT_KEYWORD` | 关键字 | 高饱和强调 |
| `DEFAULT_STRING` | 字符串 | 暖色调 |
| `DEFAULT_NUMBER` | 数字 | 常量值色调 |
| `DEFAULT_OPERATION_SIGN` | 运算符 | 次要强调 |
| `DEFAULT_BRACES` | 括号 | 结构标识 |
| `DEFAULT_DOT` | 点号 | 次要符号 |
| `DEFAULT_SEMICOLON` | 分号 | 次要符号 |
| `DEFAULT_COMMA` | 逗号 | 次要符号 |

### 代码高亮 - 语义

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `DEFAULT_CLASS_NAME` | 类名 | 冷色调强调 |
| `DEFAULT_INTERFACE_NAME` | 接口名 | 区分度高的色 |
| `DEFAULT_ENUM_NAME` | 枚举名 | 类型相关色 |
| `DEFAULT_FUNCTION_DECLARATION` | 函数声明 | 函数标识 |
| `DEFAULT_FUNCTION_CALL` | 函数调用 | 函数标识 |
| `DEFAULT_PARAMETER` | 参数 | 中性色 |
| `DEFAULT_LOCAL_VARIABLE` | 局部变量 | 中性色 |
| `DEFAULT_INSTANCE_FIELD` | 实例字段 | 成员标识 |
| `DEFAULT_STATIC_FIELD` | 静态字段 | 静态标识 |
| `DEFAULT_INSTANCE_METHOD` | 实例方法 | 方法标识 |
| `DEFAULT_STATIC_METHOD` | 静态方法 | 静态方法标识 |
| `DEFAULT_ABSTRACT_METHOD` | 抽象方法 | 抽象标识 |
| `DEFAULT_INHERITED_METHOD` | 继承方法 | 继承标识 |
| `DEFAULT_DOC_COMMENT` | 文档注释 | 低饱和绿调 |
| `DEFAULT_BLOCK_COMMENT` | 块注释 | 低饱和灰调 |
| `DEFAULT_LINE_COMMENT` | 行注释 | 低饱和灰调 |

### UI 界面颜色

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `Panel.background` | 面板背景 | 次背景色 |
| `Panel.foreground` | 面板前景 | 主文本色 |
| `Tree.background` | 树形控件背景 | 次背景色 |
| `Tree.foreground` | 树形控件前景 | 主文本色 |
| `Tree.selectionBackground` | 树选中背景 | 强调背景 |
| `Tree.selectionForeground` | 树选中前景 | 高对比色 |
| `List.background` | 列表背景 | 次背景色 |
| `List.selectionBackground` | 列表选中背景 | 强调背景 |
| `Table.background` | 表格背景 | 次背景色 |
| `Table.gridColor` | 表格网格线 | 淡分隔线 |
| `Table.selectionBackground` | 表格选中背景 | 强调背景 |

### 工具窗口

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `ToolWindow.Header.background` | 工具窗口标题栏 | 次背景强调 |
| `ToolWindow.Header.inactiveBackground` | 非激活标题栏 | 淡化背景 |
| `ToolWindow.Button.selectedBackground` | 按钮选中背景 | 中等强调 |
| `ToolWindow.Button.selectedForeground` | 按钮选中前景 | 高对比色 |
| `SidePanel.background` | 侧边面板背景 | 次背景色 |
| `StatusBar.background` | 状态栏背景 | 次背景强调 |
| `StatusBar.foreground` | 状态栏前景 | 次要文本色 |
| `StatusBar.border` | 状态栏边框 | 淡分隔线 |

### 编辑器组件

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `ScrollBar.background` | 滚动条背景 | 轨道色 |
| `ScrollBar.thumbColor` | 滚动条滑块 | 可拖动标识 |
| `ScrollBar.thumbBorderColor` | 滑块边框 | 边框修饰 |
| `EditorTabs.background` | 标签页背景 | 次背景色 |
| `EditorTabs.underlinedTabBackground` | 下划线标签背景 | 激活态背景 |
| `EditorTabs.underlinedTabForeground` | 下划线标签前景 | 激活态前景 |
| `EditorTabs.inactiveMaskColor` | 非激活标签遮罩 | 淡化效果 |
| `Borders.color` | 通用边框色 | 边框修饰 |
| `Borders.ContrastBorderColor` | 对比边框色 | 强调边框 |

### 搜索与导航

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `SearchEverywhere.Header.background` | 随处搜索标题 | 次背景强调 |
| `SearchEverywhere.SearchField.background` | 搜索框背景 | 输入区域 |
| `SearchEverywhere.List.background` | 搜索结果背景 | 主背景色 |
| `SearchEverywhere.List.selectionBackground` | 结果选中背景 | 强调背景 |
| `CompletionPopup.background` | 代码补全背景 | 主背景色 |
| `CompletionPopup.selectionBackground` | 补全选中背景 | 强调背景 |
| `Lookup.background` | 查找弹出背景 | 主背景色 |
| `Lookup.selectionBackground` | 查找选中背景 | 强调背景 |
| `HintPanel.background` | 提示面板背景 | 次背景强调 |
| `HintPanel.infoForeground` | 提示信息前景 | 信息色 |
| `HintPanel.errorForeground` | 提示错误前景 | 错误色 |
| `HintPanel.warningForeground` | 提示警告前景 | 警告色 |

### 版本控制

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `FileStatus.added` | 已添加文件 | 绿色系 |
| `FileStatus.modified` | 已修改文件 | 蓝色系 |
| `FileStatus.deleted` | 已删除文件 | 红色系 |
| `FileStatus.renamed` | 已重命名文件 | 紫色系 |
| `FileStatus.notChanged` | 未更改文件 | 中性色 |
| `FileStatus.ignored` | 已忽略文件 | 极淡灰调 |
| `FileStatus.unknown` | 未知文件 | 橙色系 |
| `VCS_ANNOTATIONS_COLOR_1` | 版本注释颜色1 | 时间梯度 |
| `VCS_ANNOTATIONS_COLOR_2` | 版本注释颜色2 | 时间梯度 |
| `VCS_ANNOTATIONS_COLOR_3` | 版本注释颜色3 | 时间梯度 |
| `VCS_ANNOTATIONS_COLOR_4` | 版本注释颜色4 | 时间梯度 |
| `VCS_ANNOTATIONS_COLOR_5` | 版本注释颜色5 | 时间梯度 |

### 调试与运行

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `EXECUTIONPOINT_ATTRIBUTES` | 执行点(当前行) | 高亮强调 |
| `BREAKPOINT_ATTRIBUTES` | 断点标记 | 红色系强调 |
| `NOT_TOP_FRAME_ATTRIBUTES` | 非顶层帧 | 淡化强调 |
| `INLINE_BREAKPOINT_ATTRIBUTES` | 内联断点 | 断点变体 |
| `DIFF_SEPARATORS_BACKGROUND` | 差异分隔背景 | 对比区分 |
| `DIFF_SEPARATORS_TOP_BORDER` | 差异顶部边框 | 分隔标识 |
| `DIFF_SEPARATORS_BOTTOM_BORDER` | 差异底部边框 | 分隔标识 |

### 通知与提示

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `Notification.background` | 通知背景 | 次背景强调 |
| `Notification.foreground` | 通知前景 | 主文本色 |
| `Notification.borderColor` | 通知边框 | 边框修饰 |
| `Notification.errorBackground` | 错误通知背景 | 红色系淡化 |
| `Notification.errorForeground` | 错误通知前景 | 错误强调 |
| `Notification.warningBackground` | 警告通知背景 | 黄色系淡化 |
| `Notification.warningForeground` | 警告通知前景 | 警告强调 |
| `Notification.infoBackground` | 信息通知背景 | 蓝色系淡化 |
| `Notification.infoForeground` | 信息通知前景 | 信息强调 |
| `Notification.ToolWindow.informative` | 信息性通知 | 信息色 |
| `Notification.ToolWindow.error` | 错误通知 | 错误色 |
| `Notification.ToolWindow.warning` | 警告通知 | 警告色 |

### 代码检查/检查

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `ERRORS_ATTRIBUTES` | 错误标记 | 红色系 |
| `WARNING_ATTRIBUTES` | 警告标记 | 黄色/橙色系 |
| `WEAK_WARNING_ATTRIBUTES` | 弱警告标记 | 淡化警告 |
| `INFO_ATTRIBUTES` | 信息标记 | 蓝色系 |
| `TYPO` | 拼写错误 | 特殊标记 |
| `DEPRECATED_ATTRIBUTES` | 弃用标记 | 删除线样式 |
| `HYPERLINK_ATTRIBUTES` | 超链接 | 链接色 |
| `FOLLOWED_HYPERLINK_ATTRIBUTES` | 已访问链接 | 访问后链接色 |
| `MATCHED_BRACE_ATTRIBUTES` | 匹配括号 | 高亮强调 |
| `UNMATCHED_BRACE_ATTRIBUTES` | 不匹配括号 | 错误强调 |

### 代码折叠

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `FOLDED_TEXT_ATTRIBUTES` | 折叠文本 | 特殊背景 |
| `FOLDING_TREE_COLOR` | 折叠树颜色 | 淡分隔线 |
| `FOLDING_TREE_HAS_LINES_COLOR` | 有行折叠树 | 中等分隔线 |
| `FOLDING_TREE_HIGHLIGHTED_COLOR` | 高亮折叠树 | 强调分隔线 |
| `FOLDING_TREE_PREVIEW_BACKGROUND` | 折叠预览背景 | 半透明背景 |

### 终端颜色

| 接口/值 | 说明 | 颜色方向 |
|---------|------|----------|
| `TERMINAL_BACKGROUND` | 终端背景 | 深色/主背景 |
| `TERMINAL_FOREGROUND` | 终端前景 | 主文本色 |
| `TERMINAL_SELECTION_BACKGROUND` | 终端选中背景 | 半透明强调 |
| `TERMINAL_ANSI_BLACK` | ANSI黑色 | 终端色板 |
| `TERMINAL_ANSI_RED` | ANSI红色 | 终端色板 |
| `TERMINAL_ANSI_GREEN` | ANSI绿色 | 终端色板 |
| `TERMINAL_ANSI_YELLOW` | ANSI黄色 | 终端色板 |
| `TERMINAL_ANSI_BLUE` | ANSI蓝色 | 终端色板 |
| `TERMINAL_ANSI_MAGENTA` | ANSI洋红 | 终端色板 |
| `TERMINAL_ANSI_CYAN` | ANSI青色 | 终端色板 |
| `TERMINAL_ANSI_WHITE` | ANSI白色 | 终端色板 |

---
