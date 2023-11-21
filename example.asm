program

  push  0
  load    ; カウンタを出力する
  
  output

  push 0
  load   　; カウンタの値をインクリメントする
  
  push 1
  
  add

  push 0
  store 

  push 0
  load

  push 1　; 目標値とカウンタを比較する
  load  
  
  comp

  push 0 ; 一致しなかったら最初に戻る
  jump  　

  halt
  
data

   1 ; カウンタ
  10 ; 目標値
