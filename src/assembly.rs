use crate::vm::{as_bin, Instruction};

enum Mode {
    Data,
    Program,
}

/// アセンブラ
pub fn assembly(asm: String) -> (Vec<Instruction>, Vec<i32>) {
    println!("アセンブル中・・・");

    let mut mode = Mode::Program;

    let mut result: Vec<Instruction> = Vec::new();
    let mut memory: Vec<i32> = Vec::new();

    for code in asm.split("\n") {
        let code = code.trim().split(";").collect::<Vec<&str>>()[0];
        let args: Vec<&str> = if code.contains(" ") || code.contains("　") {
            code.split_whitespace().collect()
        } else {
            vec![code.trim()]
        };
        if args[0] == "DATA" || args[0] == "data" {
            mode = Mode::Data;
            continue;
        }

        if args[0] == "PROGRAM" || args[0] == "program" {
            mode = Mode::Program;
            continue;
        }

        if let Mode::Program = mode {
            result.push(match args[0] {
                "ADD" | "add" => Instruction::Add,
                "SUB" | "sub" => Instruction::Sub,
                "PUSH" | "push" => Instruction::Push(args[1].trim().parse().unwrap_or(0)),
                "POP" | "pop" => Instruction::Pop,
                "COMP" | "comp" => Instruction::Compare,
                "JUMP" | "jump" => Instruction::JumpIfZero,
                "LOAD" | "load" => Instruction::Load,
                "STORE" | "store" => Instruction::Store,
                "INPUT" | "input" => Instruction::Input,
                "OUTPUT" | "output" => Instruction::Output,
                "HALT" | "halt" => Instruction::Halt,
                "" => continue,
                _ => {
                    println!("エラー! 不明な命令です");
                    continue;
                }
            })
        } else {
            if !args[0].is_empty() {
                memory.push(args[0].trim().parse().unwrap_or(0))
            }
        }
    }
    println!("変換されたプログラム");
    result.iter().for_each(|r| println!("| {}", as_bin(*r)));

    return (result, memory);
}
