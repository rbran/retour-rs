use crate::arch::MidFuncHook;

use crate::error::Result;

// NOTE this should be equal to [arc::x86::thunk::x86::push_all_reg]
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct RegistersX86 {
  pub edi: u32,
  pub esi: u32,
  pub edx: u32,
  pub ecx: u32,
  pub ebx: u32,
  pub eax: u32,
  pub ebp: u32,
  pub esp: u32,
}

// NOTE this should be equal to [arc::x86::thunk::x64::push_all_reg]
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct RegistersX64 {
  pub r15: u64,
  pub r14: u64,
  pub r13: u64,
  pub r12: u64,
  pub r11: u64,
  pub r10: u64,
  pub r9: u64,
  pub r8: u64,
  pub rdi: u64,
  pub rsi: u64,
  pub rdx: u64,
  pub rcx: u64,
  pub rbx: u64,
  pub rax: u64,
  pub rbp: u64,
  pub rsp: u64,
}

#[derive(Debug)]
pub struct RawMidFuncHook(MidFuncHook);

// TODO: stop all threads in target during patch?
impl RawMidFuncHook {
  /// Constructs a new mid-function hook.
  ///
  /// The hook is disabled by default.
  pub unsafe fn new(target: *const (), hook: *const (), original_first: bool) -> Result<Self> {
    MidFuncHook::new(target, hook, original_first).map(RawMidFuncHook)
  }

  /// Enables the detour.
  pub unsafe fn enable(&self) -> Result<()> {
    self.0.enable()
  }

  /// Disables the detour.
  pub unsafe fn disable(&self) -> Result<()> {
    self.0.disable()
  }

  /// Returns whether the detour is enabled or not.
  pub fn is_enabled(&self) -> bool {
    self.0.is_enabled()
  }
}

unsafe impl Send for RawMidFuncHook {}
unsafe impl Sync for RawMidFuncHook {}
