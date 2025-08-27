# ZUC Encryption Tool

一个基于ZUC算法的加密工具，支持文件加密和Web服务。

## 功能特性

- 🔐 ZUC算法加密/解密
- 📁 文件加密支持
- 🌐 Web API服务
- ⚙️ 配置管理
- 📝 完整的错误处理
- 🧪 单元测试

## 安装

```bash
cargo build --release
```

## 使用方法

### 命令行工具

#### 启动Web服务
```bash
./target/release/zuc -s
```

#### 加密文件
```bash
./target/release/zuc -e /path/to/file
```

#### 解密文件
```bash
./target/release/zuc -d /path/to/file
```

#### 查看帮助
```bash
./target/release/zuc --help
```

### Web API

#### 健康检查
```bash
curl http://localhost:8080/health
```

#### 获取配置
```bash
curl http://localhost:8080/config
```

#### 加密数据
```bash
curl -X POST -F "file=@/path/to/file" http://localhost:8080/encrypt
```

## 配置

通过环境变量配置：

- `ZUC_HOST`: 服务器主机地址 (默认: 127.0.0.1)
- `ZUC_PORT`: 服务器端口 (默认: 8080)

## 项目结构

```
src/
├── main.rs          # 主程序入口
├── lib.rs           # 库入口
├── error.rs         # 错误处理
├── config.rs        # 配置管理
├── core/            # 核心加密模块
│   ├── algorithm.rs # 算法实现
│   ├── cipher.rs    # 加密器
│   ├── constant.rs  # 常量定义
│   └── zuc.rs       # ZUC算法
├── server/          # Web服务
│   └── encrypt_service.rs
└── bash/            # 命令行工具
    └── encrypt_bash.rs
```

## 开发

### 运行测试
```bash
cargo test
```

### 代码格式化
```bash
cargo fmt
```

### 代码检查
```bash
cargo clippy
```

## 许可证

MIT License