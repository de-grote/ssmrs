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
    AND,
    OR,
    XOR,
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
    ANNOTE(Reg, i32, i32, Color, String),
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
            Instr::AND => {
                vec![0x29]
            }
            Instr::OR => {
                vec![0x2A]
            }
            Instr::XOR => {
                vec![0x2B]
            }
            Instr::LABEL(_) => {
                panic!("LABEL should never be executed!")
            }
            Instr::ANNOTE(_, _, _, _, _) => {
                panic!("ANNOTE should never be executed!")
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
            Self::ANNOTE(_, _, _, _, _) => 0,
            _ => self.convert().len(),
        }
    }

    pub fn name_and_params(&self) -> Vec<String> {
        match self {
            Self::STR(n) => vec![String::from("STR"), n.to_string()],
            Self::STL(n) => vec![String::from("STL"), n.to_string()],
            Self::STS(n) => vec![String::from("STS"), n.to_string()],
            Self::STA(n) => vec![String::from("STA"), n.to_string()],
            Self::LDR(n) => vec![String::from("LDR"), n.to_string()],
            Self::LDL(n) => vec![String::from("LDL"), n.to_string()],
            Self::LDS(n) => vec![String::from("LDS"), n.to_string()],
            Self::LDA(n) => vec![String::from("LDA"), n.to_string()],
            Self::LDC(n) => vec![String::from("LDC"), n.to_string()],
            Self::LDLA(n) => vec![String::from("LDLA"), n.to_string()],
            Self::LDSA(n) => vec![String::from("LDSA"), n.to_string()],
            Self::LDAA(n) => vec![String::from("LDAA"), n.to_string()],
            Self::BRA(n) => vec![String::from("BRA"), n.to_string()],
            Self::BRF(n) => vec![String::from("BRF"), n.to_string()],
            Self::BRT(n) => vec![String::from("BRT"), n.to_string()],
            Self::BSR(n) => vec![String::from("BSR"), n.to_string()],
            Self::LINK(n) => vec![String::from("LINK"), n.to_string()],
            Self::AJS(n) => vec![String::from("AJS"), n.to_string()],
            Self::SWPR(n) => vec![String::from("SWPR"), n.to_string()],
            Self::SWPRR(n, m) => vec![String::from("SWPRR"), n.to_string(), m.to_string()],
            Self::LDRR(n, m) => vec![String::from("LDRR"), n.to_string(), m.to_string()],
            Self::TRAP(n) => vec![String::from("TRAP"), n.to_string()],
            Self::LABEL(n) => vec![String::from("LABEL"), n.to_string()],
            Self::Bra(n) => vec![String::from("Bra"), n.to_string()],
            Self::Brf(n) => vec![String::from("Brf"), n.to_string()],
            Self::Brt(n) => vec![String::from("Brt"), n.to_string()],
            Self::Bsr(n) => vec![String::from("Bsr"), n.to_string()],
            Self::ANNOTE(n, m, o, p, q) => vec![
                String::from("ANNOTE"),
                n.to_string(),
                m.to_string(),
                o.to_string(),
                p.to_string(),
                q.to_string(),
            ],
            x => vec![x.to_string().trim().to_string()],
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
            0x29 => Instr::AND,
            0x2A => Instr::OR,
            0x2B => Instr::XOR,
            _ => panic!("Invalid instruction!"),
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::STR(r) => write!(f, "STR {}", r),
            Instr::STL(n) => write!(f, "STL {}", n),
            Instr::STS(n) => write!(f, "STS {}", n),
            Instr::STA(n) => write!(f, "STA {}", n),
            Instr::LDR(r) => write!(f, "LDR {}", r),
            Instr::LDL(n) => write!(f, "LDL {}", n),
            Instr::LDS(n) => write!(f, "LDS {}", n),
            Instr::LDA(n) => write!(f, "LDA {}", n),
            Instr::LDC(n) => write!(f, "LDC {}", n),
            Instr::LDLA(n) => write!(f, "LDLA {}", n),
            Instr::LDSA(n) => write!(f, "LDSA {}", n),
            Instr::LDAA(n) => write!(f, "LDAA {}", n),
            Instr::BRA(n) => write!(f, "BRA {}", n),
            Instr::Bra(n) => write!(f, "BRA {}", n),
            Instr::BRF(n) => write!(f, "BRF {}", n),
            Instr::Brf(n) => write!(f, "BRF {}", n),
            Instr::BRT(n) => write!(f, "BRT {}", n),
            Instr::Brt(n) => write!(f, "BRT {}", n),
            Instr::BSR(n) => write!(f, "BSR {}", n),
            Instr::Bsr(n) => write!(f, "BSR {}", n),
            Instr::ADD => write!(f, "ADD"),
            Instr::SUB => write!(f, "SUB"),
            Instr::MUL => write!(f, "MUL"),
            Instr::DIV => write!(f, "DIV"),
            Instr::MOD => write!(f, "MOD"),
            Instr::EQ => write!(f, "EQ"),
            Instr::NE => write!(f, "NE"),
            Instr::LT => write!(f, "LT"),
            Instr::LE => write!(f, "LE"),
            Instr::GT => write!(f, "GT"),
            Instr::GE => write!(f, "GE"),
            Instr::NEG => write!(f, "NEG"),
            Instr::NOT => write!(f, "NOT"),
            Instr::RET => write!(f, "RET"),
            Instr::UNLINK => write!(f, "UNLINK"),
            Instr::LINK(n) => write!(f, "LINK {}", n),
            Instr::AJS(n) => write!(f, "AJS {}", n),
            Instr::SWP => write!(f, "SWP"),
            Instr::SWPR(r) => write!(f, "SWPR {}", r),
            Instr::SWPRR(r1, r2) => write!(f, "SWPRR {}, {}", r1, r2),
            Instr::LDRR(r1, r2) => write!(f, "LDRR {}, {}", r1, r2),
            Instr::JSR => write!(f, "JSR"),
            Instr::TRAP(n) => write!(f, "TRAP {}", n),
            Instr::NOP => write!(f, "NOP"),
            Instr::HALT => write!(f, "HALT"),
            Instr::AND => write!(f, "AND"),
            Instr::OR => write!(f, "OR"),
            Instr::XOR => write!(f, "XOR"),
            Instr::LABEL(n) => write!(f, "{}:", n),
            Instr::ANNOTE(a, b, c, d, e) => write!(f, "ANNOTE {} {} {} {} {}", a, b, c, d, e),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    DarkGray,
    Gray,
    Green,
    LightGray,
    Magenta,
    Orange,
    Pink,
    Red,
    Yellow,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "black"),
            Color::Blue => write!(f, "blue"),
            Color::Cyan => write!(f, "cyan"),
            Color::DarkGray => write!(f, "darkGray"),
            Color::Gray => write!(f, "gray"),
            Color::Green => write!(f, "green"),
            Color::LightGray => write!(f, "lightGray"),
            Color::Magenta => write!(f, "magenta"),
            Color::Orange => write!(f, "orange"),
            Color::Pink => write!(f, "pink"),
            Color::Red => write!(f, "red"),
            Color::Yellow => write!(f, "yellow"),
        }
    }
}
