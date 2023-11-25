mod assembly;
mod io;
mod vm;

use std::env;
use vm::Instruction;
use vm::Mode;
use vm::VirtualMachine;

fn main() {
    println!("Simple 仮想マシン");
    println!("コンピュータの動作原理を深く学ぶ仮想マシン");
    println!("(c) 2023 梶塚太智. All right reserved");
    let args = env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        match io::get_file_contents(args[1].clone()) {
            Ok(code) => {
                let mode = if args.len() > 2 {
                    if args[2].contains("e") {
                        Mode::Execute
                    } else {
                        Mode::Debug
                    }
                } else {
                    Mode::Debug
                };
                let res = assembly::assembly(code);
                let program: Vec<Instruction> = res.0;
                let memory: Vec<i32> = res.1;
                let mut vm = VirtualMachine::new(program, memory, mode);
                vm.run();
            }
            Err(e) => {
                println!("エラー {e}")
            }
        }
    } else {
        println!("アセンブリのファイルを指定してください")
    }
}
