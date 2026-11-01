# BitClock 项目总结

## 项目概述

BitClock（比特时钟）是一款像素风格的 Windows 电脑使用时长追踪工具。它自动检测前台应用，记录使用时长，通过复古像素风 UI 展示统计数据，帮助用户了解自己的电脑使用习惯。

- **版本**: 1.0.0
- **平台**: Windows 10/11
- **许可证**: MIT

## 技术架构

| 层级 | 技术 | 版本 |
|------|------|------|
| 前端框架 | Svelte (Runes) | 5.55+ |
| 前端构建 | Vite | 8.x |
| 前端语言 | TypeScript | 6.x |
| 后端框架 | Tauri | 2.x |
| 后端语言 | Rust (edition 2021) | 1.70+ |
| 数据库 | SQLite (rusqlite, bundled) | 0.31 |
| 类型同步 | ts-rs | 10.x |
| 安装包 | NSIS | — |

### 架构特点

- **前后端分离**: Svelte 负责 UI 渲染，Rust 负责系统级操作
- **IPC 通信**: Tauri 命令系统，20 个 `#[tauri::command]`
- **类型安全**: ts-rs 自动从 Rust 结构体生成 TypeScript 类型
- **事件驱动**: 前台窗口变化通过 Tauri 事件推送，非轮询
- **数据持久化**: SQLite WAL 模式，TTL 查询缓存

## 已实现功能

### 核心追踪
- Win32 `GetForegroundWindow` 前台窗口检测
- `GetLastInputInfo` 空闲检测（可配置 1-60 分钟）
- HWND 缓存，减少系统调用
- 每 30 秒数据库 checkpoint，防数据丢失
- 暂停/恢复追踪
- 自适应轮询（活跃 2s / 空闲 5s）

### 数据展示
- **首页**: 今日总时长、当前应用、应用排行条形图、饼图分布、分类统计、7 天迷你图、每日目标进度条
- **历史**: 周/月切换、交互式柱状图（悬停 + 点击）、汇总统计（总计/日均/最长/活跃天数）、Top 5 应用
- **设置**: 主题（暗/亮/自动）、强调色、语言（中/英）、开机自启、退出提醒、空闲阈值、休息提醒、每日目标、数据保留、清理、导出、备份/恢复、应用分类

### UI 组件库
| 组件 | 用途 |
|------|------|
| PixelButton | 按钮（primary / secondary / danger） |
| PixelCard | 卡片容器（像素边框 + 阴影） |
| PixelBar | 水平条形图项 |
| PixelPieChart | SVG 饼图（悬停交互） |
| PixelMiniChart | 7 天迷你柱状图 |
| PixelNumber | 数字显示（large / medium / small） |
| PixelToggle | 开关 |
| PixelHeader | 页面头部（返回按钮 + 主题切换 + 设置入口） |
| PixelModal | 弹窗（焦点陷阱 + 滚动锁定 + ESC 关闭） |
| PixelCategoryStats | 分类统计展示 |
| ExitReminder | 退出提醒弹窗（随机鼓励语） |

### 系统集成
- 系统托盘（菜单 + tooltip + 左键显示）
- 开机自启（Windows 注册表）
- 全局快捷键（Ctrl+Shift+S/P/T）
- 窗口置顶
- 最小化到托盘（不退出）
- Windows 原生通知

### 数据管理
- JSON 流式导出
- CSV 导出（带 BOM，Excel 兼容）
- 数据库备份（含 WAL/SHM）
- 数据库恢复
- 自动清理旧记录（可配置保留天数）
- 增量 VACUUM

### 工程质量
- 结构化错误类型 (`AppError`，带错误码)
- Rust 单元测试（db / error / tray 模块）
- 前端全局错误处理（转发到后端日志）
- Panic Hook 日志记录
- Mutex 中毒恢复
- i18n 国际化系统（zh/en）

## 项目结构

```
BitClock/
├── src/                           # Svelte 前端
│   ├── components/                # 11 个像素风 UI 组件
│   ├── pages/                     # 3 个页面（Home / History / Settings）
│   ├── stores/                    # 4 个 Runes 状态模块
│   ├── styles/global.css          # 全局样式 + CSS 变量 + 字体声明
│   ├── utils/format.ts            # 时间格式化工具
│   ├── types/                     # 类型定义
│   │   └── generated/             #   ts-rs 自动生成
│   ├── App.svelte                 # 根组件（路由 / 快捷键 / 托盘事件）
│   └── main.ts                    # 入口（全局错误处理）
├── src-tauri/                     # Rust 后端
│   ├── src/
│   │   ├── lib.rs                 # 应用初始化、日志、插件注册
│   │   ├── commands.rs            # 20 个 Tauri 命令
│   │   ├── db.rs                  # 数据库操作、缓存、导出
│   │   ├── tracker.rs             # 窗口追踪、空闲检测
│   │   ├── tray.rs                # 系统托盘
│   │   └── error.rs               # 结构化错误类型
│   ├── capabilities/              # Tauri 权限配置
│   ├── icons/                     # 应用图标（32/128/256/ICO）
│   └── nsis/                      # NSIS 安装脚本 + 图片
├── index.html                     # HTML 入口（早期主题注入）
├── package.json                   # Node.js 依赖
├── vite.config.ts                 # Vite 配置（端口 1420）
├── svelte.config.js               # Svelte 配置
├── tsconfig.json                  # TypeScript 配置
├── setup.bat                      # 一键安装脚本
├── dev.bat                        # 开发模式启动
├── build.bat                      # 构建发布版
├── build-with-env.bat             # MSVC 环境感知构建
├── generate_icon.py               # 像素图标生成（Pillow）
└── generate_welcome.py            # NSIS 安装图片生成（Pillow）
```

## Tauri 命令列表

| 命令 | 说明 | 缓存 |
|------|------|------|
| `get_today_stats` | 今日统计（含当前应用） | 5s TTL |
| `get_app_ranking_range` | 日期范围应用排行 | — |
| `get_weekly_stats` | 近 7 天统计 | 30s TTL |
| `get_monthly_stats` | 本月统计 | 60s TTL |
| `get_settings` | 获取设置 | 60s TTL |
| `update_settings` | 更新设置（事务写入） | 失效 |
| `toggle_autostart` | 切换开机自启（注册表） | — |
| `get_current_app` | 获取当前前台应用 | — |
| `cleanup_old_records` | 清理旧记录 | 失效 |
| `toggle_pause` | 暂停/恢复追踪 | — |
| `quit_app` | 退出应用 | — |
| `export_data` | 导出全部记录（JSON，内存） | — |
| `export_to_file` | 流式导出 JSON 到文件 | — |
| `export_csv_to_file` | 流式导出 CSV 到文件 | — |
| `backup_database` | 备份数据库（含 WAL/SHM） | — |
| `restore_database` | 恢复数据库 | — |
| `log_message` | 前端日志转发到后端 | — |
| `toggle_always_on_top` | 窗口置顶切换 | — |
| `set_app_category` | 设置应用分类 | — |
| `get_category_stats` | 获取分类统计 | — |
| `get_all_categories` | 获取全部分类 | — |

## 数据存储

| 文件 | 路径 | 说明 |
|------|------|------|
| 数据库 | `%LOCALAPPDATA%\BitClock\bitclock.db` | SQLite, WAL 模式 |
| WAL 日志 | `bitclock.db-wal` | 自动管理 |
| SHM 文件 | `bitclock.db-shm` | 自动管理 |
| 应用日志 | `bitclock.log` | 包含 panic 信息 |
| 主题偏好 | `localStorage` key `bitclock-theme` | dark / light / auto |
| 语言偏好 | `localStorage` key `bitclock-locale` | zh / en |

## 开发命令

```bash
# 安装依赖
npm install

# 开发模式（前端热重载 + Rust 后端）
npm run tauri dev

# 构建生产版本
npm run tauri build

# 运行 Rust 测试 + 生成 TypeScript 类型
cd src-tauri && cargo test

# 一键安装（检查环境 + 安装依赖 + 构建）
setup.bat
```

## 版本历史

### v1.0.0 (2026-05-30)
- 初始发布
- 完整的应用追踪、统计、提醒功能
- 像素风 UI 组件库
- 中英文双语支持
- 数据导出/备份/恢复
