use std;
use std::io;
use std::io::Write;

use crate::vm::VM;

/// 汇编语言 REPL 的核心结构
pub struct REPL {
    command_buffer: Vec<String>,
    // REPL 将用来执行代码的虚拟机
    vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        Self { command_buffer: vec![], vm: VM::new() }
    }
}

impl REPL {
    pub fn run() {}
}
