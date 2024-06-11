# rust_shellcode_loader

项目描述：本项目是通过一个简单的基于rust的shellcode加载器，整合了一些开源项目的思路，便于直接生成对应的exe文件，可以在此项目的基础上自己添加功能模块。

## 安装

### 先决条件

本项目是由rust编译的exe文件，由于使用了rust测试功能所以需要安装nightly版本

运行所需的先决条件。

- rust
rust环境 [https://course.rs/first-try/installation.html]
- 编译exe文件需要
nightly版本
```shell
rustup install nightly
rustup default nightly
```

### 安装步骤

描述如何安装和配置项目。

```bash
# 克隆仓库
git clone https://github.com/你的用户名/你的项目.git

# 进入项目目录
cd 你的项目


```

## 使用
### 使用方式

```
rust_shell_code 1.0
An example application for loading shellcode

USAGE:
    rust_shellcode_loader.exe [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -l               所有加载方式的列表
    -V, --version    Print version information

SUBCOMMANDS:
    help         Print this message or the help of the given subcommand(s)
    shellcode    Rust shellcode加载器
```
![alt text](/docs/image.png)

```
USAGE:
    rust_shellcode_loader.exe shellcode -b <bin_path> -m <method> -f <file>

OPTIONS:
    -b <bin_path>        bin文件的路径，请使用绝对路径，基于转义的情况使用正斜杠，例如
                         D:/src/shell.bin
    -f <file>            输出的exe的名字，例如shellcode
    -h, --help           Print help information
    -m <method>          使用哪种加载方式，例如asm
```

![alt text](/docs/image1.png)