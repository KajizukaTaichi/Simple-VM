use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::io::{Error, Read};

fn main() {
    println!("Simple 仮想マシン");
    println!("コンピュータの動作原理を深く学ぶ仮想マシン");
    println!("(c) 2023 梶塚太智. All right reserved");

    let mut program: Vec<Instruction> = Vec::new();
    let mut memory: Vec<i32> = Vec::new();
    let args = env::args().collect::<Vec<_>>();

    match get_file_contents(args[1].clone()) {
        Ok(code) => {
            let res = assembly(code);
            program = res.0;
            memory = res.1;
        }
        Err(e) => println!("エラー {e}"),
    }

    let mut vm = VirtualMachine::new(program, memory);
    vm.run();
}
pub fn input(prompt: &str) -> String {
    print!("{}", prompt.to_string());
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    return result.trim().parse().ok().unwrap();
}

/// ファイルを読み込む
fn get_file_contents(name: String) -> Result<String, Error> {
    let mut f = File::open(name.trim())?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}
/// アセンブラ
fn assembly(asm: String) -> (Vec<Instruction>, Vec<i32>) {
    println!("アセンブル中・・・");
    
    enum Mode {
        Data,
        Program,
    }
    let mut mode = Mode::Program;

    let mut result: Vec<Instruction> = Vec::new();
    let mut memory: Vec<i32> = Vec::new();

    for code in asm.split("\n") {
        let args: Vec<&str> = code.split(" ").collect();
        if args[0] == "DATA" || args[0] == "data" {
            mode = Mode::Data;
            continue;
        }

        if args[0] == "PROGRAM" || args[0] == "program"{
            mode = Mode::Program;
            continue;
        }

        if let Mode::Program = mode {
            result.push(match args[0] {
                "ADD" | "add" => Instruction::Add,
                "SUB" | "sub" => Instruction::Sub,
                "PUSH" | "push" => Instruction::Push(args[1].parse().unwrap_or(0)),
                "POP" | "pop" => Instruction::Pop,
                "COMP" | "comp" => Instruction::Compare,
                "JUMP" | "jump" => Instruction::JumpIfZero(args[1].parse().unwrap_or(0)),
                "LOAD" | "load" => Instruction::Load(args[1].parse().unwrap_or(0)),
                "STORE" | "store" => Instruction::Store(args[1].parse().unwrap_or(0)),
                "INPUT" | "input" => Instruction::Input,
                "OUTPUT" | "output" => Instruction::Output,
                "HALT" | "halt" => Instruction::Halt,
                _ => {
                    println!("エラー! 不明な命令です");
                    Instruction::Nop
                }
            })
        } else {
            memory.push(args[0].trim().parse().unwrap_or(0))
        }
    }
    println!("変換されたプログラム");
    result.iter().for_each(|r| println!("| {:?}", r));

    return (result, memory);
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
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
    Nop,
    Halt,
}

struct VirtualMachine {
    program: Vec<Instruction>,
    data: Vec<i32>,
    stack: Vec<i32>,
    pc: usize,
}

impl VirtualMachine {
    fn new(program: Vec<Instruction>, data: Vec<i32>) -> VirtualMachine {
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
            if menu.contains("stack") {
                println!("スタック {:?}", self.stack);
            } else if menu.contains("mem") {
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

    fn run(&mut self) {
        println!("プログラムを実行します");
        while self.pc < self.program.len() {
            let instruction = self.program[self.pc].clone();
            println!("{}行目の{:?}を実行します", self.pc, instruction);
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
                        println!("{target}行目にジャンプします");
                        self.pc = target;
                        continue;
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
                Instruction::Nop => {}
                Instruction::Halt => return,
            }
            self.debug_menu();
            self.pc += 1;
        }
    }
}
