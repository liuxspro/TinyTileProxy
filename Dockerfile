FROM rust:latest AS builder

WORKDIR /app

# 安装 musl 工具链
RUN rustup target add x86_64-unknown-linux-musl

# 安装 musl
RUN apt-get update && apt-get install -y musl-tools

# 复制当前目录的所有内容到工作目录
COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

# 使用一个轻量级的镜像来运行应用程序
FROM alpine:latest

WORKDIR /app
# 复制构建好的二进制文件
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/TinyTileProxy /usr/local/bin/TinyTileProxy

# 设置环境变量
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV ROCKET_USE_HTTPS=false

# 暴露端口
EXPOSE 8000

# 创建挂载点
VOLUME /app

# 运行应用程序
CMD ["TinyTileProxy"]