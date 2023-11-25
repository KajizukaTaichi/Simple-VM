program

  ; カウンタを出力する
  push  0
  load    
  output

  ; カウンタの値をインクリメントする
  push 0
  load   
  push 1
  add
  push 0
  store 

  ; 目標値とカウンタが等しいか？
  push 0
  load
  push 1　
  load  
  comp = 

  ; 一致しなかったら最初に戻る
  push 0
  jump  　

  halt
  
data

   1 ; カウンタ
  10 ; 目標値
