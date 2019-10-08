# 開発手順

このリポジトリでの開発手順についてのガイドです。

## データモデルの追加

`aworld_datagen`では、`diesel`を用いてMySQLスキーマベースのデータモデル管理をしています。MySQLについてはリファレンスマニュアルを参照してください。

- [MySQL :: MySQL 5.6 リファレンスマニュアル](https://dev.mysql.com/doc/refman/5.6/ja/)

Rustプログラム内におけるデータモデルは構造体で、`src/models/`直下に定義されています。これらは手動で生成する必要がありますが、構造体名とフィールドは全て`src/schema.rs`と対応します。記法は`diesel`に準じ、`src/schema.rs`は`diesel migration run`コマンドによって、`migrations/**/up.sql`を順番に実行した結果MySQLテーブル定義から自動生成されます。`migrations/**/*.sql`は`diesel migration generate [migration_name]`コマンドによって生成され、内容は手動で書き換える必要があります。

よって、例えば新たに`Character`データモデルを追加するには以下の手順を踏む必要があります。

1. `diesel migration generate create_characters`コマンドを実行し、`migrations/**_create_characters/*.sql`を生成
1. `migrations/**_create_characters/*.sql`を編集し、テーブル作成・破棄のクエリを記述
1. `diesel migration run`コマンドを実行し、`src/schema.rs`を更新
1. `src/models/character.rs`を作成し、テーブルに対応する構造体定義・挿入用構造体定義
1. `src/models/mod.rs`に`pub mod character`を追記し、他のファイルから使用可能にする

## データの自動生成

`impl std::default::Default for ** {}`
