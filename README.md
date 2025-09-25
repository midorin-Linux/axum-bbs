# axum-bbs
このリポジトリは自身の学習用に作成されました。将来的に整備されない可能性もありますが、参考程度にしてください。

## 構造
```aiexclude
axum-bbs/
├── migrations/
│   └── 20250921020554_init.sql     // migrationファイル
├── src/
│   ├── main.rs                     // エントリーポイント
│   ├── config.rs                   // 設定を読み込む
│   ├── database.rs                 // データベースの初期化
│   └── response.rs                 // 静的コンテンツ配信処理
├── static/                         // 静的コンテンツ置き場所
│   └── images/
│       ├── favicon.ico
│       └── ferris.png
├── templates/                      // htmlテンプレート置き場所
│   └── index.html
├── .env
├── .gitignore                      // 設定ファイル
├── Cargo.lock
├── Cargo.toml
└── README.md
```