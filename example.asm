program

  load   0 ; カウンタを出力する
  output

  load   0　; カウンタの値をインクリメントする
  push   1
  add
  store  0

  load   0　; 目標値とカウンタを比較する
  load   1
  comp
  jump   0　; 一致しなかったら最初に戻る

  halt
  
data

   1 ; カウンタ
  10 ; 目標値