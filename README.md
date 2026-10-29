# noobenkyo

为中文母语者设计的日语学习桌面 app，目前支持 Mac 和 Windows。

## 功能简述

- 去掉了所有看汉字答意思的无效题型。
- 支持 agent 调用工具生成词库和剧本。
- 自创角色，属于自己的有声小说。
- 支持用本地的 Qwen TTS 生成语音，未来将支持用本地模型生成剧本。
- 剧本支持 4 种模式播放：只日语、只中文、先日后中、先中后日。

## 安装

### 自己构建（推荐）

自己构建的 app 不会被系统拦下，还能装上给 agent 用的命令行 `noobenkyo`。

需要 Rust（1.94 或更新）、Node.js 22 和 pnpm，其余依赖见 Tauri 的[环境准备](https://v2.tauri.app/start/prerequisites/)。克隆本仓库以后，在仓库根目录运行：

```bash
cd web
pnpm install
pnpm tauri build
```

打好的安装包在 `target/release/bundle/` 下面。

再装上给 agent 用的命令行：

```bash
cargo install --path crates/agent/cli
```

app 里「交给 agent」生成的委托会用到它。更新 app 时，命令行也要用同一份代码重新装一次。

### 下载安装包

从 [Releases](../../releases/latest) 下载。安装包里没有给 agent 用的命令行。

| 系统 | 文件 |
|---|---|
| macOS（Apple Silicon） | `noobenkyo_版本号_aarch64.dmg` |
| Windows | `noobenkyo_版本号_x64-setup.exe` |

安装包没有开发者证书签名，第一次打开时会被系统拦下，要手动放行：

- **macOS**：把 noobenkyo 拖进「应用程序」，第一次打开时提示无法验证，点「完成」；然后打开「系统设置 → 隐私与安全性」，找到 noobenkyo 那一行，点「仍要打开」。
- **Windows**：运行安装程序时弹出「Windows 已保护你的电脑」，点「更多信息」，再点「仍要运行」。

## 导入格式

词库用 JSON Lines 导入，一行一个义项，例子见 [samples/basic.jsonl](samples/basic.jsonl)。

导入前会先预览。有格式错误的行可以一键剪出来，app 已经准备好了修正用的提示词，交给 AI 或 agent 修好后再贴回来就行。

## 许可证

[MIT](LICENSE)
