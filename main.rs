enum Instruction {
    Add,
    Sub,
    Read
    Write
    JumpIfZero(usize),
}

struct VirtualMachine {
    stack: Vec<i32>,
    pc: usize, // プログラムカウンタ
    program_memory: Vec<Instruction>,
    data_memory: Vec<i32>,
}

impl VirtualMachine {
    fn new() -> Self {
        VirtualMachine {
            stack: Vec::new(),
            pc: 0,
            program_memory: Vec::new(),
            data_memory: vec![0; 100], // データ領域を100要素の0で初期化
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    fn fetch_program(&mut self) -> Option<&Instruction> {
        if self.pc < self.program_memory.len() {
            let instruction = &self.program_memory[self.pc];
            self.pc += 1;
            Some(instruction)
        } else {
            None
        }
    }

    fn fetch_data(&mut self, addr: usize) -> Option<i32> {
        if addr < self.data_memory.len() {
            Some(self.data_memory[addr])
        } else {
            None
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Read => {
                if let Some(val) = self.stack.pop() {;
                self.stack.push(if let Some(i) = fetch_data(val) {
                    i
                    }else{
                        0
                    }
                    );
                }
            }
            Instruction::Write => {
                if let Some(val) = self.stack.pop() {;
                self.stack.push(if let Some(i) = fetch_data(val) {
                    i
                    }else{
                        0
                    }
                    );
                }
                
            }
            Instruction::Add => {
                if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
                    self.push(a + b);
                }
            }
            Instruction::Sub=> {
                if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
                    self.push(a - b);
                }
            }
            Instruction::Read => {}

            Instruction::JumpIfZero(addr) => {
                if let Some(top) = self.pop() {
                    if top == 0 {
                        self.pc = *addr;
                    }
                }
            } // 他の命令の実装を追加
        }
    }

    fn run(&mut self) {
        loop {
            let instruction = match self.fetch_program() {
                Some(instruction) => instruction,
                None => break,
            };
            self.execute(instruction);
        }
    }
}

fn main() {
    let mut vm = VirtualMachine {
        stack: Vec::new(),
        pc: 0,
        program_memory: vec![
            Instruction::Push(5),
            Instruction::Push(3),
            Instruction::Add,
            Instruction::Subtract,
        ],
        data_memory: vec![0; 100],
    };

    vm.run();

    if let Some(result) = vm.pop() {
        println!("Result: {}", result);
    }
}
