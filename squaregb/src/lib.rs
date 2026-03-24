use arbitrary_int::{u2, u3, u5};
use wasm_bindgen::prelude::*;
use web_sys;

type RegPair = u8;
const BC: RegPair = 0b00;
const DE: RegPair = 0b01;
const HL: RegPair = 0b10;
// Not stricty a Register pair.
// TODO: Rename Regpair
const AF: RegPair = 0b11;

type Reg = u8;
const A: Reg = 0b111;
const B: Reg = 0b000;
const C: Reg = 0b001;
const D: Reg = 0b010;
const E: Reg = 0b011;
const F: Reg = 0b100;
const H: Reg = 0b101;
const L: Reg = 0b110;

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

pub struct Machine {}

impl Machine {
    pub fn new() -> Self {
        Machine {}
    }

    pub fn set_rom(&self, _rom: &[u8]) {}
    pub fn set_pc(&self, _addr: u16) {}

    pub fn eval(&self, _steps: usize) {}

    pub fn get_reg_a(&self) -> u8 {
        0xFF
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    LoadReg { src: Reg, dest: Reg },
    LoadImm { dest: Reg, imm: u8 },
    LoadIndirectHL { dest: Reg },
    StoreIndirectHL { src: Reg },
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
    LoadImm16 { dest: RegPair, imm: u16 },
    StoreSP16 { addr: u16 },
    LoadSPHL,
    Push { src: RegPair },
    Pop { dest: RegPair },
    LoadHLSPOffset { offset: u8 },
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

        let b7_6: u8 = u2::extract_u8(*first_byte, 6).value();
        let b2_0: u8 = u3::extract_u8(*first_byte, 0).value();
        let b7_3: u8 = u5::extract_u8(*first_byte, 3).value();

        match (first_byte, b7_3, b7_6, b2_0) {
            (0b1111_1000, _, _, _) => {
                let Some(offset) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::LoadHLSPOffset { offset: *offset })
            }
            (0b11_00_0001 | 0b11_01_0001 | 0b11_10_0001 | 0b11_11_0001, _, _, _) => {
                let dest: u8 = u2::extract_u8(*first_byte, 4).value();

                Ok(Instruction::Pop { dest })
            }
            (0b11_00_0101 | 0b11_01_0101 | 0b11_10_0101 | 0b11_11_0101, _, _, _) => {
                let src: u8 = u2::extract_u8(*first_byte, 4).value();

                Ok(Instruction::Push { src })
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
                let dest: u8 = u2::extract_u8(*first_byte, 4).value();
                let imm: u16 = u16::from_le_bytes(imm.try_into().unwrap());

                Ok(Instruction::LoadImm16 { dest, imm })
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
            (_, 0b01110, _, _) => {
                let src = u3::extract_u8(*first_byte, 0).value();
                Ok(Instruction::StoreIndirectHL { src })
            }
            (_, _, 0b01, 0b110) => {
                let dest = u3::extract_u8(*first_byte, 3).value();
                Ok(Instruction::LoadIndirectHL { dest })
            }
            (_, _, 0b01, _) => {
                let dest = u3::extract_u8(*first_byte, 3).value();
                let src = u3::extract_u8(*first_byte, 0).value();

                Ok(Instruction::LoadReg {
                    src: src,
                    dest: dest,
                })
            }
            (_, _, 0b00, 0b110) => {
                let dest = u3::extract_u8(*first_byte, 3).value();

                let Some(imm) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::LoadImm { dest, imm: *imm })
            }
            _ => Err(DecodeError::UnknownOpcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn it_decodes_load_reg_opcode() {
        // 0b01_xxx_yyy
        let memory = [0b01_000_001];
        let decoded = Instruction::decode(&memory).expect("LoadReg should decode");

        assert_eq!(decoded, Instruction::LoadReg { src: C, dest: B });
    }

    #[test]
    fn it_decodes_load_imm_opcode() {
        // 0b00_xxx_110, <1-byte imm>
        let memory = [0b00_111_110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("LoadImm should decode");

        assert_eq!(decoded, Instruction::LoadImm { dest: A, imm: 1 });
    }

    #[test]
    fn it_decodes_load_indirect_hl_opcode() {
        // 0b01_xxx_110
        let memory = [0b01_000_110];
        let decoded = Instruction::decode(&memory).expect("LoadIndirectHL should decode");

        assert_eq!(decoded, Instruction::LoadIndirectHL { dest: B });
    }

    #[test]
    fn it_decodes_store_indirect_hl_opcode() {
        // 0b01110_xxx
        let memory = [0b01110_111];
        let decoded = Instruction::decode(&memory).expect("StoreIndirectHL should decode");

        assert_eq!(decoded, Instruction::StoreIndirectHL { src: A });
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

    #[test]
    fn it_decodes_load_imm_16() {
        // 0b00xx0001
        let memory = [0b00_11_0001, 0b1111_0000, 0b0000_1111];
        let decoded = Instruction::decode(&memory).expect("LoadImm16 should decode");

        assert_eq!(
            decoded,
            Instruction::LoadImm16 {
                dest: AF,
                imm: 0x0FF0
            }
        );
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

    #[test]
    fn it_decodes_push() {
        // 0b11_xx_0101
        let memory = [0b1100_0101];
        let decoded = Instruction::decode(&memory).expect("Push");

        assert_eq!(decoded, Instruction::Push { src: BC });
    }

    #[test]
    fn it_decodes_pop() {
        // 0b11_xx_0001
        let memory = [0b1101_0001];
        let decoded = Instruction::decode(&memory).expect("Pop");

        assert_eq!(decoded, Instruction::Pop { dest: DE });
    }

    #[test]
    fn it_decodes_load_hl_sp_offset() {
        // 0b1111_1000
        let memory = [0b1111_1000, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("LoadHLSPOffset");

        assert_eq!(decoded, Instruction::LoadHLSPOffset { offset: 1 });
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
}
