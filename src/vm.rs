pub struct VM {
    registers: [i32; 32],
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_vm() {
        let vm = VM::new();
        assert_eq!(vm.registers[0], 0);
    }
}
