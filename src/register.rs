use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RegisterFile {
    pub pc: i32,
    pub sp: i32,
    pub mp: i32,
    pub r3: i32,
    pub r4: i32,
    pub r5: i32,
    pub r6: i32,
    pub r7: i32,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            mp: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
        }
    }
}

impl Index<Reg> for RegisterFile {
    type Output = i32;

    fn index(&self, index: Reg) -> &Self::Output {
        match index {
            Reg::PC => &self.pc,
            Reg::SP => &self.sp,
            Reg::MP => &self.mp,
            Reg::R3 => &self.r3,
            Reg::R4 => &self.r4,
            Reg::R5 => &self.r5,
            Reg::R6 => &self.r6,
            Reg::R7 => &self.r7,
        }
    }
}

impl IndexMut<Reg> for RegisterFile {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        match index {
            Reg::PC => &mut self.pc,
            Reg::SP => &mut self.sp,
            Reg::MP => &mut self.mp,
            Reg::R3 => &mut self.r3,
            Reg::R4 => &mut self.r4,
            Reg::R5 => &mut self.r5,
            Reg::R6 => &mut self.r6,
            Reg::R7 => &mut self.r7,
        }
    }
}

impl Index<i32> for RegisterFile {
    type Output = i32;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.pc,
            1 => &self.sp,
            2 => &self.mp,
            3 => &self.r3,
            4 => &self.r4,
            5 => &self.r5,
            6 => &self.r6,
            7 => &self.r7,
            _ => panic!("Invalid register index"),
        }
    }
}

impl IndexMut<i32> for RegisterFile {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        match index {
            0 => &mut self.pc,
            1 => &mut self.sp,
            2 => &mut self.mp,
            3 => &mut self.r3,
            4 => &mut self.r4,
            5 => &mut self.r5,
            6 => &mut self.r6,
            7 => &mut self.r7,
            _ => panic!("Invalid register index"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Reg {
    PC,
    SP,
    MP,
    R3,
    R4,
    R5,
    R6,
    R7,
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::PC => write!(f, "PC"),
            Reg::SP => write!(f, "SP"),
            Reg::MP => write!(f, "MP"),
            Reg::R3 => write!(f, "R3"),
            Reg::R4 => write!(f, "R4"),
            Reg::R5 => write!(f, "R5"),
            Reg::R6 => write!(f, "R6"),
            Reg::R7 => write!(f, "R7"),
        }
    }
}

impl TryFrom<i32> for Reg {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Reg::PC),
            1 => Ok(Reg::SP),
            2 => Ok(Reg::MP),
            3 => Ok(Reg::R3),
            4 => Ok(Reg::R4),
            5 => Ok(Reg::R5),
            6 => Ok(Reg::R6),
            7 => Ok(Reg::R7),
            _ => Err("Invalid register"),
        }
    }
}
