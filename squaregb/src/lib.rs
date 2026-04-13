use Flag::{Carry, HalfCarryBCD, SubBCD, Zero};
use R16::{AF, BC, DE, HL, SP};
use arbitrary_int::{u1, u2, u3, u4, u5, u12};
use wasm_bindgen::prelude::*;
use web_sys;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Flag {
    Zero = 7,
    SubBCD = 6,
    HalfCarryBCD = 5,
    Carry = 4,
}

impl Into<u8> for Flag {
    fn into(self) -> u8 {
        self as u8
    }
}

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
            0b00 => BC,
            0b01 => DE,
            0b10 => HL,
            0b11 => SP,
            x => panic!("Unknown R16 id {:#b}", x),
        }
    }
    pub fn stk(id: u2) -> Self {
        match id.value() {
            0b00 => BC,
            0b01 => DE,
            0b10 => HL,
            0b11 => AF,
            x => panic!("Unknown R16 id {:#b}", x),
        }
    }
}

impl Into<u8> for R16 {
    fn into(self) -> u8 {
        match self {
            BC => 0b00,
            DE => 0b01,
            HL => 0b10,
            AF => 0b11,
            SP => 0b11,
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
            BC => {
                u16::from_be_bytes([self.gp_registers[B as usize], self.gp_registers[C as usize]])
            }
            DE => {
                u16::from_be_bytes([self.gp_registers[D as usize], self.gp_registers[E as usize]])
            }
            HL => {
                u16::from_be_bytes([self.gp_registers[H as usize], self.gp_registers[L as usize]])
            }
            SP => self.sp,
            AF => {
                u16::from_be_bytes([self.gp_registers[A as usize], self.gp_registers[F as usize]])
            }
        }
    }

    pub fn get_mem8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn get_mem16(&self, addr: u16) -> u16 {
        let addr = addr as usize;
        u16::from_le_bytes([self.memory[addr], self.memory[addr + 1]])
    }

    pub fn set_mem16(&mut self, addr: u16, value: u16) {
        let addr = addr as usize;

        let [low, high] = value.to_le_bytes();

        self.memory[addr] = low;
        self.memory[addr + 1] = high;
    }

    pub fn set_mem8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
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
            BC => {
                self.set_r8(B, high);
                self.set_r8(C, low);
            }
            DE => {
                self.set_r8(D, high);
                self.set_r8(E, low);
            }
            HL => {
                self.set_r8(H, high);
                self.set_r8(L, low);
            }
            AF => {
                self.set_r8(A, high);
                self.set_r8(F, low);
            }
            SP => self.set_sp(value),
        }
    }

    pub fn get_pc(&self) -> u16 {
        return self.pc;
    }

    // Return value of corresponding flag
    pub fn get_flag(&self, flag: Flag) -> bool {
        let bit: u8 = flag.into();

        let f_reg = self.get_r8(F);

        let value = u1::extract_u8(f_reg, bit as usize);
        value.into()
    }

    pub fn set_flag(&mut self, flag: Flag) {
        let f_reg = self.get_r8(F);
        let bit: u8 = flag.into();

        let reg = f_reg | (1 << bit);
        self.set_r8(F, reg);
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        let f_reg = self.get_r8(F);
        let bit: u8 = flag.into();

        let mask = !(1 << bit);
        let reg = f_reg & mask;
        self.set_r8(F, reg);
    }

    pub fn assign_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    pub fn inc_pc(&mut self) {
        self.pc = self.pc + 1
    }

    pub fn adv_pc(&mut self, offset: u16) {
        self.pc = self.pc + offset
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
                let addr = self.get_r16(HL);
                let value = self.get_mem8(addr);
                self.set_r8(dest, value);
                self.inc_pc();
            }
            StoreIndirectHL { src } => {
                let addr = self.get_r16(HL);
                let value = self.get_r8(src);

                self.set_mem8(addr, value);
                self.inc_pc();
            }
            StoreImmIndirectHL { imm } => {
                let addr = self.get_r16(HL);

                self.set_mem8(addr, imm);
                self.adv_pc(2)
            }
            LoadAccIndirectBC => {
                let addr = self.get_r16(BC);

                let value = self.get_mem8(addr);

                self.set_r8(A, value);
                self.inc_pc();
            }
            LoadAccIndirectDE => {
                let addr = self.get_r16(DE);

                let value = self.get_mem8(addr);

                self.set_r8(A, value);
                self.inc_pc();
            }
            StoreAccIndirectBC => {
                let addr = self.get_r16(BC);
                let value = self.get_r8(A);

                self.set_mem8(addr, value);
                self.inc_pc();
            }
            StoreAccIndirectDE => {
                let addr = self.get_r16(DE);
                let value = self.get_r8(A);

                self.set_mem8(addr, value);
                self.inc_pc();
            }
            LoadAcc16 { addr } => {
                let value = self.get_mem8(addr);

                self.set_r8(A, value);
                self.adv_pc(3);
            }
            StoreAcc16 { addr } => {
                let value = self.get_r8(A);

                self.set_mem8(addr, value);
                self.adv_pc(3);
            }
            LoadAccIndirectC => {
                let base: u16 = 0xFF00;
                let offset = self.get_r8(C);
                let addr = base + (offset as u16);
                let value = self.get_mem8(addr);
                self.set_r8(A, value);
                self.inc_pc();
            }
            StoreAccIndirectC => {
                let base: u16 = 0xFF00;
                let offset = self.get_r8(C);
                let addr = base + (offset as u16);
                let value = self.get_r8(A);
                self.set_mem8(addr, value);
                self.inc_pc();
            }
            LoadAccDirect8 { offset } => {
                let base: u16 = 0xFF00;
                let addr = base + (offset as u16);
                let value = self.get_mem8(addr);
                self.set_r8(A, value);
                self.inc_pc();
            }
            StoreAccDirect8 { offset } => {
                let base: u16 = 0xFF00;
                let addr = base + (offset as u16);
                let value = self.get_r8(A);
                self.set_mem8(addr, value);
                self.inc_pc();
            }
            LoadAccIndirectHLDec => {
                let addr = self.get_r16(HL);

                let value = self.get_mem8(addr);

                self.set_r8(A, value);
                self.inc_pc();
                self.set_r16(HL, addr - 1);
            }
            StoreAccIndirectHLDec => {
                let addr = self.get_r16(HL);
                let value = self.get_r8(A);

                self.set_mem8(addr, value);
                self.inc_pc();
                self.set_r16(HL, addr - 1);
            }
            LoadAccIndirectHLInc => {
                let addr = self.get_r16(HL);

                let value = self.get_mem8(addr);

                self.set_r8(A, value);
                self.inc_pc();
                self.set_r16(HL, addr + 1);
            }
            StoreAccIndirectHLInc => {
                let addr = self.get_r16(HL);
                let value = self.get_r8(A);

                self.set_mem8(addr, value);
                self.inc_pc();
                self.set_r16(HL, addr + 1);
            }
            LoadImm16 { dest, imm } => {
                self.set_r16(dest, imm);
                self.adv_pc(3);
            }

            StoreSP16 { addr } => {
                let value = self.get_r16(SP);
                self.set_mem16(addr, value);
                self.adv_pc(3);
            }

            LoadSPHL => {
                let value = self.get_r16(HL);
                self.set_r16(SP, value);
                self.inc_pc();
            }

            Push { src } => {
                let value = self.get_r16(src);
                let sp = self.get_r16(SP);
                self.set_mem16(sp - 1, value);

                self.set_sp(sp - 2);
                self.inc_pc();
            }
            Pop { dest } => {
                let sp = self.get_r16(SP);
                let value = self.get_mem16(sp + 1);
                self.set_r16(dest, value);

                self.set_sp(sp + 2);
                self.inc_pc();
            }
            LoadHLSPOffset { offset } => {
                let sp = self.get_r16(SP);

                let offsetu16 = (offset as i16).cast_unsigned();
                let value = sp.wrapping_add(offsetu16);
                self.set_r16(HL, value);

                self.clear_flag(Zero);
                self.clear_flag(SubBCD);

                let sp_3_0 = u4::extract_u16(sp, 0);
                let offset_3_0 = u4::extract_u16(offsetu16, 0);
                let (_, half_carry) = sp_3_0.overflowing_add(offset_3_0);

                let sp_7_0: u8 = (sp & 0x00FF).try_into().unwrap();
                let offset_7_0: u8 = (offsetu16 & 0x00FF).try_into().unwrap();
                let (_, carry) = sp_7_0.overflowing_add(offset_7_0);

                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);

                self.adv_pc(0x02);
            }

            Add8 { src } => {
                let x = self.get_r8(A);
                let y = self.get_r8(src);
                let (sum, carry) = u8::overflowing_add(x, y);

                self.set_r8(A, sum);
                self.inc_pc();

                self.assign_flag(Zero, sum == 0);
                self.clear_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let (_, half_carry) = x_3_0.overflowing_add(y_3_0);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            Add8IndirectHL => {
                let x = self.get_r8(A);
                let addr = self.get_r16(HL);
                let y = self.get_mem8(addr);
                let (sum, carry) = u8::overflowing_add(x, y);

                self.set_r8(A, sum);
                self.inc_pc();

                self.assign_flag(Zero, sum == 0);
                self.clear_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let (_, half_carry) = x_3_0.overflowing_add(y_3_0);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            AddImm8 { imm } => {
                let x = self.get_r8(A);
                let y = imm;
                let (sum, carry) = u8::overflowing_add(x, y);

                self.set_r8(A, sum);
                self.adv_pc(0x02);

                self.assign_flag(Zero, sum == 0);
                self.clear_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let (_, half_carry) = x_3_0.overflowing_add(y_3_0);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            AddC8 { src } => {
                let x = self.get_r8(A);
                let y = self.get_r8(src);
                let cry: u8 = self.get_flag(Carry).into();
                let (sum, carry) = u8::overflowing_add(x, y);
                let (sum, carry2) = u8::overflowing_add(sum, cry);

                // Its a carry if either addition results in a carry.
                let carry = carry | carry2;

                self.set_r8(A, sum);
                self.inc_pc();

                self.assign_flag(Zero, sum == 0);
                self.clear_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let cry_3_0 = u4::extract_u8(cry, 0);
                let (hc_sum, half_carry) = x_3_0.overflowing_add(y_3_0);
                let (_, half_carry2) = hc_sum.overflowing_add(cry_3_0);
                let half_carry = half_carry | half_carry2;

                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            AddC8IndirectHL => {
                let x = self.get_r8(A);
                let addr = self.get_r16(HL);
                let y = self.get_mem8(addr);
                let cry: u8 = self.get_flag(Carry).into();
                let (sum, carry) = u8::overflowing_add(x, y);
                let (sum, carry2) = u8::overflowing_add(sum, cry);

                // Its a carry if either addition results in a carry.
                let carry = carry | carry2;

                self.set_r8(A, sum);
                self.inc_pc();

                self.assign_flag(Zero, sum == 0);
                self.clear_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let cry_3_0 = u4::extract_u8(cry, 0);
                let (hc_sum, half_carry) = x_3_0.overflowing_add(y_3_0);
                let (_, half_carry2) = hc_sum.overflowing_add(cry_3_0);
                let half_carry = half_carry | half_carry2;
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            AddCImm8 { imm } => {
                let x = self.get_r8(A);
                let y = imm;
                let cry: u8 = self.get_flag(Carry).into();
                let (sum, carry) = u8::overflowing_add(x, y);
                let (sum, carry2) = u8::overflowing_add(sum, cry);

                // Its a carry if either addition results in a carry.
                let carry = carry | carry2;

                self.set_r8(A, sum);
                self.adv_pc(0x02);

                self.assign_flag(Zero, sum == 0);
                self.clear_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let cry_3_0 = u4::extract_u8(cry, 0);
                let (hc_sum, half_carry) = x_3_0.overflowing_add(y_3_0);
                let (_, half_carry2) = hc_sum.overflowing_add(cry_3_0);
                let half_carry = half_carry | half_carry2;
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            Sub8 { src } => {
                let x = self.get_r8(A);
                let y = self.get_r8(src);
                let (sum, carry) = u8::overflowing_sub(x, y);

                self.set_r8(A, sum);
                self.inc_pc();

                self.assign_flag(Zero, sum == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let (_, half_carry) = x_3_0.overflowing_sub(y_3_0);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            Sub8IndirectHL => {
                let x = self.get_r8(A);
                let addr = self.get_r16(HL);
                let y = self.get_mem8(addr);
                let (sum, carry) = u8::overflowing_sub(x, y);

                self.set_r8(A, sum);
                self.inc_pc();

                self.assign_flag(Zero, sum == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let (_, half_carry) = x_3_0.overflowing_sub(y_3_0);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            SubImm8 { imm } => {
                let x = self.get_r8(A);
                let y = imm;
                let (sum, carry) = u8::overflowing_sub(x, y);

                self.set_r8(A, sum);
                self.adv_pc(0x02);

                self.assign_flag(Zero, sum == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let (_, half_carry) = x_3_0.overflowing_sub(y_3_0);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            SubC8 { src } => {
                let x = self.get_r8(A);
                let y = self.get_r8(src);
                let cry: u8 = self.get_flag(Carry).into();
                let (res, carry) = u8::overflowing_sub(x, y);
                let (res, carry2) = u8::overflowing_sub(res, cry);

                // Its a carry if either addition results in a carry.
                let carry = carry | carry2;

                self.set_r8(A, res);
                self.inc_pc();

                self.assign_flag(Zero, res == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let cry_3_0 = u4::extract_u8(cry, 0);
                let (hc_sum, half_carry) = x_3_0.overflowing_sub(y_3_0);
                let (_, half_carry2) = hc_sum.overflowing_sub(cry_3_0);
                let half_carry = half_carry | half_carry2;

                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            SubC8IndirectHL => {
                let x = self.get_r8(A);
                let addr = self.get_r16(HL);
                let y = self.get_mem8(addr);
                let cry: u8 = self.get_flag(Carry).into();
                let (res, carry) = u8::overflowing_sub(x, y);
                let (res, carry2) = u8::overflowing_sub(res, cry);

                // Its a carry if either addition results in a carry.
                let carry = carry | carry2;

                self.set_r8(A, res);
                self.inc_pc();

                self.assign_flag(Zero, res == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let cry_3_0 = u4::extract_u8(cry, 0);
                let (hc_sum, half_carry) = x_3_0.overflowing_sub(y_3_0);
                let (_, half_carry2) = hc_sum.overflowing_sub(cry_3_0);
                let half_carry = half_carry | half_carry2;
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            SubCImm8 { imm } => {
                let x = self.get_r8(A);
                let y = imm;
                let cry: u8 = self.get_flag(Carry).into();
                let (res, carry) = u8::overflowing_sub(x, y);
                let (res, carry2) = u8::overflowing_sub(res, cry);

                // Its a carry if either addition results in a carry.
                let carry = carry | carry2;

                self.set_r8(A, res);
                self.adv_pc(0x02);

                self.assign_flag(Zero, res == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let cry_3_0 = u4::extract_u8(cry, 0);
                let (hc_sum, half_carry) = x_3_0.overflowing_sub(y_3_0);
                let (_, half_carry2) = hc_sum.overflowing_sub(cry_3_0);
                let half_carry = half_carry | half_carry2;
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            Cmp8 { src } => {
                let x = self.get_r8(A);
                let y = self.get_r8(src);
                let (sum, carry) = u8::overflowing_sub(x, y);

                self.inc_pc();

                self.assign_flag(Zero, sum == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let (_, half_carry) = x_3_0.overflowing_sub(y_3_0);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            Cmp8IndirectHL => {
                let x = self.get_r8(A);
                let addr = self.get_r16(HL);
                let y = self.get_mem8(addr);
                let (sum, carry) = u8::overflowing_sub(x, y);

                self.inc_pc();

                self.assign_flag(Zero, sum == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let (_, half_carry) = x_3_0.overflowing_sub(y_3_0);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            CmpImm8 { imm } => {
                let x = self.get_r8(A);
                let y = imm;
                let (sum, carry) = u8::overflowing_sub(x, y);

                self.adv_pc(0x02);

                self.assign_flag(Zero, sum == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let y_3_0 = u4::extract_u8(y, 0);
                let (_, half_carry) = x_3_0.overflowing_sub(y_3_0);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }
            Inc8 { src } => {
                let x = self.get_r8(src);
                let (sum, _) = u8::overflowing_add(x, 1);

                self.inc_pc();

                self.set_r8(src, sum);

                self.assign_flag(Zero, sum == 0);
                self.clear_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let (_, half_carry) = x_3_0.overflowing_add(u4::new(1u8));
                self.assign_flag(HalfCarryBCD, half_carry);
            }
            Inc8IndirectHL => {
                let addr = self.get_r16(HL);
                let x = self.get_mem8(addr);
                let (sum, _) = u8::overflowing_add(x, 1);

                self.inc_pc();

                self.set_mem8(addr, sum);

                self.assign_flag(Zero, sum == 0);
                self.clear_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let (_, half_carry) = x_3_0.overflowing_add(u4::new(1u8));
                self.assign_flag(HalfCarryBCD, half_carry);
            }
            Dec8 { src } => {
                let x = self.get_r8(src);
                let (sum, _) = u8::overflowing_sub(x, 1);

                self.inc_pc();

                self.set_r8(src, sum);

                self.assign_flag(Zero, sum == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let (_, half_carry) = x_3_0.overflowing_sub(u4::new(1u8));
                self.assign_flag(HalfCarryBCD, half_carry);
            }
            Dec8IndirectHL => {
                let addr = self.get_r16(HL);
                let x = self.get_mem8(addr);
                let (sum, _) = u8::overflowing_sub(x, 1);

                self.inc_pc();

                self.set_mem8(addr, sum);

                self.assign_flag(Zero, sum == 0);
                self.set_flag(SubBCD);
                let x_3_0 = u4::extract_u8(x, 0);
                let (_, half_carry) = x_3_0.overflowing_sub(u4::new(1u8));
                self.assign_flag(HalfCarryBCD, half_carry);
            }

            And8 { src } => {
                let x = self.get_r8(A);
                let y = self.get_r8(src);
                let res = x & y;

                self.inc_pc();
                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(SubBCD);
                self.set_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            And8IndirectHL => {
                let x = self.get_r8(A);
                let addr = self.get_r16(HL);
                let y = self.get_mem8(addr);
                let res = x & y;

                self.inc_pc();
                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(SubBCD);
                self.set_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            AndImm8 { imm } => {
                let x = self.get_r8(A);
                let y = imm;
                let res = x & y;

                self.adv_pc(0x02);
                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(SubBCD);
                self.set_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            Or8 { src } => {
                let x = self.get_r8(A);
                let y = self.get_r8(src);
                let res = x | y;

                self.inc_pc();
                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            Or8IndirectHL => {
                let x = self.get_r8(A);
                let addr = self.get_r16(HL);
                let y = self.get_mem8(addr);
                let res = x | y;

                self.inc_pc();
                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            OrImm8 { imm } => {
                let x = self.get_r8(A);
                let y = imm;
                let res = x | y;

                self.adv_pc(0x02);
                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            Xor8 { src } => {
                let x = self.get_r8(A);
                let y = self.get_r8(src);
                let res = x ^ y;

                self.inc_pc();
                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            Xor8IndirectHL => {
                let x = self.get_r8(A);
                let addr = self.get_r16(HL);
                let y = self.get_mem8(addr);
                let res = x ^ y;

                self.inc_pc();
                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            XorImm8 { imm } => {
                let x = self.get_r8(A);
                let y = imm;
                let res = x ^ y;

                self.adv_pc(0x02);
                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            CmplCarryFlag => {
                let carry = self.get_flag(Carry);

                self.inc_pc();
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, !carry);
            }

            SetCarryFlag => {
                self.inc_pc();
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.set_flag(Carry);
            }

            DecAdjAcc => {
                let value = self.get_r8(A);

                let sub = self.get_flag(SubBCD);
                let hc = self.get_flag(HalfCarryBCD);
                let cry = self.get_flag(Carry);

                let mut res = value;

                if !sub {
                    // Push 0xA - 0xF => 0x0 - 0x6
                    if (res & 0x0F) > 0x09 {
                        res = res.wrapping_add(0x06);
                    }

                    if res > 0x90 {
                        // Push 0xA0 - 0xF0 => 0x00 -> 0x60
                        res = res.wrapping_add(0x60);
                    }
                }

                if hc {
                    // Honor half - carries/borrows
                    res = if sub {
                        res.wrapping_sub(0x06)
                    } else {
                        res.wrapping_add(0x06)
                    };
                }

                if cry {
                    // Honor carries/borrows
                    res = if sub {
                        res.wrapping_sub(0x60)
                    } else {
                        res.wrapping_add(0x60)
                    }
                }

                self.inc_pc();

                self.set_r8(A, res);

                self.assign_flag(Zero, res == 0);
                self.clear_flag(HalfCarryBCD);
                // Res > 0x99 check here is relevant in sitatuations where the
                // sub flag is set but neither carry nor half carry are set.
                // We've bailed out on the DAA and need to indicate the value left has not been
                // adjusted.
                self.assign_flag(Carry, res > 0x99 || cry);
            }

            CmplAcc => {
                let value = self.get_r8(A);

                self.inc_pc();

                self.set_r8(A, !value);

                self.set_flag(SubBCD);
                self.set_flag(HalfCarryBCD);
            }
            Inc16 { src } => {
                let value = self.get_r16(src);
                let res = value.wrapping_add(1u16);

                self.inc_pc();
                self.set_r16(src, res);
            }

            Dec16 { src } => {
                let value = self.get_r16(src);
                let res = value.wrapping_sub(1u16);

                self.inc_pc();
                self.set_r16(src, res);
            }
            Add16HL { src } => {
                let base = self.get_r16(HL);
                let value = self.get_r16(src);
                let (res, carry) = base.overflowing_add(value);
                self.inc_pc();

                self.set_r16(HL, res);

                let base_11_0 = u12::extract_u16(base, 0);
                let src_11_0 = u12::extract_u16(value, 0);
                let (_, half_carry) = base_11_0.overflowing_add(src_11_0);

                self.assign_flag(Carry, carry);
                self.assign_flag(HalfCarryBCD, half_carry);
            }

            AddSPImm8 { imm } => {
                let sp = self.get_r16(SP);
                let (res, _) = sp.overflowing_add_signed(imm as i16);

                self.adv_pc(0x02);
                self.set_r16(SP, res);

                let sp_7_0: u8 = sp.to_be_bytes()[1];
                let imm_u: u8 = imm.cast_unsigned();

                let (_, carry) = sp_7_0.overflowing_add(imm_u);

                // From my understanding, the game boy performs
                // this instruction by performing an unsigned add in the low 8 bits,
                // then adjusting the upper 8 bits based on the carry + sign from lower 8 bits.
                // Its real weird.
                // What this does mean though is that the half carry flag is therefore going to be
                // set as though an unsigned addition had occured, so we emulate that here.
                let sp_3_0 = u4::extract_u16(sp, 0);
                let imm_3_0 = u4::extract_u8(imm.cast_unsigned(), 0);

                let (_, half_carry) = sp_3_0.overflowing_add(imm_3_0);

                self.clear_flag(Zero);
                self.clear_flag(SubBCD);
                self.assign_flag(HalfCarryBCD, half_carry);
                self.assign_flag(Carry, carry);
            }

            RLCA => {
                let value = self.get_r8(A);
                let res = u8::rotate_left(value, 1);

                self.inc_pc();
                self.set_r8(A, res);

                self.clear_flag(Zero);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, u1::extract_u8(res, 0).into());
            }

            RRCA => {
                let value = self.get_r8(A);
                let res = u8::rotate_right(value, 1);

                self.inc_pc();
                self.set_r8(A, res);

                self.clear_flag(Zero);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, u1::extract_u8(value, 0).into());
            }

            RLA => {
                let from = self.get_r8(A);
                let fc = self.get_flag(Carry);

                let res = (from << 1) | (if fc { 0x01 } else { 0x00 });
                let tc = (from & 0x80) == 0x80;

                self.inc_pc();
                self.set_r8(A, res);

                self.clear_flag(Zero);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, tc);
            }

            RRA => {
                let from = self.get_r8(A);
                let fc = self.get_flag(Carry);

                let res = (from >> 1) | (if fc { 0x80 } else { 0x00 });
                let tc = (from & 0x01) == 0x01;

                self.inc_pc();
                self.set_r8(A, res);

                self.clear_flag(Zero);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, tc);
            }

            RLC { src } => {
                let from = self.get_r8(src);
                let res = u8::rotate_left(from, 1);
                let carry = (from & 0x80) == 0x80;

                self.adv_pc(2);

                self.set_r8(src, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            RLCIndirectHL => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);
                let res = u8::rotate_left(from, 1);
                let carry = (from & 0x80) == 0x80;

                self.adv_pc(2);

                self.set_mem8(addr, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            RRC { src } => {
                let from = self.get_r8(src);
                let res = u8::rotate_right(from, 1);
                let carry = (from & 0x01) == 0x01;

                self.adv_pc(2);

                self.set_r8(src, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            RRCIndirectHL => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);
                let res = u8::rotate_right(from, 1);
                let carry = (from & 0x01) == 0x01;

                self.adv_pc(2);

                self.set_mem8(addr, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            RL { src } => {
                let from = self.get_r8(src);
                let fc = self.get_flag(Carry);

                let res = (from << 1) | (if fc { 0x01 } else { 0x00 });
                let carry = (from & 0x80) == 0x80;

                self.adv_pc(2);

                self.set_r8(src, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            RLIndirectHL => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);

                let fc = self.get_flag(Carry);

                let res = (from << 1) | (if fc { 0x01 } else { 0x00 });
                let carry = (from & 0x80) == 0x80;

                self.adv_pc(2);

                self.set_mem8(addr, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            RR { src } => {
                let from = self.get_r8(src);
                let fc = self.get_flag(Carry);

                let res = (from >> 1) | (if fc { 0x80 } else { 0x00 });
                let carry = (from & 0x01) == 0x01;

                self.adv_pc(2);

                self.set_r8(src, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            RRIndirectHL => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);
                let fc = self.get_flag(Carry);

                let res = (from >> 1) | (if fc { 0x80 } else { 0x00 });
                let carry = (from & 0x01) == 0x01;

                self.adv_pc(2);

                self.set_mem8(addr, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            SLA { src } => {
                let from = self.get_r8(src);
                let res = from << 1;
                let carry = (from & 0x80) == 0x80;

                self.adv_pc(2);

                self.set_r8(src, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            SLAIndirectHL => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);
                let res = from << 1;
                let carry = (from & 0x80) == 0x80;

                self.adv_pc(2);

                self.set_mem8(addr, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            SRA { src } => {
                let from = self.get_r8(src);
                let res = (from >> 1) | (if from & 0x80 == 0x80 { 0x80 } else { 0x00 });
                let carry = (from & 0x01) == 0x01;

                self.adv_pc(2);

                self.set_r8(src, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            SRAIndirectHL => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);
                let res = (from >> 1) | (if from & 0x80 == 0x80 { 0x80 } else { 0x00 });
                let carry = (from & 0x01) == 0x01;

                self.adv_pc(2);

                self.set_mem8(addr, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            Swap { src } => {
                let from = self.get_r8(src);
                let from_3_0 = u4::extract_u8(from, 0);
                let from_7_4 = u4::extract_u8(from, 4);

                let res = (from_3_0.value() << 4) | (from_7_4.value());

                self.adv_pc(0x02);
                self.set_r8(src, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            SwapIndirectHL => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);
                let from_3_0 = u4::extract_u8(from, 0);
                let from_7_4 = u4::extract_u8(from, 4);

                let res = (from_3_0.value() << 4) | (from_7_4.value());

                self.adv_pc(0x02);
                self.set_mem8(addr, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.clear_flag(Carry);
            }

            SRL { src } => {
                let from = self.get_r8(src);
                let res = (from >> 1);
                let carry = (from & 0x01) == 0x01;

                self.adv_pc(2);

                self.set_r8(src, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            SRLIndirectHL => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);
                let res = (from >> 1);
                let carry = (from & 0x01) == 0x01;

                self.adv_pc(2);

                self.set_mem8(addr, res);

                self.assign_flag(Zero, res == 0x00);
                self.clear_flag(SubBCD);
                self.clear_flag(HalfCarryBCD);
                self.assign_flag(Carry, carry);
            }

            BIT { src, bit } => {
                let from = self.get_r8(src);

                self.adv_pc(2);

                let mask = 0x01 << bit;
                let bit_set = from & mask == mask;
                self.assign_flag(Zero, bit_set);
                self.clear_flag(SubBCD);
                self.set_flag(HalfCarryBCD);
            }

            BITIndirectHL { bit } => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);

                self.adv_pc(2);

                let mask = 0x01 << bit;
                let bit_set = from & mask == mask;
                self.assign_flag(Zero, bit_set);
                self.clear_flag(SubBCD);
                self.set_flag(HalfCarryBCD);
            }

            RES { src, bit } => {
                let from = self.get_r8(src);
                let mask: u8 = !(0x01 << bit);

                let res = from & mask;

                self.adv_pc(2);

                self.set_r8(src, res);

                // No flags affected.
            }

            RESIndirectHL { bit } => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);

                let mask: u8 = !(0x01 << bit);

                let res = from & mask;

                self.adv_pc(2);
                self.set_mem8(addr, res);

                // No flags affected.
            }

            SET { src, bit } => {
                let from = self.get_r8(src);
                let mask: u8 = 0x01 << bit;

                let res = from | mask;

                self.adv_pc(2);

                self.set_r8(src, res);

                // No flags affected.
            }

            SETIndirectHL { bit } => {
                let addr = self.get_r16(HL);
                let from = self.get_mem8(addr);

                let mask: u8 = (0x01 << bit);

                let res = from | mask;

                self.adv_pc(2);
                self.set_mem8(addr, res);

                // No flags affected.
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
    fn it_decodes_load_imm_16(#[values(BC, DE, HL, SP)] dest: R16) {
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
    fn it_decodes_push(#[values(BC, DE, HL, AF)] src: R16) {
        // 0b11_xx_0101
        let opcode = 0b11_00_0101 + (<R16 as Into<u8>>::into(src) << 4);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Push");

        assert_eq!(decoded, Instruction::Push { src });
    }

    #[rstest]
    fn it_decodes_pop(#[values(BC, DE, HL, AF)] dest: R16) {
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
    fn it_decodes_inc16(#[values(BC, DE, HL, SP)] reg_pair: R16) {
        //0b00_xx_0011
        let opcode = 0b00_00_0011 + (<R16 as Into<u8>>::into(reg_pair) << 4);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Inc16");
        assert_eq!(decoded, Instruction::Inc16 { src: reg_pair });
    }

    #[rstest]
    fn it_decodes_dec16(#[values(BC, DE, HL, SP)] reg_pair: R16) {
        //0b00_xx_1011
        let opcode = 0b00_00_1011 + (<R16 as Into<u8>>::into(reg_pair) << 4);
        let memory = [opcode];
        let decoded = Instruction::decode(&memory).expect("Dec16");
        assert_eq!(decoded, Instruction::Dec16 { src: reg_pair });
    }

    #[rstest]
    fn it_decodes_add16_hl(#[values(BC, DE, HL, SP)] reg_pair: R16) {
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
    #[case(BC, B, C)]
    #[case(DE, D, E)]
    #[case(HL, H, L)]
    #[case(AF, A, F)]
    fn it_sets_reg_pairs(#[case] pair: R16, #[case] h_reg: R8, #[case] l_reg: R8) {
        let mut machine = Machine::new();

        machine.set_r16(pair, 0xABCD);

        assert_eq!(machine.get_r8(h_reg), 0xAB);
        assert_eq!(machine.get_r8(l_reg), 0xCD);
    }

    #[rstest]
    #[case(BC, B, C)]
    #[case(DE, D, E)]
    #[case(HL, H, L)]
    #[case(AF, A, F)]
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
        machine.set_r16(HL, 0x0001);

        machine.set_memory(1, &[0xFE]);

        let ins = Instruction::LoadIndirectHL { dest };
        machine.exec(ins);

        assert_eq!(0xFE, machine.get_r8(dest));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[rstest]
    fn it_execs_store_indirect_hl_8(#[values(B, C, D, E, A)] src: R8) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(src, 0xFE);
        machine.set_r16(HL, 0x0001);

        machine.set_memory(1, &[0x00]);

        let ins = Instruction::StoreIndirectHL { src };
        machine.exec(ins);

        assert_eq!(0xFE, machine.get_mem8(0x0001));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[rstest]
    fn it_execs_store_imm_indirect_hl_8(#[values(0x01, 0xFF)] imm: u8) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r16(HL, 0x0001);

        machine.set_memory(1, &[0x00]);

        let ins = Instruction::StoreImmIndirectHL { imm };
        machine.exec(ins);

        assert_eq!(imm, machine.get_mem8(0x0001));
        assert_eq!(machine.get_pc(), 0x02);
    }
    #[test]
    fn it_execs_load_acc_indirect_bc() {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, 0x0);
        machine.set_r16(BC, 0x0001);

        machine.set_memory(1, &[value]);

        let ins = Instruction::LoadAccIndirectBC;
        machine.exec(ins);

        assert_eq!(value, machine.get_r8(A));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[test]
    fn it_execs_load_acc_indirect_de() {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, 0x0);
        machine.set_r16(DE, 0x0001);

        machine.set_memory(1, &[value]);

        let ins = Instruction::LoadAccIndirectDE;
        machine.exec(ins);

        assert_eq!(value, machine.get_r8(A));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[test]
    fn it_execs_store_acc_indirect_bc() {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, value);
        machine.set_r16(BC, 0x0001);
        machine.set_memory(1, &[0x00]);

        let ins = Instruction::StoreAccIndirectBC;
        machine.exec(ins);

        assert_eq!(value, machine.get_mem8(0x0001));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[test]
    fn it_execs_store_acc_indirect_de() {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, value);
        machine.set_r16(DE, 0x0001);
        machine.set_memory(1, &[0x00]);

        let ins = Instruction::StoreAccIndirectDE;
        machine.exec(ins);

        assert_eq!(value, machine.get_mem8(0x0001));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[rstest]
    fn it_execs_load_acc_16(#[values(0x0001, 0xFFFF)] addr: u16) {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_memory(addr as usize, &[value]);

        let ins = Instruction::LoadAcc16 { addr };
        machine.exec(ins);

        assert_eq!(value, machine.get_r8(A));
        assert_eq!(machine.get_pc(), 0x03);
    }
    #[rstest]
    fn it_execs_store_acc_16(#[values(0x0001, 0xFFFF)] addr: u16) {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, value);
        machine.set_memory(addr as usize, &[0x00]);

        let ins = Instruction::StoreAcc16 { addr };
        machine.exec(ins);

        assert_eq!(value, machine.get_mem8(addr));
        assert_eq!(machine.get_pc(), 0x03);
    }
    #[rstest]
    fn it_execs_load_acc_indirect_c(#[values(0x01, 0xFF)] offset: u8) {
        let mut machine = Machine::new();

        let base: u16 = 0xFF00;
        let value = 0xFE;

        machine.set_pc(0x00);

        machine.set_r8(A, 0x00);
        machine.set_r8(C, offset);

        let addr: u16 = base + (offset as u16);
        machine.set_memory(addr as usize, &[value]);

        let ins = Instruction::LoadAccIndirectC;
        machine.exec(ins);

        assert_eq!(value, machine.get_r8(A));
        assert_eq!(machine.get_pc(), 0x01);
    }
    #[rstest]
    fn it_execs_store_acc_indirect_c(#[values(0x01, 0xFF)] offset: u8) {
        let mut machine = Machine::new();

        let base: u16 = 0xFF00;
        let value = 0xFE;

        machine.set_pc(0x00);

        machine.set_r8(A, value);
        machine.set_r8(C, offset);

        let addr: u16 = base + (offset as u16);
        machine.set_memory(addr as usize, &[0x00]);

        let ins = Instruction::StoreAccIndirectC;
        machine.exec(ins);

        assert_eq!(value, machine.get_mem8(addr));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[rstest]
    fn it_execs_load_acc_direct_8(#[values(0x01, 0xFF)] offset: u8) {
        let mut machine = Machine::new();

        let base: u16 = 0xFF00;
        let value = 0xFE;

        machine.set_pc(0x00);

        let addr: u16 = base + (offset as u16);
        machine.set_memory(addr as usize, &[value]);

        let ins = Instruction::LoadAccDirect8 { offset };
        machine.exec(ins);

        assert_eq!(value, machine.get_r8(A));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[rstest]
    fn it_execs_store_acc_direct_8(#[values(0x01, 0xFF)] offset: u8) {
        let mut machine = Machine::new();

        let base: u16 = 0xFF00;
        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, value);

        let addr: u16 = base + (offset as u16);
        machine.set_memory(addr as usize, &[0x00]);

        let ins = Instruction::StoreAccDirect8 { offset };
        machine.exec(ins);

        assert_eq!(value, machine.get_mem8(addr));
        assert_eq!(machine.get_pc(), 0x01);
    }

    #[test]
    fn it_execs_load_acc_indirect_hl_dec() {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, 0x0);
        machine.set_r16(HL, 0x0001);

        machine.set_memory(1, &[value]);

        let ins = Instruction::LoadAccIndirectHLDec;
        machine.exec(ins);

        assert_eq!(value, machine.get_r8(A));
        assert_eq!(machine.get_pc(), 0x01);
        assert_eq!(0x00, machine.get_r16(HL));
    }

    #[test]
    fn it_execs_store_acc_indirect_dec() {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, value);
        machine.set_r16(HL, 0x0001);
        machine.set_memory(1, &[0x00]);

        let ins = Instruction::StoreAccIndirectHLDec;
        machine.exec(ins);

        assert_eq!(value, machine.get_mem8(0x0001));
        assert_eq!(machine.get_pc(), 0x01);
        assert_eq!(0x00, machine.get_r16(HL));
    }
    #[test]
    fn it_execs_load_acc_indirect_hl_inc() {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, 0x0);
        machine.set_r16(HL, 0x0001);

        machine.set_memory(1, &[value]);

        let ins = Instruction::LoadAccIndirectHLInc;
        machine.exec(ins);

        assert_eq!(value, machine.get_r8(A));
        assert_eq!(machine.get_pc(), 0x01);
        assert_eq!(0x02, machine.get_r16(HL));
    }

    #[test]
    fn it_execs_store_acc_indirect_inc() {
        let mut machine = Machine::new();

        let value = 0xFE;

        machine.set_pc(0x00);
        machine.set_r8(A, value);
        machine.set_r16(HL, 0x0001);
        machine.set_memory(1, &[0x00]);

        let ins = Instruction::StoreAccIndirectHLInc;
        machine.exec(ins);

        assert_eq!(value, machine.get_mem8(0x0001));
        assert_eq!(machine.get_pc(), 0x01);
        assert_eq!(0x02, machine.get_r16(HL));
    }

    #[rstest]
    fn it_execs_load_imm_16(
        #[values(BC, DE, HL, SP)] dest: R16,
        #[values(0x0001, 0x000F)] imm: u16,
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r16(dest, 0x00);

        let ins = Instruction::LoadImm16 { dest, imm };
        machine.exec(ins);

        assert_eq!(imm, machine.get_r16(dest));
        assert_eq!(0x03, machine.get_pc());
    }

    #[rstest]
    fn it_execs_store_sp_16(#[values(0x0000, 0xFFFE)] addr: u16) {
        let mut machine = Machine::new();
        let value = 0xFEEF;
        machine.set_pc(0x00);
        machine.set_r16(SP, value);

        let ins = Instruction::StoreSP16 { addr };
        machine.exec(ins);

        assert_eq!(value, machine.get_mem16(addr));
        assert_eq!(0x03, machine.get_pc());
    }

    #[rstest]
    fn it_execs_load_sp_hl(#[values(0x0001, 0xFFFF)] value: u16) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r16(HL, value);
        machine.set_r16(SP, 0x00);

        let ins = Instruction::LoadSPHL;
        machine.exec(ins);

        assert_eq!(value, machine.get_r16(SP));
        assert_eq!(0x01, machine.get_pc());
    }

    #[rstest]
    fn it_execs_push(#[values(BC, DE, HL, AF)] src: R16) {
        let mut machine = Machine::new();
        let value = 0xFEEF;

        machine.set_pc(0x00);
        machine.set_r16(src, value);
        machine.set_sp(0xFFFF);

        let ins = Instruction::Push { src };
        machine.exec(ins);

        let new_sp = machine.get_r16(SP);
        assert_eq!(0xFFFD, new_sp);
        assert_eq!(value, machine.get_mem16(new_sp + 1));
        assert_eq!(0x01, machine.get_pc());
    }
    #[rstest]
    fn it_execs_pop(#[values(BC, DE, HL, AF)] dest: R16) {
        let mut machine = Machine::new();
        let value = 0xFEEF;

        machine.set_pc(0x00);
        machine.set_sp(0xFFFD);
        machine.set_mem16(0xFFFE, value);

        let ins = Instruction::Pop { dest };
        machine.exec(ins);

        let new_sp = machine.get_r16(SP);
        assert_eq!(0xFFFF, new_sp);
        assert_eq!(value, machine.get_mem16(new_sp - 1));
        assert_eq!(0x01, machine.get_pc());
    }

    #[rstest]
    #[case(0xFFFF, 0x00, (false, false, false, false))]
    // LD HL,SP+e always sets zero flag = 0
    #[case(0xFFFF, 0x01, (false, false, true, true))]
    #[case(0xFFFF, 0x02, (false, false, true, true))]
    // Half carry check
    #[case(0x000F, 0x0F, (false, false, true, false))]
    #[case(0x00F0, 0x10, (false, false, false, true))]
    fn it_execs_load_hlspoffset(
        #[case] sp: u16,
        #[case] offset: i8,
        #[case] (zero, sub_bcd, hc_bcd, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();

        machine.set_r16(SP, sp);
        machine.set_r16(HL, 0);
        machine.set_pc(0x00);

        let ins = Instruction::LoadHLSPOffset { offset };
        machine.exec(ins);

        let result: i32 = ((sp as i32) + (offset as i32)) % (0x1_0000);
        assert_eq!(0x02, machine.get_pc());
        assert_eq!(result as u16, machine.get_r16(HL));
        assert_eq!(
            zero,
            machine.get_flag(Zero),
            "Zero flag is {}, should be {}",
            machine.get_flag(Zero),
            zero
        );
        assert_eq!(
            sub_bcd,
            machine.get_flag(SubBCD),
            "Sub BCD flag is {}, should be {}",
            machine.get_flag(SubBCD),
            sub_bcd
        );
        assert_eq!(
            hc_bcd,
            machine.get_flag(HalfCarryBCD),
            "Half carry flag is {}, should be {}",
            machine.get_flag(HalfCarryBCD),
            hc_bcd
        );
        assert_eq!(
            carry,
            machine.get_flag(Carry),
            "Carry flag is {}, should be {}",
            machine.get_flag(Carry),
            carry
        );
    }

    // ADD R8, A
    #[rstest]
    #[case(0x00, 0x00, 0x00, (true, false, false, false))]
    #[case(0x00, 0x01, 0x01, (false, false, false, false))]
    #[case(0xFF, 0x01, 0x00, (true, false, true, true))]
    #[case(0xFF, 0xFF, 0xFE,(false, false, true, true))]
    #[case(0x0F, 0x0F, 0x1E,(false, false, true, false))]
    fn it_execs_add_8(
        #[values(B, C, D, E, H, L)] src: R8,
        #[case] x: u8,
        #[case] y: u8,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, y);
        machine.set_r8(A, x);

        let ins = Instruction::Add8 { src };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    // Special set of tests for Add A,A
    #[rstest]
    #[case(0x00, 0x00, (true, false, false, false))]
    #[case(0x01, 0x02, (false, false, false, false))]
    #[case(0xFF, 0xFE,  (false, false, true, true))]
    #[case(0x0F, 0x1E, (false, false, true, false))]
    #[case(0xF0, 0xE0, (false, false, false, true))]
    fn it_execs_add_8_a(
        #[case] a: u8,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, a);

        let ins = Instruction::Add8 { src: A };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00, (true, false, false, false))]
    #[case(0x00, 0x01, 0x01, (false, false, false, false))]
    #[case(0xFF, 0x01, 0x00, (true, false, true, true))]
    #[case(0xFF, 0xFF, 0xFE,(false, false, true, true))]
    #[case(0x0F, 0x0F, 0x1E,(false, false, true, false))]
    fn it_execs_add8_indirect_hl(
        #[case] x: u8,
        #[case] y: u8,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let addr = 0x00FF;
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, y);

        let ins = Instruction::Add8IndirectHL;
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00, (true, false, false, false))]
    #[case(0x00, 0x01, 0x01, (false, false, false, false))]
    #[case(0xFF, 0x01, 0x00, (true, false, true, true))]
    #[case(0xFF, 0xFF, 0xFE,(false, false, true, true))]
    #[case(0x0F, 0x0F, 0x1E,(false, false, true, false))]
    fn it_execs_add_imm8(
        #[case] x: u8,
        #[case] y: u8,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let ins = Instruction::AddImm8 { imm: y };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x02, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }
    // ADDC R8, A
    #[rstest]
    #[case(0x00, 0x00, false, 0x00, (true, false, false, false))]
    #[case(0x00, 0x00, true, 0x01, (false, false, false, false))]
    #[case(0x00, 0x01, false, 0x01, (false, false, false, false))]
    #[case(0x00, 0x01, true, 0x02, (false, false, false, false))]
    #[case(0xFF, 0x00, true, 0x00, (true, false, true, true))]
    #[case(0xFF, 0x01, false, 0x00, (true, false, true, true))]
    #[case(0xFF, 0xFF, false, 0xFE,(false, false, true, true))]
    #[case(0xFF, 0xFF, true, 0xFF,(false, false, true, true))]
    #[case(0x0F, 0x0F, false, 0x1E,(false, false, true, false))]
    #[case(0x0F, 0x0F, true, 0x1F,(false, false, true, false))]
    #[case(0x0F, 0x00, true, 0x10,(false, false, true, false))]
    fn it_execs_addc_8(
        #[values(B, C, D, E, H, L)] src: R8,
        #[case] x: u8,
        #[case] y: u8,
        #[case] cry: bool,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, y);
        machine.set_r8(A, x);
        machine.set_r8(F, 0);
        machine.assign_flag(Carry, cry);

        let ins = Instruction::AddC8 { src };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    // Special set of tests for Add A,A
    #[rstest]
    #[case(0x00, 0x00, false, (true, false, false, false))]
    #[case(0x00, 0x01, true, (false, false, false, false))]
    #[case(0x01, 0x02, false, (false, false, false, false))]
    #[case(0x01, 0x03, true, (false, false, false, false))]
    #[case(0xFF, 0xFE, false,  (false, false, true, true))]
    #[case(0xFF, 0xFF, true, (false, false, true, true))]
    #[case(0x0F, 0x1E, false, (false, false, true, false))]
    #[case(0x0F, 0x1F, true, (false, false, true, false))]
    #[case(0xF0, 0xE0, false, (false, false, false, true))]
    #[case(0xF0, 0xE1, true, (false, false, false, true))]
    fn it_execs_addc_8_a(
        #[case] a: u8,
        #[case] sum: u8,
        #[case] cry: bool,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, a);
        machine.set_r8(F, 0);
        machine.assign_flag(Carry, cry);

        let ins = Instruction::AddC8 { src: A };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, false, 0x00, (true, false, false, false))]
    #[case(0x00, 0x00, true, 0x01, (false, false, false, false))]
    #[case(0x00, 0x01, false, 0x01, (false, false, false, false))]
    #[case(0x00, 0x01, true, 0x02, (false, false, false, false))]
    #[case(0xFF, 0x01, false, 0x00, (true, false, true, true))]
    #[case(0xFF, 0x01, true, 0x01, (false, false, true, true))]
    #[case(0xFF, 0xFF, false, 0xFE,(false, false, true, true))]
    #[case(0xFF, 0xFF, true, 0xFF,(false, false, true, true))]
    #[case(0x0F, 0x0F, false, 0x1E,(false, false, true, false))]
    #[case(0x0F, 0x0F, true, 0x1F,(false, false, true, false))]
    #[case(0x0F, 0x00, true, 0x10,(false, false, true, false))]
    #[case(0xFF, 0x00, true, 0x00,(true, false, true, true))]
    fn it_execs_addc8_indirect_hl(
        #[case] x: u8,
        #[case] y: u8,
        #[case] cry: bool,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);
        machine.set_r8(F, 0);
        machine.assign_flag(Carry, cry);

        let addr = 0x00FF;
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, y);

        let ins = Instruction::AddC8IndirectHL;
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, false, 0x00, (true, false, false, false))]
    #[case(0x00, 0x00, true, 0x01, (false, false, false, false))]
    #[case(0x00, 0x01, false, 0x01, (false, false, false, false))]
    #[case(0x00, 0x01, true, 0x02, (false, false, false, false))]
    #[case(0xFF, 0x01, false, 0x00, (true, false, true, true))]
    #[case(0xFF, 0x01, true, 0x01, (false, false, true, true))]
    #[case(0xFF, 0xFF, false, 0xFE,(false, false, true, true))]
    #[case(0xFF, 0xFF, true, 0xFF,(false, false, true, true))]
    #[case(0x0F, 0x0F, false, 0x1E,(false, false, true, false))]
    #[case(0x0F, 0x0F, true, 0x1F,(false, false, true, false))]
    #[case(0x0F, 0x00, true, 0x10,(false, false, true, false))]
    #[case(0xFF, 0x00, true, 0x00,(true, false, true, true))]
    fn it_execs_addc_imm8(
        #[case] x: u8,
        #[case] y: u8,
        #[case] cry: bool,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);
        machine.set_r8(F, 0);
        machine.assign_flag(Carry, cry);

        let ins = Instruction::AddCImm8 { imm: y };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x02, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    // SUB R8, A
    #[rstest]
    #[case(0x00, 0x00, 0x00, (true, true, false, false))]
    #[case(0x00, 0x01, 0xFF, (false, true, true, true))]
    #[case(0x01, 0x01, 0x00, (true, true, false, false))]
    #[case(0xFF, 0xFF, 0x00, (true, true, false, false))]
    #[case(0x10, 0x01, 0x0F, (false, true, true, false))]
    fn it_execs_sub_8(
        #[values(B, C, D, E, H, L)] src: R8,
        #[case] x: u8,
        #[case] y: u8,
        #[case] res: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, y);
        machine.set_r8(A, x);

        let ins = Instruction::Sub8 { src };
        machine.exec(ins);

        assert_eq!(res, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    // Special set of tests for Sub A,A
    #[rstest]
    #[case(0x00,  (true, true, false, false))]
    #[case(0x01, (true, true, false, false))]
    #[case(0xFF,   (true, true, false, false))]
    fn it_execs_sub_8_a(
        #[case] a: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, a);

        let ins = Instruction::Sub8 { src: A };
        machine.exec(ins);

        assert_eq!(0, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00, (true, true, false, false))]
    #[case(0x00, 0x01, 0xFF, (false, true, true, true))]
    #[case(0x01, 0x01, 0x00, (true, true, false, false))]
    #[case(0xFF, 0xFF, 0x00, (true, true, false, false))]
    fn it_execs_sub8_indirect_hl(
        #[case] x: u8,
        #[case] y: u8,
        #[case] res: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let addr = 0x00FF;
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, y);

        let ins = Instruction::Sub8IndirectHL;
        machine.exec(ins);

        assert_eq!(res, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00, (true, true, false, false))]
    #[case(0x00, 0x01, 0xFF, (false, true, true, true))]
    #[case(0x01, 0x01, 0x00, (true, true, false, false))]
    #[case(0xFF, 0xFF, 0x00, (true, true, false, false))]
    #[case(0x10, 0x01, 0x0F, (false, true, true, false))]
    fn it_execs_sub_imm8(
        #[case] x: u8,
        #[case] y: u8,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let ins = Instruction::SubImm8 { imm: y };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x02, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }
    // SUBC R8, A
    #[rstest]
    #[case(0x00, 0x00, false, 0x00, (true, true, false, false))]
    #[case(0x00, 0x00, true, 0xFF, (false, true, true, true))]
    #[case(0x00, 0x01, false, 0xFF, (false, true, true, true))]
    #[case(0x00, 0x01, true, 0xFE, (false, true, true, true))]
    #[case(0x01, 0x01, false, 0x00, (true, true, false, false))]
    #[case(0x01, 0x01, true, 0xFF, (false, true, true, true))]
    #[case(0xFF, 0xFF, false, 0x00, (true, true, false, false))]
    #[case(0xFF, 0xFF, true, 0xFF, (false, true, true, true))]
    #[case(0x0F, 0xFF, false, 0x10, (false, true, false, true))]
    #[case(0x10, 0xF0, false, 0x20, (false, true, false, true))]
    #[case(0x10, 0xF0, true, 0x1F, (false, true, true, true))]
    #[case(0x10, 0x01, false, 0x0F, (false, true, true, false))]
    #[case(0x10, 0x00, true, 0x0F, (false, true, true, false))]
    fn it_execs_subc_8(
        #[values(B, C, D, E, H, L)] src: R8,
        #[case] x: u8,
        #[case] y: u8,
        #[case] cry: bool,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, y);
        machine.set_r8(A, x);
        machine.set_r8(F, 0);
        machine.assign_flag(Carry, cry);

        let ins = Instruction::SubC8 { src };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    // Special set of tests for SubC A,A
    #[rstest]
    #[case(0x00, false, 0x00, (true, true, false, false))]
    #[case(0x00, true, 0xFF, (false, true, true, true))]
    #[case(0x01, false, 0x00, (true, true, false, false))]
    #[case(0x01, true, 0xFF, (false, true, true, true))]
    #[case(0xFF, false, 0x00, (true, true, false, false))]
    #[case(0xFF, true, 0xFF, (false, true, true, true))]
    fn it_execs_subc_8_a(
        #[case] a: u8,
        #[case] cry: bool,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, a);
        machine.set_r8(F, 0);
        machine.assign_flag(Carry, cry);

        let ins = Instruction::SubC8 { src: A };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, false, 0x00, (true, true, false, false))]
    #[case(0x00, 0x00, true, 0xFF, (false, true, true, true))]
    #[case(0x00, 0x01, false, 0xFF, (false, true, true, true))]
    #[case(0x00, 0x01, true, 0xFE, (false, true, true, true))]
    #[case(0x01, 0x01, false, 0x00, (true, true, false, false))]
    #[case(0x01, 0x01, true, 0xFF, (false, true, true, true))]
    #[case(0xFF, 0xFF, false, 0x00, (true, true, false, false))]
    #[case(0xFF, 0xFF, true, 0xFF, (false, true, true, true))]
    #[case(0x10, 0xF0, false, 0x20, (false, true, false, true))]
    #[case(0x10, 0xF0, true, 0x1F, (false, true, true, true))]
    fn it_execs_subc8_indirect_hl(
        #[case] x: u8,
        #[case] y: u8,
        #[case] cry: bool,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);
        machine.set_r8(F, 0);
        machine.assign_flag(Carry, cry);

        let addr = 0x00FF;
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, y);

        let ins = Instruction::SubC8IndirectHL;
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, false, 0x00, (true, true, false, false))]
    #[case(0x00, 0x00, true, 0xFF, (false, true, true, true))]
    #[case(0x00, 0x01, false, 0xFF, (false, true, true, true))]
    #[case(0x00, 0x01, true, 0xFE, (false, true, true, true))]
    #[case(0x01, 0x01, false, 0x00, (true, true, false, false))]
    #[case(0x01, 0x01, true, 0xFF, (false, true, true, true))]
    #[case(0xFF, 0xFF, false, 0x00, (true, true, false, false))]
    #[case(0xFF, 0xFF, true, 0xFF, (false, true, true, true))]
    #[case(0x10, 0xF0, false, 0x20, (false, true, false, true))]
    #[case(0x10, 0xF0, true, 0x1F, (false, true, true, true))]
    fn it_execs_subc8_imm8(
        #[case] x: u8,
        #[case] y: u8,
        #[case] cry: bool,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);
        machine.set_r8(F, 0);
        machine.assign_flag(Carry, cry);

        let ins = Instruction::SubCImm8 { imm: y };
        machine.exec(ins);

        assert_eq!(sum, machine.get_r8(A));
        assert_eq!(0x02, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    // CMP R8, A
    #[rstest]
    #[case(0x00, 0x00, 0x00, (true, true, false, false))]
    #[case(0x00, 0x01, 0xFF, (false, true, true, true))]
    #[case(0x01, 0x01, 0x00, (true, true, false, false))]
    #[case(0xFF, 0xFF, 0x00, (true, true, false, false))]
    #[case(0x10, 0x01, 0x0F, (false, true, true, false))]
    fn it_execs_cp_8(
        #[values(B, C, D, E, H, L)] src: R8,
        #[case] x: u8,
        #[case] y: u8,
        #[case] res: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, y);
        machine.set_r8(A, x);

        let ins = Instruction::Cmp8 { src };
        machine.exec(ins);

        assert_eq!(x, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    // Special set of tests for CMP A,A
    #[rstest]
    #[case(0x00,  (true, true, false, false))]
    #[case(0x01, (true, true, false, false))]
    #[case(0xFF,   (true, true, false, false))]
    fn it_execs_cp_8_a(
        #[case] a: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, a);

        let ins = Instruction::Cmp8 { src: A };
        machine.exec(ins);

        assert_eq!(a, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00, (true, true, false, false))]
    #[case(0x00, 0x01, 0xFF, (false, true, true, true))]
    #[case(0x01, 0x01, 0x00, (true, true, false, false))]
    #[case(0xFF, 0xFF, 0x00, (true, true, false, false))]
    fn it_execs_cmp8_indirect_hl(
        #[case] x: u8,
        #[case] y: u8,
        #[case] res: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let addr = 0x00FF;
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, y);

        let ins = Instruction::Cmp8IndirectHL;
        machine.exec(ins);

        assert_eq!(x, machine.get_r8(A));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00, (true, true, false, false))]
    #[case(0x00, 0x01, 0xFF, (false, true, true, true))]
    #[case(0x01, 0x01, 0x00, (true, true, false, false))]
    #[case(0xFF, 0xFF, 0x00, (true, true, false, false))]
    #[case(0x10, 0x01, 0x0F, (false, true, true, false))]
    fn it_execs_cmp_imm8(
        #[case] x: u8,
        #[case] y: u8,
        #[case] sum: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let ins = Instruction::CmpImm8 { imm: y };
        machine.exec(ins);

        assert_eq!(x, machine.get_r8(A));
        assert_eq!(0x02, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x01, (false, false, false, false))]
    #[case(0xF, 0x10, (false, false, true, false))]
    // NB INC does not affect carry flag
    #[case(0xFF, 0x00, (true, false, true, false))]
    fn it_execs_inc8(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] from: u8,
        #[case] to: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, from);

        let ins = Instruction::Inc8 { src };
        machine.exec(ins);

        assert_eq!(to, machine.get_r8(src));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x01, (false, false, false, false))]
    #[case(0xF, 0x10, (false, false, true, false))]
    // NB INC does not affect carry flag
    #[case(0xFF, 0x00, (true, false, true, false))]
    fn it_execs_inc8_indirect_hl(
        #[case] from: u8,
        #[case] to: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);

        machine.set_r16(HL, 0x0001);
        machine.set_mem8(0x0001, from);

        let ins = Instruction::Inc8IndirectHL;
        machine.exec(ins);

        assert_eq!(to, machine.get_mem8(0x0001));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    // NB Dec does not affect carry flag
    #[case(0x01, 0x00, (true, true, false, false))]
    #[case(0x00, 0xFF, (false, true, true, false))]
    #[case(0x10, 0x0F, (false, true, true, false))]
    #[case(0x02, 0x01, (false, true, false, false))]
    fn it_execs_dec8(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] from: u8,
        #[case] to: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, from);

        let ins = Instruction::Dec8 { src };
        machine.exec(ins);

        assert_eq!(to, machine.get_r8(src));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    // NB Dec does not affect carry flag.
    #[case(0x01, 0x00, (true, true, false, false))]
    #[case(0x00, 0xFF, (false, true, true, false))]
    #[case(0x10, 0x0F, (false, true, true, false))]
    #[case(0x02, 0x01, (false, true, false, false))]
    fn it_execs_dec8_indirect_hl(
        #[case] from: u8,
        #[case] to: u8,
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);

        machine.set_r16(HL, 0x0001);
        machine.set_mem8(0x0001, from);

        let ins = Instruction::Dec8IndirectHL;
        machine.exec(ins);

        assert_eq!(to, machine.get_mem8(0x0001));
        assert_eq!(0x01, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00)]
    #[case(0xFF, 0xFF, 0xFF)]
    #[case(0x00, 0xFF, 0x00)]
    #[case(0xF0, 0xF1, 0xF0)]
    fn it_execs_and8(
        #[values(B, C, D, E, H, L)] src: R8,
        #[case] x: u8,
        #[case] y: u8,
        #[case] res: u8,
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);
        machine.set_r8(src, y);

        let ins = Instruction::And8 { src };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(res, machine.get_r8(A));

        assert_eq!(res == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(true, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00)]
    #[case(0xFF)]
    #[case(0xF0)]
    fn it_execs_and8_a(#[case] x: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let ins = Instruction::And8 { src: A };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());

        assert_eq!(x, machine.get_r8(A));
        assert_eq!(x == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(true, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00)]
    #[case(0xFF, 0xFF, 0xFF)]
    #[case(0x00, 0xFF, 0x00)]
    #[case(0xF0, 0xF1, 0xF0)]
    fn it_execs_and8_indirect_hl(#[case] x: u8, #[case] y: u8, #[case] res: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let addr = 0x0001;
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, y);

        let ins = Instruction::And8IndirectHL;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(res, machine.get_r8(A));

        assert_eq!(res == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(true, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00)]
    #[case(0xFF, 0xFF, 0xFF)]
    #[case(0x00, 0xFF, 0x00)]
    #[case(0xF0, 0xF1, 0xF0)]
    fn it_execs_and8_imm(#[case] x: u8, #[case] y: u8, #[case] res: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let ins = Instruction::AndImm8 { imm: y };
        machine.exec(ins);

        assert_eq!(0x02, machine.get_pc());
        assert_eq!(res, machine.get_r8(A));

        assert_eq!(res == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(true, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00)]
    #[case(0xFF, 0xFF, 0xFF)]
    #[case(0x00, 0xFF, 0xFF)]
    #[case(0xF0, 0xF1, 0xF1)]
    fn it_execs_or8(
        #[values(B, C, D, E, H, L)] src: R8,
        #[case] x: u8,
        #[case] y: u8,
        #[case] res: u8,
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);
        machine.set_r8(src, y);

        let ins = Instruction::Or8 { src };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(res, machine.get_r8(A));

        assert_eq!(res == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00)]
    #[case(0xFF)]
    #[case(0xF0)]
    fn it_execs_or8_a(#[case] x: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let ins = Instruction::Or8 { src: A };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());

        assert_eq!(x, machine.get_r8(A));
        assert_eq!(x == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00)]
    #[case(0xFF, 0xFF, 0xFF)]
    #[case(0x00, 0xFF, 0xFF)]
    #[case(0xF0, 0xF1, 0xF1)]
    fn it_execs_or8_indirect_hl(#[case] x: u8, #[case] y: u8, #[case] res: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let addr = 0x0001;
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, y);

        let ins = Instruction::Or8IndirectHL;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(res, machine.get_r8(A));

        assert_eq!(res == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00)]
    #[case(0xFF, 0xFF, 0xFF)]
    #[case(0x00, 0xFF, 0xFF)]
    #[case(0xF0, 0xF1, 0xF1)]
    fn it_execs_or8_imm(#[case] x: u8, #[case] y: u8, #[case] res: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let ins = Instruction::OrImm8 { imm: y };
        machine.exec(ins);

        assert_eq!(0x02, machine.get_pc());
        assert_eq!(res, machine.get_r8(A));

        assert_eq!(res == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00)]
    #[case(0xFF, 0xFF, 0x00)]
    #[case(0x00, 0xFF, 0xFF)]
    #[case(0xF0, 0x0F, 0xFF)]
    fn it_execs_xor8(
        #[values(B, C, D, E, H, L)] src: R8,
        #[case] x: u8,
        #[case] y: u8,
        #[case] res: u8,
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);
        machine.set_r8(src, y);

        let ins = Instruction::Xor8 { src };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(res, machine.get_r8(A));

        assert_eq!(res == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00)]
    #[case(0xFF)]
    #[case(0xF0)]
    fn it_execs_xor8_a(#[case] x: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let ins = Instruction::Xor8 { src: A };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());

        assert_eq!(0, machine.get_r8(A));
        assert_eq!(true, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00)]
    #[case(0xFF, 0xFF, 0x00)]
    #[case(0x00, 0xFF, 0xFF)]
    #[case(0xF0, 0x0F, 0xFF)]
    fn it_execs_xor8_indirect_hl(#[case] x: u8, #[case] y: u8, #[case] res: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let addr = 0x0001;
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, y);

        let ins = Instruction::Xor8IndirectHL;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(res, machine.get_r8(A));

        assert_eq!(res == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x00, 0x00, 0x00)]
    #[case(0xFF, 0xFF, 0x00)]
    #[case(0x00, 0xFF, 0xFF)]
    #[case(0xF0, 0x0F, 0xFF)]
    fn it_execs_xor8_imm(#[case] x: u8, #[case] y: u8, #[case] res: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, x);

        let ins = Instruction::XorImm8 { imm: y };
        machine.exec(ins);

        assert_eq!(0x02, machine.get_pc());
        assert_eq!(res, machine.get_r8(A));

        assert_eq!(res == 0, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    fn it_execs_ccf(
        #[values(true, false)] sub_bcd: bool,
        #[values(true, false)] half_carry: bool,
        #[values(true, false)] carry: bool,
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(F, 0);

        machine.assign_flag(SubBCD, sub_bcd);
        machine.assign_flag(HalfCarryBCD, half_carry);
        machine.assign_flag(Carry, carry);

        let ins = Instruction::CmplCarryFlag;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(!carry, machine.get_flag(Carry));
    }

    #[rstest]
    fn it_execs_scf(
        #[values(true, false)] sub_bcd: bool,
        #[values(true, false)] half_carry: bool,
        #[values(true, false)] carry: bool,
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(F, 0);

        machine.assign_flag(SubBCD, sub_bcd);
        machine.assign_flag(HalfCarryBCD, half_carry);
        machine.assign_flag(Carry, carry);

        let ins = Instruction::SetCarryFlag;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(true, machine.get_flag(Carry));
    }

    #[rstest]
    // 0x00 + 0x00
    #[case(0x00, 0x00, (true, false, false, false), (true, false, false, false))]
    // 0x00 + 0x01
    #[case(0x01, 0x01, (false, false, false, false), (false, false, false, false))]
    // 0x09 + 0x01
    #[case(0x0A, 0x10, (false, false, false, false), (false, false, false, false))]
    // 0x99 + 0x01
    #[case(0x9A, 0x00, (false, false, false, false), (true, false, false, false))]
    // 0x09 + 0x09
    #[case(0x12, 0x18, (false, false, true, false), (false, false, false,  false))]
    // 0x90 + 0x90
    #[case(0x20, 0x80, (false, false, false, true), (false, false, false,  true))]
    // 0x99 + 0x99
    #[case(0x32, 0x98, (false, false, true, true), (false, false, false,  true))]
    // 0x00 - 0x00
    #[case(0x00, 0x00, (true, true, false, false), (true, true, false,  false))]
    // 0x00 - 0x01
    #[case(0xFF, 0x99, (false, true, true, true), (false, true, false, true))]
    // 0x10 - 0x01
    #[case(0x0F, 0x09, (false, true, true, false), (false, true, false, false))]
    // 0x00 - 0x10
    #[case(0xF0, 0x90, (false, true, false, true), (false, true, false, true))]
    // 0x00 - 0x99
    #[case(0x67, 0x01, (false, true, true, true), (false, true, false, true))]
    // Edge case, we have sub set, but have not borrowed (no carry flag set), but value is > 9
    // Game boy will not actually adjust in this case.
    #[case(0xF0, 0xF0, (false, true, false, false), (false, true, false, true))]
    fn it_execs_daa(
        #[case] from: u8,
        #[case] to: u8,
        #[case] (z, sub, hc, cry): (bool, bool, bool, bool),
        #[case] (zero, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(F, 0);
        machine.assign_flag(Zero, z);
        machine.assign_flag(SubBCD, sub);
        machine.assign_flag(HalfCarryBCD, hc);
        machine.assign_flag(Carry, cry);

        machine.set_r8(A, from);

        let ins = Instruction::DecAdjAcc;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(to, machine.get_r8(A));

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    fn it_execs_cmpl_acc(#[values(0x00, 0xFF, 0xF0, 0x0F)] value: u8) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(A, value);

        let ins = Instruction::CmplAcc;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(!value, machine.get_r8(A));

        assert_eq!(true, machine.get_flag(SubBCD));
        assert_eq!(true, machine.get_flag(HalfCarryBCD));
    }

    #[rstest]
    #[case(0x0000, 0x0001)]
    #[case(0x0001, 0x0002)]
    #[case(0xFFFF, 0x0000)]
    fn it_execs_inc16(#[values(BC, DE, HL, SP)] src: R16, #[case] from: u16, #[case] to: u16) {
        let mut machine = Machine::new();
        machine.set_r16(src, from);

        machine.set_pc(0x00);

        let ins = Instruction::Inc16 { src };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(to, machine.get_r16(src));
    }
    #[rstest]
    #[case(0x0001, 0x0000)]
    #[case(0x0002, 0x0001)]
    #[case(0x0000, 0xFFFF)]
    fn it_execs_dec16(#[values(BC, DE, HL, SP)] src: R16, #[case] from: u16, #[case] to: u16) {
        let mut machine = Machine::new();
        machine.set_r16(src, from);

        machine.set_pc(0x00);

        let ins = Instruction::Dec16 { src };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(to, machine.get_r16(src));
    }

    #[rstest]
    #[case(0x0001, 0x0000, 0x0001, (false, false, false, false))]
    #[case(0x000F, 0x0001, 0x0010, (false, false, false, false))]
    #[case(0x0F00, 0x0100, 0x1000, (false, false, true, false))]
    #[case(0xF000, 0x1000, 0x0000, (false, false, false, true))]
    #[case(0xFFFF, 0x0001, 0x0000, (false, false, true, true))]
    fn it_execs_add16_hl(
        #[values(BC, DE, SP)] src: R16,
        #[case] x: u16,
        #[case] y: u16,
        #[case] res: u16,
        #[case] (_, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_r16(src, x);
        machine.set_r16(HL, y);

        let ins = Instruction::Add16HL { src };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(res, machine.get_r16(HL));

        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }
    #[rstest]
    #[case(0x0001,  0x0002, (false, false, false, false))]
    #[case(0x000F,  0x001E, (false, false, false, false))]
    #[case(0x0F00,  0x1E00, (false, false, true, false))]
    #[case(0xF000,  0xE000, (true, false, false, true))]
    #[case(0xFFFF,  0xFFFE, (true, false, true, true))]
    fn it_execs_add16_hl_hl(
        #[case] x: u16,
        #[case] res: u16,
        #[case] (_, sub_bcd, half_carry, carry): (bool, bool, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_r16(HL, x);

        let ins = Instruction::Add16HL { src: HL };
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(res, machine.get_r16(HL));

        assert_eq!(sub_bcd, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0x0000, 0x00, 0x0000, (false, false))]
    #[case(0x0000, 0x01, 0x0001, (false, false))]
    #[case(0x000F, 0x01, 0x0010, (true, false))]
    #[case(0x00F0, 0x10, 0x0100, (false, true))]
    #[case(0x00FF, 0x01, 0x0100, (true, true))]
    #[case(0x0001, -0x01, 0x0000, (true, true))]
    #[case(0x0000, -0x10, 0xFFF0, (false, false))]
    #[case(0xFFF0, -0x01, 0xFFEF, (false, true))]
    #[case(0x0000, -0x01, 0xFFFF, (false, false))]
    fn it_execs_add16_sp(
        #[case] sp: u16,
        #[case] imm: i8,
        #[case] res: u16,
        #[case] (half_carry, carry): (bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r16(SP, sp);

        let ins = Instruction::AddSPImm8 { imm };
        machine.exec(ins);

        assert_eq!(0x02, machine.get_pc());
        assert_eq!(res, machine.get_r16(SP));

        assert_eq!(false, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(half_carry, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, 0b0000_00000, false)]
    #[case(0b0000_0001, 0b0000_00010, false)]
    #[case(0b1111_1111, 0b1111_1111, true)]
    #[case(0b1000_0000, 0b0000_00001, true)]
    fn it_execs_rlca(#[case] from: u8, #[case] to: u8, #[case] carry: bool) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(A, from);

        let ins = Instruction::RLCA;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(to, machine.get_r8(A));

        assert_eq!(false, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, 0b0000_0000, false)]
    #[case(0b0000_0001, 0b1000_0000, true)]
    #[case(0b1000_0000, 0b0100_0000, false)]
    #[case(0b1111_1111, 0b1111_1111, true)]
    fn it_execs_rrca(#[case] from: u8, #[case] to: u8, #[case] carry: bool) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(A, from);

        let ins = Instruction::RRCA;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(to, machine.get_r8(A));

        assert_eq!(false, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case((0b0000_0000, false), (0b0000_00000,false))]
    #[case((0b0000_0000, true), (0b0000_00001,false))]
    #[case((0b1000_0000, false), (0b0000_0000,true))]
    #[case((0b0000_0001, false), (0b0000_00010,false))]
    fn it_execs_rla(#[case] (from, fc): (u8, bool), #[case] (to, tc): (u8, bool)) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(A, from);
        machine.assign_flag(Carry, fc);

        let ins = Instruction::RLA;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(to, machine.get_r8(A));

        assert_eq!(false, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(tc, machine.get_flag(Carry));
    }

    #[rstest]
    #[case((0b0000_0000, false), (0b0000_00000,false))]
    #[case((0b0000_0000, true), (0b1000_0000,false))]
    #[case((0b1000_0000, false), (0b0100_0000,false))]
    #[case((0b0000_0001, false), (0b0000_00000,true))]
    #[case((0b0001_0000, true), (0b1000_1000,false))]
    fn it_execs_rra(#[case] (from, fc): (u8, bool), #[case] (to, tc): (u8, bool)) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(A, from);
        machine.assign_flag(Carry, fc);

        let ins = Instruction::RRA;
        machine.exec(ins);

        assert_eq!(0x01, machine.get_pc());
        assert_eq!(to, machine.get_r8(A));

        assert_eq!(false, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(tc, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_00000, true, false))]
    #[case(0b1000_0000, (0b0000_0001, false, true))]
    #[case(0b0000_0001, (0b0000_0010, false, false))]
    #[case(0b1000_0001, (0b0000_0011, false, true))]
    fn it_execs_rlc(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] value: u8,
        #[case] (res, zero, carry): (u8, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, value);

        let ins = Instruction::RLC { src };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_r8(src));
        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_00000, true, false))]
    #[case(0b1000_0000, (0b0000_0001, false, true))]
    #[case(0b0000_0001, (0b0000_0010, false, false))]
    #[case(0b1000_0001, (0b0000_0011, false, true))]
    fn it_execs_rlchl(#[case] value: u8, #[case] (res, zero, carry): (u8, bool, bool)) {
        let addr = 0x0001;

        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r16(HL, addr);

        machine.set_mem8(addr, value);

        let ins = Instruction::RLCIndirectHL;
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_mem8(addr));
        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_00000, true, false))]
    #[case(0b0000_0001, (0b1000_0000, false, true))]
    #[case(0b1000_0000, (0b0100_0000, false, false))]
    #[case(0b1000_0001, (0b1100_0000, false, true))]
    fn it_execs_rrc(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] value: u8,
        #[case] (res, zero, carry): (u8, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, value);

        let ins = Instruction::RRC { src };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_r8(src));
        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_00000, true, false))]
    #[case(0b0000_0001, (0b1000_0000, false, true))]
    #[case(0b1000_0000, (0b0100_0000, false, false))]
    #[case(0b1000_0001, (0b1100_0000, false, true))]
    fn it_execs_rrchl(#[case] value: u8, #[case] (res, zero, carry): (u8, bool, bool)) {
        let addr = 0x0001;

        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r16(HL, addr);

        machine.set_mem8(addr, value);

        let ins = Instruction::RRCIndirectHL;
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_mem8(addr));
        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case((0b0000_0000, false), (0b0000_00000, true, false))]
    #[case((0b0000_0000, true), (0b0000_00001, false, false))]
    #[case((0b1000_0000, false), (0b0000_0000, true, true))]
    #[case((0b1000_0000, true), (0b0000_0001, false, true))]
    #[case((0b0000_0001, false), (0b0000_0010, false, false))]
    #[case((0b0000_0001, true), (0b0000_0011, false, false))]
    fn it_execs_rl(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] (from, fc): (u8, bool),
        #[case] (res, zero, tc): (u8, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, from);
        machine.assign_flag(Carry, fc);

        let ins = Instruction::RL { src };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_r8(src));
        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(tc, machine.get_flag(Carry));
    }

    #[rstest]
    #[case((0b0000_0000, false), (0b0000_00000, true, false))]
    #[case((0b0000_0000, true), (0b0000_00001, false, false))]
    #[case((0b1000_0000, false), (0b0000_0000, true, true))]
    #[case((0b1000_0000, true), (0b0000_0001, false, true))]
    #[case((0b0000_0001, false), (0b0000_0010, false, false))]
    #[case((0b0000_0001, true), (0b0000_0011, false, false))]
    fn it_execs_rlhl(#[case] (from, fc): (u8, bool), #[case] (res, zero, carry): (u8, bool, bool)) {
        let addr = 0x0001;

        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r16(HL, addr);
        machine.assign_flag(Carry, fc);

        machine.set_mem8(addr, from);

        let ins = Instruction::RLIndirectHL;
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_mem8(addr));
        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case((0b0000_0000, false), (0b0000_00000, true, false))]
    #[case((0b0000_0000, true), (0b1000_0000, false, false))]
    #[case((0b0000_0001, false), (0b0000_0000, true, true))]
    #[case((0b0000_0001, true), (0b1000_0000, false, true))]
    #[case((0b1000_0000, false), (0b0100_0000, false, false))]
    #[case((0b1000_0000, true), (0b1100_0000, false, false))]
    fn it_execs_rr(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] (from, fc): (u8, bool),
        #[case] (res, zero, tc): (u8, bool, bool),
    ) {
        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r8(src, from);
        machine.assign_flag(Carry, fc);

        let ins = Instruction::RR { src };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_r8(src));
        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(tc, machine.get_flag(Carry));
    }

    #[rstest]
    #[case((0b0000_0000, false), (0b0000_00000, true, false))]
    #[case((0b0000_0000, true), (0b1000_0000, false, false))]
    #[case((0b0000_0001, false), (0b0000_0000, true, true))]
    #[case((0b0000_0001, true), (0b1000_0000, false, true))]
    #[case((0b1000_0000, false), (0b0100_0000, false, false))]
    #[case((0b1000_0000, true), (0b1100_0000, false, false))]
    fn it_execs_rrhl(#[case] (from, fc): (u8, bool), #[case] (res, zero, carry): (u8, bool, bool)) {
        let addr = 0x0001;

        let mut machine = Machine::new();
        machine.set_pc(0x00);
        machine.set_r16(HL, addr);
        machine.assign_flag(Carry, fc);

        machine.set_mem8(addr, from);

        let ins = Instruction::RRIndirectHL;
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_mem8(addr));
        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_0000, true, false))]
    #[case(0b0000_0001, (0b0000_0010, false, false))]
    #[case(0b1000_0000, (0b0000_0000, true, true))]
    #[case(0b1100_0000, (0b1000_0000, false, true))]
    fn it_execs_sla(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] from: u8,
        #[case] (res, zero, carry): (u8, bool, bool),
    ) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(src, from);

        let ins = Instruction::SLA { src };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_r8(src));

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_0000, true, false))]
    #[case(0b0000_0001, (0b0000_0010, false, false))]
    #[case(0b1000_0000, (0b0000_0000, true, true))]
    #[case(0b1100_0000, (0b1000_0000, false, true))]
    fn it_execs_slahl(#[case] from: u8, #[case] (res, zero, carry): (u8, bool, bool)) {
        let mut machine = Machine::new();

        let addr = 0x0001;

        machine.set_pc(0x00);
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, from);

        let ins = Instruction::SLAIndirectHL;
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_mem8(addr));

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_0000, true, false))]
    #[case(0b1000_0000, (0b1100_0000, false, false))]
    #[case(0b0000_0001, (0b0000_0000, true, true))]
    #[case(0b0000_0011, (0b0000_0001, false, true))]
    #[case(0b1000_0011, (0b1100_0001, false, true))]
    fn it_execs_sra(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] from: u8,
        #[case] (res, zero, carry): (u8, bool, bool),
    ) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(src, from);

        let ins = Instruction::SRA { src };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_r8(src));

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_0000, true, false))]
    #[case(0b1000_0000, (0b1100_0000, false, false))]
    #[case(0b0000_0001, (0b0000_0000, true, true))]
    #[case(0b0000_0011, (0b0000_0001, false, true))]
    #[case(0b1000_0011, (0b1100_0001, false, true))]
    fn it_execs_srahl(#[case] from: u8, #[case] (res, zero, carry): (u8, bool, bool)) {
        let mut machine = Machine::new();

        let addr = 0x0001;

        machine.set_pc(0x00);
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, from);

        let ins = Instruction::SRAIndirectHL;
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_mem8(addr));

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, 0b0000_0000)]
    #[case(0b1111_0000, 0b0000_1111)]
    #[case(0b0000_1111, 0b1111_0000)]
    #[case(0b1111_1111, 0b1111_1111)]
    fn it_execs_swap(#[values(A, B, C, D, E, H, L)] src: R8, #[case] from: u8, #[case] to: u8) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(src, from);
        let ins = Instruction::Swap { src };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(to, machine.get_r8(src));

        assert_eq!(to == 0x00, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, 0b0000_0000)]
    #[case(0b1111_0000, 0b0000_1111)]
    #[case(0b0000_1111, 0b1111_0000)]
    #[case(0b1111_1111, 0b1111_1111)]
    fn it_execs_swap_hl(#[case] from: u8, #[case] to: u8) {
        let mut machine = Machine::new();

        let addr = 0x0001;

        machine.set_pc(0x00);
        machine.set_r16(HL, addr);

        machine.set_mem8(addr, from);

        let ins = Instruction::SwapIndirectHL;
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(to, machine.get_mem8(addr));

        assert_eq!(to == 0x00, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_0000, true, false))]
    #[case(0b1000_0000, (0b0100_0000, false, false))]
    #[case(0b0000_0001, (0b0000_0000, true, true))]
    #[case(0b0000_0011, (0b0000_0001, false, true))]
    #[case(0b1000_0011, (0b0100_0001, false, true))]
    fn it_execs_srl(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] from: u8,
        #[case] (res, zero, carry): (u8, bool, bool),
    ) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(src, from);

        let ins = Instruction::SRL { src };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_r8(src));

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, (0b0000_0000, true, false))]
    #[case(0b1000_0000, (0b0100_0000, false, false))]
    #[case(0b0000_0001, (0b0000_0000, true, true))]
    #[case(0b0000_0011, (0b0000_0001, false, true))]
    #[case(0b1000_0011, (0b0100_0001, false, true))]
    fn it_execs_srlhl(#[case] from: u8, #[case] (res, zero, carry): (u8, bool, bool)) {
        let mut machine = Machine::new();

        let addr = 0x0001;

        machine.set_pc(0x00);
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, from);

        let ins = Instruction::SRLIndirectHL;
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(res, machine.get_mem8(addr));

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(carry, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, 0, false)]
    #[case(0b0000_0001, 0, true)]
    #[case(0b0000_0010, 1, true)]
    #[case(0b1000_0000, 7, true)]
    #[case(0b0000_0000, 7, false)]
    fn it_execs_bit(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] from: u8,
        #[case] bit: u8,
        #[case] zero: bool,
    ) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(src, from);

        let ins = Instruction::BIT { src, bit };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(true, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }
    #[rstest]
    #[case(0b0000_0000, 0, false)]
    #[case(0b0000_0001, 0, true)]
    #[case(0b0000_0010, 1, true)]
    #[case(0b1000_0000, 7, true)]
    #[case(0b0000_0000, 7, false)]
    fn it_execs_bit_hl(#[case] from: u8, #[case] bit: u8, #[case] zero: bool) {
        let mut machine = Machine::new();

        let addr = 0x0001;

        machine.set_pc(0x00);
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, from);

        let ins = Instruction::BITIndirectHL { bit };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());

        assert_eq!(zero, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(true, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, 0, 0b0000_00000)]
    #[case(0b0000_0001, 0, 0b0000_0000)]
    #[case(0b1000_0000, 7, 0b0000_0000)]
    #[case(0b1111_1111, 3, 0b1111_0111)]
    fn it_execs_res(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] from: u8,
        #[case] bit: u8,
        #[case] to: u8,
    ) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(src, from);

        let ins = Instruction::RES { src, bit };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(to, machine.get_r8(src));

        assert_eq!(false, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, 0, 0b0000_00000)]
    #[case(0b0000_0001, 0, 0b0000_0000)]
    #[case(0b1000_0000, 7, 0b0000_0000)]
    #[case(0b1111_1111, 3, 0b1111_0111)]
    fn it_execs_res_hl(#[case] from: u8, #[case] bit: u8, #[case] to: u8) {
        let mut machine = Machine::new();

        let addr = 0x0001;

        machine.set_pc(0x00);
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, from);

        let ins = Instruction::RESIndirectHL { bit };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(to, machine.get_mem8(addr));

        assert_eq!(false, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, 0, 0b0000_00001)]
    #[case(0b0000_0001, 0, 0b0000_0001)]
    #[case(0b01111_1111, 7, 0b1111_1111)]
    #[case(0b0000_0000, 3, 0b0000_1000)]
    fn it_execs_set(
        #[values(A, B, C, D, E, H, L)] src: R8,
        #[case] from: u8,
        #[case] bit: u8,
        #[case] to: u8,
    ) {
        let mut machine = Machine::new();

        machine.set_pc(0x00);
        machine.set_r8(src, from);

        let ins = Instruction::SET { src, bit };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(to, machine.get_r8(src));

        assert_eq!(false, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }

    #[rstest]
    #[case(0b0000_0000, 0, 0b0000_00001)]
    #[case(0b0000_0001, 0, 0b0000_0001)]
    #[case(0b01111_1111, 7, 0b1111_1111)]
    #[case(0b0000_0000, 3, 0b0000_1000)]
    fn it_execs_set_hl(#[case] from: u8, #[case] bit: u8, #[case] to: u8) {
        let mut machine = Machine::new();

        let addr = 0x0001;

        machine.set_pc(0x00);
        machine.set_r16(HL, addr);
        machine.set_mem8(addr, from);

        let ins = Instruction::SETIndirectHL { bit };
        machine.exec(ins);

        assert_eq!(0x0002, machine.get_pc());
        assert_eq!(to, machine.get_mem8(addr));

        assert_eq!(false, machine.get_flag(Zero));
        assert_eq!(false, machine.get_flag(SubBCD));
        assert_eq!(false, machine.get_flag(HalfCarryBCD));
        assert_eq!(false, machine.get_flag(Carry));
    }
}

#[cfg(test)]
mod flag_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0x00, Zero, false)]
    #[case(0x00, SubBCD, false)]
    #[case(0x00, HalfCarryBCD, false)]
    #[case(0x00, Carry, false)]
    #[case(0xFF, Zero, true)]
    #[case(0xFF, SubBCD, true)]
    #[case(0xFF, HalfCarryBCD, true)]
    #[case(0xFF, Carry, true)]
    #[case(0b1000_0000, Zero, true)]
    #[case(0b0100_0000, SubBCD, true)]
    #[case(0b0010_0000, HalfCarryBCD, true)]
    #[case(0b0001_0000, Carry, true)]
    fn it_can_read_flags(#[case] value: u16, #[case] flag: Flag, #[case] state: bool) {
        let mut machine = Machine::new();

        machine.set_r16(AF, value);

        assert_eq!(state, machine.get_flag(flag));
    }

    #[rstest]
    #[case(Zero, 0b1000_0000)]
    #[case(SubBCD, 0b0100_0000)]
    #[case(HalfCarryBCD, 0b0010_0000)]
    #[case(Carry, 0b0001_0000)]
    fn it_can_set_flags(#[case] flag: Flag, #[case] flags: u8) {
        let mut machine = Machine::new();

        machine.set_r8(F, 0x00);
        machine.set_flag(flag);

        assert_eq!(flags, machine.get_r8(F));
    }

    #[rstest]
    #[case(Zero, 0b0111_0000)]
    #[case(SubBCD, 0b1011_0000)]
    #[case(HalfCarryBCD, 0b1101_0000)]
    #[case(Carry, 0b1110_0000)]
    fn it_can_clear_flags(#[case] flag: Flag, #[case] flags: u8) {
        let mut machine = Machine::new();

        machine.set_r8(F, 0xF0);
        machine.clear_flag(flag);

        assert_eq!(flags, machine.get_r8(F));
    }
}

#[cfg(test)]
mod int_tests {

    use rstest::rstest;

    #[rstest]
    #[case(0x0000, 0x01, 0x0001)]
    #[case(0xFFFF, 0x01, 0x0000)]
    #[case(0xFFFF, -0x01, 0xFFFE)]
    #[case(0xFFFF, -0x01, 0xFFFE)]
    #[case(0xFFFF, -0x7F, 0xFF80)]
    fn it_adds_i8_to_u16(#[case] a: u16, #[case] b: i8, #[case] result: u16) {
        let b: u16 = (b as i16).cast_unsigned();
        assert_eq!(result, a.wrapping_add(b));
    }
}
