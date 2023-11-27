# 使用するRustのバージョンを指定
FROM rust:1.74 as builder

# ビルド引数
ARG APP_NAME=shiratama_bot

# 作業ディレクトリを設定
WORKDIR /usr/src/$APP_NAME

# ソースコードをコピー
COPY . .

# プロジェクトをビルド
RUN cargo build --release

# 実行ステージ
FROM debian:buster-slim
COPY --from=builder /usr/src/$APP_NAME/target/release/$APP_NAME /usr/local/bin/$APP_NAME

# コンテナ起動時に実行されるコマンド
CMD [$APP_NAME]