FROM rust:latest AS builder

WORKDIR /app

# 复制当前目录的所有内容到工作目录
COPY ./ .

RUN cargo build --release

# 使用一个轻量级的镜像来运行应用程序
FROM debian:bookworm-slim

# 安装必要的依赖
# 缺少 libssl.so.3 无法运行 -> 安装 libssl3
# 容器无法访问 https 网站 -> 安装 ca-certificates
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# 复制构建好的二进制文件
COPY --from=builder /app/target/release/TinyTileProxy /usr/local/bin/TinyTileProxy

# 设置环境变量
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# 暴露端口
EXPOSE 8000

# 创建挂载点
VOLUME /app

# 运行应用程序
CMD ["TinyTileProxy"]