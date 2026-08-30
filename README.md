# KeyCounter 键盘鼠标统计工具（Rust 原生版）

统计键盘按键、鼠标点击、滚轮和移动距离，带本地网页仪表盘、键盘热力图和系统托盘。
Rust 原生实现：**exe 约 3MB，托盘常驻约 19MB**（关闭面板后 WebView2 全部释放）。

## 使用

双击项目根目录的 `KeyCounterNative.exe`：

- 启动后系统托盘常驻，并打开统计面板
- 托盘右键菜单：打开统计面板 / 暂停继续统计 / 开机自启 / 导出 CSV / 导出热力图 PNG / 退出
- 面板地址 `http://127.0.0.1:8765`，数据每秒刷新，支持今日/昨日/近7天/近30天/本月/全部/自定义日期
- 关闭面板窗口时按设置（每次询问 / 最小化到托盘 / 直接退出）；选最小化时 WebView 完全销毁，
  从托盘重开约 1 秒，常驻内存回到 ~19MB
- 统计数据保存在 `data\keycounter.db`（SQLite，WAL）

## 命令行参数

```cmd
KeyCounterNative.exe                      默认：托盘 + 面板，数据在 exe 旁 data\
KeyCounterNative.exe --port 9000          指定端口
KeyCounterNative.exe --data-dir 某目录    指定数据目录
KeyCounterNative.exe --no-window          只在托盘运行
```

## 从源码构建

前置：Rust（rustup，本机装在 `D:\rust`）与 VS Build Tools（本机装在 `D:\VS2022BuildTools`）。

```cmd
cd native
build.cmd cargo build --release
```

产物：`native\target\release\KeyCounterNative.exe`（前端资源已内嵌，单文件即可分发）。

## 结构

```
native/    Rust 源码（Cargo 项目）
web/       仪表盘前端（构建时嵌入 exe）
data/      统计数据库与设置
```

## 隐私

只记录按键名称与次数、鼠标事件计数与移动距离，不保存输入内容、窗口标题或剪贴板；数据仅存本地。
