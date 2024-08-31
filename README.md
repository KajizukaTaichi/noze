# Noze

アセンブリ風の低レイヤ技術教育向け日本語プログラミング言語なのぜ！

|サンプル画像|
|:-|
|<img width="639" alt="image" src="https://github.com/user-attachments/assets/a1c205ce-c1b0-4951-a106-e44ba3f21c75">|

## 仕様

プログラム文は`。`で区切りられます。空白行の実行は無視されるのぜ。
終端には必ず`のぜ`を付ける必要があるのぜ。だからNozeという言語名なのぜ。

### リテラル

数値リテラルは、漢数字ではなくインド・アラビア数字を使うのぜ。全角か半角かは問わないのぜ。
数値型は６４ビットの浮動小数点数なのぜ。
```
３.１４
```


文字列リテラルは、鉤括弧`「」`で囲むのぜ。
```
「ソヴィエト万歳！」
```

論理値のリテラルは、`真`か`偽`の二つなのぜ。単純なので例はないのぜ。

処理できなかった値は最終的に文字列として処理されるのぜ。

### 定義

このコードの場合、ＯＯという変数にＸＸというリテラルが代入されるのぜ。
```
ＯＯはＸＸなのぜ。
```

このように変数名を省略した場合はＸＸというラベルが現在のプログラムカウンタの値で定義されるのぜ。
```
ＸＸなのぜ。
```
ラベルは関数呼び出しや制御構造でジャンプする時などに使うのぜ。


## インストール
以下のコマンドでインストール出来るのぜ。
```sh
rade install noze
```
radeのインストールはこちらのぜ：
https://github.com/rade-package-manager/rade-package-manager

または、このリポジトリを直接クローンしてビルドしても良いのぜ。
