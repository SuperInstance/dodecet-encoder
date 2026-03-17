@echo off
REM Build script for dodecet-wasm package (Windows)

echo 🔨 Building dodecet-wasm package...
echo.

REM Check if wasm-pack is installed
where wasm-pack >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ wasm-pack not found!
    echo Install it with: cargo install wasm-pack
    exit /b 1
)

REM Clean previous build
echo 🧹 Cleaning previous build...
if exist pkg rmdir /s /q pkg
if exist ..\..\target\wasm32-unknown-unknown\release\*.wasm del /q ..\..\target\wasm32-unknown-unknown\release\*.wasm

REM Build for web target
echo 📦 Building for web target...
wasm-pack build --target web --release

REM Check if build was successful
if exist pkg (
    echo ✅ Build successful!
    echo.
    echo 📊 Package contents:
    dir pkg

    echo.
    echo 📦 To publish:
    echo   cd pkg
    echo   npm publish --access public
) else (
    echo ❌ Build failed!
    exit /b 1
)
