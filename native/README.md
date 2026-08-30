# KeyCounter Native（Rust 版）

Python 版的 Rust 重写：功能一致，资源占用大幅降低。

## 与 Python 版的关系

- **功能与界面完全一致**：复用同一套 `web/` 前端（已嵌入二进制），API 逐字段兼容
- **数据互通**：数据库 schema 相同；用 `--data-dir` 指向 Python 版的 `data` 目录即可继承历史统计
- **互不干扰**：Python 版代码一行未动；两个 exe 是独立程序
- ⚠️ 端口默认都是 8765，**两个版本不要同时运行**
- ⚠️ 开机自启用的是同一个注册表值（`HKCU\...\Run\KeyCounter`），同一时间只对一个版本启用

## 构建

前置：Rust 工具链（rustup，本机装在 `D:\rust`）+ VS Build Tools（本机装在 `D:\VS2022BuildTools`）。

```cmd
cd native
build.cmd cargo build --release
```

产物：`native\target\release\KeyCounterNative.exe`（单文件，前端资源已内嵌）。

## 运行

双击 `KeyCounterNative.exe` 即可：托盘常驻 + 打开统计面板。

```cmd
KeyCounterNative.exe                       # 默认：托盘 + 面板，数据存 exe 旁 data\
KeyCounterNative.exe --port 9000           # 指定端口
KeyCounterNative.exe --data-dir D:\codex_project\Key_counter\data   # 继承 Python 版数据
KeyCounterNative.exe --no-window           # 只在托盘运行，不打开面板
```

## 行为说明

- 关闭面板窗口时按设置（每次询问 / 最小化到托盘 / 直接退出）
- 选择"最小化到托盘"时**窗口和 WebView 会被真正销毁**，WebView2 的内存完全释放，
  常驻内存约 10-20MB；从托盘"打开统计面板"时重新创建（约 1 秒）
- 按键/鼠标事件先入内存队列，由独立线程批量写库——全屏游戏等高负载场景不会丢按键

## 资源占用参考

| 状态 | 内存 |
|---|---|
| exe 体积 | ~6 MB |
| 托盘常驻（面板已关闭） | ~15 MB |
| 面板打开 | WebView2 正常渲染开销（所有 WebView2 应用一致） |
