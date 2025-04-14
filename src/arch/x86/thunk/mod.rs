#![allow(dead_code)]

/// Implements x86 operations
pub mod x86;

/// Implements x64 operations
#[cfg(target_arch = "x86_64")]
pub mod x64;

#[cfg(target_arch = "x86")]
mod arch {
  pub use super::x86::call_rel32 as call;
  pub use super::x86::jcc_rel32 as jcc;
  pub use super::x86::jmp_rel32 as jmp;
}

#[cfg(target_arch = "x86_64")]
mod arch {
  pub use super::x64::call_abs as call;
  pub use super::x64::jcc_abs as jcc;
  pub use super::x64::jmp_abs as jmp;
}

use crate::pic::Thunkable;

// Export the default architecture
pub use self::arch::*;

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Register {
  // ax = 0, cx = 1, dx = 2, bx = 3
  sp = 4,
  bp = 5,
  // si = 6, di = 7,
}

pub fn push_reg(register: Register) -> [u8; 1] {
  // when extended registers for x64 is added to Register, this has to have a x64
  // and x86 variant
  let opcode = 0x50;
  let register = register as u8;
  [opcode + register]
}

pub fn pop_reg(register: Register) -> [u8; 1] {
  // when extended registers for x64 is added to Register, this has to have a x64
  // and x86 variant
  let opcode = 0x58;
  let register = register as u8;
  [opcode + register]
}

#[cfg(target_arch = "x86_64")]
const PUSH_ALL_REGS: &[u8] = x64::PUSH_ALL_REGS;
#[cfg(target_arch = "x86")]
const PUSH_ALL_REGS: &[u8] = x86::PUSH_ALL_REGS;

#[cfg(target_arch = "x86_64")]
const POP_ALL_REGS: &[u8] = x64::POP_ALL_REGS;
#[cfg(target_arch = "x86")]
const POP_ALL_REGS: &[u8] = x86::POP_ALL_REGS;
