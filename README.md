# S3 Manager

S3 Manager 是一款易用的 S3 兼容的存储管理工具（目前支持 Cloudflare R2、阿里云 OSS）。

![Screenshot 1](https://github.com/user-attachments/assets/56cd0fa4-ffd1-47fe-9b16-d6621430357a)
![Screenshot 2](https://github.com/user-attachments/assets/f6a7df5a-93b9-4d61-bcb0-3be0d051eb2f)
![Screenshot 3](https://github.com/user-attachments/assets/50e083da-4084-4621-b4d4-2d3a26bc2fa8)

## 注
- 本项目fork自 https://github.com/jlvihv/R2Uploader ，在其基础上进行了管理能力和操作逻辑的优化

## 特性

- **易用性：** 简单直观的用户界面，轻松上手。
- **多文件上传：** 支持同时上传多个文件。
- **大文件处理：** 针对大文件上传进行了优化。
- **跨平台：** 跨平台桌面应用程序。

## 技术栈

- **前端：** Svelte
- **构建工具：** Bun
- **后端：** Rust, Tauri

## 环境要求

- **Rust:** 确保您的电脑上已安装 Rust。
- **Bun:** 确保您的电脑上已安装 Bun。

## 开发

1.  克隆代码库到本地。
2.  使用 `bun tauri dev` 命令进行快速开发。

## 构建

1.  使用 `bun tauri build` 命令构建可执行文件。
2.  构建后的可执行文件位于 `src-tauri/target/release/bundle` 目录下。
