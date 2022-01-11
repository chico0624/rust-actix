# Rust actix-web
## 概要
RustでWebアプリケーションを作成する際のサンプル。  
アプリケーション自体の内容はTodoリスト

処理内容は下記参考書Capter 5の内容です。
一部、クレートのバージョンによるもので、本書通り動かない箇所があるため改変済み。
また、Dockerを利用しているが、バージョンについては、docker hubを確認すること。  

[実践Rustプログラミング入門](https://www.amazon.co.jp/%E5%AE%9F%E8%B7%B5Rust%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E5%85%A5%E9%96%80-%E5%88%9D%E7%94%B0-%E7%9B%B4%E4%B9%9F/dp/4798061700)

## Usage
```
# ビルド
docker build -t todo-app .

# run
docker run -p 8080:8080 todo-app
```

## docker hub
[Rust](https://hub.docker.com/_/rust)  
[Debian](https://hub.docker.com/_/debian)