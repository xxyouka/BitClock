# 贡献指南 / Contributing Guide

感谢你对 BitClock 项目的关注！我们欢迎各种形式的贡献。

## 如何贡献

### 报告 Bug

1. 在 [Issues](https://github.com/xxyouka/BitClock/issues) 页面搜索是否已有相同问题
2. 如果没有，创建一个新的 Issue，包含：
   - 清晰的标题和描述
   - 复现步骤
   - 期望行为 vs 实际行为
   - 系统环境（Windows 版本、BitClock 版本）
   - 日志文件（`%LOCALAPPDATA%\BitClock\bitclock.log`）的相关片段

### 功能建议

在 Issues 中创建 `feature` 标签的请求，描述：
- 你想要的功能
- 使用场景
- 可能的实现方式（如果有想法的话）

### 提交代码

1. Fork 本仓库
2. 创建你的特性分支：`git checkout -b feature/my-feature`
3. 提交你的改动：`git commit -m 'feat: add my feature'`
4. 推送到分支：`git push origin feature/my-feature`
5. 创建一个 Pull Request

## 开发环境

### 前置要求

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) >= 1.70
- Windows 10/11 + MSVC 工具链
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)（C++ 桌面开发工作负载）

### 设置

```bash
# 克隆你的 fork
git clone https://github.com/your-username/BitClock.git
cd BitClock

# 安装依赖
npm install

# 启动开发模式
npm run tauri dev
```

### 项目结构

```
src/                   # Svelte 前端
├── components/        # UI 组件
├── pages/             # 页面
├── stores/            # 状态管理
└── utils/             # 工具函数

src-tauri/             # Rust 后端
└── src/
    ├── commands.rs    # Tauri 命令
    ├── db.rs          # 数据库
    ├── tracker.rs     # 窗口追踪
    ├── tray.rs        # 系统托盘
    └── error.rs       # 错误处理
```

## 代码规范

### Rust

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 为新功能添加单元测试
- 错误处理使用 `AppError` 类型

### TypeScript / Svelte

- 使用 Svelte 5 Runes（`$state` / `$derived` / `$effect` / `$props`）
- 新组件放在 `src/components/` 目录
- 新页面放在 `src/pages/` 目录
- 使用 `t()` 函数进行国际化，不要硬编码字符串

### 提交信息

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

```
<type>(<scope>): <description>

feat(i18n): 添加日语支持
fix(tracker): 修复空闲检测误报
docs: 更新 README
refactor(db): 优化查询缓存
```

类型：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `refactor`: 代码重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具链

## 添加新功能

### 添加新的 Tauri 命令

1. 在 `src-tauri/src/commands.rs` 中添加函数：

```rust
#[tauri::command]
pub fn my_command(db: State<'_, Arc<Mutex<Database>>>) -> Result<String, AppError> {
    let db = db.lock().map_err(|e| AppError::lock(e))?;
    // ... 你的逻辑
    Ok("result".to_string())
}
```

2. 在 `src-tauri/src/lib.rs` 的 `invoke_handler` 中注册：

```rust
.invoke_handler(tauri::generate_handler![
    // ... 现有命令
    commands::my_command,
])
```

3. 在 `src-tauri/capabilities/default.json` 中添加所需权限（如果需要）

4. 前端调用：

```typescript
import { invoke } from "@tauri-apps/api/core";
const result = await invoke<string>("my_command");
```

### 添加新的 i18n key

在 `src/stores/i18n.svelte.ts` 的 `zh` 和 `en` 对象中同时添加：

```typescript
zh: {
    "my.new.key": "中文文本",
},
en: {
    "my.new.key": "English text",
},
```

使用：

```svelte
<script>
  import { t } from "../stores/i18n.svelte";
</script>

<p>{t("my.new.key")}</p>
```

### 添加新的像素风组件

1. 在 `src/components/` 创建 `PixelXxx.svelte`
2. 使用 CSS 变量（`--bg-primary`、`--accent-*`、`--font-pixel` 等）
3. 保持像素风格：2px 边框、像素阴影、像素字体
4. 支持 `role="presentation"` 和 `aria-*` 属性

## 测试

### Rust 测试

```bash
cd src-tauri
cargo test
```

测试文件位于各模块的 `#[cfg(test)] mod tests` 中。

### 前端测试

目前前端没有自动化测试。欢迎贡献测试方案。

## Pull Request 检查清单

- [ ] 代码能正常编译（`npm run tauri build`）
- [ ] Rust 测试通过（`cargo test`）
- [ ] 新功能添加了 i18n 支持（中英文）
- [ ] 提交信息符合 Conventional Commits 格式
- [ ] 没有引入新的编译警告

## 许可证

提交代码即表示你同意你的贡献将在 [MIT License](LICENSE) 下发布。

---

如有任何问题，欢迎在 [Issues](https://github.com/xxyouka/BitClock/issues) 中提问。
