# KDL 集成使用指南

## 📦 新增功能

### 1. KDL 解析器集成

**依赖**: `kdl = "4.6"`

**模块位置**: `scripts/src/rules/`

**功能**:
- ✅ 解析 RULES.kdl 文件
- ✅ 提取规则配置
- ✅ 验证规则一致性

---

## 🚀 快速开始

### 编译

```bash
cd scripts
cargo build --release
```

### 使用 rules_validator

#### 1. 验证所有规则

```bash
./target/release/rules_validator validate --file ../RULES.kdl
```

**输出示例**:
```
=== RULES.kdl 验证报告 ===

版本: 2.0

✅ [paradigm] 范式规则验证通过
   ✅ live_data 规则正确
   ✅ execution_model 符合单线程约束

✅ [anti_hallucination] 反幻觉规则验证通过
   ✅ 正确禁止伪并发，允许显式 subagent
   ✅ 允许显式调用 subagent

✅ [ooda_loop] OODA 循环规则验证通过
   ✅ OODA 映射包含 observe 阶段
   ✅ OODA 映射包含 orient 阶段
   ✅ OODA 映射包含 decide 阶段
   ✅ OODA 映射包含 act 阶段

✅ 所有规则验证通过
```

#### 2. 验证单个规则

```bash
./target/release/rules_validator check --file ../RULES.kdl --rule paradigm
```

#### 3. 解析 RULES.kdl

```bash
./target/release/rules_validator parse --file ../RULES.kdl
```

**输出示例**:
```
=== RULES.kdl 解析结果 ===

版本: 2.0
规则数量: 12

规则列表:
  - meta
  - paradigm
  - anti_hallucination
  - live_code
  - ooda_loop
  - orthogonal_control
  - token_economics
  - memory_system
  - security
  - code_quality
  - design_process
  - user_interaction
  - output_style
```

#### 4. JSON 格式输出

```bash
./target/release/rules_validator validate --file ../RULES.kdl --json
```

---

## 🔄 CI/CD 集成

### GitHub Actions 工作流

#### 1. 规则验证工作流

**文件**: `.github/workflows/rules-validation.yml`

**触发条件**:
- RULES.kdl 文件变更
- rules 模块代码变更
- 工作流文件变更

**工作流步骤**:
1. 编译 rules_validator
2. 验证所有规则
3. 检查关键规则
4. 生成验证报告
5. 上传报告作为 artifact

#### 2. 主 CI 工作流

**文件**: `.github/workflows/ci.yml`

**触发条件**:
- push 到 main/dev 分支
- pull request 到 main/dev 分支

**工作流步骤**:
1. 编译所有二进制文件
2. 运行所有测试
3. 运行 clippy 检查
4. 检查代码格式
5. 验证 RULES.kdl
6. 测试 skill_router

---

## 📊 验证规则说明

### 1. Paradigm 规则验证

**检查项**:
- ✅ live_data 包含"动态配置"
- ✅ execution_model 包含"单线程"

**失败示例**:
```
❌ [paradigm] 范式规则验证失败
   ❌ live_data 规则不符合预期
   ❌ execution_model 应为单线程
```

### 2. Anti-Hallucination 规则验证

**检查项**:
- ✅ ban 包含"伪并发"
- ✅ ban 不包含"subagent"
- ✅ allow_subagent 包含"显式调用"

**失败示例**:
```
❌ [anti_hallucination] 反幻觉规则验证失败
   ❌ 不应禁止 subagent（与实现矛盾）
```

### 3. OODA Loop 规则验证

**检查项**:
- ✅ mapping 包含 observe 阶段
- ✅ mapping 包含 orient 阶段
- ✅ mapping 包含 decide 阶段
- ✅ mapping 包含 act 阶段

### 4. Token Economics 规则验证

**检查项**:
- ✅ config_format 为 "KDL"
- ✅ data_format 为 "JSON"

### 5. Memory System 规则验证

**检查项**:
- ✅ strategy 包含"索引"
- ✅ load 包含"按需"

### 6. Security 规则验证

**检查项**:
- ✅ secret 包含"环境变量"

---

## 🧪 本地测试

### 运行单元测试

```bash
cd scripts
cargo test --lib rules
```

### 测试 KDL 解析

```bash
cargo test test_parse_simple_rule
cargo test test_parse_nested_rule
cargo test test_parse_file
```

### 测试规则验证

```bash
cargo test test_validate_valid_rules
cargo test test_validate_invalid_rules
```

---

## 📝 添加新规则验证

### 1. 在 validator.rs 中添加新方法

```rust
pub fn validate_new_rule(&self) -> ValidationResult {
    let mut details = Vec::new();
    let mut is_valid = true;

    if let Some(rule) = self.config.rules.get("new_rule") {
        // 验证逻辑
        if let Some(value) = RulesParser::get_rule_value(rule, "key") {
            if value.contains("预期内容") {
                details.push("✅ 规则正确".to_string());
            } else {
                details.push("❌ 规则不符合预期".to_string());
                is_valid = false;
            }
        }
    } else {
        details.push("❌ 缺少 new_rule 规则".to_string());
        is_valid = false;
    }

    ValidationResult {
        rule_name: "new_rule".to_string(),
        is_valid,
        message: if is_valid { "验证通过".to_string() } else { "验证失败".to_string() },
        details,
    }
}
```

### 2. 在 validate_all 中调用

```rust
pub fn validate_all(&self) -> Vec<ValidationResult> {
    vec![
        self.validate_paradigm(),
        self.validate_anti_hallucination(),
        self.validate_new_rule(),  // 添加新规则
    ]
}
```

### 3. 在 CLI 中添加支持

```rust
Commands::Check { file, rule } => {
    let result = match rule.as_str() {
        "paradigm" => validator.validate_paradigm(),
        "new_rule" => validator.validate_new_rule(),  // 添加新规则
        _ => { /* ... */ }
    };
}
```

---

## 🔧 故障排除

### 问题 1: 网络连接失败

**错误信息**:
```
failed to connect to index.crates.io port 443
```

**解决方案**:
```bash
# 使用国内镜像
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

# 或使用代理
export HTTP_PROXY=http://127.0.0.1:7890
export HTTPS_PROXY=http://127.0.0.1:7890
```

### 问题 2: KDL 解析错误

**错误信息**:
```
KdlParseError: expected node
```

**解决方案**:
检查 RULES.kdl 语法是否正确：
```bash
# 使用在线 KDL 验证器
# https://kdl.dev/
```

### 问题 3: 规则验证失败

**错误信息**:
```
❌ [paradigm] 范式规则验证失败
```

**解决方案**:
1. 检查 RULES.kdl 中的规则定义
2. 对比 docs/RULES_VALIDATION.md 中的预期格式
3. 使用 `--json` 输出详细错误信息

---

## 📚 参考资源

- [KDL 官方文档](https://kdl.dev/)
- [Rust kdl crate](https://docs.rs/kdl/)
- [GitHub Actions 文档](https://docs.github.com/en/actions)

---

## ✅ 总结

**KDL 集成已完成**:
- ✅ 添加 kdl 依赖
- ✅ 创建 KDL 解析模块
- ✅ 实现规则自动验证
- ✅ 创建 CI/CD 配置
- ✅ 提供完整的 CLI 工具

**下一步**:
1. 等待网络恢复后编译新功能
2. 运行完整的测试套件
3. 推送到 GitHub 触发 CI/CD
