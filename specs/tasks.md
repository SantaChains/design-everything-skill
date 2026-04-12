# Tasks

## Phase 1: Frontmatter 重构

- [x] Task 1.1: 重写 description，整合触发条件
  - [x] 合并所有 triggers 关键词到 description
  - [x] 添加"何时不触发"的反模式清单
  - [x] 控制总长度在 ~150 词

- [x] Task 1.2: 移除独立的 triggers 字段
  - [x] 删除 frontmatter 中的 triggers 列表
  - [x] 确保所有触发信息在 description 中

## Phase 2: Body 架构优化

- [x] Task 2.1: 引入 Pipeline 工作流
  - [x] 定义 6 阶段工作流：需求解析 → 风格提案 → 色彩方案 → 构图建议 → 元素清单 → 禁忌提醒
  - [x] 每阶段添加检查点指令
  - [x] 添加"禁止跳步"的门禁指令

- [x] Task 2.2: 添加 Reviewer 模式检查清单
  - [x] 创建 references/checklists.md
  - [x] 定义每个阶段的验收标准

- [x] Task 2.3: 应用 Inversion 模式
  - [x] 复杂需求场景添加分阶段提问模板
  - [x] 定义"信息不足时返回上一步"的闭环机制

- [x] Task 2.4: 精简 body 内容
  - [x] 移除冗余的模块索引表
  - [x] 将详细配色表移至 references/
  - [x] 将风格速查表移至 references/
  - [x] 控制总行数 <400 行 (实际 242 行)

## Phase 3: 三级加载优化

- [x] Task 3.1: 优化 references/ 结构
  - [x] 创建 references/modules.md（模块索引）
  - [x] 创建 references/checklists.md（检查清单）
  - [x] 确保 body 中有清晰的引用链接

- [x] Task 3.2: 优化脚本使用说明
  - [x] 简化脚本命令说明
  - [x] 添加"何时使用脚本"的决策树

## Phase 4: 自由度分级

- [x] Task 4.1: 定义自由度分级标准
  - [x] 高自由度任务清单（文字引导）
  - [x] 中自由度任务清单（模板+参数）
  - [x] 低自由度任务清单（脚本执行）

- [x] Task 4.2: 更新 body 中的自由度说明
  - [x] 添加自由度决策树
  - [x] 标注每个任务类型的自由度等级

## Phase 5: 评测机制

- [x] Task 5.1: 创建评测用例
  - [x] 创建 data/evaluation_cases.md
  - [x] 定义 8 个典型评测场景
  - [x] 定义通过/失败标准

- [x] Task 5.2: 建立迭代闭环
  - [x] 定义"评测失败 → 修改 skill → 回归测试"流程
  - [x] 添加版本记录机制

## Phase 6: 文档清理

- [x] Task 6.1: 移除冗余文档
  - [x] 创建 references/design-refs/ 目录
  - [ ] 移动临时文档 (用户取消操作，保留原位置)

- [ ] Task 6.2: 更新 README
  - [ ] 更新 assets/README.md (待用户确认)
  - [ ] 添加 skill 使用说明 (待用户确认)

# Task Dependencies

- Task 2.x depends on Task 1.x (frontmatter 先行) ✅
- Task 3.x depends on Task 2.x (body 结构确定后优化 references) ✅
- Task 5.x depends on Task 1-4 (评测基于完整架构) ✅
- Task 6.x can run in parallel with Task 3-5 ✅
