use std::{fs::File, io::Read};

use crate::assembly;
use crate::instruction::Instruction;
use crate::io;

/// 実行モード
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Execute,
    Debug,
}

/// 仮想マシン
pub struct VirtualMachine {
    memory: Vec<i32>, // メモリ内部
    stack: Vec<i32>,  // スタック
    storage: File,    // 補助記憶装置
    pc: usize,        // プログラムカウンタ
    mode: Mode,       // 実行モード
    output: String,   // 出力した文字列
}

impl VirtualMachine {
    pub fn new(mut storage: File, mode: Mode) -> VirtualMachine {
        let mut code = String::new();
        let _ = storage.read_to_string(&mut code);
        let memory = assembly::assembly(code.to_string());
        let mut vm = VirtualMachine {
            memory: {
                let mut temp = vec![0; 512];
                for i in 0..memory.clone().len() {
                    temp[i] = memory[i];
                }
                temp
            },
            stack: Vec::new(),
            storage,
            pc: 0,
            mode,
            output: String::new(),
        };

        for i in 0..memory.len() {
            vm.memory[i] = memory[i]
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
            let menu = io::input("デバッグメニュー>>> ");
            if menu.contains("s") {
                println!("スタック {:?}", self.stack);
            } else if menu.contains("m") {
                println!("+-- メモリ内部");
                for i in 0..self.memory.len() {
                    if self.memory[i] != 0 {
                        println!("| {i:0>3} :  {}", self.memory[i]);
                    }
                }
            } else if menu.contains("o") {
                println!("+-- 標準出力");
                for i in self.output.split("\n").collect::<Vec<&str>>() {
                    println!("| {i}");
                }
            } else if menu.contains("exit") {
                io::input("デバッグを中断します");
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
                let b = self.pop();
                let a = self.pop();
                self.log_print(format!("{a}と{b}を足します"));
                self.stack.push(a + b);
            }
            Instruction::Sub => {
                let b = self.pop();
                let a = self.pop();
                self.log_print(format!("{a}から{b}を引きます"));
                self.stack.push(a - b);
            }
            Instruction::Mul => {
                let b = self.pop();
                let a = self.pop();
                self.log_print(format!("{a}と{b}を掛けます"));
                self.stack.push(a * b);
            }
            Instruction::Div => {
                let b = self.pop();
                let a = self.pop();
                self.log_print(format!("{a}を{b}で割ります"));
                self.stack.push(a / b);
            }
            Instruction::Mod => {
                let b = self.pop();
                let a = self.pop();
                self.log_print(format!("{a}÷{b}の余りを求めます"));
                self.stack.push(a % b);
            }
            Instruction::Push(value) => {
                self.log_print(format!("{value}をスタックに追加します"));
                self.stack.push(value)
            }
            Instruction::Pop => {
                self.log_print(format!("スタックから値を削除します"));
                let _ = self.pop();
            }
            Instruction::Equal => {
                let b = self.pop();
                let a = self.pop();
                self.log_print(format!("{}と{}が等しいかを判断します", a, b));
                let result = a == b;
                if result {
                    self.log_print(format!("条件が一致したので1を返します"));
                    self.stack.push(1);
                } else {
                    self.log_print(format!("条件が一致なかったので0を返します"));
                    self.stack.push(0);
                }
            }
            Instruction::LessThan => {
                let b = self.pop();
                let a = self.pop();
                self.log_print(format!("{}が{}が未満かを判断します", a, b));
                let result = a < b;
                if result {
                    self.log_print(format!("条件が一致したので1を返します"));
                    self.stack.push(1);
                } else {
                    self.log_print(format!("条件が一致なかったので0を返します"));
                    self.stack.push(0);
                }
            }
            Instruction::And => {
                let b = self.pop();
                let a = self.pop();
                self.log_print(format!("{}と{}でAND条件が成立するかを判断します", a, b));
                let result = a != 0 && b != 0;
                if result {
                    self.log_print(format!("条件が一致したので1を返します"));
                    self.stack.push(1);
                } else {
                    self.log_print(format!("条件が一致なかったので0を返します"));
                    self.stack.push(0);
                }
            }
            Instruction::Or => {
                let b = self.pop();
                let a = self.pop();
                self.log_print(format!("{}と{}でOR条件が成立するかを判断します", a, b));
                let result = a != 0 || b != 0;
                if result {
                    self.log_print(format!("条件が一致したので1を返します"));
                    self.stack.push(1);
                } else {
                    self.log_print(format!("条件が一致なかったので0を返します"));
                    self.stack.push(0);
                }
            }
            Instruction::Not => {
                let b = self.pop();
                self.log_print(format!("{}の値を否定します", b));
                self.stack.push(!b);
            }
            Instruction::JumpIfZero => {
                let condition = self.pop();
                let target = self.pop();
                if condition == 0 {
                    self.log_print(format!("値が0に一致したので{target}行目にジャンプします"));
                    self.pc = target as usize;
                } else {
                    self.log_print("値が0にが一致しなかったのでジャンプしません".to_string());
                }
            }
            Instruction::Load => {
                let index = self.pop();
                self.log_print(format!("メモリ{index}番地の値を読み込みます"));
                let value = self.memory[index as usize];
                self.stack.push(value);
            }
            Instruction::Store => {
                let index = self.pop();
                let value = self.pop();
                self.log_print(format!("メモリ{index}番地に{value}を書き込みます"));
                self.memory[index as usize] = value;
            }
            Instruction::Input => {
                self.log_print(format!("入力を受け付けます"));
                if let Mode::Execute = self.mode {
                    self.stack.push(io::input("> ").parse().unwrap_or(0));
                } else {
                    self.stack.push(io::input("[入力]> ").parse().unwrap_or(0));
                }
            }
            Instruction::Output => {
                let value = self.pop();
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
            Instruction::Read => {
                let index = self.pop();
                self.log_print(format!("ストレージ{}行目の値を読み込みます", index));
                self.stack.push(
                    io::read_specific_line(&self.storage, index as usize)
                        .unwrap()
                        .parse()
                        .unwrap_or(0),
                );
            }
            Instruction::Write => {
                let index = self.pop();
                let value = self.pop();
                self.log_print(format!("ストレージ{}行目に値{}を書き込みます", index, value));
                let _ = io::write_specific_line(
                    &self.storage,
                    index as usize,
                    value.to_string().as_str(),
                );
            }
            Instruction::Halt => {
                self.log_print(format!("プログラムを終了します"));
                std::process::exit(0);
            }
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().expect("Stack underflow")
    }

    pub fn run(&mut self) {
        println!("プログラムを実行します");
        while self.pc < self.memory.len() {
            let instruction = self.memory[self.pc].clone();
            let result = match instruction {
                0 => Instruction::Add,
                1 => Instruction::Sub,
                2 => Instruction::Mul,
                3 => Instruction::Div,
                4 => Instruction::Mod,
                5 => {
                    self.pc += 1;
                    Instruction::Push(self.memory[self.pc])
                }
                6 => Instruction::Pop,
                7 => Instruction::Equal,
                8 => Instruction::LessThan,
                9 => Instruction::And,
                10 => Instruction::Or,
                11 => Instruction::Not,
                12 => Instruction::JumpIfZero,
                13 => Instruction::Load,
                14 => Instruction::Store,
                15 => Instruction::Input,
                16 => Instruction::Output,
                17 => Instruction::Read,
                18 => Instruction::Write,
                19 => Instruction::Halt,
                _ => {
                    self.pc += 1;
                    continue;
                }
            };
            self.log_print(format!(
                "メモリ{}番目の命令コード{}を実行します",
                self.pc, instruction
            ));
            self.execute(result);

            if let Mode::Debug = self.mode {
                self.debug_menu();
            }
            self.pc += 1;
        }
    }
}
