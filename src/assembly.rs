enum Mode {
    Data,
    Program,
}

/// アセンブラ
pub fn assembly(asm: String) -> Vec<i32> {
    println!("アセンブル中・・・");

    let mut mode = Mode::Program;
    let mut memory: Vec<i32> = Vec::new();

    for code in asm.split("\n") {
        let code = code.trim().split(";").collect::<Vec<&str>>()[0];
        let args: Vec<&str> = if code.contains(" ") || code.contains("　") {
            code.split_whitespace().collect()
        } else {
            vec![code.trim()]
        };

        if args[0] == "data" {
            mode = Mode::Data;
            continue;
        }

        if args[0] == "program" {
            mode = Mode::Program;
            continue;
        }

        if let Mode::Program = mode {
            match args[0] {
                "add" => memory.push(0),
                "sub" => memory.push(1),
                "mul" => memory.push(2),
                "div" => memory.push(3),
                "mod" => memory.push(4),
                "push" => {
                    memory.push(5);
                    memory.push(args[1].trim().parse().unwrap_or(0))
                }
                "pop" => memory.push(6),
                "equal" => memory.push(7),
                "lessthan" => memory.push(8),
                "and" => memory.push(9),
                "or" => memory.push(10),
                "not" => memory.push(11),
                "jump" => memory.push(12),
                "load" => memory.push(13),
                "store" => memory.push(14),
                "input" => memory.push(15),
                "output" => memory.push(16),
                "halt" => memory.push(17),
                "" => continue,
                _ => {
                    println!("エラー! 不明な命令です");
                    continue;
                }
            }
        } else {
            if !args[0].is_empty() {
                memory.push(args[0].trim().parse().unwrap_or(0))
            }
        }
    }

    return memory;
}
