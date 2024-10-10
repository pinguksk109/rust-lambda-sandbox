# rust-lambda-sandbox

# ローカルで実行
```sh
cd rust_app
```
```sh
cargo lambda start
```
別タブ開いて、プロジェクト直下で以下を実行
```sh
cargo lambda invoke --data-file event.json rust-lambda-sandbox
```