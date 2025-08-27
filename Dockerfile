# 使用官方Rust镜像作为构建环境
FROM rust:1.70 as builder

# 设置工作目录
WORKDIR /usr/src/zuc

# 复制Cargo文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟项目以缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 复制源代码
COPY . .

# 构建项目
RUN cargo build --release

# 使用轻量级运行时镜像
FROM debian:bullseye-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -r -s /bin/false zuc

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /usr/src/zuc/target/release/zuc /app/zuc

# 更改文件所有者
RUN chown zuc:zuc /app/zuc

# 切换到非root用户
USER zuc

# 暴露端口
EXPOSE 8080

# 设置环境变量
ENV RUST_LOG=info
ENV ZUC_HOST=0.0.0.0
ENV ZUC_PORT=8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 启动命令
CMD ["./zuc", "-s"]