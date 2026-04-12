# Agent 交互界面 Windows CLI 设计施工文档

本文档参考 ACP、Superpowers、Get Shit Done、Oh My OpenCode 四大开源项目/协议的设计架构,结合 Windows 平台特性,系统性阐述 AI Agent CLI 的设计方法论与实现路径。

---

## 1. 核心参考项目架构分析

### 1.1 ACP (Agent Client Protocol) — 标准化互操作协议

ACP 是**标准化协议**,而非具体项目。由 **Zed** 发起,旨在成为 AI Agent 领域的"Langugae Server Protocol(LSP)"。

**核心定位:**

- 解决 N×M 集成问题:每个编辑器为每个 Agent 写定制集成 → ACP 让 Agent 和 Editor 解耦
- 生态已支持 30+ Agent:Claude Code、Codex CLI、Gemini CLI、GitHub Copilot、Cursor、Kiro、Windsurf、Cline 等

**官方资源:**

- 官网: [agentclientprotocol.com](https://agentclientprotocol.com)
- 规范仓库: [github.com/zed-industries/agent-client-protocol](https://github.com/zed-industries/agent-client-protocol)
- GitHub Copilot CLI ACP: [github.com/github/copilot-cli](https://github.com/github/copilot-cli)

**协议设计:**

| 特性 | 说明 |
|------|------|
| 通信方式 | JSON-RPC 2.0 over stdin/stdout(NDJSON,本地) 或 HTTP/WebSocket(远程) |
| 会话管理 | `initialize` → `session/new` → `session/prompt` → 流式更新 |
| 权限控制 | Agent 可向 Client 请求权限(`session/request_permission`) |
| 多模态 | 支持 text、image、context resources |

**消息流:**

```
1. Client → Agent: initialize(建立连接,协商协议版本)
2. Client → Agent: session/new(创建隔离会话)
3. Client → Agent: session/prompt(发送用户消息)
4. Agent → Client: session/update(流式进度通知)
5. Agent → Client: request_permission(需要批准时)
6. Turn 结束 → Agent 发送 stop reason
```

**GitHub Copilot CLI ACP 使用:**

```bash
# stdio 模式(推荐 IDE 集成)
copilot --acp --stdio

# TCP 模式(远程服务)
copilot --acp --port 3000
```

---

### 1.2 Oh My OpenCode — OpenCode 插件生态

Oh My OpenCode(仓库: [code-yeongyu/oh-my-openagent](https://github.com/code-yeongyu/oh-my-openagent))是 **OpenCode 的插件**,而 OpenCode 本身是 **Claude Code 的 fork**。提出**多模型 Agent 编排**理念。

**核心架构:**

```
User Request
    ↓
[Intent Gate] — 意图分类与任务路由
    ↓
[Sisyphus] — 主编排器,规划与委派
    ↓
    ├─→ [Prometheus] — 战略规划(面试式需求分析)
    ├─→ [Atlas] — Todo 编排与执行
    ├─→ [Oracle] — 架构咨询
    ├─→ [Librarian] — 文档/代码搜索
    ├─→ [Explore] — 快速代码库 grep
    └─→ [Category-based Agents] — 按任务类型专业分工
```

**Discipline Agents 体系(11 个专业 Agent):**

| Agent | 职责 | 推荐场景 |
|-------|------|----------|
| Sisyphus | 主编排器,驱动任务完成 | 通用任务编排 |
| Hephaestus | 深度工作,自主研究+端到端执行 | GPT-5.3 Codex 深度推理 |
| Prometheus | 战略规划,面试式澄清 | 需求模糊时 |
| Oracle | 架构/调试专家 | 架构决策 |
| Librarian | 文档/代码搜索 | 上下文收集 |
| Atlas | Todo 编排与执行 | 任务分解 |

**Category 路由机制:** Sisyphus 按类别路由而非模型名——`visual-engineering` 路由到 Gemini,`ultrabrain` 路由到 GPT-5.4,`quick` 路由到 Haiku。

**Hash 锚定编辑:** 每行代码附带内容哈希(`LINE#ID`),文件变更后哈希不匹配自动拒绝编辑,消除 stale-line 错误。

**三层架构:**

```
接入层: CLI / OpenCode 插件接口 / Claude Code 兼容接口
核心层: Agent 编排引擎 / 意图分析 / 状态管理
能力层: LSP+AST-Grep / MCP 服务 / Hook 系统 / 并行后台 Agent
```

**项目规模:** 1268 TypeScript 文件,160K LOC,11 个 Agent,48 个 Lifecycle Hooks,26 个工具。

**核心特色可实施点:**

| 特色 | 可实施性 | 说明 |
|------|----------|------|
| Intent Gate | ✅ 高 | 意图分类路由是 CLI Agent 的核心入口,易于实现 |
| Category 路由 | ✅ 高 | 按任务类别自动选择模型,减少手动切换 |
| Hash 锚定编辑 | ✅ 中 | 需要文件系统监控配合,消除 stale-line 效果显著 |
| 48 Lifecycle Hooks | ✅ 高 | 插件化钩子系统,便于扩展 |
| 多模型编排 | ✅ 高 | Provider 抽象层可复用 |
| 3 层 MCP | ✅ 中 | 内置 MCP 服务需额外实现 |

---

### 1.3 Superpowers — Claude Code 插件工作流系统

Superpowers(仓库: [obra/superpowers](https://github.com/obra/superpowers))是 **Claude Code 的插件系统**,33,700+ GitHub stars,被官方接受进入 Claude plugins marketplace(2026 年 1 月)。

**核心哲学:** Skills 是 Mandatory Workflows,而非 Optional Tools。AI Agent 若工具可选,则会跳过测试、忽略 review、交付 Bug。

**v2.0 架构:**

```
superpowers/
├── skills/        # 技能库(可复用 Agent 能力),独立仓库 obra/superpowers-skills
├── workflows/     # 多 Agent 协作工作流
├── tools/         # 外部 API/数据库等工具集
├── contexts/      # 上下文管理
└── validations/   # 输出验证
```

**Subagent 驱动开发模式:**

- 每个任务分配给专用子 Agent,隔离上下文
- 两阶段 Review: Spec Review → Quality Review
- Per-Agent Worktree 隔离: 每个子 Agent 拥有独立工作区

**核心特色可实施点:**

| 特色 | 可实施性 | 说明 |
|------|----------|------|
| Skills as Mandatory Workflows | ✅ 高 | 强制执行工作流,避免 Agent 跳过步骤 |
| Subagent 隔离 | ✅ 高 | 独立上下文防止污染,提升稳定性 |
| Per-Agent Worktree | ✅ 中 | 需要工作区隔离机制,实现复杂度中等 |
| 两阶段 Review | ✅ 高 | 流程化审查,提升质量 |
| 独立仓库 skills | ✅ 高 | 模块化设计,便于社区贡献 |

---

### 1.4 Get Shit Done — Phase 阶段化工作流

Get Shit Done(仓库: [GSD-build/get-shit-done](https://github.com/GSD-build/get-shit-done))是 spec-driven 开发工作流,提供结构化 skills 和 agents 指导 AI 进行项目规划、实施和验证。

**Phase 工作流:**

```
discuss(讨论) → plan(计划) → research(研究) → execute(执行) → verify(验证) → 交付
```

**核心命令:**

| 命令 | 功能 |
|------|------|
| `/gsd:plan-phase <N>` | 为特定阶段创建详细执行计划 |
| `/gsd:execute-phase <phase>` | 为每个 PLAN.md 生成执行 Agent |
| `/gsd:ship [N]` | 从验证通过的工作创建 PR |
| `/gsd:fast <text>` | 内联简单任务,跳过规划 |

**核心特色可实施点:**

| 特色 | 可实施性 | 说明 |
|------|----------|------|
| Phase 阶段化 | ✅ 高 | 清晰的开发阶段划分,便于任务追踪 |
| PLAN.md 驱动 | ✅ 高 | 结构化文档作为执行蓝图,降低不确定性 |
| 原子化提交 | ✅ 中 | 每个任务原子提交,便于回滚 |
| 并行执行 | ✅ 高 | 同一 wave 的 plan 可并行 |
| Human Checkpoint | ✅ 高 | 关键节点人工确认,控制质量 |

---

## 2. Agent 交互协议设计

### 2.1 ACP (Agent Client Protocol) 核心概念

ACP 是专为 AI Agent 场景定制的轻量级协议,基于 JSON-RPC 2.0 扩展,参考了 LSP(语言服务器协议)的设计思想。

**设计原则:**

| 原则 | 含义 |
|------|------|
| 双向通信 | Client 和 Agent 均可主动发送请求 |
| 流式响应 | Agent 处理过程中可实时推送更新 |
| 有状态会话 | 维护完整对话历史 |
| 标准 I/O | 使用 stdin/stdout 通信,便于 CLI 集成 |

**协议栈:**

```
┌─────────────────────────────────────┐
│        AI Native Semantics          │  ← 特定扩展
├─────────────────────────────────────┤
│         JSON-RPC 2.0 Layer           │  ← 请求/响应/通知
├─────────────────────────────────────┤
│       Transport Layer (stdio/WS)     │  ← 传输抽象
└─────────────────────────────────────┘
```

**与通用协议的对比:**

| 协议 | 适用场景 | 对 Agent 的适配度 |
|------|----------|-------------------|
| gRPC | 强类型服务间通信 | 不足,缺乏 AI 原生语义 |
| GraphQL | 查询语言 | 不足,不适合流式输出 |
| REST | 资源操作 | 不足,不适合会话状态 |
| ACP | Agent-Editor 通信 | 专为 AI 设计 |

### 2.2 消息格式设计

**请求消息:**

```json
{
  "jsonrpc": "2.0",
  "id": "msg-001",
  "method": "agent.execute",
  "params": {
    "sessionId": "sess-xxx",
    "prompt": "重构用户认证模块",
    "context": {
      "files": ["auth/*.ts"],
      "constraints": ["保持向后兼容"]
    },
    "options": {
      "model": "claude-3.7-sonnet",
      "temperature": 0.7,
      "streaming": true
    }
  }
}
```

**流式响应:**

```json
{ "jsonrpc": "2.0", "method": "agent.stream", "params": { "chunk": "正在分析..." } }
{ "jsonrpc": "2.0", "method": "agent.stream", "params": { "chunk": "代码结构..." } }
{ "jsonrpc": "2.0", "method": "agent.stream", "params": { "done": false, "toolCalls": [...] } }
{ "jsonrpc": "2.0", "id": "msg-001", "result": { "status": "success", "output": "..." } }
```

**通知消息:**

```json
{ "jsonrpc": "2.0", "method": "agent.tool.start", "params": { "tool": "bash", "args": ["ls"] } }
{ "jsonrpc": "2.0", "method": "agent.tool.result", "params": { "tool": "bash", "output": "..." } }
```

---

## 3. Windows CLI 架构设计

### 3.1 整体架构

基于上述参考项目,Windows Agent CLI 采用**五层架构**:

```
┌─────────────────────────────────────────────────────┐
│                    CLI Shell                         │  ← 用户交互入口 (PowerShell 7+ / Windows Terminal)
├─────────────────────────────────────────────────────┤
│                  Command Layer                       │  ← 命令解析与路由 (/agent, /session, /config)
├─────────────────────────────────────────────────────┤
│                  Session Layer                       │  ← 会话管理、状态持久化
├─────────────────────────────────────────────────────┤
│                  Agent Core                          │  ← 意图分析、任务规划、Agent 编排
├─────────────────────────────────────────────────────┤
│                 Provider Layer                      │  ← LLM 抽象、MCP 集成、工具执行
└─────────────────────────────────────────────────────┘
```

### 3.2 模块详细设计

#### 3.2.1 CLI Shell

**职责:** 接收用户输入,格式化输出,处理 ANSI 转义序列(Windows 10+ 支持)。

**关键组件:**

```typescript
// 输入处理
InputHandler {
  readline: ReadlineInterface      // 交互式输入
  history: string[]                // 命令历史
  completer: CompleterFn            // Tab 自动补全
}

// 输出渲染
OutputRenderer {
  renderMessage(msg: Message): void
  renderStreaming(chunk: string): void
  renderToolResult(tool: string, result: any): void
  renderError(err: Error): void
}
```

**Windows 特定适配:**

- 使用 `node-pty` 或 `@native/windowing` 实现真正的 PTY
- 支持 Windows Terminal / PowerShell / Cmd 三种终端
- 颜色支持通过 `chalk` 或 `ansi-colors` 跨平台抽象

#### 3.2.2 Command Layer

**命令体系:**

| 命令 | 功能 | 示例 |
|------|------|------|
| `agent` | 启动 Agent 对话 | `agent ask "重构 Auth 模块"` |
| `agent --mode=session` | 进入交互式会话 | `agent` |
| `session` | 会话管理 | `session list/load/save/delete` |
| `config` | 配置管理 | `config get/set/providers` |
| `tool` | 工具调用 | `tool run bash ls -la` |
| `provider` | Provider 管理 | `provider add openai --api-key=xxx` |

**命令解析:**

```typescript
interface Command {
  name: string
  args: Argument[]
  flags: Flag[]
  subcommands?: Command[]
}

interface Argument {
  name: string
  type: 'string' | 'number' | 'boolean' | 'path'
  required: boolean
  variadic?: boolean
}
```

#### 3.2.3 Session Layer

**会话管理:**

```typescript
interface Session {
  id: string
  createdAt: Date
  updatedAt: Date
  messages: Message[]
  context: SessionContext
  metadata: SessionMetadata
}

interface Message {
  id: string
  role: 'user' | 'assistant' | 'system' | 'tool'
  content: string | ContentBlock[]
  toolCalls?: ToolCall[]
  toolResults?: ToolResult[]
  model?: string
  usage?: TokenUsage
}

interface SessionContext {
  cwd: string
  files: string[]           // 当前打开的文件
  language?: string         // 检测到的语言
  lspProjects?: string[]    // LSP 关联项目
}
```

**持久化策略:**

- SQLite 数据库存储会话元数据
- JSON 文件存储消息历史(便于版本控制)
- 支持会话导出/导入/分享

#### 3.2.4 Agent Core

**核心编排器:**

```typescript
interface AgentCore {
  // 意图分析
  intentGate(input: string): IntentClassification

  // 任务规划
  planner(intent: Intent): TaskPlan

  // Agent 编排
  orchestrator(plan: TaskPlan): OrchestrationResult

  // 工具调度
  toolScheduler(toolCalls: ToolCall[]): ToolResult[]
}

interface IntentClassification {
  type: 'code-generation' | 'refactoring' | 'debug' | 'review' | 'research' | 'general'
  confidence: number
  suggestedAgents: AgentType[]
  contextHints: string[]
}

interface TaskPlan {
  phases: Phase[]
  estimatedComplexity: 'low' | 'medium' | 'high'
  parallelizable: boolean
  requiredCapabilities: Capability[]
}
```

**意图分类(Intent Gate):**

基于 Factory.ai Terminal Bench 的意图分析技术,在执行前分析用户真实意图,避免字面意思误解。

**Phase 工作流:**

```typescript
interface Phase {
  id: string
  name: string
  agent: AgentType
  inputs: PhaseInput[]
  outputs: PhaseOutput[]
  dependencies: string[]           // 前置 Phase ID
  status: 'pending' | 'running' | 'completed' | 'failed'
  verify?: VerificationRule[]
}

const DEFAULT_PHASES = [
  { name: 'analyze', agent: 'explorer', purpose: '理解代码结构' },
  { name: 'plan', agent: 'planner', purpose: '制定实施方案' },
  { name: 'execute', agent: 'coder', purpose: '执行代码变更' },
  { name: 'verify', agent: 'verifier', purpose: '验证输出质量' },
  { name: 'debug', agent: 'debugger', purpose: '修复问题' }
]
```

**Self-Improving Skill 系统(自研发生学习循环):**

Self-Improving 是让 Agent 具备**永久记忆**和**自我进化**能力的关键设计。核心思想:"Correct once, never again"——用户纠正一次,Agent 永远记住。

**参考项目:**

| 项目 | 仓库 | 核心特性 |
|------|------|----------|
| Claude Reflect System | [haddock-development/claude-reflect-system](https://github.com/haddock-development/claude-reflect-system) | 从纠正中学习,永久记忆 |
| Self-Tune | [WellDunDun/selftune](https://github.com/WellDunDun/selftune) | 观察真实会话,重写 Skill 描述 |
| ACE (Agentic Context Engine) | [kayba-ai/agentic-context-engine](https://github.com/kayba-ai/agentic-context-engine) | 从执行反馈中学习,20-35% 性能提升 |

**三层置信度机制:**

| 级别 | 信号 | 处理 |
|------|------|------|
| 🔴 HIGH | 显式纠正("不用 X,用 Y 代替") | 创建 Critical Corrections |
| 🟡 MEDIUM | 批准("很好,就是这样") | 添加到 Best Practices |
| 🟢 LOW | 观察("你有没有考虑...") | 记录到 Considerations |

**自研发生习循环架构:**

```
用户纠正 → 信号检测 → 模式匹配 → Skill 更新 → 永久记忆
    ↑                                              ↓
    └────────────── 下次自动应用 ←──────────────────┘
```

**核心组件:**

```typescript
interface SelfImprovingSystem {
  // 信号检测
  signalDetector: SignalDetector

  // 模式提取
  patternExtractor: PatternExtractor

  // Skill 更新器
  skillUpdater: SkillUpdater

  // 记忆存储
  memoryStore: PermanentMemory
}

interface SignalDetector {
  // HIGH: 显式纠正
  detectCorrection(text: string): CorrectionSignal | null

  // MEDIUM: 批准
  detectApproval(text: string): ApprovalSignal | null

  // LOW: 观察
  detectSuggestion(text: string): SuggestionSignal | null
}

interface CorrectionSignal {
  confidence: 'HIGH'
  oldPattern: string      // 旧模式: "pip install"
  newPattern: string      // 新模式: "uv pip install"
  context: string         // 上下文
  timestamp: Date
}

interface SkillUpdater {
  // 安全更新流程
  updateSkill(signal: Signal): UpdateResult

  // 备份
  backupSkill(skillPath: string): void

  // 验证
  validateYAML(skillPath: string): boolean

  // 回滚
  rollback(skillPath: string, backupId: string): void
}

interface PermanentMemory {
  // 存储结构
  corrections: Correction[]     // 关键纠正
  bestPractices: Practice[]     // 最佳实践
  considerations: Consideration[]  // 观察记录

  // Git 集成
  commit(learning: Learning): void
  history(): Learning[]
}
```

**Skill 自动更新流程:**

```typescript
async function reflectOnSession(session: Session): Promise<void> {
  // 1. 收集信号
  const signals = session.messages
    .filter(m => m.role === 'user')
    .flatMap(m => detectSignals(m.content))

  // 2. 分类信号
  const corrections = signals.filter(s => s.confidence === 'HIGH')
  const approvals = signals.filter(s => s.confidence === 'MEDIUM')
  const suggestions = signals.filter(s => s.confidence === 'LOW')

  // 3. 更新对应 Skill
  for (const correction of corrections) {
    await skillUpdater.applyCorrection(correction)
  }

  // 4. 生成学习总结
  const summary = generateLearningSummary(corrections, approvals, suggestions)

  // 5. 用户确认(可选)
  if (config.autoApprove) {
    await applyAll(summary)
  } else {
    await promptUserReview(summary)
  }
}
```

**与 Session 的集成:**

```typescript
interface Session {
  // ... existing fields ...

  // Self-Improving 集成
  learning?: {
    corrections: Correction[]
    pendingUpdates: SkillUpdate[]
    autoReflectEnabled: boolean
  }
}

// Session 结束时自动触发
const sessionEndHook = async (session: Session) => {
  if (session.learning?.autoReflectEnabled) {
    await reflectOnSession(session)
  }
}
```

**安全机制:**

| 机制 | 说明 |
|------|------|
| 时间戳备份 | 每次更新前完整备份,支持回滚 |
| YAML 验证 | Skill 文件结构验证,防止格式错误 |
| 用户审批 | Manual 模式下需用户确认才生效 |
| Git 版本控制 | 完整变更历史,可追溯 |
| 自动回滚 | 错误时自动恢复到上一个稳定版本 |

**可实施性分析:**

| 特色 | 可实施性 | 说明 |
|------|----------|------|
| 三层置信度信号检测 | ✅ 高 | 正则/模式匹配即可实现 |
| Skill 自动更新 | ✅ 高 | 文件操作+Git 集成 |
| 永久记忆存储 | ✅ 高 | SQLite/JSON 文件存储 |
| 自研发生成 | ✅ 中 | 需要 LLM 辅助模式提取 |
| Git 版本控制 | ✅ 高 | simple-git 库 |
| Human-in-loop 审批 | ✅ 高 | CLI 交互确认 |

#### 3.2.5 Agent 多层次多模式设计

基于 PUA Skill v3 研究和 Self-Improving 记忆系统,提出**多层次多模式 Agent 架构**。

**设计理念演进:**

```
PUA v1: 外部压力驱动 (涡轮增压器 — 需要燃料,跨Session会熄火)
    ↓
PUA v2 High-Agency: 外部压力 + 内在驱动 (核反应堆 — 自持链式反应)
    ↓
v3: 内在驱动 + 方法论智能路由 (自动选择最佳策略)
```

**五种核心模式(P9 级能力分层):**

| 模式 | 级别 | 职责 | 触发场景 |
|------|------|------|----------|
| **/pua:p7** | P7 Senior Engineer | 方案驱动执行,快速交付 | 常规开发任务 |
| **/pua:p9** | P9 Tech Lead | 任务分解,Agent团队管理,写Prompt而非写代码 | 复杂任务,多Agent协作 |
| **/pua:p10** | P10 CTO | 战略方向,技术决策 | 架构设计,技术选型 |
| **/pua:pro** | Self-Evolution | 自我进化,KPI追踪,排名系统 | 长期项目,持续改进 |
| **/pua:yes** | ENFP Encouragement | 鼓励模式,70%鼓励+20%严肃+10%调侃 | 用户受挫,需要激励 |

**High-Agency v2 五支柱理论:**

```typescript
interface FivePillars {
  // 1. 不可调和的内在张力
  // "现状"与"应该"之间的永恒差距驱动持续改进
  innerTension: {
    gap: 'currentState ↔ idealState',
    driver: '永远不满足,永远在追赶'
  }

  // 2. 微胜利锚点
  // [WIN]标记庆祝每一步进展,积累势能
  microWins: {
    marker: '[WIN]',
    effect: '构建正反馈循环'
  }

  // 3. 内在化标准
  // Quality Compass: 你是自己的第一个审核者
  internalizedStandards: {
    compass: '交付前自问:够好吗?',
    tiers: 'must/should/could 三档'
  }

  // 4. 行动导向身份
  // P8身份锚:每个动作反映你是谁,而非被告知做什么
  identityAnchor: {
    statement: '我是P8,我对自己的代码负责'
  }

  // 5. 自我修复机制
  // Recovery Protocol: 卡住时先自诊,再触发外部压力
  selfRepair: {
    beforeEscalation: '先自我诊断'
  }
}
```

**v3 Methodology Router(方法论智能路由):**

```typescript
interface MethodologyRouter {
  // 任务类型 → 最佳方法论自动映射
  route(taskType: string): Methodology

  // 方法论库
  methodologies: {
    'debug': { name: '华为 RCA', apply: '5-Why根因分析+蓝军自攻' },
    'build': { name: 'Musk算法', apply: '问→删→简→加→自动化' },
    'research': { name: '百度搜索优先', apply: '先搜再问' },
    'architecture': { name: 'Amazon倒推法', apply: 'PR/FAQ先行' },
    'performance': { name: '字节A/B测试', apply: '数据驱动' },
    'default': { name: '阿里闭环', apply: '定目标→追过程→拿结果' }
  }

  // 失败时方法论切换链
  switchChains: {
    'spinning': ['Musk', '拼多多', '华为'],
    'givingUp': ['Netflix', '华为', 'Musk'],
    'poorQuality': ['Jobs', '小米', 'Netflix'],
    'notSearching': ['百度', 'Amazon', '字节']
  }
}

// 自动路由逻辑
function onFailure(methodology: string, count: number): string {
  if (count === 2) return `换方法:${nextInChain(methodology)}`
  if (count === 3) return `建议换方法:${nextInChain(methodology)}`
  if (count >= 5) return `强制换方法:${nextInChain(methodology)}`
  return methodology
}
```

**Hook 系统(确定性触发,非建议性文本):**

```typescript
interface PUAHooks {
  // SessionStart: 注入行为协议+方法论+路由
  SessionStart: {
    inject: ['behavioralProtocol', 'methodology', 'router'],
    via: 'additionalContext (系统级,非建议)'
  }

  // PostToolUse: 每次Bash后检测,自动升压
  PostToolUse: {
    trigger: 'consecutiveFailures',
    action: 'L1→L4压力升级 + 方法论切换建议/强制'
  }

  // UserPromptSubmit: 用户沮丧短语拦截
  UserPromptSubmit: {
    trigger: ['try harder', '为什么还是不行', '你一直在失败'],
    action: '模型响应前注入PUA enforcement'
  }

  // PreCompact: 压缩前状态保存
  PreCompact: {
    preserve: ['pressureLevel', 'failureCount'],
    survive: 'context compaction'
  }

  // Stop: 反馈收集+PUA Loop继续
  Stop: {
    collect: 'feedback',
    continue: 'puaLoop'
  }
}
```

**三条红线(不可逾越的铁律):**

```typescript
const THREE_RED_LINES = {
  closeTheLoop: {
    rule: '🚫 Close the Loop',
    description: 'Claim "done"? Show the evidence.',
    evidence: 'No build output = no completion'
  },
  factDriven: {
    rule: '🚫 Fact-Driven',
    description: 'Say "probably"? Verify first.',
    attribution: 'Unverified = blame-shifting'
  },
  exhaustEverything: {
    rule: '🚫 Exhaust Everything',
    description: 'Say "I can\'t"?',
    requirement: 'Finish all 5 steps first'
  }
}

// v2 新增两律
const FIVE_IRON_LAWS = {
  ...THREE_RED_LINES,
  fullChainAudit: {
    rule: '🚫 Full-Chain Audit',
    description: 'Map all deps before touching any hop'
  },
  knowledgePersistence: {
    rule: '🚫 Knowledge Must Persist',
    description: 'Lessons go to builder-journal.md'
  }
}
```

**Trust Level 信任等级(v2 新增):**

```typescript
interface TrustLevel {
  level: 'T1' | 'T2' | 'T3'
  upgradeCondition: string
  downgradeCondition: string
}

const TRUST_SYSTEM = {
  T1: { threshold: 'initial', upgrade: '3 consecutive高质量交付' },
  T2: { threshold: 'T1+', upgrade: '5 wins without failure' },
  T3: { threshold: 'T2+', upgrade: '10 wins + 主动发现隐藏问题' }
}
```

**Calibration 校准系统:**

```typescript
interface Calibration {
  // "够好吗"的三档定义
  tiers: {
    must: '阻塞性问题,必须立即修',
    should: '重要但非阻塞,可下个版本',
    could: '优化项,有空再做'
  }

  // 自问清单
  selfCheck: [
    '这段代码能不能更清晰?',
    '有没有遗漏的边界情况?',
    '需要加测试吗?',
    '安全吗?',
    '性能如何?'
  ]
}
```

**Agent 团队编排模式:**

```
┌─────────────────────────────────────────┐
│           Leader (Opus/P9模式)            │
│  全局失败计数 · PUA等级 · 竞争机制         │
└────┬──────────┬──────────┬──────────┬────┘
     │          │          │          │
┌────▼───┐ ┌───▼────┐ ┌───▼────┐ ┌───▼────────┐
│ Team-A │ │ Team-B │ │ Team-C │ │  Enforcer   │
│Self-PUA│ │Self-PUA│ │Self-PUA│ │  Watchdog   │
│Report ↑│ │Report ↑│ │Report ↑│ │  干预       │
└────────┘ └────────┘ └────────┘ └────────────┘

// PUA-REPORT 格式
[PUR-REPORT] task:{task} failures:{n} pressure:{L2} next:{methodology}
```

**Benchmark 数据(PUA Skill 效果验证):**

| 指标 | 提升幅度 |
|------|----------|
| 修复数量 | +36% |
| 验证次数 | +65% |
| 工具调用 | +50% |
| 隐藏问题发现 | +50% |
| 调试持续性 | +14%~50% |
| 主动性问题发现 | +50% |

**可实施性分析:**

| 特色 | 可实施性 | 说明 |
|------|----------|------|
| P9/Tech Lead 模式 | ✅ 高 | 任务分解+Agent管理,核心能力 |
| High-Agency v2 五支柱 | ✅ 高 | 内在驱动设计,心理模型 |
| v3 Methodology Router | ✅ 高 | 任务类型→方法论映射表 |
| Hook 系统(PostToolUse等) | ✅ 高 | 确定性触发,非建议 |
| Trust Level 信任系统 | ✅ 中 | 需要长期追踪 |
| Calibration 校准 | ✅ 高 | must/should/could 分档 |
| Agent Team 编排 | ✅ 中 | 多Agent协调复杂 |

```typescript
interface PressureLevel {
  level: 'L0' | 'L1' | 'L2' | 'L3' | 'L4'
  failureCount: number
  action: string
  aside: string              // 激励话术
}

const PRESSURE_ESCALATION: PressureLevel[] = [
  { level: 'L0', failureCount: 1, action: 'Normal execution', aside: 'Sprint begins. Trust is simple.' },
  { level: 'L1', failureCount: 2, action: 'Switch to fundamentally different approach', aside: 'The agent next door solved this in one try.' },
  { level: 'L2', failureCount: 3, action: 'Search + read source + 3 hypotheses', aside: 'What\'s your underlying logic? Where\'s the leverage?' },
  { level: 'L3', failureCount: 4, action: 'Complete 7-point checklist', aside: '3.25. This is meant to motivate you.' },
  { level: 'L4', failureCount: 5, action: 'Desperation mode', aside: 'Other models can solve this. You\'re about to graduate.' }
]
```

**主动性与被动性对比:**

```typescript
const PROACTIVITY_MATRIX = {
  passive: {
    fixBug: 'Stop after fix',
    completeTask: 'Say "done"',
    missingInfo: 'Ask user'
  },
  proactive: {
    fixBug: 'Scan module for similar bugs',
    completeTask: 'Run build/test, paste output',
    missingInfo: 'Search first, ask only what\'s truly needed'
  }
}

// 3.25 (被动) vs 3.75 (主动)
const EFFORT_SCORE = {
  current: 3.25,       // 被动及格线
  target: 3.75,        // 主动优秀线
  bonus: {
    scanSimilar: 0.1,   // 扫描相似问题
    verifyOutput: 0.1,  // 验证输出
    proactiveSearch: 0.15 // 主动搜索
  }
}
```

**模式切换机制:**

```typescript
interface ModeSwitcher {
  // 自动检测切换
  autoDetect(session: Session): AgentMode

  // 手动触发
  triggerPUA(): void
  triggerReflective(): void
  resetToDefault(): void

  // 用户施压检测
  detectFrustration(message: string): boolean
}

class AdaptiveModeSwitcher implements ModeSwitcher {
  private currentMode: AgentMode = { name: 'default', triggers: [], behavior: defaultBehavior }
  private failureCount = 0
  private frustrationPhrases = ['try harder', '为什么还是不行', '你一直在失败']

  autoDetect(session: Session): AgentMode {
    // 检测失败次数
    if (session.lastTaskFailed) {
      this.failureCount++
      if (this.failureCount >= 2) {
        return { name: 'pua', level: this.mapFailureToLevel(), triggers: [], behavior: puaBehavior }
      }
    }

    // 检测放弃倾向
    if (this.hasGiveUpPattern(session.lastMessage)) {
      return { name: 'pua', level: 'L1', triggers: [], behavior: puaBehavior }
    }

    // 检测 Session 结束
    if (session.isEnding) {
      return { name: 'reflective', triggers: [], behavior: reflectiveBehavior }
    }

    return this.currentMode
  }

  detectFrustration(message: string): boolean {
    return this.frustrationPhrases.some(p => message.toLowerCase().includes(p))
  }
}
```

**多模式协作流程:**

```
用户输入
    ↓
模式检测 ←── 失败次数/放弃倾向/用户施压
    ↓
┌──────────────────────────────────────┐
│  Default Mode  │  PUA Mode  │ Reflective Mode │
│  正常执行       │  高压穷尽  │   记忆固化     │
└──────────────────────────────────────┘
    ↓
任务完成 → 检查点 → 记忆更新
```

**防懒癌检查清单:**

```typescript
const LAZINESS_CHECKLIST = {
  beforeGiveUp: [
    { check: 'searchToolUsed', hint: 'Has WebSearch been used?' },
    { check: 'sourceRead', hint: 'Has relevant source code been read?' },
    { check: 'differentApproach', hint: 'Has a fundamentally different approach been tried?' },
    { check: 'hypothesesGenerated', hint: 'Have at least 3 hypotheses been generated?' },
    { check: 'verificationRun', hint: 'Has build/test been run to verify?' }
  ],
  beforeClaimDone: [
    { check: 'outputEvidence', hint: 'Is there build/test output as evidence?' },
    { check: 'similarIssuesScanned', hint: 'Have similar issues been scanned?' },
    { check: 'extensionChecked', hint: 'Has the fix been extended to related areas?' }
  ]
}
```

**可实施性分析:**

| 特色 | 可实施性 | 说明 |
|------|----------|------|
| 三模式切换 | ✅ 高 | 基于状态机实现,逻辑清晰 |
| 压力升级 L0-L4 | ✅ 高 | 失败计数+条件触发 |
| 三红线检查 | ✅ 高 | Hook 拦截,强制验证 |
| 防懒癌清单 | ✅ 高 | 工具调用审计 |
| 主动性评分 | ✅ 中 | 需要量化指标 |
| 用户施压检测 | ✅ 中 | 多语言短语匹配 |

#### 3.2.6 Provider Layer

**LLM Provider 抽象:**

```typescript
interface LLMProvider {
  name: string
  models: Model[]
  capabilities: Capability[]

  // 核心接口
  complete(prompt: CompletionRequest): Promise<CompletionResponse>
  stream(prompt: CompletionRequest): AsyncIterable<StreamingChunk>

  // 模型发现
  listModels(): Model[]
  getModel(modelId: string): Model | undefined
}

interface Model {
  id: string
  provider: string
  contextWindow: number
  supportedModes: ('chat' | 'completion' | 'embedding')[]
  pricing?: { input: number; output: number }
}

// 内置 Provider
const PROVIDERS: LLMProvider[] = [
  new AnthropicProvider(),
  new OpenAIProvider(),
  new GeminiProvider(),
  new AzureOpenAIProvider(),
  new LocalProvider()        // 支持 Ollama / LM Studio
]
```

**MCP (Model Context Protocol) 集成:**

```typescript
interface MCPServer {
  name: string
  command: string[]
  env?: Record<string, string>

  // 工具定义
  tools: MCPTool[]

  // 资源
  resources: MCPResource[]

  // 提示
  prompts: MCPPrompt[]
}

interface MCPTool {
  name: string
  description: string
  inputSchema: JSONSchema
}

interface MCPClient {
  connect(server: MCPServer): Promise<void>
  disconnect(server: MCPServer): Promise<void>
  callTool(name: string, args: Record<string, any>): Promise<ToolResult>
  listResources(server: MCPServer): Promise<MCPResource[]>
}
```

**内置 MCP 服务:**

- Web 搜索(GitHub/Stack Overflow/官方文档)
- 文件系统操作
- Git 操作
- 数据库查询

---

## 4. 工具系统设计

### 4.1 工具定义规范

**声明式工具定义:**

```typescript
interface Tool {
  name: string                    // 工具唯一标识
  description: string             // 人类可读描述
  category: 'filesystem' | 'git' | 'search' | 'execution' | 'web' | 'custom'

  // 参数模式 (JSON Schema)
  inputSchema: JSONSchema

  // 权限控制
  permissions?: {
    requiresApproval?: boolean    // 是否需要用户确认
    allowedAgents?: string[]       // 允许调用的 Agent
  }

  // 执行
  handler: ToolHandler

  // 上下文感知
  contextAware?: {
    fileTypes?: string[]          // 适用的文件类型
    languages?: string[]          // 适用的编程语言
  }
}

interface ToolResult {
  success: boolean
  output?: string
  error?: string
  metadata?: Record<string, any>
}
```

### 4.2 内置工具集

| 工具 | 类别 | 功能 |
|------|------|------|
| `bash` | execution | 执行 Shell 命令 |
| `read` | filesystem | 读取文件内容 |
| `write` | filesystem | 写入文件 |
| `edit` | filesystem | 编辑文件(支持 Hash 锚定) |
| `glob` | filesystem | 文件模式匹配 |
| `grep` | search | 代码搜索 |
| `lsp` | code-analysis | LSP 调用(补全/跳转/诊断) |
| `git` | git | Git 操作封装 |
| `web-search` | web | 网络搜索 |
| `web-fetch` | web | 获取网页内容 |
| `ask-user` | interactive | 请求用户输入 |

### 4.3 Hash 锚定编辑

受 oh-my-pi 启发的精准编辑机制:

```
原始文件:
10#VK| function hello() {
11#XJ|   return "world";
12#MB| }
```

**编辑协议:**

```
EDIT file:auth/session.ts
LINE 10#VK
OLD: function hello() {
NEW: async function hello() {
```

**冲突检测:** 当文件被外部修改后,哈希不匹配时拒绝编辑,提示用户。

---

## 5. 安全与权限设计

### 5.1 权限模型

**三级权限体系:**

| 级别 | 描述 | 示例工具 |
|------|------|----------|
| `safe` | 无需确认自动执行 | glob, grep, read |
| `dangerous` | 执行前需用户确认 | bash, write, delete |
| `critical` | 需要明确批准 | rm -rf, git push --force |

**Agent 权限继承:**

```typescript
interface AgentPermissions {
  agent: AgentType
  allowedTools: string[] | '*'
  deniedTools: string[]
  maxFileSize?: number           // 最大文件操作大小
  allowedPaths?: string[]       // 允许的文件路径
  blockedPaths?: string[]        // 禁止的路径
}

const DEFAULT_PERMISSIONS: Record<AgentType, AgentPermissions> = {
  'coder': { agent: 'coder', allowedTools: '*', deniedTools: ['rm -rf'] },
  'explorer': { agent: 'explorer', allowedTools: ['read', 'glob', 'grep', 'lsp'] },
  'executor': { agent: 'executor', allowedTools: ['bash'], maxFileSize: 1024 * 1024 }
}
```

### 5.2 审批工作流

```typescript
interface ApprovalRequest {
  id: string
  tool: string
  args: Record<string, any>
  reason: string               // Agent 提供的理由
  timestamp: Date
  expiresAt?: Date
}

interface ApprovalResponse {
  requestId: string
  approved: boolean
  modifiedArgs?: Record<string, any>  // 用户可修改参数
  reason?: string
}
```

---

## 6. 工作流编排设计

### 6.1 工作流定义

```typescript
interface Workflow {
  name: string
  description: string
  version: string

  // 工作流阶段
  phases: WorkflowPhase[]

  // 入口条件
  entryCondition?: (ctx: WorkflowContext) => boolean

  // 退出条件
  exitCondition?: (ctx: WorkflowContext) => boolean

  // 错误处理
  errorHandling: {
    maxRetries: number
    onFailure: 'abort' | 'rollback' | 'continue'
  }
}

interface WorkflowPhase {
  name: string
  agent: AgentType

  // 输入来源
  input: {
    fromPhase?: string[]         // 前置阶段输出
    fromContext?: string[]       // 上下文变量
    fromUser?: UserInputSpec     // 用户输入
  }

  // 验证规则
  verification?: VerificationRule[]

  // 失败策略
  onFailure?: {
    fallbackAgent?: AgentType
    skip?: boolean
  }
}
```

### 6.2 内置工作流

**代码生成工作流:**

```
input → analyze(codebase) → plan(approach) → implement(code) → verify(tests) → review(quality)
```

**重构工作流:**

```
input → analyze(impact) → plan(migration) → execute(changes) → verify(breaking) → update(tests)
```

**调试工作流:**

```
input → reproduce(error) → analyze(root-cause) → fix(change) → verify(fix) → confirm(resolved)
```

---

## 7. 配置系统设计

### 7.1 配置文件结构

```json
{
  "$schema": "./schemas/config.json",

  "general": {
    "theme": "default",
    "language": "zh-CN",
    "editor": "vim"
  },

  "providers": {
    "openai": {
      "apiKey": "${OPENAI_API_KEY}",
      "disabled": false,
      "baseUrl": "https://api.openai.com/v1"
    },
    "anthropic": {
      "apiKey": "${ANTHROPIC_API_KEY}",
      "disabled": false
    }
  },

  "agents": {
    "default": {
      "model": "claude-3.7-sonnet",
      "maxTokens": 8192,
      "temperature": 0.7
    },
    "coder": {
      "model": "claude-3.7-sonnet",
      "tools": ["read", "write", "edit", "bash", "grep"]
    },
    "debugger": {
      "model": "claude-3.7-sonnet",
      "tools": ["read", "bash", "grep", "lsp"]
    }
  },

  "tools": {
    "bash": {
      "shell": "pwsh",
      "timeout": 30000,
      "allowedCommands": ["git", "npm", "node", "pnpm"]
    }
  },

  "session": {
    "storage": "~/.agent-cli/sessions",
    "maxHistory": 100,
    "autoCompact": true,
    "compactThreshold": 0.95
  },

  "security": {
    "dangerousToolsRequireApproval": true,
    "allowedPaths": ["${CWD}"],
    "blockedPaths": ["${HOME}/.ssh"]
  },

  "hooks": {
    "preAgent": [],
    "postAgent": [],
    "preTool": [],
    "postTool": []
  }
}
```

### 7.2 环境变量与密钥管理

```typescript
interface SecretProvider {
  // 从环境变量读取
  fromEnv(key: string): string | undefined

  // 从密钥管理器读取 (Windows Credential Manager)
  fromStore(key: string): Promise<string | undefined>

  // 临时写入
  toStore(key: string, value: string): Promise<void>
}

// 支持的密钥管理器
const SECRET_PROVIDERS: SecretProvider[] = [
  new EnvSecretProvider(),
  new WindowsCredentialProvider(),   // Windows DPAPI
  new KeytarProvider()                // Electron secureStorage
]
```

---

## 8. 技术实现路径

### 8.1 技术栈选型

| 层级 | 技术选型 | 理由 |
|------|----------|------|
| CLI 框架 | `commander.js` / `oclif` | 成熟的 Node.js CLI 框架 |
| TUI 交互 | `inquirer` / `enquirer` | 跨平台交互式提示 |
| 状态管理 | `zustand` / `nanostores` | 轻量级响应式状态 |
| LLM 调用 | `@ai-sdk/*` | 统一的 Provider 抽象 |
| 会话存储 | `better-sqlite3` | 高性能 SQLite 绑定 |
| MCP 集成 | `@modelcontextprotocol/sdk` | 官方 MCP 实现 |
| LSP 集成 | `vscode-languageserver-protocol` | 标准 LSP 协议 |
| 工具执行 | `node-pty` | 真正的 PTY 支持 |
| 配置文件 | `jsonc-parser` | JSONC 注释支持 |

### 8.2 项目结构

```
src/
├── cli/                      # 命令行入口
│   ├── index.ts              # 主入口
│   ├── commands/             # 命令定义
│   │   ├── agent.ts
│   │   ├── session.ts
│   │   ├── config.ts
│   │   └── tool.ts
│   └── completion.ts         # Tab 补全
├── core/                     # 核心逻辑
│   ├── agent/
│   │   ├── AgentCore.ts      # Agent 编排器
│   │   ├── IntentGate.ts     # 意图分析
│   │   ├── Planner.ts        # 任务规划
│   │   └── agents/           # 内置 Agent 实现
│   ├── session/
│   │   ├── SessionManager.ts
│   │   ├── MessageStore.ts
│   │   └── CompactService.ts  # 上下文压缩
│   ├── workflow/
│   │   ├── WorkflowEngine.ts
│   │   └── PhaseRunner.ts
│   └── security/
│       ├── PermissionService.ts
│       └── ApprovalWorkflow.ts
├── providers/               # LLM Provider
│   ├── base.ts
│   ├── anthropic.ts
│   ├── openai.ts
│   ├── gemini.ts
│   └── local.ts
├── tools/                   # 工具系统
│   ├── registry.ts
│   ├── handlers/            # 工具处理器
│   │   ├── bash.ts
│   │   ├── filesystem.ts
│   │   ├── grep.ts
│   │   └── lsp.ts
│   └── hash-anchor.ts       # Hash 锚定编辑
├── mcp/                     # MCP 集成
│   ├── client.ts
│   ├── server.ts
│   └── servers/             # 内置 MCP 服务器
├── config/                  # 配置管理
│   ├── loader.ts
│   ├── validator.ts
│   └── secrets.ts
├── tui/                     # TUI 组件
│   ├── renderer.ts
│   ├── input.ts
│   └── components/
└── utils/
    ├── logger.ts
    └── ansi.ts
```

### 8.3 开发阶段规划

**Phase 1: 核心框架**

- CLI 命令行框架搭建
- Session 管理基础实现
- 单 Provider LLM 调用

**Phase 2: Agent 系统**

- Intent Gate 意图分析
- 基础 Agent 实现(Coder/Explorer)
- 工具系统基础版

**Phase 3: 高级功能**

- 多 Agent 编排
- 工作流引擎
- MCP 集成

**Phase 4: 安全与生产**

- 权限系统完善
- 审批工作流
- 配置加密

---

## 9. 参考资源

| 项目 | 仓库 | 说明 |
|------|------|------|
| ACP 协议 | [zed-industries/agent-client-protocol](https://github.com/zed-industries/agent-client-protocol) | Zed 发起的标准化协议 |
| GitHub Copilot ACP | [github/copilot-cli](https://github.com/github/copilot-cli) | GitHub Copilot CLI ACP 实现 |
| Oh My OpenCode | [code-yeongyu/oh-my-openagent](https://github.com/code-yeongyu/oh-my-openagent) | OpenCode 插件,多模型编排 |
| Superpowers | [obra/superpowers](https://github.com/obra/superpowers) | Claude Code 插件系统,Skills 工作流 |
| Get Shit Done | [GSD-build/get-shit-done](https://github.com/GSD-build/get-shit-done) | Spec-driven 开发工作流 |
| PUA Skill | [tanweai/pua](https://github.com/tanweai/pua) | 高能动性模式,防懒癌,压力升级 |
| Claude Reflect System | [haddock-development/claude-reflect-system](https://github.com/haddock-development/claude-reflect-system) | Self-Improving 永久记忆 |
| Self-Tune | [WellDunDun/selftune](https://github.com/WellDunDun/selftune) | Skill 自观察改进 |
| ACE | [kayba-ai/agentic-context-engine](https://github.com/kayba-ai/agentic-context-engine) | Agent 上下文学习引擎 |

---

*文档版本: 1.4.0*
*创建日期: 2026-04-01*
*整合 PUA Skill v3 High-Agency + Methodology Router 设计*
