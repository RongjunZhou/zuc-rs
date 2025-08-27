.PHONY: build test clean fmt clippy run-server help

# 默认目标
all: build

# 构建项目
build:
	cargo build --release

# 开发构建
dev:
	cargo build

# 运行测试
test:
	cargo test

# 代码格式化
fmt:
	cargo fmt

# 代码检查
clippy:
	cargo clippy

# 启动服务器
run-server:
	cargo run -- -s

# 加密文件示例
encrypt-example:
	cargo run -- -e src/test.txt

# 解密文件示例
decrypt-example:
	cargo run -- -d src/test.txt

# 清理构建文件
clean:
	cargo clean

# 安装依赖
install:
	cargo install --path .

# 显示帮助
help:
	@echo "可用的命令:"
	@echo "  build          - 构建发布版本"
	@echo "  dev            - 构建开发版本"
	@echo "  test           - 运行测试"
	@echo "  fmt            - 代码格式化"
	@echo "  clippy         - 代码检查"
	@echo "  run-server     - 启动Web服务器"
	@echo "  encrypt-example - 加密示例文件"
	@echo "  decrypt-example - 解密示例文件"
	@echo "  clean          - 清理构建文件"
	@echo "  install        - 安装到系统"
	@echo "  help           - 显示此帮助"