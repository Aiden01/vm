mod instruction;
mod stack;
mod value;
mod vm;

use instruction::Binary;
use instruction::Instr::*;
use value::Value::*;
use vm::Vm;

fn main() {
    let mut vm = Vm::new();
    let instrs = vec![
        LoadConst(Int(2)),
        LoadConst(Int(3)),
        Binary(Binary::Add),
        Print,
    ];

    let result = vm.run(instrs);
    if let Err(e) = result {
        println!("An error occurred: {:?}", e);
    }
}
