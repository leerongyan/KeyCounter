@echo off
setlocal
cd /d "%~dp0"
if exist "%~dp0%%SystemDrive%%" rmdir /s /q "%~dp0%%SystemDrive%%"

python -m pip install -r requirements-build.txt
if errorlevel 1 goto :failed

python -m PyInstaller --noconfirm --clean --onefile --windowed --name KeyCounter ^
  --add-data "web;web" ^
  --hidden-import pynput.keyboard._win32 ^
  --hidden-import pynput.mouse._win32 ^
  --hidden-import pystray._win32 ^
  --hidden-import webview.platforms.winforms ^
  --hidden-import webview.platforms.win32 ^
  --hidden-import webview.platforms.edgechromium ^
  app.py
if errorlevel 1 goto :failed

echo.
echo 打包完成：dist\KeyCounter.exe
pause
exit /b 0

:failed
echo.
echo 打包失败，请把上面的错误信息截图保存。
pause
exit /b 1
