use std::env;

mod assembly;
mod io;
mod vm;

use vm::Instruction;
use vm::VirtualMachine;

fn main() {
    println!("Simple 仮想マシン");
    println!("コンピュータの動作原理を深く学ぶ仮想マシン");
    println!("(c) 2023 梶塚太智. All right reserved");
    let args = env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        match io::get_file_contents(args[1].clone()) {
            Ok(code) => {
                let res = assembly::assembly(code);
                let program: Vec<Instruction> = res.0;
                let memory: Vec<i32> = res.1;
                let mut vm = VirtualMachine::new(program, memory);
                vm.run();
            }
            Err(e) => {
                println!("エラー {e}")
            }
        }
    }
}
