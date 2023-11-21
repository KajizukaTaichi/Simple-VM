use crate::io::input;

/// バイナリに変換
pub fn as_bin(item: Instruction) -> String {
    format!(
        "{:0>9b}",
        match item {
            Instruction::Add => 1,
            Instruction::Sub => 2,
            Instruction::Push(i) => format!("3{i}").parse().unwrap(),
            Instruction::Pop => 4,
            Instruction::Compare => 5,
            Instruction::JumpIfZero(i) => format!("6{i}").parse().unwrap(),
            Instruction::Load(i) => format!("7{i}").parse().unwrap(),
            Instruction::Store(i) => format!("8{i}").parse().unwrap(),
            Instruction::Input => 9,
            Instruction::Output => 10,
            Instruction::Halt => 11,
        }
    )
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Add,
    Sub,
    Push(i32),
    Pop,
    Compare,
    JumpIfZero(usize),
    Load(usize),
    Store(usize),
    Input,
    Output,
    Halt,
}

pub struct VirtualMachine {
    program: Vec<Instruction>,
    data: Vec<i32>,
    stack: Vec<i32>,
    pc: usize,
}

impl VirtualMachine {
    pub fn new(program: Vec<Instruction>, data: Vec<i32>) -> VirtualMachine {
        let mut vm = VirtualMachine {
            program,
            data: vec![0; 100], // データ領域を100要素の配列として初期化
            stack: Vec::new(),
            pc: 0,
        };

        for i in 0..data.len() {
            vm.data[i] = data[i]
        }

        return vm;
    }

    fn show_memory(&self) {
        println!("+-- メモリ内部");
        for i in 0..self.data.len() {
            if self.data[i] != 0 {
                println!("| {i:0>3} :  {}", self.data[i]);
            }
        }
    }

    // デバッグメニューを表示する
    fn debug_menu(&mut self) {
        loop {
            let menu = input("デバッグメニュー>>> ");
            if menu.contains("s") {
                println!("スタック {:?}", self.stack);
            } else if menu.contains("m") {
                self.show_memory();
            } else if menu.contains("exit") {
                input("デバッグを中断します");
                std::process::exit(0)
            } else {
                println!("継続します");
                break;
            }
        }
    }

    pub fn run(&mut self) {
        println!("プログラムを実行します");
        while self.pc < self.program.len() {
            let instruction = self.program[self.pc].clone();
            println!(
                "プログラム{}行目の「{}」を実行します",
                self.pc,
                as_bin(instruction)
            );
            match instruction {
                Instruction::Add => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    println!("{a}と{b}を足します");
                    self.stack.push(a + b);
                }
                Instruction::Sub => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    println!("{a}から{b}を引きます");
                    self.stack.push(a - b);
                }
                Instruction::Push(value) => self.stack.push(value),
                Instruction::Pop => {
                    let _ = self.stack.pop().expect("Stack underflow");
                }
                Instruction::Compare => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    println!("{a}と{b}を比較します");
                    if a == b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                }
                Instruction::JumpIfZero(target) => {
                    let condition = self.stack.pop().expect("Stack underflow");
                    if condition == 0 {
                        println!("条件が一致したので{target}行目にジャンプします");
                        self.pc = target;
                        self.debug_menu();
                        continue;
                    } else {
                        println!("条件が一致しなかったのでジャンプしません")
                    }
                }
                Instruction::Load(index) => {
                    println!("メモリ{index}を読み込みます");
                    let value = self.data[index];
                    self.stack.push(value);
                }
                Instruction::Store(index) => {
                    let value = self.stack.pop().expect("Stack underflow");
                    println!("メモリ{index}に{value}を書き込みます");
                    self.data[index] = value;
                }
                Instruction::Input => {
                    println!("入力を受け付けます");
                    self.stack.push(input("[入力]> ").parse().unwrap_or(0));
                }
                Instruction::Output => {
                    println!("[出力]: {}", self.stack.pop().expect("Stack underflow"));
                }
                Instruction::Halt => {
                    println!("プログラムを終了します");
                    return;
                }
            }
            self.debug_menu();
            self.pc += 1;
        }
    }
}