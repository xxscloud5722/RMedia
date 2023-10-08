# Rust Media Image
> `Rust` 开发媒体库 最大的愿景实现一个开源 *无GC或低GC* 的 `数据万象` 处理程序

> 本项目建议嵌入到`Java` 使用, 并由 `Spring` 提供强大的异步零拷贝Web 服务

## 功能描述
> 1. 支持Webp 编码、解码、转码
> 2. 支持图片裁剪、缩放等样式
> 3. 支持二维码生成、二维码嵌入Logo（平滑）


## 使用方法
> 1. `Java` 目录下文件`Copy`到自己项目内
> 2. 直接调用Java API


## 编译教程
> 先安装Rust 程序, 并修改国内源 （这里忽略）
```bash
cargo build --release
```
> 在 `target/release/media_jni.dll (so)` 为二进制库 

## 鸣谢
> 作者：C猫

> 邮箱：735802488@qq.com