@echo off
setlocal enabledelayedexpansion

set SCRIPT_DIR=%~dp0
set PROJECT_ROOT=%SCRIPT_DIR:~0,-1%
for %%A in ("%PROJECT_ROOT%") do set PROJECT_ROOT=%%~dpA
set OUTPUT_DIR=%PROJECT_ROOT%docs
set BIN_DIR=%PROJECT_ROOT%src\bin

if not exist "%OUTPUT_DIR%" mkdir "%OUTPUT_DIR%"

REM Discover all binaries in src/bin/
setlocal enabledelayedexpansion
set "BINARIES="
if exist "%BIN_DIR%" (
    for /d %%B in ("%BIN_DIR%\*") do (
        set "BINARIES=!BINARIES! %%~nB"
    )
)

if "!BINARIES!"=="" (
    echo Error: No binaries found in %BIN_DIR%
    exit /b 1
)

echo Found binaries:!BINARIES!

for %%B in (!BINARIES!) do (
    set BINARY_NAME=%%B
    set BINARY_NAME=!BINARY_NAME:_=-!
    echo Building !BINARY_NAME! for WASM...

    cargo build --profile wasm-release ^
        --bin !BINARY_NAME! ^
        --target wasm32-unknown-unknown

    set BINARY_UNDERSCORE=%%B
    set WASM_FILE=%PROJECT_ROOT%target\wasm32-unknown-unknown\wasm-release\!BINARY_NAME!.wasm
    set OUTPUT_SUBDIR=%OUTPUT_DIR%\%%B

    if not exist "!OUTPUT_SUBDIR!" mkdir "!OUTPUT_SUBDIR!"

    wasm-bindgen "!WASM_FILE!" ^
        --out-dir "!OUTPUT_SUBDIR!" ^
        --out-name module ^
        --target web ^
        --no-typescript

    echo Generated WASM for !BINARY_NAME! in !OUTPUT_SUBDIR!

    copy "assets\index.html" "!OUTPUT_SUBDIR!\index.html"
)

echo WASM build complete!
