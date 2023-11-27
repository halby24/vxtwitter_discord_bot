# 使用するRustのバージョンを指定
FROM rust:1.74 as builder

# ビルド引数
ARG APP_NAME=shiratama_bot

# 作業ディレクトリを設定
WORKDIR /usr/src/$APP_NAME

# 依存関係のみを先にコピーしてビルド
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build && \
    rm -rf ./*

# 実際のソースコードをコピーしてビルド
COPY . .
RUN cargo build 

RUN ln -s /usr/src/$APP_NAME/target/debug/$APP_NAME /usr/local/bin/$APP_NAME

ENV APP_NAME $APP_NAME

# コンテナ起動時に実行されるコマンド
CMD $APP_NAME