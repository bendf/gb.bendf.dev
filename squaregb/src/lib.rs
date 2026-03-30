use arbitrary_int::{u2, u3, u5};
use wasm_bindgen::prelude::*;
use web_sys;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum R16 {
    BC,
    DE,
    HL,
    AF,
    SP,
}

impl R16 {
    pub fn new(id: u2) -> Self {
        match id.value() {
            0b00 => R16::BC,
            0b01 => R16::DE,
            0b10 => R16::HL,
            0b11 => R16::SP,
            x => panic!("Unknown R16 id {:#b}", x),
        }
    }
    pub fn stk(id: u2) -> Self {
        match id.value() {
            0b00 => R16::BC,
            0b01 => R16::DE,
            0b10 => R16::HL,
            0b11 => R16::AF,
            x => panic!("Unknown R16 id {:#b}", x),
        }
    }
}

impl Into<u8> for R16 {
    fn into(self) -> u8 {
        match self {
            R16::BC => 0b00,
            R16::DE => 0b01,
            R16::HL => 0b10,
            R16::AF => 0b11,
            R16::SP => 0b11,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum R8 {
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    F = 0b110,
    A = 0b111,
}

impl R8 {
    pub fn new(id: u3) -> Self {
        match id.value() {
            0b000 => B,
            0b001 => C,
            0b010 => D,
            0b011 => E,
            0b100 => H,
            0b101 => L,
            0b110 => F,
            0b111 => A,
            // NB In practice, this is impossible. We have matched on the 8 possible u3 values
            // already.
            x => panic!("Invalid Reg id {:#b}", x),
        }
    }
}

impl Into<u8> for R8 {
    fn into(self) -> u8 {
        self as u8
    }
}

use R8::*;

// pub const A: Reg = 0b111;
// const B: Reg = 0b000;
// const C: Reg = 0b001;
// const D: Reg = 0b010;
// const E: Reg = 0b011;
// const H: Reg = 0b100;
// const L: Reg = 0b101;
// // F is flags register, so its a bit weird.
// const F: Reg = 0b110;

// Encoded conditions

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Cond {
    NZ = 0b00,
    NCARRY = 0b10,
    Z = 0b01,
    CARRY = 0b11,
}

impl Cond {
    pub fn new(id: u2) -> Self {
        match id.value() {
            0b00 => NZ,
            0b01 => Z,
            0b10 => NCARRY,
            0b11 => CARRY,
            // NB In practice, this is impossible. We have matched on the 8 possible u2 values
            // already.
            x => panic!("Invalid Cond id {:#b}", x),
        }
    }
}

impl Into<u8> for Cond {
    fn into(self) -> u8 {
        self as u8
    }
}

use Cond::*;

// type Cond = u8;
// const NZ: Cond = 0b00;
// const NCARRY: Cond = 0b10;
// const Z: Cond = 0b01;
// const CARRY: Cond = 0b11;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("No global 'window' object");
    let document = window.document().expect("No 'document' object on window");
    let _body = document.body().expect("No 'body' object on document");

    let _root = document
        .get_element_by_id("squaregb-root")
        .expect("No 'squaregb-root' element");

    Ok(())
}

#[wasm_bindgen]
pub fn say_hello() -> String {
    return String::from("Hello, World!");
}

pub struct Machine {
    pc: u16,
    gp_registers: [u8; 8],
    sp: u16,
    memory: [u8; 65536],
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            pc: 0,
            gp_registers: [0; 8],
            sp: 0,
            memory: [0; 65536],
        }
    }

    pub fn set_memory(&mut self, start: usize, mem: &[u8]) {
        let end = start + mem.len();
        self.memory[start..end].copy_from_slice(mem);
    }
    pub fn set_pc(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn eval(&self, _steps: usize) {}

    pub fn get_r8(&self, reg: R8) -> u8 {
        self.gp_registers[reg as usize]
    }

    pub fn get_r16(&self, reg_pair: R16) -> u16 {
        match reg_pair {
            R16::BC => {
                u16::from_be_bytes([self.gp_registers[B as usize], self.gp_registers[C as usize]])
            }
            R16::DE => {
                u16::from_be_bytes([self.gp_registers[D as usize], self.gp_registers[E as usize]])
            }
            R16::HL => {
                u16::from_be_bytes([self.gp_registers[H as usize], self.gp_registers[L as usize]])
            }
            R16::SP => self.sp,
            R16::AF => {
                u16::from_be_bytes([self.gp_registers[A as usize], self.gp_registers[F as usize]])
            }
        }
    }

    pub fn get_mem8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn set_r8(&mut self, reg: R8, value: u8) {
        self.gp_registers[reg as usize] = value;
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value
    }

    pub fn set_r16(&mut self, reg_pair: R16, value: u16) {
        let [low, high] = value.to_le_bytes();
        match reg_pair {
            R16::BC => {
                self.set_r8(B, high);
                self.set_r8(C, low);
            }
            R16::DE => {
                self.set_r8(D, high);
                self.set_r8(E, low);
            }
            R16::HL => {
                self.set_r8(H, high);
                self.set_r8(L, low);
            }
            R16::AF => {
                self.set_r8(A, high);
                self.set_r8(F, low);
            }
            R16::SP => self.set_sp(value),
        }
    }

    pub fn get_pc(&self) -> u16 {
        return self.pc;
    }

    pub fn inc_pc(&mut self) {
        self.pc = self.pc + 1
    }

    pub fn exec(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            LoadReg8 { src, dest } => {
                let value = self.get_r8(src);
                self.set_r8(dest, value);
                self.inc_pc();
            }
            LoadImm8 { dest, imm } => {
                self.set_r8(dest, imm);
                self.inc_pc();
            }
            LoadIndirectHL { dest } => {
                let addr = self.get_r16(R16::HL);
                let value = self.get_mem8(addr);
                self.set_r8(dest, value);
                self.inc_pc();
            }
            _ => todo!("Missing instruction exec"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    LoadReg8 { src: R8, dest: R8 },
    LoadImm8 { dest: R8, imm: u8 },
    LoadIndirectHL { dest: R8 },
    StoreIndirectHL { src: R8 },
    StoreImmIndirectHL { imm: u8 },
    LoadAccIndirectBC,
    LoadAccIndirectDE,
    StoreAccIndirectBC,
    StoreAccIndirectDE,
    LoadAcc16 { addr: u16 },
    StoreAcc16 { addr: u16 },
    LoadAccIndirectC,
    StoreAccIndirectC,
    LoadAccDirect8 { offset: u8 },
    StoreAccDirect8 { offset: u8 },
    LoadAccIndirectHLDec,
    StoreAccIndirectHLDec,
    LoadAccIndirectHLInc,
    StoreAccIndirectHLInc,
    LoadImm16 { dest: R16, imm: u16 },
    StoreSP16 { addr: u16 },
    LoadSPHL,
    Push { src: R16 },
    Pop { dest: R16 },
    LoadHLSPOffset { offset: i8 },
    Add8 { src: R8 },
    Add8IndirectHL,
    AddImm8 { imm: u8 },
    AddC8 { src: R8 },
    AddC8IndirectHL,
    AddCImm8 { imm: u8 },
    Sub8 { src: R8 },
    Sub8IndirectHL,
    SubImm8 { imm: u8 },
    SubC8 { src: R8 },
    SubC8IndirectHL,
    SubCImm8 { imm: u8 },
    Cmp8 { src: R8 },
    Cmp8IndirectHL,
    CmpImm8 { imm: u8 },
    Inc8 { src: R8 },
    Inc8IndirectHL,
    Dec8 { src: R8 },
    Dec8IndirectHL,
    And8 { src: R8 },
    And8IndirectHL,
    AndImm8 { imm: u8 },
    Or8 { src: R8 },
    Or8IndirectHL,
    OrImm8 { imm: u8 },
    Xor8 { src: R8 },
    Xor8IndirectHL,
    XorImm8 { imm: u8 },
    CmplCarryFlag,
    SetCarryFlag,
    DecAdjAcc,
    CmplAcc,
    Inc16 { src: R16 },
    Dec16 { src: R16 },
    Add16HL { src: R16 },
    AddSPImm8 { imm: i8 },
    RLCA,
    RRCA,
    RLA,
    RRA,
    RLC { src: R8 },
    RLCIndirectHL,
    RRC { src: R8 },
    RRCIndirectHL,
    RL { src: R8 },
    RLIndirectHL,
    RR { src: R8 },
    RRIndirectHL,
    SLA { src: R8 },
    SLAIndirectHL,
    SRA { src: R8 },
    SRAIndirectHL,
    Swap { src: R8 },
    SwapIndirectHL,
    SRL { src: R8 },
    SRLIndirectHL,
    BIT { src: R8, bit: u8 },
    BITIndirectHL { bit: u8 },
    RES { src: R8, bit: u8 },
    RESIndirectHL { bit: u8 },
    SET { src: R8, bit: u8 },
    SETIndirectHL { bit: u8 },
    JP { addr: u16 },
    JPHL,
    JPCC { cond: Cond, addr: u16 },
    JR { offset: i8 },
    JRCC { cond: Cond, offset: i8 },
    Call { addr: u16 },
    CallCC { cond: Cond, addr: u16 },
    Ret,
    RetCC { cond: Cond },
    RetI,
    RST { addr: u8 },
    Halt,
    Stop { ignore: u8 },
    DI,
    EI,
    NOP,
}

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    UnknownOpcode,
    MemoryOutOfBounds,
}

impl Instruction {
    pub fn decode(memory: &[u8]) -> Result<Instruction, DecodeError> {
        let Some(first_byte) = memory.first() else {
            return Err(DecodeError::MemoryOutOfBounds);
        };

        let b7_6 = u2::extract_u8(*first_byte, 6);
        let b7_5 = u3::extract_u8(*first_byte, 5);
        let b2_0 = u3::extract_u8(*first_byte, 0);
        let b7_3 = u5::extract_u8(*first_byte, 3);
        let b5_3 = u3::extract_u8(*first_byte, 3);
        let b5_4 = u2::extract_u8(*first_byte, 4);
        let b4_3 = u2::extract_u8(*first_byte, 3);

        if b7_6.value() == 0b11 && b2_0.value() == 0b111 {
            // RST
            return Ok(Instruction::RST {
                addr: b5_3.value() << 3,
            });
        }

        if b7_5.value() == 0b110 && b2_0.value() == 0b000 {
            // RetCC
            return Ok(Instruction::RetCC {
                cond: Cond::new(b4_3),
            });
        }

        if b7_5.value() == 0b110 && b2_0.value() == 0b100 {
            // CallCC
            let Some(addr) = memory.get(1..3) else {
                return Err(DecodeError::MemoryOutOfBounds);
            };

            // Unwrap safe here as we've guaranteed the range above.
            let addr: u16 = u16::from_le_bytes(addr.try_into().unwrap());
            return Ok(Instruction::CallCC {
                cond: Cond::new(b4_3),
                addr,
            });
        }

        if b7_5.value() == 0b001 && b2_0.value() == 0b000 {
            //JRCC
            let Some(offset) = memory.get(1) else {
                return Err(DecodeError::MemoryOutOfBounds);
            };

            return Ok(Instruction::JRCC {
                cond: Cond::new(b4_3),
                offset: offset.cast_signed(),
            });
        }

        if b7_5.value() == 0b110 && b2_0.value() == 0b10 {
            // JPCC
            let Some(addr) = memory.get(1..3) else {
                return Err(DecodeError::MemoryOutOfBounds);
            };

            // Unwrap safe here as we've guaranteed the range above.
            let addr: u16 = u16::from_le_bytes(addr.try_into().unwrap());
            return Ok(Instruction::JPCC {
                cond: Cond::new(b4_3),
                addr,
            });
        }

        match (first_byte, b7_3.value(), b7_6.value(), b2_0.value()) {
            (0b0000_0000, _, _, _) => Ok(Instruction::NOP),
            (0b1111_1011, _, _, _) => Ok(Instruction::EI),
            (0b1111_0011, _, _, _) => Ok(Instruction::DI),
            (0b0001_0000, _, _, _) => {
                let Some(ignore) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::Stop { ignore: *ignore })
            }
            (0b0111_0110, _, _, _) => Ok(Instruction::Halt),
            (0b1101_1001, _, _, _) => Ok(Instruction::RetI),
            (0b1100_1001, _, _, _) => Ok(Instruction::Ret),
            (0b1100_1101, _, _, _) => {
                let Some(addr) = memory.get(1..3) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                // Unwrap safe here as we've guaranteed the range above.
                let addr: u16 = u16::from_le_bytes(addr.try_into().unwrap());
                return Ok(Instruction::Call { addr });
            }

            (0b0001_1000, _, _, _) => {
                let Some(offset) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::JR {
                    offset: offset.cast_signed(),
                })
            }
            (0b1110_1001, _, _, _) => Ok(Instruction::JPHL),
            (0xCB, _, _, _) => {
                let Some(prefixed_opcode) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };
                let b7_6: u8 = u2::extract_u8(*prefixed_opcode, 6).value();
                let b2_0 = u3::extract_u8(*prefixed_opcode, 0);
                let b7_3: u8 = u5::extract_u8(*prefixed_opcode, 3).value();
                // let b5_3: u8 = u3::extract_u8(*prefixed_opcode, 3).value();
                // let b5_4: u8 = u2::extract_u8(*prefixed_opcode, 4).value();
                let b6_4: u8 = u3::extract_u8(*prefixed_opcode, 3).value();

                match b7_6 {
                    0b11 => {
                        if b2_0.value() == 0b110 {
                            return Ok(Instruction::SETIndirectHL { bit: b6_4 });
                        } else {
                            return Ok(Instruction::SET {
                                src: R8::new(b2_0),
                                bit: b6_4,
                            });
                        }
                    }
                    0b10 => {
                        if b2_0.value() == 0b110 {
                            return Ok(Instruction::RESIndirectHL { bit: b6_4 });
                        } else {
                            return Ok(Instruction::RES {
                                src: R8::new(b2_0),
                                bit: b6_4,
                            });
                        }
                    }
                    0b01 => {
                        if b2_0.value() == 0b110 {
                            return Ok(Instruction::BITIndirectHL { bit: b6_4 });
                        } else {
                            return Ok(Instruction::BIT {
                                src: R8::new(b2_0),
                                bit: b6_4,
                            });
                        }
                    }
                    _ => {
                        // Continue to next match
                    }
                }

                match (prefixed_opcode, b7_3, R8::new(b2_0)) {
                    (0b0011_1110, _, _) => Ok(Instruction::SRLIndirectHL),
                    (_, 0b0011_1, B | C | D | E | H | L | A) => {
                        Ok(Instruction::SRL { src: R8::new(b2_0) })
                    }
                    (0b0011_0110, _, _) => Ok(Instruction::SwapIndirectHL),
                    (_, 0b0011_0, B | C | D | E | H | L | A) => {
                        Ok(Instruction::Swap { src: R8::new(b2_0) })
                    }
                    (0b0010_1110, _, _) => Ok(Instruction::SRAIndirectHL),
                    (_, 0b0010_1, B | C | D | E | H | L | A) => {
                        Ok(Instruction::SRA { src: R8::new(b2_0) })
                    }
                    (0b0010_0110, _, _) => Ok(Instruction::SLAIndirectHL),
                    (_, 0b0010_0, B | C | D | E | H | L | A) => {
                        Ok(Instruction::SLA { src: R8::new(b2_0) })
                    }
                    (0b0001_1110, _, _) => Ok(Instruction::RRIndirectHL),
                    (_, 0b0001_1, B | C | D | E | H | L | A) => {
                        Ok(Instruction::RR { src: R8::new(b2_0) })
                    }
                    (0b0001_0110, _, _) => Ok(Instruction::RLIndirectHL),
                    (_, 0b0001_0, B | C | D | E | H | L | A) => {
                        Ok(Instruction::RL { src: R8::new(b2_0) })
                    }
                    (0b0000_1110, _, _) => Ok(Instruction::RRCIndirectHL),
                    (_, 0b0000_1, B | C | D | E | H | L | A) => {
                        Ok(Instruction::RRC { src: R8::new(b2_0) })
                    }
                    (0b0000_0110, _, _) => Ok(Instruction::RLCIndirectHL),
                    (_, 0b0000_0, B | C | D | E | H | L | A) => {
                        Ok(Instruction::RLC { src: R8::new(b2_0) })
                    }

                    _ => Err(DecodeError::UnknownOpcode),
                }
            }

            (0b1100_0011, _, _, _) => {
                let Some(addr) = memory.get(1..3) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                // Unwrap safe here as we've guaranteed the range above.
                let addr: u16 = u16::from_le_bytes(addr.try_into().unwrap());
                Ok(Instruction::JP { addr })
            }
            (0b0001_1111, _, _, _) => Ok(Instruction::RRA),
            (0b0001_0111, _, _, _) => Ok(Instruction::RLA),
            (0b0000_1111, _, _, _) => Ok(Instruction::RRCA),
            (0b0000_0111, _, _, _) => Ok(Instruction::RLCA),
            (0b1110_1000, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::AddSPImm8 {
                    imm: imm.cast_signed(),
                })
            }
            (0b00_00_1001 | 0b00_01_1001 | 0b00_10_1001 | 0b00_11_1001, _, _, _) => {
                Ok(Instruction::Add16HL {
                    src: R16::new(b5_4),
                })
            }
            (0b00_00_1011 | 0b00_01_1011 | 0b00_10_1011 | 0b00_11_1011, _, _, _) => {
                Ok(Instruction::Dec16 {
                    src: R16::new(b5_4),
                })
            }
            (0b00_00_0011 | 0b00_01_0011 | 0b00_10_0011 | 0b00_11_0011, _, _, _) => {
                Ok(Instruction::Inc16 {
                    src: R16::new(b5_4),
                })
            }
            (0b0010_1111, _, _, _) => Ok(Instruction::CmplAcc),
            (0b0010_0111, _, _, _) => Ok(Instruction::DecAdjAcc),
            (0b0011_0111, _, _, _) => Ok(Instruction::SetCarryFlag),
            (0b0011_1111, _, _, _) => Ok(Instruction::CmplCarryFlag),
            (0b1110_1110, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::XorImm8 { imm: *imm })
            }
            (0b1010_1110, _, _, _) => Ok(Instruction::Xor8IndirectHL),
            (_, 0b10101, _, _) => Ok(Instruction::Xor8 { src: R8::new(b2_0) }),
            (0b1111_0110, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::OrImm8 { imm: *imm })
            }
            (0b1011_0110, _, _, _) => Ok(Instruction::Or8IndirectHL),
            (_, 0b10110, _, _) => Ok(Instruction::Or8 { src: R8::new(b2_0) }),
            (0b1110_0110, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::AndImm8 { imm: *imm })
            }
            (0b1010_0110, _, _, _) => Ok(Instruction::And8IndirectHL),
            (_, 0b10100, _, _) => Ok(Instruction::And8 { src: R8::new(b2_0) }),
            (0b0011_0101, _, _, _) => Ok(Instruction::Dec8IndirectHL),
            (_, _, 0b00, 0b101) => Ok(Instruction::Dec8 { src: R8::new(b5_3) }),
            (0b0011_0100, _, _, _) => Ok(Instruction::Inc8IndirectHL),
            (_, _, 0b00, 0b100) => Ok(Instruction::Inc8 { src: R8::new(b5_3) }),
            (0b1111_1110, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::CmpImm8 { imm: *imm })
            }
            (0b1011_1110, _, _, _) => Ok(Instruction::Cmp8IndirectHL),
            (_, 0b10111, _, _) => Ok(Instruction::Cmp8 { src: R8::new(b2_0) }),
            (0b1101_1110, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::SubCImm8 { imm: *imm })
            }
            (0b1001_1110, _, _, _) => Ok(Instruction::SubC8IndirectHL),
            (_, 0b10011, _, _) => Ok(Instruction::SubC8 { src: R8::new(b2_0) }),
            (0b1101_0110, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::SubImm8 { imm: *imm })
            }
            (0b1001_0110, _, _, _) => Ok(Instruction::Sub8IndirectHL),
            (_, 0b10010, _, _) => Ok(Instruction::Sub8 { src: R8::new(b2_0) }),
            (0b1100_1110, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::AddCImm8 { imm: *imm })
            }
            (0b1000_1110, _, _, _) => Ok(Instruction::AddC8IndirectHL),
            (_, 0b10001, _, _) => Ok(Instruction::AddC8 { src: R8::new(b2_0) }),
            (0b1100_0110, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::AddImm8 { imm: *imm })
            }
            (0b1000_0110, _, _, _) => Ok(Instruction::Add8IndirectHL),
            (_, 0b10000, _, _) => Ok(Instruction::Add8 { src: R8::new(b2_0) }),
            (0b1111_1000, _, _, _) => {
                let Some(offset) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::LoadHLSPOffset {
                    offset: offset.cast_signed(),
                })
            }
            (0b11_00_0001 | 0b11_01_0001 | 0b11_10_0001 | 0b11_11_0001, _, _, _) => {
                Ok(Instruction::Pop {
                    dest: R16::stk(b5_4),
                })
            }
            (0b11_00_0101 | 0b11_01_0101 | 0b11_10_0101 | 0b11_11_0101, _, _, _) => {
                Ok(Instruction::Push {
                    src: R16::stk(b5_4),
                })
            }
            (0b1111_1001, _, _, _) => Ok(Instruction::LoadSPHL),
            (0b0000_1000, _, _, _) => {
                let Some(addr) = memory.get(1..3) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                // Unwrap safe here as we've guaranteed the range above.
                let addr: u16 = u16::from_le_bytes(addr.try_into().unwrap());
                Ok(Instruction::StoreSP16 { addr })
            }
            (0b00_00_0001 | 0b00_01_0001 | 0b00_10_0001 | 0b00_11_0001, _, _, _) => {
                let Some(imm) = memory.get(1..3) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                // Unwrap safe here as we've guaranteed the range above.
                let imm: u16 = u16::from_le_bytes(imm.try_into().unwrap());

                Ok(Instruction::LoadImm16 {
                    dest: R16::new(b5_4),
                    imm,
                })
            }
            (0b0010_1010, _, _, _) => Ok(Instruction::LoadAccIndirectHLInc),
            (0b0010_0010, _, _, _) => Ok(Instruction::StoreAccIndirectHLInc),
            (0b0011_1010, _, _, _) => Ok(Instruction::LoadAccIndirectHLDec),
            (0b0011_0010, _, _, _) => Ok(Instruction::StoreAccIndirectHLDec),
            (0b1110_0000, _, _, _) => {
                let Some(offset) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::StoreAccDirect8 { offset: *offset })
            }
            (0b1111_0000, _, _, _) => {
                let Some(offset) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::LoadAccDirect8 { offset: *offset })
            }
            (0b1110_0010, _, _, _) => Ok(Instruction::StoreAccIndirectC),
            (0b1111_0010, _, _, _) => Ok(Instruction::LoadAccIndirectC),
            (0b1110_1010, _, _, _) => {
                let Some(addr) = memory.get(1..3) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                // Unwrap safe here as we've guaranteed the range above.
                let addr: u16 = u16::from_le_bytes(addr.try_into().unwrap());

                Ok(Instruction::StoreAcc16 { addr })
            }
            (0b1111_1010, _, _, _) => {
                let Some(addr) = memory.get(1..3) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                // Unwrap safe here as we've guaranteed the range above.
                let addr: u16 = u16::from_le_bytes(addr.try_into().unwrap());

                Ok(Instruction::LoadAcc16 { addr })
            }
            (0b0001_0010, _, _, _) => Ok(Instruction::StoreAccIndirectDE),
            (0b0000_0010, _, _, _) => Ok(Instruction::StoreAccIndirectBC),
            (0b0001_1010, _, _, _) => Ok(Instruction::LoadAccIndirectDE),
            (0b0000_1010, _, _, _) => Ok(Instruction::LoadAccIndirectBC),
            (0b0011_0110, _, _, _) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::StoreImmIndirectHL { imm: *imm })
            }
            (_, 0b01110, _, _) => Ok(Instruction::StoreIndirectHL { src: R8::new(b2_0) }),
            (_, _, 0b01, 0b110) => Ok(Instruction::LoadIndirectHL {
                dest: R8::new(b5_3),
            }),
            (_, _, 0b01, _) => Ok(Instruction::LoadReg8 {
                src: R8::new(b2_0),
                dest: R8::new(b5_3),
            }),
            (_, _, 0b00, 0b110) => {
                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::LoadImm8 {
                    dest: R8::new(b5_3),
                    imm: *imm,
                })
            }
            _ => Err(DecodeError::UnknownOpcode),
        }
    }
}

#[cfg(test)]
mod decode_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn it_decodes_load_reg_opcode(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[values(A, B, C, D, E, H, L)] dest: R8,
    ) {
        // 0b01_xxx_yyy
        let opcode: u8 =
            0b01_000_000 | (<R8 as Into<u8>>::into(dest) << 3) | <R8 as Into<u8>>::into(src);

        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("LoadReg should decode");

        assert_eq!(decoded, Instruction::LoadReg8 { src, dest });
    }

    #[rstest]
    fn it_decodes_load_imm_opcode(#[values(B, C, D, E, H, L, A)] dest: R8) {
        // 0b00_xxx_110, <1-byte imm>
        let opcode: u8 = 0b00_000_110 | (<R8 as Into<u8>>::into(dest) << 3);
        let memory = [opcode, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("LoadImm should decode");

        assert_eq!(decoded, Instruction::LoadImm8 { dest, imm: 1 });
    }

    #[rstest]
    fn it_decodes_load_indirect_hl_opcode(#[values(B, C, D, E, H, L, A)] dest: R8) {
        // 0b01_xxx_110
        let opcode: u8 = 0b01_000_110 | (<R8 as Into<u8>>::into(dest) << 3);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("LoadIndirectHL should decode");

        assert_eq!(decoded, Instruction::LoadIndirectHL { dest });
    }

    #[rstest]
    fn it_decodes_store_indirect_hl_opcode(#[values(B, C, D, E, H, L, A)] src: R8) {
        // 0b01110_xxx
        let opcode: u8 = 0b01110_000 | <R8 as Into<u8>>::into(src);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("StoreIndirectHL should decode");

        assert_eq!(decoded, Instruction::StoreIndirectHL { src });
    }

    #[test]
    fn it_decodes_store_immediate_indirect_hl_opcode() {
        // 0b0011_0110
        let memory = [0b0011_0110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("StoreImmediateIndirectHL should decode");

        assert_eq!(decoded, Instruction::StoreImmIndirectHL { imm: 1 });
    }

    #[test]
    fn it_decodes_load_acc_indirect_bc_opcode() {
        //0b0000_1010
        let memory = [0b0000_1010];
        let decoded = Instruction::decode(&memory).expect("LoadAccIndirectBC should decode");

        assert_eq!(decoded, Instruction::LoadAccIndirectBC {});
    }

    #[test]
    fn it_decodes_load_acc_indirect_de_opcode() {
        //0b0001_1010
        let memory = [0b0001_1010];
        let decoded = Instruction::decode(&memory).expect("LoadAccIndirectDE should decode");

        assert_eq!(decoded, Instruction::LoadAccIndirectDE {});
    }

    #[test]
    fn it_decodes_store_acc_indirect_bc_opcode() {
        //0b0000_0010
        let memory = [0b0000_0010];
        let decoded = Instruction::decode(&memory).expect("StoreAccIndirectBC should decode");

        assert_eq!(decoded, Instruction::StoreAccIndirectBC {});
    }

    #[test]
    fn it_decodes_store_acc_indirect_de_opcode() {
        //0b0001_0010
        let memory = [0b0001_0010];
        let decoded = Instruction::decode(&memory).expect("StoreAccIndirectDE should decode");

        assert_eq!(decoded, Instruction::StoreAccIndirectDE {});
    }

    #[test]
    fn it_decodes_load_acc() {
        //0b1111_1010
        let memory = [0b1111_1010, 0b0000_1111, 0b1111_0000];
        let decoded = Instruction::decode(&memory).expect("LoadAcc should decode");

        assert_eq!(decoded, Instruction::LoadAcc16 { addr: 0xF00F });
    }

    #[test]
    fn it_decodes_store_acc() {
        //0b1110_1010
        let memory = [0b1110_1010, 0b0000_1111, 0b1111_0000];
        let decoded = Instruction::decode(&memory).expect("LoadAcc should decode");

        assert_eq!(decoded, Instruction::StoreAcc16 { addr: 0xF00F });
    }

    #[test]
    fn it_decodes_load_acc_indirect_c() {
        //0b1111_00010
        let memory = [0b1111_0010];
        let decoded = Instruction::decode(&memory).expect("LoadAccIndirectC should decode");

        assert_eq!(decoded, Instruction::LoadAccIndirectC);
    }

    #[test]
    fn it_decodes_store_acc_indirect_c() {
        //0b1110_0010
        let memory = [0b1110_0010];
        let decoded = Instruction::decode(&memory).expect("StoreAccIndirectC should decode");

        assert_eq!(decoded, Instruction::StoreAccIndirectC);
    }

    #[test]
    fn it_decodes_load_acc_direct_8() {
        //0b1111_0000
        let memory = [0b1111_0000, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("LoadAccDirect8 should decode");

        assert_eq!(decoded, Instruction::LoadAccDirect8 { offset: 1 });
    }

    #[test]
    fn it_decodes_store_acc_direct_8() {
        //0b1110_0000
        let memory = [0b1110_0000, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("StoreAccDirect8 should decode");

        assert_eq!(decoded, Instruction::StoreAccDirect8 { offset: 1 });
    }

    #[test]
    fn it_decodes_load_acc_indirect_hl_dec() {
        //0b0011_1010
        let memory = [0b0011_1010];
        let decoded = Instruction::decode(&memory).expect("LoadAccIndirectHLDec should decode");

        assert_eq!(decoded, Instruction::LoadAccIndirectHLDec);
    }

    #[test]
    fn it_decodes_store_acc_indirect_hl_dec() {
        //0b0011_0010
        let memory = [0b0011_0010];
        let decoded = Instruction::decode(&memory).expect("StoreAccIndirectHLDec should decode");

        assert_eq!(decoded, Instruction::StoreAccIndirectHLDec);
    }

    #[test]
    fn it_decodes_load_acc_indirect_hl_inc() {
        // 0b0010_1010
        let memory = [0b0010_1010];
        let decoded = Instruction::decode(&memory).expect("LoadAccIndirectHLInc should decode");

        assert_eq!(decoded, Instruction::LoadAccIndirectHLInc);
    }

    #[test]
    fn it_decodes_store_acc_indirect_hl_inc() {
        //0b0011_1010
        let memory = [0b0010_0010];
        let decoded = Instruction::decode(&memory).expect("LoadAccIndirectHLInc should decode");

        assert_eq!(decoded, Instruction::StoreAccIndirectHLInc);
    }

    #[rstest]
    fn it_decodes_load_imm_16(#[values(R16::BC, R16::DE, R16::HL, R16::SP)] dest: R16) {
        // 0b00_xx_0001
        let opcode = 0b00_00_0001 + (<R16 as Into<u8>>::into(dest) << 4);
        let memory = [opcode, 0b1111_0000, 0b0000_1111];
        let decoded = Instruction::decode(&memory).expect("LoadImm16 should decode");

        assert_eq!(decoded, Instruction::LoadImm16 { dest, imm: 0x0FF0 });
    }

    #[test]
    fn it_decodes_store_sp_16() {
        // 0b0000_1000
        let memory = [0b0000_1000, 0b1111_0000, 0b0000_1111];
        let decoded = Instruction::decode(&memory).expect("StoreSP16");

        assert_eq!(decoded, Instruction::StoreSP16 { addr: 0x0FF0 });
    }

    #[test]
    fn it_decodes_load_sp_hl() {
        // 0b1111_1001
        let memory = [0b1111_1001];
        let decoded = Instruction::decode(&memory).expect("LoadSPHL");

        assert_eq!(decoded, Instruction::LoadSPHL);
    }

    #[rstest]
    fn it_decodes_push(#[values(R16::BC, R16::DE, R16::HL, R16::AF)] src: R16) {
        // 0b11_xx_0101
        let opcode = 0b11_00_0101 + (<R16 as Into<u8>>::into(src) << 4);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Push");

        assert_eq!(decoded, Instruction::Push { src });
    }

    #[rstest]
    fn it_decodes_pop(#[values(R16::BC, R16::DE, R16::HL, R16::AF)] dest: R16) {
        // 0b11_xx_0001
        let opcode = 0b11_00_0001 + (<R16 as Into<u8>>::into(dest) << 4);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Pop");

        assert_eq!(decoded, Instruction::Pop { dest });
    }

    #[test]
    fn it_decodes_load_hl_sp_offset() {
        // 0b1111_1000
        let memory = [0b1111_1000, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("LoadHLSPOffset");

        assert_eq!(decoded, Instruction::LoadHLSPOffset { offset: 1 });
    }

    #[rstest]
    fn it_decodes_add_8(#[values(B, C, D, E, H, L, A)] src: R8) {
        // 0b10000_xxx
        let opcode = 0b10000_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Add8");

        assert_eq!(decoded, Instruction::Add8 { src });
    }

    #[test]
    fn it_decodes_add8_indirect_hl() {
        // 0b1000_0110
        let memory = [0b1000_0110];
        let decoded = Instruction::decode(&memory).expect("Add8IndirectHL");

        assert_eq!(decoded, Instruction::Add8IndirectHL);
    }

    #[test]
    fn it_decodes_add_imm_8() {
        //0b1100_0110
        let memory = [0b1100_0110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("AddImm8");
        assert_eq!(decoded, Instruction::AddImm8 { imm: 1 });
    }

    #[rstest]
    fn it_decodes_addc_8(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b10001_xxx
        let opcode = 0b10001_000 + <R8 as Into<u8>>::into(src);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("AddC8");
        assert_eq!(decoded, Instruction::AddC8 { src });
    }

    #[test]
    fn it_decodes_addc8_indirect_hl() {
        // 0b1000_1110
        let memory = [0b1000_1110];
        let decoded = Instruction::decode(&memory).expect("AddC8IndirectHL");

        assert_eq!(decoded, Instruction::AddC8IndirectHL);
    }

    #[test]
    fn it_decodes_addc_imm_8() {
        //0b1100_0110
        let memory = [0b1100_1110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("AddCImm8");
        assert_eq!(decoded, Instruction::AddCImm8 { imm: 1 });
    }

    #[rstest]
    fn it_decodes_sub_8(#[values(B, C, D, E, H, L, A)] src: R8) {
        // 0b10010_xxx
        let opcode = 0b10010_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Sub8");

        assert_eq!(decoded, Instruction::Sub8 { src });
    }

    #[test]
    fn it_decodes_sub8_indirect_hl() {
        // 0b1001_0110
        let memory = [0b1001_0110];
        let decoded = Instruction::decode(&memory).expect("Sub8IndirectHL");

        assert_eq!(decoded, Instruction::Sub8IndirectHL);
    }

    #[test]
    fn it_decodes_sub_imm_8() {
        //0b1101_0110
        let memory = [0b1101_0110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("SubImm8");
        assert_eq!(decoded, Instruction::SubImm8 { imm: 1 });
    }

    #[rstest]
    fn it_decodes_subc_8(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b10011_xxx
        let opcode = 0b10011_000 + <R8 as Into<u8>>::into(src);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("SubC8");
        assert_eq!(decoded, Instruction::SubC8 { src });
    }

    #[test]
    fn it_decodes_subc8_indirect_hl() {
        // 0b1001_1110
        let memory = [0b1001_1110];
        let decoded = Instruction::decode(&memory).expect("SubC8IndirectHL");

        assert_eq!(decoded, Instruction::SubC8IndirectHL);
    }

    #[test]
    fn it_decodes_subc_imm_8() {
        //0b1101_0110
        let memory = [0b1101_1110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("SubCImm8");
        assert_eq!(decoded, Instruction::SubCImm8 { imm: 1 });
    }

    #[rstest]
    fn it_decodes_cmp_8(#[values(B, C, D, E, H, L, A)] src: R8) {
        // 0b10111_xxx
        let opcode = 0b10111_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Cmp8");

        assert_eq!(decoded, Instruction::Cmp8 { src });
    }

    #[test]
    fn it_decodes_cmp8_indirect_hl() {
        // 0b1011_1110
        let memory = [0b1011_1110];
        let decoded = Instruction::decode(&memory).expect("Cmp8IndirectHL");

        assert_eq!(decoded, Instruction::Cmp8IndirectHL);
    }

    #[test]
    fn it_decodes_cmp_imm_8() {
        //0b1111_1110
        let memory = [0b1111_1110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("CmpImm8");
        assert_eq!(decoded, Instruction::CmpImm8 { imm: 1 });
    }

    #[rstest]
    fn it_decodes_inc8(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b00_xxx_100
        let opcode = 0b00_000_100 + (<R8 as Into<u8>>::into(src) << 3);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Inc8");
        assert_eq!(decoded, Instruction::Inc8 { src });
    }

    #[test]
    fn it_decodes_inc8_indirect_hl() {
        // 0b0011_0100
        let memory = [0b0011_0100];
        let decoded = Instruction::decode(&memory).expect("Inc8IndirectHL");

        assert_eq!(decoded, Instruction::Inc8IndirectHL);
    }

    #[rstest]
    fn it_decodes_dec8(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b00_xxx_101
        let opcode = 0b00_000_101 + (<R8 as Into<u8>>::into(src) << 3);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Dec8");
        assert_eq!(decoded, Instruction::Dec8 { src });
    }

    #[test]
    fn it_decodes_dec8_indirect_hl() {
        // 0b0011_0101
        let memory = [0b0011_0101];
        let decoded = Instruction::decode(&memory).expect("Dec8IndirectHL");

        assert_eq!(decoded, Instruction::Dec8IndirectHL);
    }

    #[rstest]
    fn it_decodes_and_8(#[values(B, C, D, E, H, L, A)] src: R8) {
        // 0b10100_xxx
        let opcode = 0b10100_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("And8");

        assert_eq!(decoded, Instruction::And8 { src });
    }

    #[test]
    fn it_decodes_and8_indirect_hl() {
        // 0b1010_0110
        let memory = [0b1010_0110];
        let decoded = Instruction::decode(&memory).expect("And8IndirectHL");

        assert_eq!(decoded, Instruction::And8IndirectHL);
    }

    #[test]
    fn it_decodes_and_imm_8() {
        //0b1110_0110
        let memory = [0b1110_0110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("AndImm8");
        assert_eq!(decoded, Instruction::AndImm8 { imm: 1 });
    }

    #[rstest]
    fn it_decodes_or_8(#[values(B, C, D, E, H, L, A)] src: R8) {
        // 0b10110_xxx
        let opcode = 0b10110_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Or8");

        assert_eq!(decoded, Instruction::Or8 { src });
    }

    #[test]
    fn it_decodes_or8_indirect_hl() {
        // 0b1011_0110
        let memory = [0b1011_0110];
        let decoded = Instruction::decode(&memory).expect("Or8IndirectHL");

        assert_eq!(decoded, Instruction::Or8IndirectHL);
    }

    #[test]
    fn it_decodes_or_imm_8() {
        //0b1111_0110
        let memory = [0b1111_0110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("OrImm8");
        assert_eq!(decoded, Instruction::OrImm8 { imm: 1 });
    }

    #[rstest]
    fn it_decodes_xor_8(#[values(B, C, D, E, H, L, A)] src: R8) {
        // 0b10101_xxx
        let opcode = 0b10101_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Xor8");

        assert_eq!(decoded, Instruction::Xor8 { src });
    }

    #[test]
    fn it_decodes_xor8_indirect_hl() {
        // 0b1010_1110
        let memory = [0b1010_1110];
        let decoded = Instruction::decode(&memory).expect("Xor8IndirectHL");

        assert_eq!(decoded, Instruction::Xor8IndirectHL);
    }

    #[test]
    fn it_decodes_xor_imm_8() {
        //0b1110_1110
        let memory = [0b1110_1110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("XorImm8");
        assert_eq!(decoded, Instruction::XorImm8 { imm: 1 });
    }

    #[test]
    fn it_decodes_ccf() {
        //0b0011_1111
        let memory = [0b0011_1111];
        let decoded = Instruction::decode(&memory).expect("CCF");
        assert_eq!(decoded, Instruction::CmplCarryFlag);
    }

    #[test]
    fn it_decodes_scf() {
        //0b0011_0111
        let memory = [0b0011_0111];
        let decoded = Instruction::decode(&memory).expect("SCF");
        assert_eq!(decoded, Instruction::SetCarryFlag);
    }

    #[test]
    fn it_decodes_daa() {
        //0b0010_0111
        let memory = [0b0010_0111];
        let decoded = Instruction::decode(&memory).expect("DAA");
        assert_eq!(decoded, Instruction::DecAdjAcc);
    }

    #[test]
    fn it_decodes_cpl() {
        //0b0010_1111
        let memory = [0b0010_1111];
        let decoded = Instruction::decode(&memory).expect("CPL");
        assert_eq!(decoded, Instruction::CmplAcc);
    }

    #[rstest]
    fn it_decodes_inc16(#[values(R16::BC, R16::DE, R16::HL, R16::SP)] reg_pair: R16) {
        //0b00_xx_0011
        let opcode = 0b00_00_0011 + (<R16 as Into<u8>>::into(reg_pair) << 4);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Inc16");
        assert_eq!(decoded, Instruction::Inc16 { src: reg_pair });
    }

    #[rstest]
    fn it_decodes_dec16(#[values(R16::BC, R16::DE, R16::HL, R16::SP)] reg_pair: R16) {
        //0b00_xx_1011
        let opcode = 0b00_00_1011 + (<R16 as Into<u8>>::into(reg_pair) << 4);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Dec16");
        assert_eq!(decoded, Instruction::Dec16 { src: reg_pair });
    }

    #[rstest]
    fn it_decodes_add16_hl(#[values(R16::BC, R16::DE, R16::HL, R16::SP)] reg_pair: R16) {
        //0b00_xx_1001
        let opcode: u8 = 0b00_00_1001 + (<R16 as Into<u8>>::into(reg_pair) << 4);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Add16HL");
        assert_eq!(decoded, Instruction::Add16HL { src: reg_pair });
    }

    #[test]
    fn it_decodes_add_sp_imm8() {
        //0b1110_1000
        let memory = [0b1110_1000, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("AddSPImm8");
        assert_eq!(decoded, Instruction::AddSPImm8 { imm: 1 });
    }

    #[test]
    fn it_decodes_rlca() {
        //0b0000_0111
        let memory = [0b0000_0111];
        let decoded = Instruction::decode(&memory).expect("RLCA");
        assert_eq!(decoded, Instruction::RLCA);
    }

    #[test]
    fn it_decodes_rrca() {
        //0b0000_1111
        let memory = [0b0000_1111];
        let decoded = Instruction::decode(&memory).expect("RRCA");
        assert_eq!(decoded, Instruction::RRCA);
    }

    #[test]
    fn it_decodes_rla() {
        //0b0001_0111
        let memory = [0b0001_0111];
        let decoded = Instruction::decode(&memory).expect("RLA");
        assert_eq!(decoded, Instruction::RLA);
    }

    #[test]
    fn it_decodes_rra() {
        //0b0001_1111
        let memory = [0b0001_1111];
        let decoded = Instruction::decode(&memory).expect("RRA");
        assert_eq!(decoded, Instruction::RRA);
    }

    #[rstest]
    fn it_decodes_rlc(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b0000_0xxx
        let opcode = 0b0000_0_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("RLC");
        assert_eq!(decoded, Instruction::RLC { src });
    }

    #[test]
    fn it_decodes_rlc_indirect_hl() {
        //0b0000_0110
        let memory = [0xCB, 0b0000_0110];
        let decoded = Instruction::decode(&memory).expect("RLCIndirectHL");
        assert_eq!(decoded, Instruction::RLCIndirectHL);
    }

    #[rstest]
    fn it_decodes_rrc(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b0000_1xxx
        let opcode = 0b0000_1_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("RRC");
        assert_eq!(decoded, Instruction::RRC { src });
    }

    #[test]
    fn it_decodes_rrc_indirect_hl() {
        //0b0000_1110
        let memory = [0xCB, 0b0000_1_110];
        let decoded = Instruction::decode(&memory).expect("RRCIndirectHL");
        assert_eq!(decoded, Instruction::RRCIndirectHL);
    }

    #[rstest]
    fn it_decodes_rl(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b0001_0xxx
        let opcode = 0b0001_0_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("RL");
        assert_eq!(decoded, Instruction::RL { src });
    }

    #[test]
    fn it_decodes_rl_indirect_hl() {
        //0b0001_0110
        let memory = [0xCB, 0b0001_0110];
        let decoded = Instruction::decode(&memory).expect("RLIndirectHL");
        assert_eq!(decoded, Instruction::RLIndirectHL);
    }

    #[rstest]
    fn it_decodes_rr(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b0001_1xxx
        let opcode = 0b0001_1_000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("RR");
        assert_eq!(decoded, Instruction::RR { src });
    }

    #[test]
    fn it_decodes_rr_indirect_hl() {
        //0b0001_1110
        let memory = [0xCB, 0b0001_1110];
        let decoded = Instruction::decode(&memory).expect("RRIndirectHL");
        assert_eq!(decoded, Instruction::RRIndirectHL);
    }

    #[rstest]
    fn it_decodes_sla(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b0010_0xxx
        let opcode = 0b0010_0000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("SLA");
        assert_eq!(decoded, Instruction::SLA { src });
    }

    #[test]
    fn it_decodes_sla_indirect_hl() {
        //0b0010_0110
        let memory = [0xCB, 0b0010_0110];
        let decoded = Instruction::decode(&memory).expect("SLAIndirectHL");
        assert_eq!(decoded, Instruction::SLAIndirectHL);
    }

    #[rstest]
    fn it_decodes_sra(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b0010_1xxx
        let opcode = 0b0010_1000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("SRA");
        assert_eq!(decoded, Instruction::SRA { src });
    }

    #[test]
    fn it_decodes_sra_indirect_hl() {
        //0b0010_1110
        let memory = [0xCB, 0b0010_1110];
        let decoded = Instruction::decode(&memory).expect("SRAIndirectHL");
        assert_eq!(decoded, Instruction::SRAIndirectHL);
    }

    #[rstest]
    fn it_decodes_swap(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b0011_0xxx
        let opcode = 0b0011_0000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("SRA");
        assert_eq!(decoded, Instruction::Swap { src });
    }

    #[test]
    fn it_decodes_swap_indirect_hl() {
        //0b0011_0110
        let memory = [0xCB, 0b0011_0110];
        let decoded = Instruction::decode(&memory).expect("SwapIndirectHL");
        assert_eq!(decoded, Instruction::SwapIndirectHL);
    }

    #[rstest]
    fn it_decodes_srl(#[values(B, C, D, E, H, L, A)] src: R8) {
        //0b0011_1xxx
        let opcode = 0b0011_1000 + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("SRL");
        assert_eq!(decoded, Instruction::SRL { src });
    }

    #[test]
    fn it_decodes_srl_indirect_hl() {
        //0b0011_1110
        let memory = [0xCB, 0b0011_1110];
        let decoded = Instruction::decode(&memory).expect("SRLIndirectHL");
        assert_eq!(decoded, Instruction::SRLIndirectHL);
    }

    #[rstest]
    fn it_decodes_bit(
        #[values(B, C, D, E, H, L, A)] src: R8,
        #[values(0, 1, 2, 3, 4, 5, 6, 7)] bit: u8,
    ) {
        //0b01xxxyyy
        let opcode = 0b01_000_000 + (bit << 3) + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("BIT");
        assert_eq!(decoded, Instruction::BIT { src, bit });
    }

    #[rstest]
    fn it_decodes_bit_indirect_hl(#[values(0, 1, 2, 3, 4, 5, 6, 7)] bit: u8) {
        //0b01xxx110
        let opcode = 0b01_000_110 + (bit << 3);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("BITIndirectHL");
        assert_eq!(decoded, Instruction::BITIndirectHL { bit });
    }

    #[rstest]
    fn it_decodes_res(
        #[values(B, C, D, E, H, L, A)] src: R8,
        #[values(0, 1, 2, 3, 4, 5, 6, 7)] bit: u8,
    ) {
        //0b10xxxyyy
        let opcode = 0b10_000_000 + (bit << 3) + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("RES");
        assert_eq!(decoded, Instruction::RES { src, bit });
    }

    #[rstest]
    fn it_decodes_res_indirect_hl(#[values(0, 1, 2, 3, 4, 5, 6, 7)] bit: u8) {
        //0b10xxx110
        let opcode = 0b10_000_110 + (bit << 3);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("RESIndirectHL");
        assert_eq!(decoded, Instruction::RESIndirectHL { bit });
    }

    #[rstest]
    fn it_decodes_set(
        #[values(B, C, D, E, H, L, A)] src: R8,
        #[values(0, 1, 2, 3, 4, 5, 6, 7)] bit: u8,
    ) {
        //0b11xxxyyy
        let opcode = 0b11_000_000 + (bit << 3) + (<R8 as Into<u8>>::into(src) << 0);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("SET");
        assert_eq!(decoded, Instruction::SET { src, bit });
    }

    #[rstest]
    fn it_decodes_set_indirect_hl(#[values(0, 1, 2, 3, 4, 5, 6, 7)] bit: u8) {
        //0b11xxx110
        let opcode = 0b11_000_110 + (bit << 3);
        let memory = [0xCB, opcode];
        let decoded = Instruction::decode(&memory).expect("SETIndirectHL");
        assert_eq!(decoded, Instruction::SETIndirectHL { bit });
    }

    #[test]
    fn it_decodes_jp() {
        //0b1100_0011
        let memory = [0b1100_0011, 0xF0, 0x0F];
        let decoded = Instruction::decode(&memory).expect("JP");
        assert_eq!(decoded, Instruction::JP { addr: 0x0FF0 });
    }

    #[test]
    fn it_decodes_jp_hl() {
        //0b1110_1001
        let memory = [0b1110_1001];
        let decoded = Instruction::decode(&memory).expect("JPHL");
        assert_eq!(decoded, Instruction::JPHL);
    }

    #[rstest]
    fn it_decodes_jpcc(#[values(NZ, NCARRY, Z, CARRY)] cond: Cond) {
        //0b110_xx_010
        let opcode = 0b110_00_010 + (<Cond as Into<u8>>::into(cond) << 3);
        let memory = [opcode, 0xF0, 0x0F];
        let decoded = Instruction::decode(&memory).expect("JPCC");
        assert_eq!(decoded, Instruction::JPCC { cond, addr: 0x0FF0 });
    }

    #[test]
    fn it_decodes_jr() {
        //0b0001_1000
        let memory = [0b0001_1000, 0xff];
        let decoded = Instruction::decode(&memory).expect("JR");
        // 0xff (u8) is reinterpreted as -1 (i8)
        assert_eq!(decoded, Instruction::JR { offset: -0x01 });
    }

    #[rstest]
    fn it_decodes_jrcc(#[values(NZ, NCARRY, Z, CARRY)] cond: Cond) {
        //0b001_xx_000
        let opcode = 0b001_00_000 + (<Cond as Into<u8>>::into(cond) << 3);
        let memory = [opcode, 0xFF];
        let decoded = Instruction::decode(&memory).expect("JRCC");
        assert_eq!(decoded, Instruction::JRCC { cond, offset: -0x1 });
    }

    #[test]
    fn it_decodes_call() {
        //0b1100_1101
        let memory = [0b1100_1101, 0xF0, 0x0F];
        let decoded = Instruction::decode(&memory).expect("Call");
        assert_eq!(decoded, Instruction::Call { addr: 0x0FF0 });
    }

    #[rstest]
    fn it_decodes_callcc(#[values(NZ, NCARRY, Z, CARRY)] cond: Cond) {
        //0b110_xx_100
        let opcode = 0b110_00_100 + (<Cond as Into<u8>>::into(cond) << 3);
        let memory = [opcode, 0xF0, 0x0F];
        let decoded = Instruction::decode(&memory).expect("CallCC");
        assert_eq!(decoded, Instruction::CallCC { cond, addr: 0x0FF0 });
    }

    #[test]
    fn it_decodes_ret() {
        //0b1100_1001
        let memory = [0b1100_1001];
        let decoded = Instruction::decode(&memory).expect("Ret");
        assert_eq!(decoded, Instruction::Ret);
    }

    #[rstest]
    fn it_decodes_retcc(#[values(NZ, NCARRY, Z, CARRY)] cond: Cond) {
        //0b110_xx_000
        let opcode = 0b110_00_000 + (<Cond as Into<u8>>::into(cond) << 3);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("RetCC");
        assert_eq!(decoded, Instruction::RetCC { cond });
    }

    #[test]
    fn it_decodes_reti() {
        //0b1101_1001
        let memory = [0b1101_1001];
        let decoded = Instruction::decode(&memory).expect("RetI");
        assert_eq!(decoded, Instruction::RetI);
    }

    #[rstest]
    fn it_decodes_rst(#[values(0, 1, 2, 3, 4, 5, 6, 7)] addr: u8) {
        //0b11_xxx_111
        let opcode = 0b11_000_111 + (addr << 3);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("RST");
        // Bit shift address by 3
        assert_eq!(decoded, Instruction::RST { addr: addr << 3 });
    }

    #[test]
    fn it_decodes_halt() {
        //0b0111_0110

        let opcode = 0b0111_0110;
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Halt");
        assert_eq!(decoded, Instruction::Halt);
    }

    #[test]
    fn it_decodes_stop() {
        //0b0001_0000
        let memory = [0b0001_0000, 0x01];
        let decoded = Instruction::decode(&memory).expect("Stop");
        assert_eq!(decoded, Instruction::Stop { ignore: 0x01 });
    }

    #[test]
    fn it_decodes_di() {
        //0b1111_0011
        let memory = [0b1111_0011];
        let decoded = Instruction::decode(&memory).expect("DI");
        assert_eq!(decoded, Instruction::DI);
    }

    #[test]
    fn it_decodes_ei() {
        //0b1111_1011
        let memory = [0b1111_1011];
        let decoded = Instruction::decode(&memory).expect("EI");
        assert_eq!(decoded, Instruction::EI);
    }

    #[test]
    fn it_decodes_nop() {
        //0b0000_0000
        let memory = [0b0000_0000];
        let decoded = Instruction::decode(&memory).expect("NOP");
        assert_eq!(decoded, Instruction::NOP);
    }

    #[rstest]
    #[case(0xD3)]
    #[case(0xE3)]
    #[case(0xE4)]
    #[case(0xF4)]
    #[case(0xDB)]
    #[case(0xEB)]
    #[case(0xEC)]
    #[case(0xFC)]
    #[case(0xDD)]
    #[case(0xED)]
    #[case(0xFD)]
    fn it_fails_to_decode_invalid_opcodes(#[case] input: u8) {
        let memory = [input];
        let decoded = Instruction::decode(&memory);

        assert!(decoded.is_err());

        assert_eq!(decoded.unwrap_err(), DecodeError::UnknownOpcode);
    }

    #[test]
    fn it_fails_to_decode_empty_memory() {
        let memory: [u8; 0] = [];

        let decoded = Instruction::decode(&memory);

        assert_eq!(decoded.unwrap_err(), DecodeError::MemoryOutOfBounds);
    }
    #[test]
    fn it_fails_to_decode_operand_out_of_bounds() {
        let memory: [u8; 1] = [0b00_111_110];

        let decoded = Instruction::decode(&memory);

        assert_eq!(decoded.unwrap_err(), DecodeError::MemoryOutOfBounds);
    }
    #[test]
    fn it_fails_to_decode_two_byte_operand_out_of_bounds() {
        let memory: [u8; 2] = [0b1111_1010, 0b0000_0000];

        let decoded = Instruction::decode(&memory);

        assert_eq!(decoded.unwrap_err(), DecodeError::MemoryOutOfBounds);
    }

    #[test]
    fn it_covers_all_opcodes() {
        let invalid_opcodes = [
            0xD3, 0xE3, 0xE4, 0xF4, 0xDB, 0xEB, 0xEC, 0xFC, 0xDD, 0xED, 0xFD,
        ];
        for opcode in 0..=0xFF {
            if invalid_opcodes.contains(&opcode) {
                continue;
            }
            let memory = [opcode, 0xFF, 0xFF];
            let decoded = Instruction::decode(&memory);

            assert!(decoded.is_ok());
        }
        for opcode in 0..=0xFF {
            let memory = [0xCB, opcode, 0xFF, 0xFF];
            let decoded = Instruction::decode(&memory);

            assert!(decoded.is_ok());
        }
    }
}

#[cfg(test)]
mod machine_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(R16::BC, B, C)]
    #[case(R16::DE, D, E)]
    #[case(R16::HL, H, L)]
    #[case(R16::AF, A, F)]
    fn it_sets_reg_pairs(#[case] pair: R16, #[case] h_reg: R8, #[case] l_reg: R8) {
        let mut machine = Machine::new();

        machine.set_r16(pair, 0xABCD);

        assert_eq!(machine.get_r8(h_reg), 0xAB);
        assert_eq!(machine.get_r8(l_reg), 0xCD);
    }

    #[rstest]
    #[case(R16::BC, B, C)]
    #[case(R16::DE, D, E)]
    #[case(R16::HL, H, L)]
    #[case(R16::AF, A, F)]
    fn it_gets_reg_pairs(#[case] pair: R16, #[case] h_reg: R8, #[case] l_reg: R8) {
        let mut machine = Machine::new();
        machine.set_r8(h_reg, 0xAB);
        machine.set_r8(l_reg, 0xCD);

        assert_eq!(machine.get_r16(pair), 0xABCD);
    }
}

#[cfg(test)]
mod exec_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn it_execs_load_reg(
        #[values(B, C, D, E, H, L, A)] src: R8,
        #[values(B, C, D, E, H, L, A)] dest: R8,
    ) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);

        machine.set_r8(src, 0xFF);
        machine.set_r8(dest, 0x0C);

        let ins = Instruction::LoadReg8 { src, dest };

        machine.exec(ins);

        assert_eq!(machine.get_r8(src), machine.get_r8(dest));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[rstest]
    fn it_execs_load_imm8(
        #[values(B, C, D, E, H, L, A)] dest: R8,
        #[values(0x00, 0x01, 0xFE, 0xFF)] imm: u8,
    ) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(dest, 0x00);

        let ins = Instruction::LoadImm8 { imm, dest };
        machine.exec(ins);

        assert_eq!(imm, machine.get_r8(dest));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[rstest]
    fn it_execs_load_indirect_hl_8(#[values(B, C, D, E, H, L, A)] dest: R8) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(dest, 0x00);
        machine.set_r16(R16::HL, 0x0001);

        machine.set_memory(1, &[0xFE]);

        let ins = Instruction::LoadIndirectHL { dest };
        machine.exec(ins);

        assert_eq!(0xFE, machine.get_r8(dest));
        assert_eq!(machine.get_pc(), 0x01);
    }
}
