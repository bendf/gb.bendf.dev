use arbitrary_int::{u2, u3};
use wasm_bindgen::prelude::*;
use web_sys;

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

        match (b7_6, b2_0) {
            (0b01, _) => {
                let dest = u3::extract_u8(*first_byte, 3).value();
                let src = u3::extract_u8(*first_byte, 0).value();

                Ok(Instruction::LoadReg {
                    src: src,
                    dest: dest,
                })
            }
            (0b00, 0b110) => {
                let dest = u3::extract_u8(*first_byte, 3).value();

                let Some(second_byte) = memory.get(1) else {
                    return Err(DecodeError::MemoryOutOfBounds);
                };

                Ok(Instruction::LoadImm {
                    dest,
                    imm: *second_byte,
                })
            }
            _ => Err(DecodeError::UnknownOpcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_load_reg_opcode() {
        let memory = [0b0100_0001];
        let decoded = Instruction::decode(&memory).expect("LoadReg should decode");

        assert_eq!(decoded, Instruction::LoadReg { src: C, dest: B });
    }

    #[test]
    fn it_decodes_load_imm_opcode() {
        let memory = [0b00_111_110, 0b0000_0001];
        let decoded = Instruction::decode(&memory).expect("LoadImm should decode");

        assert_eq!(decoded, Instruction::LoadImm { dest: A, imm: 1 });
    }

    #[test]
    fn it_fails_to_decode_garbage() {
        let memory = [0b0000_0000];
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
}
