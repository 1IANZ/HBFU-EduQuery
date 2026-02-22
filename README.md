# HBFU-EduQuery

> 河北金融学院教务系统查询微信小程序

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

## 简介

HBFU-EduQuery 是一款面向河北金融学院学生的微信小程序，通过聚合教务系统数据，提供课程表查询、考试安排查询等便捷功能，无需打开繁琐的教务网站。

## 技术栈

| 层级    | 技术                                               |
| ----- | ------------------------------------------------ |
| 小程序前端 | [uni-app](https://uniapp.dcloud.net.cn/) + Vue 3 |
| 后端服务  | Rust + [Axum](https://github.com/tokio-rs/axum)  |

## 项目结构

```
HBFU-EduQuery/
├── apps/
│   ├── miniapp/      # uni-app 小程序前端
│   └── server/       # Rust 后端服务
├── .gitignore
├── LICENSE
└── README.md
```

## 快速开始

### 前端（miniapp）

1. 安装 [HBuilderX](https://www.dcloud.io/hbuilderx.html)
2. 在 `apps/miniapp/manifest.json` 中填入你的微信小程序 AppID：

   ```json
   "mp-weixin": {
     "appid": "YOUR_WECHAT_APPID"
   }
   ```
3. 使用 HBuilderX 打开 `apps/miniapp` 目录，运行到微信开发者工具

### 后端（server）

```bash
cd apps/server
cargo run
```

> 生产环境构建：`cargo build --release`

## 配置说明

* **微信 AppID**：在 `apps/miniapp/manifest.json` 的 `mp-weixin.appid` 字段中填入。
* **服务端配置**：`apps/server/data.json` 包含学期、开学日期等运营数据，可按需修改。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

本项目基于 [Apache License 2.0](LICENSE) 开源。
