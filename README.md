# todo-list

Rust学習用に作成した、CLIで動作するシンプルなTODO管理アプリです。

## 機能

- **Add**: タイトルと説明を入力して新しいTODOを追加
- **Delete**: IDを指定してTODOを削除（スペース区切りで複数指定可）
- **List**: 登録済みのTODOをID・タイトル・状態（pending/complete）付きで一覧表示
- **Exit**: 終了時にTODOをJSONファイルへ保存

## 技術構成

- [inquire](https://crates.io/crates/inquire) / inquire-derive: コマンド選択や入力プロンプトのUI
- [serde](https://crates.io/crates/serde) / serde_json: TODOデータのシリアライズ・デシリアライズ

## セットアップ・実行方法

```sh
cargo run
```

起動するとコマンド選択のプロンプトが表示されるので、矢印キーなどで操作を選んでください。

## データの保存先

TODOデータは `./data/todos.json` に保存されます。起動時にこのファイルが存在すればそこから読み込み、存在しなければ空の状態から開始します。

## ディレクトリ構成

```
src/
├── main.rs          # エントリーポイント、コマンドループ
├── commands.rs       # コマンド（Add/Delete/List/Exit）の定義
└── todo_mgr.rs        # TODOの追加・削除・保存・読込を管理
    └── todo.rs        # Todo構造体の定義
```
