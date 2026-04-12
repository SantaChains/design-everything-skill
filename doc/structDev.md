# Agent 工作流程架构设计文档

## 概述

本文档整合 claw-code 的 harness 工程设计理念与 gemini-cli 的 agent 运行时架构，提炼最优 agent 工作流程。核心原则：

- **状态持久化**：上下文存在文件，不在内存
- **模块化隔离**：命令/工具/技能解耦
- **事件驱动**：统一事件总线
- **GSD 驱动**：搜索→学习→执行→交付

---

## 一、核心架构分层

### 1.1 系统层次图

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI Entry (main.py)                     │
├─────────────────────────────────────────────────────────────┤
│                    Session Management                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ HistoryLog  │  │Transcript   │  │ SessionStore        │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    Query Engine (Runtime)                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ RoutePrompt │  │TurnLoop     │  │ BootstrapSession     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    Tool/Command Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ PortTools   │  │PortCommands │  │ ExecutionRegistry    │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    Subsystem Modules                         │
│  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────────┐ │
│  │assistant│ │bootstrap│ │bridge │ │buddy   │ │coordinator │ │
│  └────────┘ └────────┘ └────────┘ └────────┘ └────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 gemini-cli Agent 事件协议

```
initialize → session_update → message → agent_start → [tool_request/tool_update/tool_response]* → agent_end
```

关键事件类型：
| 事件 | 功能 | 说明 |
|------|------|------|
| `initialize` | 初始化会话 | 绑定 sessionId, workspace, agentId |
| `session_update` | 更新配置 | 修改 model, title, config |
| `message` | 消息内容 | user/agent/developer 角色 |
| `agent_start` | 启动代理 | streamId 标识 |
| `tool_request` | 工具请求 | requestId + name + args |
| `tool_update` | 工具更新 | 中间状态（子代理/长运行） |
| `tool_response` | 工具响应 | isError + content/data |
| `agent_end` | 结束代理 | reason: completed/failed/aborted/max_turns/max_budget |
| `elicitation_request` | 用户确认请求 | inline/modal 确认 |

---

## 二、命令/工具路由系统

### 2.1 双重镜像架构 (claw-code)

```python
# 命令镜像
PORTED_COMMANDS = load_command_snapshot()  # 从 reference_data 加载

# 工具镜像
PORTED_TOOLS = load_tool_snapshot()        # 从 reference_data 加载
```

### 2.2 Prompt 路由逻辑

```python
class PortRuntime:
    def route_prompt(self, prompt: str, limit: int = 5) -> list[RoutedMatch]:
        # 1. 分词 + 标准化
        tokens = {token.lower() for token in
                  prompt.replace('/', ' ').replace('-', ' ').split()}

        # 2. 收集命令/工具匹配
        by_kind = {
            'command': self._collect_matches(tokens, PORTED_COMMANDS, 'command'),
            'tool':    self._collect_matches(tokens, PORTED_TOOLS, 'tool'),
        }

        # 3. 优先级选择（命令 > 工具）
        selected: list[RoutedMatch] = []
        for kind in ('command', 'tool'):
            if by_kind[kind]:
                selected.append(by_kind[kind].pop(0))

        # 4. 分数排序补足
        leftovers = sorted(
            [m for matches in by_kind.values() for m in matches],
            key=lambda item: (-item.score, item.kind, item.name),
        )
        selected.extend(leftovers[: max(0, limit - len(selected))])
        return selected[:limit]

    @staticmethod
    def _score(tokens: set[str], module: PortingModule) -> int:
        haystacks = [module.name.lower(),
                     module.source_hint.lower(),
                     module.responsibility.lower()]
        return sum(1 for token in tokens if any(token in h for h in haystacks))
```

### 2.3 权限上下文

```python
@dataclass(frozen=True)
class PermissionDenial:
    tool_name: str
    reason: str

class ToolPermissionContext:
    def blocks(self, tool_name: str) -> bool: ...

# 使用示例
tools = filter_tools_by_permission_context(
    PORTED_TOOLS,
    ToolPermissionContext.from_iterables(deny_tool=['BashTool'], deny_prefix=['mcp_'])
)
```

---

## 三、执行与状态管理

### 3.1 Turn Loop 执行流

```python
@dataclass
class QueryEnginePort:
    manifest: PortManifest
    config: QueryEngineConfig
    session_id: str
    mutable_messages: list[str]           # 对话历史
    permission_denials: list[PermissionDenial]
    total_usage: UsageSummary
    transcript_store: TranscriptStore

    def submit_message(self, prompt: str, ...):
        # 1. 检查 max_turns
        if len(self.mutable_messages) >= self.config.max_turns:
            return TurnResult(stop_reason='max_turns_reached')

        # 2. 记录消息
        self.mutable_messages.append(prompt)
        self.transcript_store.append(prompt)

        # 3. 计算使用量
        projected_usage = self.total_usage.add_turn(prompt, output)
        stop_reason = 'completed'
        if projected_usage.input_tokens + projected_usage.output_tokens > self.config.max_budget_tokens:
            stop_reason = 'max_budget_reached'

        # 4. 压缩历史（防止 context rot）
        self.compact_messages_if_needed()

        return TurnResult(..., stop_reason=stop_reason)

    def compact_messages_if_needed(self):
        if len(self.mutable_messages) > self.config.compact_after_turns:
            self.mutable_messages[:] = self.mutable_messages[-self.config.compact_after_turns:]
        self.transcript_store.compact(self.config.compact_after_turns)
```

### 3.2 会话持久化

```python
@dataclass
class StoredSession:
    session_id: str
    messages: tuple[str, ...]
    input_tokens: int
    output_tokens: int

def save_session(session: StoredSession) -> Path: ...
def load_session(session_id: str) -> StoredSession: ...

# 使用
engine.persist_session()  # 保存到磁盘
QueryEnginePort.from_saved_session(session_id)  # 从磁盘恢复
```

---

## 四、Bootstrap 与初始化

### 4.1 启动流程

```
build_port_manifest() → run_setup() → build_port_context()
                                            ↓
                           ┌────────────────────────────────┐
                           │ WorkspaceSetup                 │
                           │ - python_version               │
                           │ - platform_name                │
                           │ - test_command                │
                           │ - startup_steps()             │
                           └────────────────────────────────┘
                                            ↓
                           build_system_init_message(trusted=True)
```

### 4.2 子系统注册表

```python
@dataclass(frozen=True)
class Subsystem:
    name: str
    path: str
    file_count: int
    notes: str

@dataclass
class PortManifest:
    top_level_modules: list[Subsystem]
    # 生成 markdown 报告
    def to_markdown(self) -> str: ...
```

---

## 五、GSD + Superpowers 最优工作流

### 5.1 核心原则

| 原则 | 说明 |
|------|------|
| **状态在文件** | Context 存储在文件系统，跨会话持久化 |
| **最小化上下文** | 仅传递必要信息，最大化交付 |
| **Context Rot 解决** | 每次新会话从文件恢复状态 |
| **RED-GREEN-REFACTOR** | TDD 循环：先测试后代码 |
| **2-5 分钟原子任务** | 小任务快速迭代 |

### 5.2 搜索→学习→执行→交付 工作流

```
用户输入
    ↓
理解意图（≤30秒）
    ↓
精准搜索（≤3次）
    ↓
快速学习（≤5分钟）
    ↓
立即执行（MVP优先）
    ↓
持续交付（永不停止）
```

### 5.3 Agent 特化分工

| Agent | 功能 | 适用场景 |
|-------|------|---------|
| `@superpower-agent` | GSD+Superpowers 整合 | 搜索+执行一体化 |
| `@gsd-executor-agent` | 快速决策+执行 | MVP 交付 |
| `@super-search-agent` | 深度搜索+学习 | 研究分析 |
| `@optimized-dev-agent` | 架构优化 | 重构/性能 |

### 5.4 性能指标

| 模式 | 搜索次数 | 总时间 | 交付标准 |
|------|---------|--------|---------|
| `/super-search` | ≤2次 | ≤2分钟 | 直接答案 |
| `/gsd` | ≤3次 | ≤15分钟 | MVP完成 |
| `/research` | ≤5次 | ≤10分钟 | 可执行方案 |

---

## 六、Skills 系统架构

### 6.1 Skill 定义结构

```
SKILL.md
├── name          # 技能名称
├── description   # 触发条件描述
├── instructions  # 执行指令
├── tools         # 可用工具列表
└── examples      # 使用示例
```

### 6.2 内置 Skills

| Skill | 功能 |
|-------|------|
| `code-reviewer` | 代码审查 |
| `docs-writer` | 文档编写 |
| `pr-creator` | PR 创建 |
| `async-pr-review` | 异步 PR 审查 |
| `string-reviewer` | 字符串审查 |
| `ci` | CI/CD 集成 |
| `docs-changelog` | 变更日志 |

---

## 七、远程 Agent 与 MCP

### 7.1 远程 Agent 类型

```python
type AgentDefinition<TOutput> =
  | LocalAgentDefinition<TOutput>   # 本地执行
  | RemoteAgentDefinition<TOutput>   # 远程 A2A

interface RemoteAgentDefinition:
  agentCardUrl?: string
  agentCardJson?: string
  auth?: A2AAuthConfig
```

### 7.2 MCP 集成

```python
interface MCPServerConfig:
  command: string
  args: string[]
  env?: Record<string, string>
```

---

## 八、Policy 引擎

### 8.1 确认策略

```python
@dataclass
class ToolPermissionContext:
    deny_tool: tuple[str, ...]      # 明确拒绝的工具
    deny_prefix: tuple[str, ...]    # 前缀匹配的拒绝

    def blocks(self, tool_name: str) -> bool:
        if tool_name in self.deny_tool:
            return True
        return any(tool_name.startswith(p) for p in self.deny_prefix)
```

### 8.2 Elicitation 确认

```typescript
interface ElicitationRequest {
  display: 'inline' | 'modal'
  title?: string
  requestId: string
  message: string
  requestedSchema: Record<string, unknown>
}

type ElicitationResponse = {
  requestId: string
  action: 'accept' | 'decline' | 'cancel'
  content: Record<string, unknown>
}
```

---

## 九、最优 Agent 工作流程实现

### 9.1 标准执行流程

```python
# 1. 初始化
engine = QueryEnginePort.from_workspace()

# 2. 路由匹配
matches = PortRuntime().route_prompt(prompt, limit=5)

# 3. 权限过滤
denials = PortRuntime()._infer_permission_denials(matches)

# 4. 提交消息
result = engine.submit_message(
    prompt,
    matched_commands=tuple(m.name for m in matches if m.kind == 'command'),
    matched_tools=tuple(m.name for m in matches if m.kind == 'tool'),
    denied_tools=denials,
)

# 5. 检查停止原因
if result.stop_reason != 'completed':
    print(f"Stopped: {result.stop_reason}")

# 6. 持久化
session_path = engine.persist_session()
```

### 9.2 Turn Loop 模式

```python
def run_turn_loop(prompt: str, max_turns: int = 3):
    engine = QueryEnginePort.from_workspace()
    engine.config = QueryEngineConfig(max_turns=max_turns)

    for turn in range(max_turns):
        turn_prompt = prompt if turn == 0 else f'{prompt} [turn {turn + 1}]'
        result = engine.submit_message(turn_prompt, ...)

        if result.stop_reason != 'completed':
            break

        # 自动压缩历史
        engine.compact_messages_if_needed()
```

---

## 十一、设计巧思与可取点

### 11.1 快照缓存模式 `@lru_cache(maxsize=1)`

**来源**: claw-code `commands.py`, `tools.py`

```python
@lru_cache(maxsize=1)
def load_command_snapshot() -> tuple[PortingModule, ...]:
    raw_entries = json.loads(SNAPSHOT_PATH.read_text())
    return tuple(
        PortingModule(...)
        for entry in raw_entries
    )

PORTED_COMMANDS = load_command_snapshot()
```

**巧思**: 使用 `@lru_cache(maxsize=1)` 确保快照数据只从磁盘读取一次，后续所有访问直接从缓存返回。`maxsize=1` 精确表达了"只缓存一份"的意图。

**可取点**: 适用于配置、快照、只读数据的单例加载场景。

---

### 11.2 不可变数据模型 `frozen=True`

**来源**: claw-code `models.py`

```python
@dataclass(frozen=True)
class PortingModule:
    name: str
    responsibility: str
    source_hint: str
    status: str = 'planned'

@dataclass(frozen=True)
class UsageSummary:
    input_tokens: int = 0
    output_tokens: int = 0

    def add_turn(self, prompt: str, output: str) -> 'UsageSummary':
        return UsageSummary(
            input_tokens=self.input_tokens + len(prompt.split()),
            output_tokens=self.output_tokens + len(output.split()),
        )
```

**巧思**: `frozen=True` 使 dataclass 不可变，所有"修改"都通过 `add_turn()` 返回新实例实现。这使得 `UsageSummary` 天然线程安全，且能追踪完整的历史变化。

**可取点**: 状态历史、审计日志、统计数据的不可变累加模式。

---

### 11.3  TranscriptStore 简单压缩

**来源**: claw-code `transcript.py`

```python
@dataclass
class TranscriptStore:
    entries: list[str] = field(default_factory=list)
    flushed: bool = False

    def compact(self, keep_last: int = 10) -> None:
        if len(self.entries) > keep_last:
            self.entries[:] = self.entries[-keep_last:]
```

**巧思**: 不同于复杂的滑动窗口实现，直接用切片 `self.entries[:] = self.entries[-keep_last:]` 原地替换。`flushed` 标记持久化状态。

**可取点**: 对话历史、事件日志的自动压缩。

---

### 11.4 HistoryLog 事件日志

**来源**: claw-code `history.py`

```python
@dataclass
class HistoryLog:
    events: list[HistoryEvent] = field(default_factory=list)

    def add(self, title: str, detail: str) -> None:
        self.events.append(HistoryEvent(title=title, detail=detail))
```

**巧思**: 简单的 `(title, detail)` 二元组记录所有关键事件，最后统一渲染为 markdown。零开销的日志系统。

**可取点**: 调试跟踪、操作审计、执行摘要生成。

---

### 11.5 Prefetch 副作用预热

**来源**: claw-code `prefetch.py`

```python
@dataclass(frozen=True)
class PrefetchResult:
    name: str
    started: bool
    detail: str

def start_mdm_raw_read() -> PrefetchResult:
    return PrefetchResult('mdm_raw_read', True, 'Simulated MDM raw-read prefetch')

def start_keychain_prefetch() -> PrefetchResult:
    return PrefetchResult('keychain_prefetch', True, 'Simulated keychain prefetch')
```

**巧思**: 将启动时的副作用（读 keychain、扫描项目）包装为 `PrefetchResult`，记录"已开始"状态而非阻塞等待。异步非阻塞初始化。

**可取点**: 启动预热、后台任务提交、依赖异步初始化。

---

### 11.6 DeferredInit 延迟初始化

**来源**: claw-code `deferred_init.py`

```python
def run_deferred_init(trusted: bool) -> DeferredInitResult:
    enabled = bool(trusted)
    return DeferredInitResult(
        trusted=trusted,
        plugin_init=enabled,
        skill_init=enabled,
        mcp_prefetch=enabled,
        session_hooks=enabled,
    )
```

**巧思**: `trusted` 模式控制所有延迟初始化的组件开关。信任模型决定功能开关，简洁的安全门控。

**可取点**: 信任模式切换、特性开关、延迟加载。

---

### 11.7 CommandGraph 三类分类

**来源**: claw-code `command_graph.py`

```python
def build_command_graph() -> CommandGraph:
    commands = get_commands()
    builtins = tuple(m for m in commands if 'plugin' not in m.source_hint.lower()
                                            and 'skills' not in m.source_hint.lower())
    plugin_like = tuple(m for m in commands if 'plugin' in m.source_hint.lower())
    skill_like = tuple(m for m in commands if 'skills' in m.source_hint.lower())
    return CommandGraph(builtins=builtins, plugin_like=plugin_like, skill_like=skill_like)
```

**巧思**: 按来源将命令分为 `builtins`、`plugin_like`、`skill_like` 三类。`flattened()` 方法可按需合并。

**可取点**: 多源数据分类、插件系统命令隔离。

---

### 11.8 AgentRegistry 多层加载

**来源**: gemini-cli `registry.ts`

```typescript
private async loadAgents(): Promise<void> {
    this.loadBuiltInAgents();                                    // 1. 内置
    const userAgents = await loadAgentsFromDirectory(userAgentsDir); // 2. 用户级 ~/.gemini/agents/
    const projectAgents = await loadAgentsFromDirectory(projectAgentsDir); // 3. 项目级 .gemini/agents/
    for (const extension of this.config.getExtensions()) {      // 4. 扩展
        if (extension.isActive && extension.agents) {
            await this.registerAgent(extension.agents);
        }
    }
}
```

**巧思**: 四层加载优先级：`built-in → user → project → extension`。同一名称可覆盖（`overriding`）。支持 `reload()` 热重载。

**可取点**: 插件系统、多租户配置、分层扩展。

---

### 11.9 模板字符串 `${...}` 占位符

**来源**: gemini-cli `agents/utils.ts`

```typescript
export function templateString(template: string, inputs: AgentInputs): string {
  const placeholderRegex = /\$\{(\w+)\}/g;
  const requiredKeys = new Set(
    Array.from(template.matchAll(placeholderRegex), (match) => match[1])
  );
  const missingKeys = Array.from(requiredKeys).filter(k => !Object.keys(inputs).includes(k));
  if (missingKeys.length > 0) {
    throw new Error(`Missing required input parameters: ${missingKeys.join(', ')}`);
  }
  return template.replace(placeholderRegex, (_match, key) => String(inputs[key]));
}
```

**巧思**: `${...}` 占位符模板系统，支持模板验证 + 友好的错误提示（列出缺失的 key 和可用的 key）。

**可取点**: 系统提示词模板化、Agent 输入配置、动态命令生成。

---

### 11.10 DeclarativeTool 验证-执行分离

**来源**: gemini-cli `tools.ts`

```typescript
export abstract class DeclarativeTool<TParams, TResult> {
  abstract build(params: TParams): ToolInvocation<TParams, TResult>;

  async buildAndExecute(params: TParams, signal: AbortSignal, ...): Promise<TResult> {
    const invocation = this.build(params);  // 先验证
    return invocation.execute(signal, ...); // 再执行
  }
}

export abstract class BaseDeclarativeTool<TParams, TResult> extends DeclarativeTool {
  build(params: TParams): ToolInvocation<TParams, TResult> {
    const validationError = this.validateToolParams(params);
    if (validationError) {
      throw new Error(validationError);
    }
    return this.createInvocation(params, ...);
  }
}
```

**巧思**: `build()` 负责验证参数 + 创建 invocation，`execute()` 负责实际执行。验证失败直接抛错，不会污染执行环境。

**可取点**: 工具系统重构、命令验证、参数校验。

---

### 11.11 MessageBus 请求-响应模式

**来源**: gemini-cli `message-bus.ts`

```typescript
async request<TRequest extends Message, TResponse extends Message>(
    request: Omit<TRequest, 'correlationId'>,
    responseType: TResponse['type'],
    timeoutMs: number = 60000,
): Promise<TResponse> {
    const correlationId = randomUUID();
    return new Promise<TResponse>((resolve, reject) => {
        const timeoutId = setTimeout(() => { cleanup(); reject(new Error(`Request timed out`)); }, timeoutMs);
        const responseHandler = (response: TResponse) => {
            if ('correlationId' in response && response.correlationId === correlationId) {
                cleanup(); resolve(response);
            }
        };
        this.subscribe<TResponse>(responseType, responseHandler);
        this.publish({ ...request, correlationId } as TRequest);
    });
}
```

**巧思**: 在事件总线上实现请求-响应模式，通过 `correlationId` 关联请求和响应。支持超时取消。

**可取点**: 事件总线增强、同步通信需求、异步回调扁平化。

---

### 11.12 MessageBus.derive() 子代理作用域

**来源**: gemini-cli `message-bus.ts`

```typescript
derive(subagentName: string): MessageBus {
    const bus = new MessageBus(this.policyEngine, this.debug);
    bus.publish = async (message: Message) => {
        if (message.type === MessageBusType.TOOL_CONFIRMATION_REQUEST) {
            return this.publish({
                ...message,
                subagent: message.subagent ? `${subagentName}/${message.subagent}` : subagentName,
            });
        }
        return this.publish(message);
    };
    // 订阅方法委托给父总线
    bus.subscribe = this.subscribe.bind(this);
    bus.on = this.on.bind(this);
    return bus;
}
```

**巧思**: 子代理的消息总线自动为所有 `tool_request` 注入 `subagent` 前缀，实现调用链追踪。订阅方法委托给父总线，保持事件传递。

**可取点**: 多租户隔离、调用链追踪、子代理通信。

---

### 11.13 PolicyEngine 分数优先级

**来源**: gemini-cli `policy/types.ts`

```typescript
export const PRIORITY_SUBAGENT_TOOL = 1.05;
export const ALWAYS_ALLOW_PRIORITY_FRACTION = 950;
export const PRIORITY_YOLO_ALLOW_ALL = 998;

interface PolicyRule {
    decision: PolicyDecision;
    priority?: number;  // 默认 0
    modes?: ApprovalMode[];
    interactive?: boolean;
}
```

**巧思**: 策略规则支持多层优先级：`priority` 数值越大优先级越高。支持 `modes`（autoEdit/plan/yolo）和 `interactive` 过滤。

**可取点**: 规则引擎、多维度匹配、策略优先级。

---

### 11.14 SchemaValidator + 循环引用检测

**来源**: gemini-cli `tools.ts`

```typescript
export function hasCycleInSchema(schema: object): boolean {
    function resolveRef(ref: string): object | null {
        if (!ref.startsWith('#/')) return null;
        const path = ref.substring(2).split('/');
        let current: unknown = schema;
        for (const segment of path) {
            if (typeof current !== 'object' || !Object.prototype.hasOwnProperty.call(current, segment)) {
                return null;
            }
            current = (current as Record<string, unknown>)[segment];
        }
        return current as object;
    }
    // DFS 检测循环
    function traverse(node: unknown, visitedRefs: Set<string>, pathRefs: Set<string>): boolean {
        if ('$ref' in node && typeof node.$ref === 'string') {
            const ref = node.$ref;
            if (ref === '#' || pathRefs.has(ref)) return true; // 循环!
            if (visitedRefs.has(ref)) return false;
            const resolved = resolveRef(ref);
            if (resolved) {
                visitedRefs.add(ref); pathRefs.add(ref);
                const hasCycle = traverse(resolved, visitedRefs, pathRefs);
                pathRefs.delete(ref);
                return hasCycle;
            }
        }
        // 递归所有属性...
    }
    return traverse(schema, new Set(), new Set());
}
```

**巧思**: `$ref` 循环引用检测（`#` 自引用、`#/` 空引用、路径循环）。使用 `visitedRefs` + `pathRefs` 双 Set 实现 DFS 环路检测。

**可取点**: JSON Schema 验证、配置校验、递归结构检测。

---

### 11.15 toolLocations() 影响路径追踪

**来源**: gemini-cli `tools.ts`

```typescript
export interface ToolLocation {
    path: string;   // 文件绝对路径
    line?: number;  // 行号
}

interface ToolInvocation<TParams, TResult> {
    toolLocations(): ToolLocation[];
}
```

**巧思**: 工具执行前声明自己会影响哪些文件路径，Policy 引擎可据此做细粒度路径限制（allowed-path checker）。

**可取点**: 文件系统安全沙箱、路径白名单、影响分析。

---

### 11.16 wait_for_previous 显式并行控制

**来源**: gemini-cli `tools.ts`

```typescript
private addWaitForPreviousParameter(schema: unknown): unknown {
    return {
        ...schema,
        properties: {
            ...propertiesObj,
            wait_for_previous: {
                type: 'boolean',
                description: 'Set to true to wait for all previously requested tools in this turn to complete before starting.',
            },
        },
    };
}
```

**巧思**: 显式声明工具间的依赖关系。`wait_for_previous=true` 确保前序工具完成再开始，实现顺序控制。

**可取点**: 工具并行/串行控制、依赖声明、工作流编排。

---

### 11.17 ToolConfirmationOutcome 多结果确认

**来源**: gemini-cli `tools.ts`

```typescript
export enum ToolConfirmationOutcome {
    ProceedOnce = 'proceed_once',
    ProceedAlways = 'proceed_always',
    ProceedAlwaysAndSave = 'proceed_always_and_save',
    ProceedAlwaysServer = 'proceed_always_server',
    ProceedAlwaysTool = 'proceed_always_tool',
    ModifyWithEditor = 'modify_with_editor',
    Cancel = 'cancel',
}
```

**巧思**: 细粒度的用户确认结果：`ProceedOnce`（本次）、`ProceedAlways`（本次+保存）、`ModifyWithEditor`（用编辑器修改）、`Cancel`。

**可取点**: 用户确认UI、多级权限、永久vs临时授权。

---

### 11.18 HookSystem 钩子系统

**来源**: gemini-cli `hooks/index.ts`

```typescript
export class HookSystem {
    async runHook(event: HookEventName, context: HookEventContext): Promise<void> {
        const handlers = this.registry.getHandlers(event, context.hookSource);
        for (const handler of handlers) {
            await handler.execute(context);
        }
    }
}
```

**巧思**: 事件驱动的钩子系统，支持 `project/user/system/extension` 四种来源。钩子可修改上下文、添加安全检查。

**可取点**: 插件生命周期、事件拦截、审计日志。

---

### 11.19 AgentRegistry.reload() 热重载

**来源**: gemini-cli `registry.ts`

```typescript
async reload(): Promise<void> {
    this.config.getA2AClientManager()?.clearCache();
    await this.config.reloadAgents();
    this.agents.clear();
    this.allDefinitions.clear();
    await this.loadAgents();
    coreEvents.emitAgentsRefreshed();
}
```

**巧思**: 清除缓存 + 清空注册表 + 重新加载 + 发送刷新事件。完整的热重载流程。

**可取点**: 配置变更监听、动态插件加载、运行时更新。

---

### 11.20 模型配置合并

**来源**: gemini-cli `registry.ts`

```typescript
get runConfig() {
    return overrides.runConfig
        ? { ...definition.runConfig, ...overrides.runConfig }
        : definition.runConfig;
}
get modelConfig() {
    return overrides.modelConfig
        ? ModelConfigService.merge(definition.modelConfig, overrides.modelConfig)
        : definition.modelConfig;
}
```

**巧思**: 使用展开运算符和安全合并（`ModelConfigService.merge`）实现配置覆盖，而非直接替换。

**可取点**: 配置继承、覆盖策略、默认配置合并。

---

## 十二、可取点清单

| 设计模式 | 来源 | 适用场景 |
|---------|------|---------|
| `@lru_cache(maxsize=1)` | claw-code | 单例只读数据加载 |
| `frozen=True` dataclass | claw-code | 不可变状态、历史追踪 |
| TranscriptStore 切片压缩 | claw-code | 对话历史自动截断 |
| HistoryLog 事件日志 | claw-code | 调试跟踪、操作审计 |
| PrefetchResult 预热 | claw-code | 异步副作用初始化 |
| DeferredInit 延迟初始化 | claw-code | 信任模式功能开关 |
| CommandGraph 三类分类 | claw-code | 多源命令隔离 |
| AgentRegistry 多层加载 | gemini-cli | 插件系统、多租户 |
| `${...}` 模板占位符 | gemini-cli | 提示词动态化 |
| DeclarativeTool 验证-执行分离 | gemini-cli | 工具安全重构 |
| MessageBus 请求-响应 | gemini-cli | 事件总线同步调用 |
| MessageBus.derive() 子代理作用域 | gemini-cli | 调用链追踪 |
| PolicyEngine 分数优先级 | gemini-cli | 规则引擎 |
| SchemaValidator 循环检测 | gemini-cli | JSON Schema 安全 |
| toolLocations() 影响追踪 | gemini-cli | 文件系统沙箱 |
| wait_for_previous 并行控制 | gemini-cli | 工具依赖声明 |
| ToolConfirmationOutcome 多结果 | gemini-cli | 细粒度用户确认 |
| HookSystem 钩子系统 | gemini-cli | 事件拦截扩展 |
| AgentRegistry.reload() 热重载 | gemini-cli | 动态插件加载 |
| 模型配置合并 | gemini-cli | 配置覆盖继承 |

---

## 十三、精简重写指南：阻塞诊断与优化

### 13.1 常见阻塞点诊断

| 阻塞点 | 症状 | 根因 | 解决方案 |
|-------|------|------|---------|
| **Policy 检查冗余** | `MessageBus.publish` + `Scheduler.checkPolicy` 双重检查 | 设计遗留 | 合并为单一检查点 |
| **状态机过渡复杂** | `transitionCall` 20+ switch 分支 | 类型安全过度设计 | 使用 `status: string` + 运行时验证 |
| **Confirmation 循环** | `resolveConfirmation` 嵌套过深 | 确认流程分层过多 | 扁平化为单层状态机 |
| **Hook 系统滥用** | 每个工具调用触发 3+ Hook | 过度可观测性 | 按需触发，非全局 |
| **Schema 循环检测** | 每次工具构建都检测 `$ref` 循环 | 不必要的防御 | 缓存检测结果 |
| **MessageBus 订阅泄漏** | `subscribedMessageBuses` WeakSet 误判 | 弱引用计数问题 | 显式 `unsubscribe` |

---

### 13.2 Scheduler 三阶段简化

**原始流程（复杂）：**
```
_startBatch
  → _validateAndCreateToolCall (每个工具)
  → state.enqueue
  → _processQueue (while 循环)
    → _processNextItem
      → _processValidatingCall (Hook + Policy + Confirmation)
      → _execute (可能递归)
```

**精简流程：**
```typescript
// 阶段 1: 批量验证
async schedule(requests: ToolCallRequestInfo[], signal: AbortSignal) {
  const validated = requests
    .map(r => this.toolRegistry.getTool(r.name))
    .filter(t => t !== null)
    .map(t => t!.build(r.args));  // 失败直接抛错

  // 阶段 2:  Policy 一票通过
  const decisions = await this.policyEngine.batchCheck(validated);

  // 阶段 3: 并行执行 + 扁平状态
  return Promise.all(
    validated.map((invocation, i) =>
      decisions[i] === 'deny' ? errorResult(invocation) : execute(invocation)
    )
  );
}
```

---

### 13.3 状态机简化方案

**原始 7 状态：**
```
Validating → AwaitingApproval → Scheduled → Executing → Success/Error/Cancelled
```

**精简 3 状态：**
```typescript
type ToolStatus = 'pending' | 'running' | 'done';

interface ToolCall {
  id: string;
  status: ToolStatus;
  result?: ToolResult;  // done 时填充
  error?: Error;        // done 时填充
}
```

**状态转换：**
```typescript
function transition(call: ToolCall, event: Event): ToolCall {
  switch (`${call.status}:${event.type}`) {
    case 'pending:execute': return { ...call, status: 'running' };
    case 'running:complete': return { ...call, status: 'done', result: event.result };
    case 'running:fail': return { ...call, status: 'done', error: event.error };
    default: return call;
  }
}
```

---

### 13.4 扁平化 Confirmation 流程

**原始 Confirmation 流程（4 层嵌套）：**
```
resolveConfirmation
  → ConfirmationStateMachine
    → getConfirmationDetails (UI)
    → Modifier.handle
    → updatePolicy
```

**精简版（单层 + 结果回调）：**
```typescript
async function confirm(tool: ToolInvocation): Promise<ConfirmationResult> {
  const decision = await policyEngine.check(tool);

  if (decision === 'allow') return { action: 'proceed' };
  if (decision === 'deny') return { action: 'deny', reason: 'policy' };

  // 扁平确认
  const userChoice = await ui.showConfirmationDialog(tool.getDescription());
  return userChoice === 'approve'
    ? { action: 'proceed' }
    : { action: 'deny', reason: 'user' };
}
```

---

### 13.5 消除冗余 Hook 检查

**问题：** gemini-cli 中每个工具调用都触发多个 Hook（before_tool, after_tool, around_tool），即使工具很简单。

**精简方案：** 按需触发 + 缓存结果
```typescript
class HookManager {
  private cache = new Map<string, HookResult>();

  async evaluate(tool: ToolInvocation, stage: HookStage): Promise<HookResult> {
    const cacheKey = `${tool.name}:${stage}`;

    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey)!;
    }

    const handlers = this.registry.getHandlers(tool.name, stage);
    if (handlers.length === 0) {
      return { modified: false };
    }

    const result = await this.runHandlers(handlers, tool);
    this.cache.set(cacheKey, result);  // 缓存结果
    return result;
  }
}
```

---

### 13.6 批量操作优化

**原始：** 逐个检查 Policy → 逐个执行
```typescript
for (const request of requests) {
  await policy.check(request);   // N 次 API 调用
  await execute(request);         // N 次执行
}
```

**优化：** 批量检查 + 并行执行
```typescript
// 1. 批量 Policy 检查（一次调用）
const decisions = await policy.batchCheck(requests);

// 2. 按决策分组
const [toExecute, toDeny] = partition(requests, (_, i) => decisions[i] === 'allow');

// 3. 并行执行允许的
const results = await Promise.all(toExecute.map(execute));

// 4. 填充拒绝的
return [...results, ...toDeny.map(r => deniedResult(r, 'policy'))];
```

---

### 13.7 Context 压缩优化

**gemini-cli 的 ContextCompressor 过于复杂（200+ 行）。精简版：**

```typescript
class SimpleCompressor {
  compress(messages: Message[], maxTokens: number): Message[] {
    const result: Message[] = [];
    let tokens = 0;

    for (let i = messages.length - 1; i >= 0; i--) {
      const msg = messages[i];
      const msgTokens = this.estimateTokens(msg.content);

      if (tokens + msgTokens <= maxTokens) {
        result.unshift(msg);
        tokens += msgTokens;
      } else if (msgTokens > 200) {
        // 压缩而非丢弃
        const compressed = this.compressTo(msg.content, maxTokens - tokens);
        if (compressed.length > 0) {
          result.unshift({ ...msg, content: compressed });
          tokens += this.estimateTokens(compressed);
        }
        break;
      }
    }

    return result;
  }

  private estimateTokens(text: string): number {
    return Math.ceil(text.length / 4);
  }

  private compressTo(content: string, maxTokens: number): string {
    const maxChars = maxTokens * 4;
    if (content.length <= maxChars) return content;
    return content.slice(0, maxChars) + '... [truncated]';
  }
}
```

---

### 13.8 快速启动路径

**gemini-cli 启动时执行大量 Prefetch（MDM、Keychain、ProjectScan），但这些可以延迟。**

```typescript
async function bootstrap(trusted: boolean): Promise<BootstrapResult> {
  // 核心：立即加载
  const [manifest, registry] = await Promise.all([
    loadManifest(),        // 必需
    loadToolRegistry(),    // 必需
  ]);

  // 非阻塞：后台预热
  if (trusted) {
    setImmediate(() => prefetchKeychain());      // 可延迟
    setImmediate(() => startProjectScan());     // 可延迟
  }

  return { manifest, registry };
}
```

---

## 十四、重写清单

### 14.1 必需实现（核心）

| 功能 | 最小实现 | 说明 |
|------|---------|------|
| **工具注册表** | `Map<name, Tool>` | 支持 `get()`, `getAll()` |
| **执行器** | `execute(tool, args)` | 验证 → 执行 → 返回结果 |
| **状态追踪** | `pending/running/done` | 三状态足够 |
| **会话存储** | `save()/load()` | JSON 文件持久化 |

### 14.2 可选实现（增强）

| 功能 | 精简版 | 完整版 |
|------|-------|-------|
| **Policy 引擎** | 单一 `check()` | 优先级规则 + 缓存 |
| **Hook 系统** | 按需触发 | 事件驱动 + 链式 |
| **Confirmation** | 同步确认 | 异步 + 多种结果 |
| **Context 压缩** | 简单截断 | 智能语义压缩 |

### 14.3 禁止项（避免过度设计）

- ❌ 每个工具调用前都执行 Schema 循环检测
- ❌ 状态机超过 5 个状态
- ❌ Confirmation 流程超过 2 层嵌套
- ❌ Hook 系统全局订阅无按需触发
- ❌ 启动时执行所有 Prefetch（应延迟）
- ❌ MessageBus 订阅无显式取消

### 14.4 性能目标

| 指标 | 目标值 | 说明 |
|------|-------|------|
| **冷启动** | < 500ms | 到首个工具可用 |
| **工具执行** | < 100ms | 纯执行（不含模型调用） |
| **Policy 检查** | < 10ms | 单次检查 |
| **状态转换** | < 5ms | 内存操作 |

---

## 十五、总结

### 15.1 设计要点

1. **事件驱动**：统一 AgentProtocol 事件总线
2. **状态持久化**：会话状态存入文件系统
3. **路由匹配**：命令/工具双重镜像 + 分数排序
4. **权限隔离**：ToolPermissionContext 细粒度控制
5. **上下文压缩**：TurnLoop 自动压缩防止 context rot
6. **模块化**：Subsystem 解耦 + ExecutionRegistry 统一调度

### 15.2 GSD 工作流关键

- 搜索→学习→执行→交付 快速迭代
- 状态在文件，跨会话恢复
- 2-5 分钟原子任务
- RED-GREEN-REFACTOR TDD 循环

### 15.3 核心架构理念

| 理念 | 实现 |
|------|------|
| **安全优先** | frozen dataclass、Schema 验证、Policy 引擎 |
| **可观测性** | HistoryLog、TranscriptStore、HookSystem |
| **可扩展性** | AgentRegistry 多层加载、MessageBus.derive() |
| **高性能** | @lru_cache、切片压缩、异步预热 |
| **易调试** | PrefetchResult、BootstrapGraph、CommandGraph |

---

*文档版本: 1.2*
*整合来源: claw-code + gemini-cli + AGENT_SPECIALIZATION_GUIDE*
