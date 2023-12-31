# Simple 仮想マシン
コンピュータの動作原理を深くコアの部分まで学ぶの教育用の仮想マシンです。`Simpleアセンブリ`でプログラムを開発できます。
現段階ではまだSimpleプログラミング言語からのコンパイルには対応していませんが、将来的にはコンパイルし、Simpleプログラミング言語の実行環境になる予定です。

## 命令セット
以下はバイトコードの命令セットです。
|アセンブリ|バイトコード|意味|
|:-|:-|:-|
|add|1|足し算する|
|sub|2|引き算する|
|mul|3|掛け算する|
|div|4|割り算する|
|mod|5|割り算の余り|
|push|6|次のメモリアドレスの値をスタックにプッシュする|
|pop|7|スタックからポップする|
|equal|8|等しいか判断する|
|lessthan|9|未満か判断する|
|and|10|And演算を行う|
|or|11|Or演算を行う|
|not|12|Not演算を行う|
|jump|13|値が0ならば指定したメモリアドレスへジャンプする|
|load|14|指定したメモリアドレスを読み込む|
|store|15|指定したメモリアドレスに値を書き込む|
|input|16|入力を受け付ける|
|output|17|値をUTF-8で出力する|
|read|18|ストレージから値を読み込む|
|write|19|ストレージに値を書き込む|
|halt|20|シャットダウンする|
|winapi|21|WindowsAPIを利用する


## WindowsAPIの関数番号リスト
|関数番号|意味|
|-:|:-|
|1|メッセージボックスAを表示する|
## 実行
Simple仮想マシンはコンピュータの動作原理を学ぶためデフォルト値ではデバッグモードになりますが、一気に実行することもできます。
そのためには`execute`とコマンドライン引数に入れてください。
```powershell
> simple_vm.exe example.asm execute 
```
