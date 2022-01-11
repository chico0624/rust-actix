FROM rust:1.57 As builder

WORKDIR /todo

COPY Cargo.toml Cargo.toml

RUN mkdir src
RUN echo "fn main(){}" > src/main.rs

# 先に依存クレートをビルドしておく
RUN cargo build --release

COPY ./src ./src
COPY ./templates ./templates

# 事前作成した成果物のうち、アプリのみを削除(cleanはしない)
RUN rm -f target/release/deps/todo*

# 改めてアプリをビルド
RUN cargo build --release

# リリース用イメージを用意、Rustツールチェーンは不要なのでdebianを使用
FROM debian:11.2

# builderイメージからtodoのみをコピーして配置「cargo install --path .」の代替
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo
CMD ["todo"]