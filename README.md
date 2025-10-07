# axum-bbs
このリポジトリは自身の学習用に作成されました。将来的に整備されない可能性もありますが、参考程度にしてください。

## 構造
```aiexclude
axum-bbs/
│  .env                         // 環境設定用
│  .gitignore
│  Cargo.lock
│  Cargo.toml
│  README.md
│
├─migrations
│      20250921020554_init.sql  // SQLのマイグレーション
│
├─src
│  │  config.rs                 // 設定の読み込み
│  │  main.rs                   // エントリーポイント
│  │  response.rs
│  │
│  ├─handlers
│  │      mod.rs
│  │      posts.rs              // ポスト処理
│  │
│  ├─middleware
│  │      logging.rs            // 通信内容をロギングするミドルウェア
│  │      mod.rs
│  │
│  └─models
│          mod.rs
│          post.rs              // ポストのデータ型
│
├─static
│  └─images
│          favicon.ico
│          ferris.png
│
└─templates
        index.html
```
