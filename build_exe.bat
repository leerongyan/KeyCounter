@echo off
setlocal
cd /d "%~dp0"
if exist "%~dp0%%SystemDrive%%" rmdir /s /q "%~dp0%%SystemDrive%%"

python -m pip install -r requirements-build.txt
if errorlevel 1 goto :failed

python -m PyInstaller --noconfirm --clean --onefile --windowed --name KeyCounter ^
  --add-data "web;web" ^
  --add-data "keycounter/assets;assets" ^
  --hidden-import pynput.keyboard._win32 ^
  --hidden-import pynput.mouse._win32 ^
  --hidden-import pystray._win32 ^
  --hidden-import webview.platforms.winforms ^
  --hidden-import webview.platforms.win32 ^
  --hidden-import webview.platforms.edgechromium ^
  --exclude-module numpy ^
  --exclude-module setuptools ^
  --exclude-module pkg_resources ^
  --exclude-module pip ^
  --exclude-module wheel ^
  --exclude-module unittest ^
  --exclude-module pydoc_data ^
  --exclude-module lib2to3 ^
  --exclude-module curses ^
  --exclude-module tkinter ^
  --exclude-module _tkinter ^
  --exclude-module bottle ^
  --exclude-module webview.platforms.qt ^
  --exclude-module webview.platforms.gtk ^
  --exclude-module webview.platforms.cef ^
  --exclude-module webview.platforms.mshtml ^
  --exclude-module webview.platforms.cocoa ^
  --exclude-module webview.platforms.android ^
  --exclude-module webview.platforms.ios ^
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
