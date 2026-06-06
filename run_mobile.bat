@echo off
setlocal enabledelayedexpansion

REM Add ADB to PATH
set PATH=%PATH%;%USERPROFILE%\AppData\Local\Android\Sdk\platform-tools

echo ============================================================
echo   ATOMAS Mobile Test Runner
echo ============================================================
echo.

REM Check for connected devices
echo Checking for connected devices...
adb devices > temp_devices.txt
findstr /C:"device" temp_devices.txt | findstr /V "List" > devices_only.txt

set DEVICE_COUNT=0
for /f "tokens=1" %%i in (devices_only.txt) do (
    set DEVICE_SERIAL=%%i
    set /a DEVICE_COUNT+=1
)

del temp_devices.txt devices_only.txt

if !DEVICE_COUNT! EQU 0 (
    echo [ERROR] No authorized devices found!
    echo.
    echo Current devices status:
    adb devices
    echo.
    echo Please authorize USB debugging on your phone:
    echo   1. Look for popup: "Allow USB debugging?"
    echo   2. Check "Always allow from this computer"
    echo   3. Tap "OK"
    echo.
    echo See AUTHORIZE_DEVICE.md for detailed instructions
    pause
    exit /b 1
)

echo [OK] Device found: !DEVICE_SERIAL!
echo.

REM Get device info
for /f "delims=" %%i in ('adb -s !DEVICE_SERIAL! shell getprop ro.product.model 2^>nul') do set MODEL=%%i
for /f "delims=" %%i in ('adb -s !DEVICE_SERIAL! shell getprop ro.build.version.release 2^>nul') do set ANDROID_VERSION=%%i
echo Device: !MODEL! (Android !ANDROID_VERSION!)
echo.

REM Menu
echo Select test to run:
echo   1. Quick test (10 moves, depth 2)
echo   2. Standard test (50 moves, depth 3)
echo   3. Deep search test (30 moves, depth 4)
echo   4. Custom test
echo.
set /p choice="Enter choice (1-4): "

if "!choice!"=="1" (
    echo.
    echo Running QUICK TEST (10 moves^)...
    cargo run --bin milestone2 -- --adb --device !DEVICE_SERIAL! --moves 10 --solver expectimax --solver-depth 2 --delay-ms 1500 --verbose
) else if "!choice!"=="2" (
    echo.
    echo Running STANDARD TEST (50 moves^)...
    cargo run --bin milestone2 -- --adb --device !DEVICE_SERIAL! --moves 50 --solver expectimax --solver-depth 3 --delay-ms 1000 --verbose 2>&1 | tee mobile_test.log
    echo.
    echo ============================================================
    echo Test complete! Log saved to: mobile_test.log
    echo.
    echo Analysis:
    findstr /C:"UseMinus" mobile_test.log > nul 2>&1 && (
        for /f %%i in ('findstr /C:"UseMinus" mobile_test.log ^| find /C /V ""') do echo   Minus atoms: %%i times
    ) || echo   Minus atoms: 0 times
    findstr /C:"UsePlus" mobile_test.log > nul 2>&1 && (
        for /f %%i in ('findstr /C:"UsePlus" mobile_test.log ^| find /C /V ""') do echo   Plus atoms: %%i times
    ) || echo   Plus atoms: 0 times
    findstr /C:"completed successfully" mobile_test.log > nul 2>&1 && (
        for /f %%i in ('findstr /C:"completed successfully" mobile_test.log ^| find /C /V ""') do echo   Success: %%i moves
    ) || echo   Success: 0 moves
) else if "!choice!"=="3" (
    echo.
    echo Running DEEP SEARCH TEST (30 moves, depth 4^)...
    cargo run --bin milestone2 -- --adb --device !DEVICE_SERIAL! --moves 30 --solver expectimax --solver-depth 4 --delay-ms 1200 --verbose
) else if "!choice!"=="4" (
    echo.
    set /p moves="Number of moves: "
    set /p depth="Solver depth (1-5): "
    set /p delay="Delay between moves (ms): "
    echo.
    echo Running CUSTOM TEST...
    cargo run --bin milestone2 -- --adb --device !DEVICE_SERIAL! --moves !moves! --solver expectimax --solver-depth !depth! --delay-ms !delay! --verbose
) else (
    echo [ERROR] Invalid choice
    pause
    exit /b 1
)

echo.
echo ============================================================
echo Done!
echo.
pause
