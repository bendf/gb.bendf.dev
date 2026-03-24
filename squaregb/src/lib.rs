use arbitrary_int::u2;
use wasm_bindgen::prelude::*;
use web_sys;

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
#[repr(u8)]
pub enum Reg {
    B = 0,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    LoadReg { src: Reg, dest: Reg },
    // LoadImm { dest: Reg, imm: u8 },
}

#[derive(Debug, PartialEq)]
pub struct DecodeError(String);

impl Instruction {
    pub fn decode(opcode: u8) -> Result<Instruction, DecodeError> {
        let b7_6 = u2::extract_u8(opcode, 6);

        if b7_6 == u2::new(0b01) {
            Ok(Instruction::LoadReg {
                src: Reg::B,
                dest: Reg::B,
            })
        } else {
            Err(DecodeError(String::from("Unknown opcode")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_load_opcode() {
        let opcode = 0b0100_0000;
        let decoded = Instruction::decode(opcode).expect("LD should decode");

        assert_eq!(
            decoded,
            Instruction::LoadReg {
                src: Reg::B,
                dest: Reg::B
            }
        );
    }

    #[test]
    fn it_fails_to_decode_garbage() {
        let opcode = 0b0000_0000;
        let decoded = Instruction::decode(opcode);

        assert!(decoded.is_err());

        assert_eq!(
            decoded.unwrap_err(),
            DecodeError(String::from("Unknown opcode"))
        );
    }
}
