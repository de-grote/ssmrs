use std::fmt::Display;

use crate::register::Reg;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instr {
    STR(Reg),
    STL(i32),
    STS(i32),
    STA(i32),
    LDR(Reg),
    LDL(i32),
    LDS(i32),
    LDA(i32),
    LDC(i32),
    LDLA(i32),
    LDSA(i32),
    LDAA(i32),
    BRA(i32),
    Bra(String),
    BRF(i32),
    Brf(String),
    BRT(i32),
    Brt(String),
    BSR(i32),
    Bsr(String),
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
    NEG,
    NOT,
    RET,
    UNLINK,
    LINK(i32),
    AJS(i32),
    SWP,
    SWPR(Reg),
    SWPRR(Reg, Reg),
    LDRR(Reg, Reg),
    JSR,
    TRAP(i32),
    NOP,
    HALT,
    LABEL(String),
}

impl Instr {
    pub fn convert(&self) -> Vec<i32> {
        match self {
            Instr::STR(n) => {
                vec![0x00, *n as i32]
            }
            Instr::STL(n) => {
                vec![0x01, *n]
            }
            Instr::STS(n) => {
                vec![0x02, *n]
            }
            Instr::STA(n) => {
                vec![0x03, *n]
            }
            Instr::LDR(n) => {
                vec![0x04, *n as i32]
            }
            Instr::LDL(n) => {
                vec![0x05, *n]
            }
            Instr::LDS(n) => {
                vec![0x06, *n]
            }
            Instr::LDA(n) => {
                vec![0x07, *n]
            }
            Instr::LDC(n) => {
                vec![0x08, *n]
            }
            Instr::LDLA(n) => {
                vec![0x09, *n]
            }
            Instr::LDSA(n) => {
                vec![0x0A, *n]
            }
            Instr::LDAA(n) => {
                vec![0x0B, *n]
            }
            Instr::BRA(n) => {
                vec![0x0C, *n]
            }
            Instr::Bra(_) => {
                panic!("Bra should never be executed!")
            }
            Instr::BRF(n) => {
                vec![0x0D, *n]
            }
            Instr::Brf(_) => {
                panic!("Brf should never be executed!")
            }
            Instr::BRT(n) => {
                vec![0x0E, *n]
            }
            Instr::Brt(_) => {
                panic!("Brt should never be executed!")
            }
            Instr::BSR(n) => {
                vec![0x0F, *n]
            }
            Instr::Bsr(_) => {
                panic!("Bsr should never be executed!")
            }
            Instr::ADD => {
                vec![0x10]
            }
            Instr::SUB => {
                vec![0x11]
            }
            Instr::MUL => {
                vec![0x12]
            }
            Instr::DIV => {
                vec![0x13]
            }
            Instr::MOD => {
                vec![0x14]
            }
            Instr::EQ => {
                vec![0x15]
            }
            Instr::NE => {
                vec![0x16]
            }
            Instr::LT => {
                vec![0x17]
            }
            Instr::LE => {
                vec![0x18]
            }
            Instr::GT => {
                vec![0x19]
            }
            Instr::GE => {
                vec![0x1A]
            }
            Instr::NEG => {
                vec![0x1B]
            }
            Instr::NOT => {
                vec![0x1C]
            }
            Instr::RET => {
                vec![0x1D]
            }
            Instr::UNLINK => {
                vec![0x1E]
            }
            Instr::LINK(n) => {
                vec![0x1F, *n]
            }
            Instr::AJS(n) => {
                vec![0x20, *n]
            }
            Instr::SWP => {
                vec![0x21]
            }
            Instr::SWPR(n) => {
                vec![0x22, *n as i32]
            }
            Instr::SWPRR(n, m) => {
                vec![0x23, *n as i32, *m as i32]
            }
            Instr::LDRR(n, m) => {
                vec![0x24, *n as i32, *m as i32]
            }
            Instr::JSR => {
                vec![0x25]
            }
            Instr::TRAP(n) => {
                vec![0x26, *n]
            }
            Instr::NOP => {
                vec![0x27]
            }
            Instr::HALT => {
                vec![0x28]
            }
            Instr::LABEL(_) => {
                panic!("LABEL should never be executed!")
            }
        }
    }

    pub fn instr_size(&self) -> usize {
        match self {
            Self::LABEL(_) => 0,
            Self::Bra(_) => 2,
            Self::Brf(_) => 2,
            Self::Brt(_) => 2,
            Self::Bsr(_) => 2,
            _ => self.convert().len(),
        }
    }
}

impl From<&[i32]> for Instr {
    fn from(v: &[i32]) -> Self {
        match v[0] {
            0x00 => Instr::STR(Reg::try_from(v[1]).unwrap()),
            0x01 => Instr::STL(v[1]),
            0x02 => Instr::STS(v[1]),
            0x03 => Instr::STA(v[1]),
            0x04 => Instr::LDR(Reg::try_from(v[1]).unwrap()),
            0x05 => Instr::LDL(v[1]),
            0x06 => Instr::LDS(v[1]),
            0x07 => Instr::LDA(v[1]),
            0x08 => Instr::LDC(v[1]),
            0x09 => Instr::LDLA(v[1]),
            0x0A => Instr::LDSA(v[1]),
            0x0B => Instr::LDAA(v[1]),
            0x0C => Instr::BRA(v[1]),
            0x0D => Instr::BRF(v[1]),
            0x0E => Instr::BRT(v[1]),
            0x0F => Instr::BSR(v[1]),
            0x10 => Instr::ADD,
            0x11 => Instr::SUB,
            0x12 => Instr::MUL,
            0x13 => Instr::DIV,
            0x14 => Instr::MOD,
            0x15 => Instr::EQ,
            0x16 => Instr::NE,
            0x17 => Instr::LT,
            0x18 => Instr::LE,
            0x19 => Instr::GT,
            0x1A => Instr::GE,
            0x1B => Instr::NEG,
            0x1C => Instr::NOT,
            0x1D => Instr::RET,
            0x1E => Instr::UNLINK,
            0x1F => Instr::LINK(v[1]),
            0x20 => Instr::AJS(v[1]),
            0x21 => Instr::SWP,
            0x22 => Instr::SWPR(Reg::try_from(v[1]).unwrap()),
            0x23 => Instr::SWPRR(Reg::try_from(v[1]).unwrap(), Reg::try_from(v[2]).unwrap()),
            0x24 => Instr::LDRR(Reg::try_from(v[1]).unwrap(), Reg::try_from(v[2]).unwrap()),
            0x25 => Instr::JSR,
            0x26 => Instr::TRAP(v[1]),
            0x27 => Instr::NOP,
            0x28 => Instr::HALT,
            _ => panic!("Invalid instruction!"),
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::STR(r) => write!(f, "\tSTR {}", r),
            Instr::STL(n) => write!(f, "\tSTL {}", n),
            Instr::STS(n) => write!(f, "\tSTS {}", n),
            Instr::STA(n) => write!(f, "\tSTA {}", n),
            Instr::LDR(r) => write!(f, "\tLDR {}", r),
            Instr::LDL(n) => write!(f, "\tLDL {}", n),
            Instr::LDS(n) => write!(f, "\tLDS {}", n),
            Instr::LDA(n) => write!(f, "\tLDA {}", n),
            Instr::LDC(n) => write!(f, "\tLDC {}", n),
            Instr::LDLA(n) => write!(f, "\tLDLA {}", n),
            Instr::LDSA(n) => write!(f, "\tLDSA {}", n),
            Instr::LDAA(n) => write!(f, "\tLDAA {}", n),
            Instr::BRA(n) => write!(f, "\tBRA {}", n),
            Instr::Bra(n) => write!(f, "\tBRA {}", n),
            Instr::BRF(n) => write!(f, "\tBRF {}", n),
            Instr::Brf(n) => write!(f, "\tBRF {}", n),
            Instr::BRT(n) => write!(f, "\tBRT {}", n),
            Instr::Brt(n) => write!(f, "\tBRT {}", n),
            Instr::BSR(n) => write!(f, "\tBSR {}", n),
            Instr::Bsr(n) => write!(f, "\tBSR {}", n),
            Instr::ADD => write!(f, "\tADD"),
            Instr::SUB => write!(f, "\tSUB"),
            Instr::MUL => write!(f, "\tMUL"),
            Instr::DIV => write!(f, "\tDIV"),
            Instr::MOD => write!(f, "\tMOD"),
            Instr::EQ => write!(f, "\tEQ"),
            Instr::NE => write!(f, "\tNE"),
            Instr::LT => write!(f, "\tLT"),
            Instr::LE => write!(f, "\tLE"),
            Instr::GT => write!(f, "\tGT"),
            Instr::GE => write!(f, "\tGE"),
            Instr::NEG => write!(f, "\tNEG"),
            Instr::NOT => write!(f, "\tNOT"),
            Instr::RET => write!(f, "\tRET"),
            Instr::UNLINK => write!(f, "\tUNLINK"),
            Instr::LINK(n) => write!(f, "\tLINK {}", n),
            Instr::AJS(n) => write!(f, "\tAJS {}", n),
            Instr::SWP => write!(f, "\tSWP"),
            Instr::SWPR(r) => write!(f, "\tSWPR {}", r),
            Instr::SWPRR(r1, r2) => write!(f, "\tSWPRR {}, {}", r1, r2),
            Instr::LDRR(r1, r2) => write!(f, "\tLDRR {}, {}", r1, r2),
            Instr::JSR => write!(f, "\tJSR"),
            Instr::TRAP(n) => write!(f, "\tTRAP {}", n),
            Instr::NOP => write!(f, "\tNOP"),
            Instr::HALT => write!(f, "\tHALT"),
            Instr::LABEL(n) => write!(f, "{}:", n),
        }
    }
}
