/// 命令セット
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Add,        // 足し算する
    Sub,        // 引き算する
    Mul,        // 掛け算する
    Div,        // 割り算する
    Mod,        // 割り算の余り
    Push(i32),  // スタックに値をプッシュ
    Pop,        // スタックの値をポップ
    Equal,      // 等しいか判断
    LessThan,   // 未満か判断
    And,        // AND演算を行う
    Or,         // OR演算を行う
    Not,        // NOT演算を行う
    JumpIfZero, // 値が0の場合ジャンプする
    Load,       // メモリの値を読み込む
    Store,      // メモリに値を保存する
    Input,      // 入力を受け付ける
    Output,     // UTF-8で出力する
    Read,       // ストレージを読み込む
    Write,      // ストレージに書き込む
    Halt,       // プログラムを終了する
    WinAPI,     // Windows APIを呼び出す
}
