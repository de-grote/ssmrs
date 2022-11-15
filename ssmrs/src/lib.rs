pub mod cpu;
pub mod instruction;
pub mod parser;
pub mod register;

pub type Code = Vec<Instr>;

pub use cpu::Cpu;
pub use instruction::Instr;
pub use parser::parse;
