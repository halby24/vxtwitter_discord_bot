# vxtwitter discord bot

## 概要
Discord上でx.comのリンクが投稿されると、vxtwitter.comのリンクに書き換えて投稿するDiscord Botです。

## 機能
- **リンクの自動検出**: ユーザーがDiscordチャンネルにx.comのリンクを投稿すると、Botがそれを検出します。
- **リンクの自動書き換え**: 検出されたリンクは、自動的にvxtwitter.comに置き換えられたうえでBOTによって投稿されます。

## インストール方法
Docker Composeでの利用方法を記載します。
1. `docker-compose.yml` ファイルを作成します（以下にサンプルを示します）。
1. `BOT_TOKEN` 環境変数にDiscord Botのトークンを設定します。
1. `docker-compose up -d` コマンドでDockerコンテナを起動します。

### `docker-compose.yml` 設定例
```yaml
version: '3.8'
services:
  vxtwitter_discord_bot:
    image: ghcr.io/halby24/vxtwitter_discord_bot:latest
    environment:
      - BOT_TOKEN=あなたのDiscordBotトークン
    restart: unless-stopped
```

### BOTのパーミッション
下記のパーミッションをBOTに付与してください。
ボットの権限フラグの整数は`19456`です。
- Read Messages/View Channels
- Send Messages
- Embed Links

## BOTの使用方法
- BotをあなたのDiscordサーバーに招待します。
- x.comのリンクを任意のチャンネルに投稿します。
- Botがリンクを自動的にvxtwitter.comのフォーマットに書き換えて投稿します。

## 開発環境の設定
このプロジェクトの開発環境は、Visual Studio CodeのDevContainer機能を利用してセットアップされています。以下の手順に従って開発環境を構築してください。

1. リポジトリの `.devcontainer` ディレクトリ内にある `.env.example` ファイルを複製します。
2. 複製したファイルを `.env` としてリネームします。
3. `.env` ファイルに開発用のDiscord Botのトークンを `BOT_TOKEN` として記述します。
4. Visual Studio Codeでリポジトリを開き、DevContainer機能を使用してコンテナを起動します。

## ライセンス
このプロジェクトは [MITライセンス](LICENSE) の下で公開されています。
