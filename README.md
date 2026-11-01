<div align="center">

# ■ BitClock 比特时钟

**像素风电脑使用时长追踪器**

一款复古像素风格的 Windows 桌面应用，自动追踪每个应用的使用时间，用数据帮你了解自己的电脑使用习惯。

![Version](https://img.shields.io/badge/version-1.1.0-blue?style=flat-square)
![Platform](https://img.shields.io/badge/platform-Windows%2010%20|%2011-lightgrey?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)
![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131?style=flat-square&logo=tauri)
![Svelte](https://img.shields.io/badge/Svelte-5-FF3E00?style=flat-square&logo=svelte)
![Rust](https://img.shields.io/badge/Rust-2021-DEA584?style=flat-square&logo=rust)

[English](#english) | 中文

</div>

---

## 功能特性

### 🕐 应用追踪
- 自动检测前台窗口，记录每个应用的使用时长
- HWND 缓存优化，最小化 Win32 API 调用
- 自适应轮询间隔（活跃 2s / 空闲 5s）
- 空闲检测 — 无操作自动暂停追踪（可配置 1-60 分钟）

### 📊 数据统计
- **首页仪表盘** — 今日总时长、当前应用、应用排行条形图、饼图分布、分类统计
- **历史统计** — 按周/月查看，交互式柱状图（悬停提示、点击选中查看详情）
- **7 天趋势** — 首页迷你图表，一眼掌握使用趋势
- **每日目标** — 设定使用上限，接近时预警，达到时弹窗提醒

### 🎨 像素风格 UI
- 11 个自定义像素风组件（Button / Card / Bar / PieChart / MiniChart / Modal 等）
- 三款像素字体：Press Start 2P（英文）、VT323（数字）、zpix（中文）
- 暗色 / 亮色 / 自动三种主题模式
- 自动模式：7:00-19:00 亮色，其余时间暗色
- 自定义强调色

### 🌐 多语言支持
- 中文 / English 双语界面
- 一键切换，即时生效

### ⏰ 智能提醒
- **休息提醒** — 连续使用超过设定时间后提醒休息（10-120 分钟可调）
- **目标提醒** — 达到每日使用目标后弹窗 + 系统通知
- **退出提醒** — 关闭应用时显示今日使用总结

### ⌨️ 快捷操作
| 快捷键 | 功能 | 范围 |
|--------|------|------|
| `1` `2` `3` | 切换页面（首页/历史/设置） | 应用内 |
| `Space` | 暂停 / 恢复追踪 | 应用内 |
| `T` | 窗口置顶 / 取消置顶 | 应用内 |
| `Esc` | 返回首页 / 关闭弹窗 | 应用内 |
| `Ctrl+Shift+S` | 显示 / 隐藏窗口 | 全局 |
| `Ctrl+Shift+P` | 暂停 / 恢复追踪 | 全局 |
| `Ctrl+Shift+T` | 窗口置顶 / 取消 | 全局 |

### 🖥️ 系统集成
- 系统托盘常驻，左键显示窗口，右键弹出菜单
- 托盘悬停显示今日使用时长
- 开机自启动（注册表方式）
- 关闭窗口时最小化到托盘，不退出应用

### 💾 数据管理
- **JSON / CSV 导出** — 流式写入，支持大数据量
- **数据库备份 / 恢复** — 包含 WAL 日志的完整备份
- **自动清理** — 可配置保留 30 / 60 / 90 / 180 / 365 天
- **应用分类** — 为应用设置分类标签，查看分类统计

---

## 快速开始

### 安装发布版（推荐）

1. 前往 [Releases](https://github.com/xxyouka/BitClock/releases) 页面
2. 下载 `BitClock_1.1.0_x64-setup.exe`
3. 运行安装程序，支持简体中文和 English

### 从源码构建

**前置要求：**
- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) >= 1.70
- Windows 10/11 + MSVC 工具链

**一键构建：**
```bash
git clone https://github.com/xxyouka/BitClock.git
cd BitClock
setup.bat
```

**手动构建：**
```bash
git clone https://github.com/xxyouka/BitClock.git
cd BitClock
npm install
npm run tauri dev      # 开发模式（热重载）
npm run tauri build    # 构建安装包
```

构建产物位于 `src-tauri/target/release/bundle/nsis/`。

---

## 技术架构

```
┌─────────────────────────────────────────────────┐
│                  BitClock 应用                    │
├────────────────────┬────────────────────────────┤
│    Svelte 5 前端    │       Rust 后端 (Tauri 2)   │
│                    │                            │
│  ┌──────────────┐  │  ┌──────────────────────┐  │
│  │  Pages       │  │  │  WindowTracker       │  │
│  │  Home        │  │  │  - 前台窗口检测       │  │
│  │  History     │  │  │  - 空闲检测           │  │
│  │  Settings    │◄─┼──┤  - 事件推送           │  │
│  └──────────────┘  │  └──────────────────────┘  │
│  ┌──────────────┐  │  ┌──────────────────────┐  │
│  │  Components  │  │  │  Database (SQLite)   │  │
│  │  Pixel*      │  │  │  - WAL 模式          │  │
│  │  (11 个)     │  │  │  - TTL 查询缓存      │  │
│  └──────────────┘  │  │  - 流式导出           │  │
│  ┌──────────────┐  │  └──────────────────────┘  │
│  │  Stores      │  │  ┌──────────────────────┐  │
│  │  stats       │  │  │  System Tray         │  │
│  │  settings    │  │  │  - 菜单 / tooltip    │  │
│  │  theme       │  │  │  - 事件监听           │  │
│  │  i18n        │  │  └──────────────────────┘  │
│  └──────────────┘  │                            │
├────────────────────┴────────────────────────────┤
│              SQLite (WAL 模式)                    │
│         %LOCALAPPDATA%\BitClock\bitclock.db      │
└─────────────────────────────────────────────────┘
```

### 技术栈

| 层级 | 技术 | 说明 |
|------|------|------|
| 前端框架 | Svelte 5 (Runes) | `$state` / `$derived` / `$effect` 响应式系统 |
| 前端构建 | Vite 8 | 开发热重载 + 生产构建 |
| 前端语言 | TypeScript 6 | 严格模式，bundler 模块解析 |
| 后端框架 | Tauri 2 | 桌面应用框架，IPC 通信 |
| 后端语言 | Rust (2021) | 内存安全，高性能 |
| 数据库 | SQLite (rusqlite) | WAL 模式，bundled 编译 |
| 类型同步 | ts-rs | Rust 结构体自动生成 TypeScript 类型 |
| 窗口追踪 | winapi | GetForegroundWindow / GetLastInputInfo |
| 安装包 | NSIS | Windows 安装程序，中英文支持 |

### 性能优化

- **HWND 缓存** — 仅在窗口句柄变化时调用 Win32 API
- **查询缓存** — 今日 5s / 周 30s / 月 60s / 设置 60s TTL
- **自适应轮询** — 有前台应用时 2s，空闲时 5s
- **增量 VACUUM** — 启动时自动回收已删除记录的空间
- **流式导出** — 直接从 Rust 写入文件，不经过前端内存
- **事件推送** — 前台窗口变化通过 Tauri 事件通知前端

---

## 项目结构

```
BitClock/
├── src/                           # Svelte 前端
│   ├── components/                # 像素风 UI 组件
│   │   ├── PixelBar.svelte        #   条形图项
│   │   ├── PixelButton.svelte     #   按钮（primary/secondary/danger）
│   │   ├── PixelCard.svelte       #   卡片容器
│   │   ├── PixelCategoryStats.svelte  # 分类统计
│   │   ├── PixelHeader.svelte     #   页面头部
│   │   ├── PixelMiniChart.svelte  #   7 天迷你柱状图
│   │   ├── PixelModal.svelte      #   弹窗（焦点陷阱）
│   │   ├── PixelNumber.svelte     #   数字显示
│   │   ├── PixelPieChart.svelte   #   饼图
│   │   ├── PixelToggle.svelte     #   开关
│   │   └── ExitReminder.svelte    #   退出提醒弹窗
│   ├── pages/                     # 页面
│   │   ├── Home.svelte            #   首页仪表盘
│   │   ├── History.svelte         #   历史统计
│   │   └── Settings.svelte        #   设置
│   ├── stores/                    # Svelte 5 Runes 状态
│   │   ├── stats.svelte.ts        #   今日统计
│   │   ├── settings.svelte.ts     #   设置
│   │   ├── theme.svelte.ts        #   主题管理
│   │   └── i18n.svelte.ts         #   国际化（zh/en）
│   ├── styles/global.css          # 全局样式 + CSS 变量
│   ├── utils/format.ts            # 时间格式化
│   ├── types/                     # 类型定义
│   │   └── generated/             #   ts-rs 自动生成
│   ├── App.svelte                 # 根组件（路由/快捷键/托盘事件）
│   └── main.ts                    # 入口（全局错误处理）
├── src-tauri/                     # Rust 后端
│   ├── src/
│   │   ├── lib.rs                 #   应用初始化、日志、插件注册
│   │   ├── commands.rs            #   20 个 Tauri 命令
│   │   ├── db.rs                  #   数据库操作、缓存、导出
│   │   ├── tracker.rs             #   窗口追踪、空闲检测
│   │   ├── tray.rs                #   系统托盘
│   │   └── error.rs               #   结构化错误类型
│   ├── capabilities/              # Tauri 权限配置
│   ├── icons/                     # 应用图标
│   └── nsis/                      # NSIS 安装脚本
├── index.html                     # HTML 入口（早期主题注入）
├── package.json                   # Node.js 依赖
├── vite.config.ts                 # Vite 配置
├── svelte.config.js               # Svelte 配置
├── setup.bat                      # 一键安装脚本
├── dev.bat                        # 开发模式启动
├── build.bat                      # 构建发布版
└── build-with-env.bat             # MSVC 环境感知构建
```

---

## 数据存储

| 文件 | 路径 |
|------|------|
| 数据库 | `%LOCALAPPDATA%\BitClock\bitclock.db` |
| WAL 日志 | `%LOCALAPPDATA%\BitClock\bitclock.db-wal` |
| 应用日志 | `%LOCALAPPDATA%\BitClock\bitclock.log` |
| 主题偏好 | `localStorage` (`bitclock-theme`) |
| 语言偏好 | `localStorage` (`bitclock-locale`) |

### 数据库表

| 表名 | 用途 |
|------|------|
| `app_usage` | 使用记录（进程名、窗口标题、开始/结束时间、时长、日期） |
| `settings` | 键值对设置 |
| `app_categories` | 应用分类映射 |

---

## 开发指南

### 开发命令

```bash
# 安装依赖
npm install

# 启动开发模式（前端热重载 + Rust 后端）
npm run tauri dev

# 构建生产版本
npm run tauri build

# 仅构建前端
npm run build

# 运行 Rust 测试 + 生成 TypeScript 类型
cd src-tauri && cargo test
```

### 生成 TypeScript 类型

Rust 结构体通过 `ts-rs` 宏自动生成 TypeScript 类型定义：

```bash
cd src-tauri
cargo test
# 生成的类型文件位于 src/types/generated/
```

### 添加新的 Tauri 命令

1. 在 `src-tauri/src/commands.rs` 中添加函数，使用 `#[tauri::command]` 宏
2. 在 `src-tauri/src/lib.rs` 的 `invoke_handler` 中注册
3. 在 `src-tauri/capabilities/default.json` 中添加所需权限
4. 前端通过 `invoke("command_name", { args })` 调用

### 添加新的 i18n key

在 `src/stores/i18n.svelte.ts` 的 `zh` 和 `en` 两个对象中同时添加翻译：

```typescript
zh: { "my.key": "中文文本" },
en: { "my.key": "English text" },
```

前端使用 `t("my.key", { param: value })` 调用。

---

## 截图

> 欢迎提交 PR 添加截图

| 首页 | 历史统计 | 设置 |
|------|----------|------|
| ![Home](docs/screenshots/home.png) | ![History](docs/screenshots/history.png) | ![Settings](docs/screenshots/settings.png) |

---

## 更新日志

### v1.0.0 (2026-05-30)

**核心功能**
- 应用使用时长自动追踪（Win32 前台窗口检测）
- 首页仪表盘（总时长、应用排行、饼图、分类统计）
- 历史统计（周/月视图，交互式柱状图）
- 设置页面（主题、语言、目标、提醒、数据管理）

**特色功能**
- 像素风 UI（11 个自定义组件）
- 暗色/亮色/自动主题 + 自定义强调色
- 中文/English 双语支持
- 全局快捷键（Ctrl+Shift+S/P/T）
- 系统托盘集成
- 每日目标 + 休息提醒
- 空闲检测
- JSON/CSV 导出、数据库备份/恢复

---

## 许可证

[MIT License](LICENSE) © 2026 BitClock Team

---

## 致谢

- [Press Start 2P](https://fonts.google.com/specimen/Press+Start+2P) — 像素英文字体
- [VT323](https://fonts.google.com/specimen/VT323) — 数字显示字体
- [zpix](https://github.com/SolidZORO/zpix-pixel-font) — 像素中文字体
- [Tauri](https://tauri.app/) — 跨平台桌面应用框架
- [Svelte](https://svelte.dev/) — 前端框架

---

<div align="center">

**Made with ❤️ by BitClock Team**

</div>
