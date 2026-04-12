# Checklist v2.0

## Frontmatter 检查

- [x] description 包含所有触发关键词
- [x] description 包含"何时不触发"的反模式
- [x] description 长度控制在 ~150 词
- [x] triggers 字段已移除
- [x] frontmatter 只包含 name 和 description
- [x] **新增**: Agent Identity 声明
- [x] **新增**: 强势描述（"必须激活"）

## Body 架构检查

- [x] 包含清晰的 7 阶段 StateGraph 工作流
- [x] 每阶段有 State 定义（TypeScript 接口）
- [x] 每阶段有 Checkpoint 检查点
- [x] 每阶段有 Transition 条件
- [x] 每阶段有 Error Handling 策略
- [x] 包含"禁止跳步"门禁指令
- [x] body 总行数 <500 行 (实际 499 行)
- [x] 使用祈使语气

## 四级加载检查

- [x] L1 (frontmatter) ~150 词
- [x] L2 (body) <500 行
- [x] L3 (references) 有清晰的引用链接
- [x] **新增**: L4 (scripts) 零 token 声明
- [x] 大型参考文件有目录

## StateGraph 检查

- [x] 有状态定义（TypeScript 接口）
- [x] 有状态转换图
- [x] 有 Transition 条件
- [x] 有 Checkpoint 结构定义
- [x] 有恢复机制说明

## 自由度分级检查

- [x] 定义了高/中/低/固定四级自由度
- [x] 包含自由度决策树
- [x] 脚本用于低自由度任务
- [x] **新增**: 固定级（安全检查、格式验证）

## 评测机制检查

- [x] 创建了评测用例文件
- [x] 定义了通过/失败标准
- [x] 包含迭代闭环说明
- [x] **新增**: L1 触发验证层（20 用例）
- [x] **新增**: L2 流程验证层（StateGraph）
- [x] **新增**: L3 输出验证层（端到端）
- [x] **新增**: CI/CD 集成示例
- [x] **新增**: 指标监控 Dashboard

## Agent Identity 检查

- [x] 有身份声明（Design Expert）
- [x] 有专长定义
- [x] 有风格描述
- [x] 有身份保持说明

## 文档清理检查

- [ ] 根目录无临时文档 (用户取消移动操作，保留原位置)
- [x] references/ 结构清晰
- [ ] README 已更新 (待用户确认是否需要)
