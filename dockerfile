FROM rust:1.57

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

RUN cargo install --path .

CMD ["todo"]