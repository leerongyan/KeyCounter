@echo off
rem Rust build helper: init MSVC env then run cargo
call "D:\VS2022BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1
set RUSTUP_HOME=D:\rust\rustup
set CARGO_HOME=D:\rust\cargo
set PATH=%CARGO_HOME%\bin;%PATH%
cd /d "%~dp0"
%*
