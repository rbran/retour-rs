use std::env;
use std::path::PathBuf;

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  std::fs::write(out_dir.join("push_all_regs_x64.bin"), push_all_regs_x64()).unwrap();
  std::fs::write(out_dir.join("pop_all_regs_x64.bin"), pop_all_regs_x64()).unwrap();

  std::fs::write(out_dir.join("push_all_regs_x86.bin"), push_all_regs_x86()).unwrap();
  std::fs::write(out_dir.join("pop_all_regs_x86.bin"), pop_all_regs_x86()).unwrap();
}

fn push_all_regs_x86() -> Vec<u8> {
  use iced_x86::code_asm::*;
  let mut builder = CodeAssembler::new(32).unwrap();
  builder.pusha().unwrap();
  builder.assemble(0x00).unwrap()
}

fn pop_all_regs_x86() -> Vec<u8> {
  use iced_x86::code_asm::*;
  let mut builder = CodeAssembler::new(32).unwrap();
  builder.popa().unwrap();
  builder.assemble(0x00).unwrap()
}

fn push_all_regs_x64() -> Vec<u8> {
  use iced_x86::code_asm::*;
  let mut builder = CodeAssembler::new(64).unwrap();
  builder.push(rsp).unwrap();
  builder.push(rbp).unwrap();
  builder.push(rax).unwrap();
  builder.push(rbx).unwrap();
  builder.push(rcx).unwrap();
  builder.push(rdx).unwrap();
  builder.push(rsi).unwrap();
  builder.push(rdi).unwrap();
  builder.push(r8).unwrap();
  builder.push(r9).unwrap();
  builder.push(r10).unwrap();
  builder.push(r11).unwrap();
  builder.push(r12).unwrap();
  builder.push(r13).unwrap();
  builder.push(r14).unwrap();
  builder.push(r15).unwrap();
  builder.assemble(0x00).unwrap()
}

pub fn pop_all_regs_x64() -> Vec<u8> {
  use iced_x86::code_asm::*;
  let mut builder = CodeAssembler::new(64).unwrap();
  builder.pop(r15).unwrap();
  builder.pop(r14).unwrap();
  builder.pop(r13).unwrap();
  builder.pop(r12).unwrap();
  builder.pop(r11).unwrap();
  builder.pop(r10).unwrap();
  builder.pop(r9).unwrap();
  builder.pop(r8).unwrap();
  builder.pop(rdi).unwrap();
  builder.pop(rsi).unwrap();
  builder.pop(rdx).unwrap();
  builder.pop(rcx).unwrap();
  builder.pop(rbx).unwrap();
  builder.pop(rax).unwrap();
  builder.pop(rbp).unwrap();
  builder.pop(rsp).unwrap();
  builder.assemble(0x00).unwrap()
}
