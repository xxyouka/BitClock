@echo off
chcp 65001 >nul 2>&1

echo.
echo  BitClock 构建发布版 / Build Release
echo ════════════════════════════════════════════════════════════════
echo.

:: Check prerequisites
echo  检查环境 / Checking environment...

where node >nul 2>&1
if %errorlevel% neq 0 (
    echo  [X] Node.js 未安装 / Node.js not found
    pause
    exit /b 1
)

where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo  [X] Rust 未安装 / Rust not found
    pause
    exit /b 1
)

echo  [√] 环境检查通过 / Environment OK
echo.

:: Step 1: Install dependencies
echo  [1/3] 安装依赖 / Installing dependencies...
call npm install
if %errorlevel% neq 0 (
    echo  [X] 依赖安装失败 / Dependencies installation failed
    pause
    exit /b 1
)
echo  [√] 依赖安装完成 / Dependencies installed
echo.

:: Step 2: Build frontend
echo  [2/3] 构建前端 / Building frontend...
call npm run build
if %errorlevel% neq 0 (
    echo  [X] 前端构建失败 / Frontend build failed
    pause
    exit /b 1
)
echo  [√] 前端构建完成 / Frontend built
echo.

:: Step 3: Build Tauri application
echo  [3/3] 构建应用 / Building application...
echo  这可能需要几分钟 / This may take a few minutes...
call npm run tauri build
if %errorlevel% neq 0 (
    echo  [X] 应用构建失败 / Application build failed
    pause
    exit /b 1
)

echo.
echo ════════════════════════════════════════════════════════════════
echo  [√] 构建完成 / Build completed!
echo ════════════════════════════════════════════════════════════════
echo.
echo  安装程序位置 / Installer location:
echo  src-tauri\target\release\bundle\nsis\BitClock_1.0.0_x64-setup.exe
echo.
echo  可执行文件位置 / Executable location:
echo  src-tauri\target\release\bitclock.exe
echo.
pause
