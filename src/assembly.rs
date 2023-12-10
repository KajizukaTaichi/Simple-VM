/// アセンブラ
pub fn assembly(asm: String) -> Vec<i32> {
    println!("アセンブル中・・・");
    let mut memory: Vec<i32> = Vec::new();

    for code in asm.split("\n") {
        let code = code.trim().split(";").collect::<Vec<&str>>()[0];
        let args: Vec<&str> = if code.contains(" ") || code.contains("　") {
            code.split_whitespace().collect()
        } else {
            vec![code.trim()]
        };

        if args[0] == "data" {
            continue;
        }

        if args[0] == "program" {
            continue;
        }

        match args[0] {
            "add" => memory.push(1),
            "sub" => memory.push(2),
            "mul" => memory.push(3),
            "div" => memory.push(4),
            "mod" => memory.push(5),
            "push" => memory.push(6),
            "pop" => memory.push(7),
            "equal" => memory.push(8),
            "lessthan" => memory.push(9),
            "and" => memory.push(10),
            "or" => memory.push(11),
            "not" => memory.push(12),
            "jump" => memory.push(13),
            "load" => memory.push(14),
            "store" => memory.push(15),
            "input" => memory.push(16),
            "output" => memory.push(17),
            "read" => memory.push(18),
            "write" => memory.push(19),
            "halt" => memory.push(20),
            "winapi" => memory.push(21),
            "" => memory.push(0),
            _ => memory.push(args[0].trim().parse().unwrap_or(0)),
        }
    }

    return memory;
}
