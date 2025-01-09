pub mod cpu;
pub mod instruction;
pub mod parser;
pub mod register;

pub type Code = Vec<Instr>;

pub const MAX_STACK_SIZE: usize = 2000;

pub use chumsky::Parser;
pub use cpu::Cpu;
pub use instruction::Instr;
pub use parser::parse;
