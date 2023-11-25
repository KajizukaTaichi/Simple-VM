use crate::io::input;

/// 命令セット
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Push(i32),
    Pop,
    Compare(Comparison),
    JumpIfZero,
    Load,
    Store,
    Input,
    Output,
    Halt,
}

/// 比較演算子
#[derive(Debug, Clone, Copy)]
pub enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

/// 実行モード
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Execute,
    Debug,
}

/// 仮想マシン
pub struct VirtualMachine {
    program: Vec<Instruction>, // プログラム領域
    data: Vec<i32>,            // データ領域
    stack: Vec<i32>,           // スタック
    pc: usize,                 // プログラムカウンタ
    mode: Mode,                // 実行モード
    output: String             // 出力した文字列 
}

impl VirtualMachine {
    pub fn new(program: Vec<Instruction>, data: Vec<i32>, mode: Mode) -> VirtualMachine {
        let mut vm = VirtualMachine {
            program,
            data: vec![0; 100], // データ領域を100要素の配列として初期化
            stack: Vec::new(),
            pc: 0,
            mode,
            output: String::new()
        };

        for i in 0..data.len() {
            vm.data[i] = data[i]
        }

        return vm;
    }

    /// ログ出力
    fn log_print(&mut self, text: String) {
        if let Mode::Debug = self.mode {
            println!("{text}");
        }
    }

    /// デバッグメニューを表示する
    fn debug_menu(&mut self) {
        loop {
            let menu = input("デバッグメニュー>>> ");
            if menu.contains("s") {
                println!("スタック {:?}", self.stack);
            } else if menu.contains("m") {
                println!("+-- メモリ内部");
                for i in 0..self.data.len() {
                    if self.data[i] != 0 {
                        println!("| {i:0>3} :  {}", self.data[i]);
                    }
                }
            } else if menu.contains("o") {
                println!("+-- 標準出力");
                for i in self.output.split("\n").collect::<Vec<&str>>(){
                    println!("| {i}");
                }
            } else if menu.contains("exit") {
                input("デバッグを中断します");
                std::process::exit(0)
            } else {
                println!("継続します");
                break;
            }
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.log_print(format!("{a}と{b}を足します"));
                self.stack.push(a + b);
            }
            Instruction::Sub => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.log_print(format!("{a}から{b}を引きます"));
                self.stack.push(a - b);
            }
            Instruction::Mul => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.log_print(format!("{a}と{b}を掛けます"));
                self.stack.push(a * b);
            }
            Instruction::Div => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.log_print(format!("{a}を{b}で割ります"));
                self.stack.push(a / b);
            }
            Instruction::Mod => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.log_print(format!("{a}÷{b}の余りを求めます"));
                self.stack.push(a % b);
            }
            Instruction::Push(value) => {
                self.log_print(format!("{value}をスタックに追加します"));
                self.stack.push(value)
            }
            Instruction::Pop => {
                self.log_print(format!("スタックから値を削除します"));
                let _ = self.stack.pop().expect("Stack underflow");
            }
            Instruction::Compare(comparison) => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.log_print(format!(
                    "条件「{} {} {}」を判断します",
                    a,
                    match comparison {
                        Comparison::Equal => "=",
                        Comparison::NotEqual => "!",
                        Comparison::LessThan => "<",
                        Comparison::GreaterThan => ">",
                    },
                    b
                ));
                let result = match comparison {
                    Comparison::Equal => a == b,
                    Comparison::NotEqual => a != b,
                    Comparison::LessThan => a < b,
                    Comparison::GreaterThan => a > b,
                };
                if result {
                    self.log_print(format!("条件が一致したので1を返します"));
                    self.stack.push(1);
                } else {
                    self.log_print(format!("条件が一致なかったので0を返します"));
                    self.stack.push(0);
                }
            }
            Instruction::JumpIfZero => {
                let target = self.stack.pop().expect("Stack underflow");
                let condition = self.stack.pop().expect("Stack underflow");
                if condition == 0 {
                    self.log_print(format!("値が0に一致したので{target}行目にジャンプします"));
                    self.pc = target as usize;
                } else {
                    self.log_print("値が0にが一致しなかったのでジャンプしません".to_string());
                }
            }
            Instruction::Load => {
                let index = self.stack.pop().expect("Stack underflow");
                self.log_print(format!("メモリ{index}を読み込みます"));
                let value = self.data[index as usize];
                self.stack.push(value);
            }
            Instruction::Store => {
                let index = self.stack.pop().expect("Stack underflow");
                let value = self.stack.pop().expect("Stack underflow");
                self.log_print(format!("メモリ{index}に{value}を書き込みます"));
                self.data[index as usize] = value;
            }
            Instruction::Input => {
                self.log_print(format!("入力を受け付けます"));
                if let Mode::Execute = self.mode {
                    self.stack.push(input("> ").parse().unwrap_or(0));
                } else {
                    self.stack.push(input("[入力]> ").parse().unwrap_or(0));
                }
            }
            Instruction::Output => {
                let value = self.stack.pop().expect("Stack underflow");
                self.log_print(format!("{value}をUTF-8の文字として出力します"));
                if let Some(c) = std::char::from_u32(value as u32) {
                    if let Mode::Debug = self.mode {
                        println!("[出力]: {}", c);
                        self.output.push(c);
                    } else {
                        print!("{c}")
                    }
                } else {
                    panic!("Invalid UTF-8 character code");
                }
            }
            Instruction::Halt => {
                self.log_print(format!("プログラムを終了します"));
                std::process::exit(0);
            }
        }
    }

    pub fn run(&mut self) {
        println!("プログラムを実行します");
        while self.pc < self.program.len() {
            self.pc += 1;
            let instruction = self.program[self.pc - 1].clone();
            self.log_print(format!(
                "プログラム{}行目の「{:?}」を実行します",
                self.pc - 1,
                instruction
            ));
            self.execute(instruction);

            if let Mode::Debug = self.mode {
                self.debug_menu();
            }
        }
    }
}
