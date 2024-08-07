mod le_bytes_junk {
    pub trait ToLeTrait {
        #[allow(dead_code)]
        fn to_le_bytes(self) -> Vec<u8>;
    }
    impl ToLeTrait for u8 {
        fn to_le_bytes(self) -> Vec<u8> {
            self.to_le_bytes().to_vec()
        }
    }

    impl ToLeTrait for u16 {
        fn to_le_bytes(self) -> Vec<u8> {
            self.to_le_bytes().to_vec()
        }
    }
    impl ToLeTrait for u32 {
        fn to_le_bytes(self) -> Vec<u8> {
            self.to_le_bytes().to_vec()
        }
    }
    pub trait OurFrom<T> {
        #[allow(dead_code)]
        fn our_from(value: T) -> Self;
    }
    impl OurFrom<u32> for u16 {
        fn our_from(value: u32) -> Self {
            value as u16
        }
    }
    impl OurFrom<u32> for u32 {
        fn our_from(value: u32) -> Self {
            value as u32
        }
    }
    impl OurFrom<u32> for u8 {
        fn our_from(value: u32) -> Self {
            value as u8
        }
    }
}

#[cfg(test)]
mod tests {

    mod stack {
        use crate::{
            cpu::{Cpu, SP},
            opcodes::Opcode,
        };

        fn create_cpu() -> Cpu {
            let mut cpu = Cpu::new();
            cpu.registers[SP] = 50;
            return cpu;
        }

        #[test]
        fn push_byte_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 1;

            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::PushByteReg as u8, 0]);
            cpu.run();

            assert_eq!(cpu.memory.byte(expected_sp), 100);
        }
        #[test]
        fn push_byte_imm() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 1;

            cpu.load_program(&[Opcode::PushByteImm as u8, 100]);
            cpu.run();

            assert_eq!(cpu.memory.byte(expected_sp), 100);
        }
        #[test]
        fn push_byte_mem() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 1;
            cpu.memory.set_byte(100, 0xFF);

            cpu.load_program(&[Opcode::PushByteMem as u8, 100]);
            cpu.run();

            assert_eq!(cpu.memory.byte(expected_sp), 0xFF);
        }

        #[test]
        fn push_short_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 2;
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::PushShortReg as u8, 0]);
            cpu.run();

            assert_eq!(cpu.memory.short(expected_sp), 100);
        }

        #[test]
        fn push_short_imm() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 2;
            cpu.load_program(&[Opcode::PushShortImm as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.memory.short(expected_sp), 100);
        }

        #[test]
        fn push_short_mem() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 2;
            cpu.memory.set_short(100, 0xFF);

            cpu.load_program(&[Opcode::PushShortMem as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.memory.short(expected_sp), 0xFF);
        }

        #[test]
        fn push_long_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 4;
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::PushLongReg as u8, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.memory.long(expected_sp), 100);
        }

        #[test]
        fn push_long_imm() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 4;
            cpu.load_program(&[Opcode::PushLongImm as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.memory.long(expected_sp), 100);
        }

        #[test]
        fn push_long_mem() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 4;

            cpu.memory.set_long(100, 0xFF_FF_FF_FF);

            cpu.load_program(&[Opcode::PushLongMem as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.memory.long(expected_sp), 0xFF_FF_FF_FF);
        }

        #[test]
        fn pop_byte_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 1;
            cpu.memory.set_byte(cpu.sp(), 100);

            cpu.load_program(&[Opcode::PopByteReg as u8, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 100);
            assert_eq!(cpu.sp(), expected_sp);
        }

        #[test]
        fn pop_byte_mem() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 1;
            cpu.memory.set_byte(cpu.sp(), 100);
            cpu.memory.set_byte(100, 0xFF);

            cpu.load_program(&[Opcode::PopByteMem as u8, 100]);
            cpu.run();

            assert_eq!(cpu.memory.byte(100), 100);
            assert_eq!(cpu.sp(), expected_sp);
        }

        #[test]
        fn pop_short_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 2;
            cpu.memory.set_short(cpu.sp(), 100);

            cpu.load_program(&[Opcode::PopShortReg as u8, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 100);
            assert_eq!(cpu.sp(), expected_sp);
        }

        #[test]
        fn pop_short_mem() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 2;
            cpu.memory.set_short(cpu.sp(), 100);
            cpu.memory.set_short(100, 0xFF_FF);

            cpu.load_program(&[Opcode::PopShortMem as u8, 100]);
            cpu.run();

            assert_eq!(cpu.memory.short(100), 100);
            assert_eq!(cpu.sp(), expected_sp);
        }

        #[test]
        fn pop_long_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 4;
            cpu.memory.set_long(cpu.sp(), 100);

            cpu.load_program(&[Opcode::PopLongReg as u8, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 100);
            assert_eq!(cpu.sp(), expected_sp);
        }

        #[test]
        fn pop_long_mem() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 4;
            cpu.memory.set_long(cpu.sp(), 100);
            cpu.memory.set_long(100, 0xFF_FF_FF_FF);

            cpu.load_program(&[Opcode::PopLongMem as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.memory.long(100), 100);
            assert_eq!(cpu.sp(), expected_sp);
        }
    }

    mod general {
        use crate::cpu::Cpu;

        #[test]
        fn hlt() {
            let mut cpu = Cpu::new();
            cpu.load_program(&[0]);
            cpu.run();

            assert_eq!(cpu.ip(), 1);
            assert_eq!((cpu.flags() & Cpu::HALT_FLAG), Cpu::HALT_FLAG);
        }
    }

    mod add {
        use crate::{cpu::Cpu, opcodes::Opcode};
        #[test]
        fn add_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;

            cpu.load_program(&[Opcode::AddByteReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 150);
        }

        #[test]
        fn add_byte_mem() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.memory.set_byte(100, 50);

            cpu.load_program(&[Opcode::AddLongMem as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 150);
        }

        #[test]
        fn add_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;

            cpu.load_program(&[Opcode::AddShortReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 150);
        }

        #[test]
        fn add_short_mem() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.memory.set_short(100, 50);

            cpu.load_program(&[Opcode::AddShortMem as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 150);
        }

        #[test]
        fn add_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;

            cpu.load_program(&[Opcode::AddLongReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 150);
        }

        #[test]
        fn add_long_mem() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.memory.set_long(100, 50);

            cpu.load_program(&[Opcode::AddLongMem as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 150);
        }
        #[test]
        fn add_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddByteImm as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 200);
        }
        #[test]
        fn add_byte_imm_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 156;
            cpu.load_program(&[Opcode::AddByteImm as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn add_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddShortImm as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 200);
        }
        #[test]
        fn add_short_imm_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 65535;
            cpu.load_program(&[Opcode::AddShortImm as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 99);
        }
        #[test]
        fn add_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddLongImm as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 200);
        }
        #[test]
        fn add_long_imm_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 4_294_967_295;
            cpu.load_program(&[Opcode::AddLongImm as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 99);
        }
    }
    mod sub {
        use crate::{cpu::Cpu, opcodes::Opcode};
        #[test]
        fn sub_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubByteImm as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn sub_byte_imm_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0;
            cpu.load_program(&[Opcode::SubByteImm as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 156);
        }
        #[test]
        fn sub_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubShortImm as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn sub_short_imm_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0;
            cpu.load_program(&[Opcode::SubShortImm as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 65536 - 100);
        }
        #[test]
        fn sub_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubLongImm as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn sub_long_imm_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0;
            cpu.load_program(&[Opcode::SubLongImm as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 4_294_967_295 - 99);
        }
        #[test]
        fn sub_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;
            cpu.load_program(&[Opcode::SubByteReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 50);
        }
        #[test]
        fn sub_byte_mem() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.memory.set_byte(100, 50);
            cpu.load_program(&[Opcode::SubByteMem as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 50);
        }
        #[test]
        fn sub_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;
            cpu.load_program(&[Opcode::SubShortReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 50);
        }
        #[test]
        fn sub_short_mem() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.memory.set_short(100, 50);
            cpu.load_program(&[Opcode::SubShortMem as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 50);
        }
        #[test]
        fn sub_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;
            cpu.load_program(&[Opcode::SubLongReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 50);
        }
        #[test]
        fn sub_long_mem() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.memory.set_long(100, 50);
            cpu.load_program(&[Opcode::SubLongMem as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 50);
        }
    }
    mod mul {
        use crate::{cpu::Cpu, opcodes::Opcode};
        #[test]
        fn mul_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2;
            cpu.load_program(&[Opcode::MulByteImm as u8, 10]);
            cpu.run();

            assert_eq!(cpu.registers[0], 20);
        }
        #[test]
        fn mul_byte_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::MulByteImm as u8, 100]);
            cpu.run();

            let result = 100u8.wrapping_mul(100u8);
            assert_eq!((cpu.registers[0] & 0xFF) as u8, result);
        }
        #[test]
        fn mul_short() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2;
            cpu.load_program(&[Opcode::MulShortImm as u8, 10, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 20);
        }
        #[test]
        fn mul_short_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 70_000;
            cpu.load_program(&[Opcode::MulShortImm as u8, 100, 0]);
            cpu.run();

            let result = 4_464u16.wrapping_mul(100u16);
            assert_eq!((cpu.registers[0] & 0xFFFF) as u16, result);
        }
        #[test]
        fn mul_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2;
            cpu.load_program(&[Opcode::MulLongImm as u8, 10, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 20);
        }
        #[test]
        fn mul_long_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2_000_000_000;
            cpu.load_program(&[Opcode::MulLongImm as u8, 100, 0, 0, 0]);
            cpu.run();

            let result = 2_000_000_000u32.wrapping_mul(100);
            assert_eq!(cpu.registers[0], result);
        }
    }
    mod div {
        use crate::{cpu::Cpu, opcodes::Opcode};
        #[test]
        fn div_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.load_program(&[Opcode::DivByteImm as u8, 2]);
            cpu.run();

            assert_eq!(cpu.registers[0], 5);
            assert_eq!(cpu.registers[1], 0);
        }
        #[test]
        fn div_byte_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 1000;
            cpu.load_program(&[Opcode::DivByteImm as u8, 2]);
            cpu.run();
            // 1000 as a byte wraps to 232
            let result = 232u8.wrapping_div(2);
            assert_eq!((cpu.registers[0] & 0xFF) as u8, result);
            assert_eq!((cpu.registers[1] & 0xFF) as u8, 0);
        }
        #[test]
        fn div_byte_remainder() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 9;
            cpu.load_program(&[Opcode::DivByteImm as u8, 2]);
            cpu.run();
            assert_eq!((cpu.registers[0] & 0xFF) as u8, 4);
            assert_eq!((cpu.registers[1] & 0xFF) as u8, 1);
        }

        #[test]
        fn div_short() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.load_program(&[Opcode::DivShortImm as u8, 2, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 5);
            assert_eq!(cpu.registers[1], 0);
        }
        #[test]
        fn div_short_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 70_000; // Value larger than u16::MAX
            cpu.load_program(&[Opcode::DivShortImm as u8, 2, 0]);
            cpu.run();
            // 70,000 as a u16 wraps to 4,464
            let result = 4_464u16 / 2;
            assert_eq!((cpu.registers[0] & 0xFFFF) as u16, result);
            assert_eq!(cpu.registers[1], 0);
        }
        #[test]
        fn div_short_remainder() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 9;
            cpu.load_program(&[Opcode::DivShortImm as u8, 2, 0]);
            cpu.run();
            assert_eq!(cpu.registers[0], 4);
            assert_eq!(cpu.registers[1], 1);
        }
        #[test]
        fn div_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.load_program(&[Opcode::DivLongImm as u8, 2, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 5);
            assert_eq!(cpu.registers[1], 0);
        }
        #[test]
        fn div_long_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2_000_000_000;
            cpu.load_program(&[Opcode::DivLongImm as u8, 2, 0, 0, 0]);
            cpu.run();

            let result = 1_000_000_000u32;
            assert_eq!(cpu.registers[0], result);
            assert_eq!(cpu.registers[1], 0);
        }

        #[test]
        fn div_long_remainder() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 9;
            cpu.load_program(&[Opcode::DivLongImm as u8, 2, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 4);
            assert_eq!(cpu.registers[1], 1);
        }
    }

    mod jump {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn jump() {
            let mut cpu = Cpu::new();
            let mut program = vec![];
            program.push(Opcode::JumpImm as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], 10);
        }

        #[test]
        fn jump_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            let mut program = vec![];
            program.push(Opcode::JumpReg as u8);
            program.push(0);
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], 10);
        }

        #[test]
        fn jne() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 11;
            let mut program = vec![];
            program.push(Opcode::JumpNotEqual as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], 10);
        }

        #[test]
        fn jne_negative() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 10;
            let mut program = vec![];
            program.push(Opcode::JumpNotEqual as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], no_jmp_addr);
        }

        #[test]
        fn je() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 10;
            let mut program = vec![];
            program.push(Opcode::JumpEqual as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], 10);
        }

        #[test]
        fn je_negative() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 11;
            let mut program = vec![];
            program.push(Opcode::JumpEqual as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], no_jmp_addr);
        }

        #[test]
        fn jl_when_less() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 11;
            let mut program = vec![];
            program.push(Opcode::JumpLess as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], 10);
        }
        #[test]
        fn jl_when_eq() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 10;
            let mut program = vec![];
            program.push(Opcode::JumpLess as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], no_jmp_addr);
        }
        #[test]
        fn jl_when_gr() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 9;
            let mut program = vec![];
            program.push(Opcode::JumpLess as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], no_jmp_addr);
        }

        #[test]
        fn jg_when_less() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 11;
            let mut program = vec![];
            program.push(Opcode::JumpGreater as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], no_jmp_addr);
        }
        #[test]
        fn jg_when_eq() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 10;
            let mut program = vec![];
            program.push(Opcode::JumpGreater as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], no_jmp_addr);
        }
        #[test]
        fn jg_when_gr() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 9;
            let mut program = vec![];
            program.push(Opcode::JumpGreater as u8);
            program.extend_from_slice(&10_u32.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], 10);
        }
    }

    mod compare {
        use crate::{cpu::Cpu, opcodes::Opcode};
        #[test]
        fn compare_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 100;
            cpu.load_program(&[Opcode::CompareReg as u8, 1]);
            cpu.run();
            assert_eq!(cpu.registers[0], 1);
        }

        #[test]
        fn compare_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::CompareByteImm as u8, 100]);
            cpu.run();
            assert_eq!(cpu.registers[0], 1);
        }
        #[test]
        fn compare_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 511;
            cpu.load_program(&[Opcode::CompareShortImm as u8, 255, 1]);
            cpu.run();
            assert_eq!(cpu.registers[0], 1);
        }
        #[test]
        fn compare_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFF_FF_FF_FF;
            cpu.load_program(&[Opcode::CompareLongImm as u8, 0xFF, 0xFF, 0xFF, 0xFF]);
            cpu.run();
            assert_eq!(cpu.registers[0], 1);
        }
    }

    mod and {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn and_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFF;
            cpu.load_program(&[Opcode::AndByteImm as u8, 0xCC]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCC);
        }

        #[test]
        fn and_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFFFFFF;
            cpu.load_program(&[Opcode::AndLongImm as u8, 0xCC, 0xCC, 0xCC, 0xCC]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCCCCCCCC);
        }

        #[test]
        fn and_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFF;
            cpu.load_program(&[Opcode::AndShortImm as u8, 0xCC, 0xCC]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCCCC);
        }
    }

    mod control_flow {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn call() {
            let mut cpu = Cpu::new();
            cpu.memory.set_byte(100, Opcode::Hlt as u8);
            cpu.load_program(&[Opcode::Call as u8, 100]);
            cpu.run();
            assert_eq!(cpu.ip(), 101);
        }
        #[test]
        fn _return() {
            let mut cpu = Cpu::new();
            // 1 cycle in routine
            cpu.memory.set_byte(100, Opcode::Return as u8);

            // call: 1 cycle
            // long addr: 4 cycles
            // ret instruction: 1 cycle
            // halt on return: 1 cycle
            cpu.load_program(&[Opcode::Call as u8, 100, 0, 0, 0, 0]);
            cpu.run();
            assert_eq!(cpu.ip(), 6)
        }
    }

    mod int {
        use crate::{
            cpu::{Cpu, FLAGS, IDT},
            opcodes::Opcode,
        };
        
        
        #[test]
        fn interrupt() {
            let mut cpu = Cpu::new();

            // address of idt ptr.
            cpu.registers[IDT] = 3;

            cpu.load_program(&[
                Opcode::Interrupt as u8,
                0, // ip : 2
                Opcode::Hlt as u8,
                // idt
                // isr 0 just loads eax and ebx with 10, 15
                Opcode::MoveImmRegByte as u8,
                0,
                10,
                Opcode::MoveImmRegByte as u8,
                1,
                15,
                Opcode::InterruptReturn as u8,
            ]);

            cpu.run();
            assert_eq!(cpu.ip(), 3);
            assert_eq!(cpu.registers[0], 10);
            assert_eq!(cpu.registers[1], 15);
            assert_eq!((cpu.registers[FLAGS] & Cpu::INTERRUPT_FLAG as u32), 0);
        }
    }

    mod mov {
        mod to_reg {
            mod from_imm {
                use crate::test::le_bytes_junk::{OurFrom, ToLeTrait};
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + ToLeTrait + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    program.push(op as u8);
                    program.push(0);
                    program.extend_from_slice(&val.to_le_bytes());
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveImmRegByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveImmRegShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveImmRegLong);
                }
            }
            mod from_reg {
                use crate::test::le_bytes_junk::OurFrom;
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + Into<u32> + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    cpu.registers[1] = val.into();
                    program.push(op as u8);
                    program.push(0);
                    program.push(1);
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveRegRegByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveRegRegShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveRegRegLong);
                }
            }
            mod from_abs {
                use crate::test::le_bytes_junk::{OurFrom, ToLeTrait};
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + ToLeTrait + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    program.push(op as u8);
                    program.push(0);
                    program.extend_from_slice(&6_u32.to_le_bytes());
                    program.extend_from_slice(&val.to_le_bytes());
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveAbsRegByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveAbsRegShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveAbsRegLong);
                }
            }
            mod from_mem {
                use crate::test::le_bytes_junk::{OurFrom, ToLeTrait};
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + ToLeTrait + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    program.push(op as u8);
                    program.push(0);
                    program.extend_from_slice(&2_u32.to_le_bytes());
                    program.push(0);
                    program.push(0);
                    program.extend_from_slice(&val.to_le_bytes());
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveMemRegByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveMemRegShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveMemRegLong);
                }
            }
            mod from_ind {
                use crate::test::le_bytes_junk::{OurFrom, ToLeTrait};
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + ToLeTrait + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    program.push(op as u8);
                    program.push(0);
                    program.push(1);
                    cpu.registers[1] = program.len() as u32;
                    program.extend_from_slice(&val.to_le_bytes());
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveIndirectRegByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveIndirectRegShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveIndirectRegLong);
                }
            }
        }
        mod to_abs {
            mod from_imm {
                use crate::test::le_bytes_junk::{OurFrom, ToLeTrait};
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + ToLeTrait + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    program.push(op as u8);
                    program.push(0);
                    program.extend_from_slice(&val.to_le_bytes());
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveImmAbsByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveImmAbsShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveImmAbsLong);
                }
            }
            mod from_reg {
                use crate::test::le_bytes_junk::OurFrom;
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + Into<u32> + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    cpu.registers[1] = val.into();
                    program.push(op as u8);
                    program.push(0);
                    program.push(1);
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveRegAbsByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveRegAbsShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveRegAbsLong);
                }
            }
            mod from_abs {
                use crate::test::le_bytes_junk::{OurFrom, ToLeTrait};
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + ToLeTrait + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    program.push(op as u8);
                    program.push(0);
                    program.extend_from_slice(&6_u32.to_le_bytes());
                    program.extend_from_slice(&val.to_le_bytes());
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveAbsAbsByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveAbsAbsShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveAbsAbsLong);
                }
            }
            mod from_mem {
                use crate::test::le_bytes_junk::{OurFrom, ToLeTrait};
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + ToLeTrait + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    program.push(op as u8);
                    program.push(0);
                    program.extend_from_slice(&2_u32.to_le_bytes());
                    program.push(0);
                    program.push(0);
                    program.extend_from_slice(&val.to_le_bytes());
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveMemAbsByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveMemAbsShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveMemAbsLong);
                }
            }
            mod from_ind {
                use crate::test::le_bytes_junk::{OurFrom, ToLeTrait};
                fn cycle<T>(op: crate::opcodes::Opcode)
                where
                    T: Copy + std::fmt::Debug + OurFrom<u32> + ToLeTrait + Eq,
                {
                    let mut cpu = crate::cpu::Cpu::new();
                    let mut program = vec![];
                    let val: T = T::our_from(0x10101010);
                    program.push(op as u8);
                    program.push(0);
                    program.push(1);
                    cpu.registers[1] = program.len() as u32;
                    program.extend_from_slice(&val.to_le_bytes());
                    cpu.load_program(program.as_slice());
                    cpu.cycle();
                    assert_eq!(T::our_from(cpu.registers[0]), val);
                }
                #[test]
                fn byte() {
                    cycle::<u8>(crate::opcodes::Opcode::MoveIndirectAbsByte);
                }
                #[test]
                fn short() {
                    cycle::<u16>(crate::opcodes::Opcode::MoveIndirectAbsShort);
                }
                #[test]
                fn long() {
                    cycle::<u32>(crate::opcodes::Opcode::MoveIndirectAbsLong);
                }
            }
        }
    }
}
