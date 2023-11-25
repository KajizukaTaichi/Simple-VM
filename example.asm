program
  
  ; カウンタを加算する
  push 0
  load
  push 1
  add
  push 0
  store

  ; 出力する
  push 0
  load
  load
  output

　; 比較する
  push 0
  load

  push 1
  load
  push 2
  add

  comp =

  push 0
  jump

  halt
data
   01 ; カウンタ
   12 ; 文字列の長さ

   72 ; 'H'のUTF-8コード
  101 ; 'e'のUTF-8コード
  108 ; 'l'のUTF-8コード
  108 ; 'l'のUTF-8コード
  111 ; 'o'のUTF-8コード
   44 ; ','のUTF-8コード
   32 ; ' 'のUTF-8コード
  119 ; 'w'のUTF-8コード
  111 ; 'o'のUTF-8コード
  114 ; 'r'のUTF-8コード
  108 ; 'l'のUTF-8コード
  100 ; 'd'のUTF-8コード
   33 ; '!'のUTF-8コード