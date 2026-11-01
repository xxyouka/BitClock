@echo off
chcp 65001 >nul 2>&1
setlocal enabledelayedexpansion

echo.
echo  ██████╗ ██╗████████╗ ██████╗██╗      ██████╗  ██████╗██╗  ██╗
echo  ██╔══██╗██║╚══██╔══╝██╔════╝██║     ██╔═══██╗██╔════╝██║ ██╔╝
echo  ██████╔╝██║   ██║   ██║     ██║     ██║   ██║██║     █████╔╝
echo  ██╔══██╗██║   ██║   ██║     ██║     ██║   ██║██║     ██╔═██╗
echo  ██████╔╝██║   ██║   ╚██████╗███████╗╚██████╔╝╚██████╗██║  ██╗
echo  ╚═════╝ ╚═╝   ╚═╝    ╚═════╝╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝
echo.
echo  比特时钟 - 像素风使用时长追踪器
echo  Pixel-style computer usage time tracker
echo.
echo ════════════════════════════════════════════════════════════════
echo.

:: Step 1: Check prerequisites
echo [1/5] 检查环境依赖 / Checking prerequisites...
echo.

set MISSING=0

:: Check Node.js
where node >nul 2>&1
if %errorlevel% equ 0 (
    for /f "tokens=*" %%i in ('node --version') do set NODE_VER=%%i
    echo   [√] Node.js !NODE_VER!
) else (
    echo   [X] Node.js 未安装 / Node.js not found
    echo      请访问 https://nodejs.org/ 下载安装 / Visit https://nodejs.org/ to install
    set MISSING=1
)

:: Check npm
where npm >nul 2>&1
if %errorlevel% equ 0 (
    for /f "tokens=*" %%i in ('npm --version') do set NPM_VER=%%i
    echo   [√] npm !NPM_VER!
) else (
    echo   [X] npm 未安装 / npm not found
    set MISSING=1
)

:: Check Rust
where rustc >nul 2>&1
if %errorlevel% equ 0 (
    for /f "tokens=*" %%i in ('rustc --version') do set RUST_VER=%%i
    echo   [√] Rust !RUST_VER!
) else (
    echo   [X] Rust 未安装 / Rust not found
    echo      请访问 https://rustup.rs/ 下载安装 / Visit https://rustup.rs/ to install
    set MISSING=1
)

:: Check cargo
where cargo >nul 2>&1
if %errorlevel% equ 0 (
    echo   [√] Cargo 可用 / Cargo available
) else (
    echo   [X] Cargo 未安装 / Cargo not found
    set MISSING=1
)

echo.

if %MISSING% equ 1 (
    echo ════════════════════════════════════════════════════════════════
    echo  缺少必要依赖，请先安装后重新运行此脚本
    echo  Missing prerequisites. Please install them and run this script again.
    echo ════════════════════════════════════════════════════════════════
    pause
    exit /b 1
)

echo  所有依赖已就绪 / All prerequisites satisfied!
echo.
echo ════════════════════════════════════════════════════════════════
echo.

:: Step 2: Install npm dependencies
echo [2/5] 安装前端依赖 / Installing frontend dependencies...
call npm install
if %errorlevel% neq 0 (
    echo  [X] npm install 失败 / npm install failed
    pause
    exit /b 1
)
echo  [√] 前端依赖安装完成 / Frontend dependencies installed
echo.

:: Step 3: Install Tauri CLI
echo [3/5] 检查 Tauri CLI / Checking Tauri CLI...
where tauri >nul 2>&1
if %errorlevel% neq 0 (
    echo  正在安装 Tauri CLI / Installing Tauri CLI...
    cargo install tauri-cli
    if %errorlevel% neq 0 (
        echo  [X] Tauri CLI 安装失败 / Tauri CLI installation failed
        pause
        exit /b 1
    )
)
echo  [√] Tauri CLI 就绪 / Tauri CLI ready
echo.

:: Step 4: Build the application
echo [4/5] 构建应用 / Building application...
echo  这可能需要几分钟，请耐心等待 / This may take a few minutes, please wait...
echo.
call npm run tauri build
if %errorlevel% neq 0 (
    echo  [X] 构建失败 / Build failed
    pause
    exit /b 1
)
echo  [√] 应用构建完成 / Application built successfully
echo.

:: Step 5: Show results
echo [5/5] 安装完成 / Installation complete!
echo.
echo ════════════════════════════════════════════════════════════════
echo.
echo  安装程序位置 / Installer location:
echo  src-tauri\target\release\bundle\nsis\BitClock_1.0.0_x64-setup.exe
echo.
echo  你可以运行安装程序将 BitClock 安装到系统中
echo  You can run the installer to install BitClock to your system
echo.
echo ════════════════════════════════════════════════════════════════
echo.

:: Ask if user wants to run the installer
set /p RUN_INSTALLER=是否立即运行安装程序？(Y/N) Run installer now? (Y/N):
if /i "%RUN_INSTALLER%"=="Y" (
    start "" "src-tauri\target\release\bundle\nsis\BitClock_1.0.0_x64-setup.exe"
)

pause
