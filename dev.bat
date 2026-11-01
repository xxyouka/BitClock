@echo off
chcp 65001 >nul 2>&1

echo.
echo  BitClock 开发模式 / Development Mode
echo ════════════════════════════════════════════════════════════════
echo.

:: Check prerequisites
echo  检查环境 / Checking environment...

where node >nul 2>&1
if %errorlevel% neq 0 (
    echo  [X] Node.js 未安装 / Node.js not found
    echo      请先运行 setup.bat 或安装 Node.js / Run setup.bat or install Node.js
    pause
    exit /b 1
)

where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo  [X] Rust 未安装 / Rust not found
    echo      请先运行 setup.bat 或安装 Rust / Run setup.bat or install Rust
    pause
    exit /b 1
)

echo  [√] 环境检查通过 / Environment OK
echo.

:: Install dependencies if needed
if not exist "node_modules" (
    echo  首次运行，正在安装依赖 / First run, installing dependencies...
    call npm install
    if %errorlevel% neq 0 (
        echo  [X] 依赖安装失败 / Dependencies installation failed
        pause
        exit /b 1
    )
    echo.
)

:: Start development server
echo  启动开发服务器 / Starting development server...
echo  按 Ctrl+C 停止 / Press Ctrl+C to stop
echo.
call npm run tauri dev
