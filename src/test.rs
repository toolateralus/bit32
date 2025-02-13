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
        fn pop_byte_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 1;
            cpu.memory.set_byte(cpu.sp(), 100);

            cpu.load_program(&[Opcode::PopByte as u8, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 100);
            assert_eq!(cpu.sp(), expected_sp);
        }

        #[test]
        fn pop_short_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 2;
            cpu.memory.set_short(cpu.sp(), 100);

            cpu.load_program(&[Opcode::PopShort as u8, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 100);
            assert_eq!(cpu.sp(), expected_sp);
        }

        #[test]
        fn pop_long_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 4;
            cpu.memory.set_long(cpu.sp(), 100);

            cpu.load_program(&[Opcode::PopLong as u8, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 100);
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
            assert!(cpu.has_flag(Cpu::HALT_FLAG));
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
        fn add_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;

            cpu.load_program(&[Opcode::AddShortReg as u8, 1]);
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
        #[test]
        fn add_carry_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;

            cpu.load_program(&[Opcode::AddByteReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 150);
        }

        #[test]
        fn add_carry_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;

            cpu.load_program(&[Opcode::AddShortReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 150);
        }

        #[test]
        fn add_carry_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;

            cpu.load_program(&[Opcode::AddCarryLongReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 150);
        }
        #[test]
        fn add_carry_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddCarryByteImm as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 200);
        }
        #[test]
        fn add_carry_byte_imm_carry_set() {
            let mut cpu = Cpu::new();
            cpu.set_flag(Cpu::CARRY_FLAG, true);
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddCarryByteImm as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 201);
        }
        #[test]
        fn add_carry_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddCarryShortImm as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 200);
        }
        #[test]
        fn add_carry_short_imm_carry_set() {
            let mut cpu = Cpu::new();
            cpu.set_flag(Cpu::CARRY_FLAG, true);
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddCarryShortImm as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 201);
        }
        #[test]
        fn add_carry_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0F;
            cpu.load_program(&[Opcode::AddCarryLongImm as u8, 0xF0, 0xFF, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0xFFFF);
        }
        #[test]
        fn add_carry_long_imm_carry_set() {
            let mut cpu = Cpu::new();
            cpu.set_flag(Cpu::CARRY_FLAG, true);
            cpu.registers[0] = 0x0F;
            cpu.load_program(&[Opcode::AddCarryLongImm as u8, 0xF0, 0xFF, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0x10000);
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
        fn sub_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;
            cpu.load_program(&[Opcode::SubShortReg as u8, 1]);
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
        fn sub_borrow_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubBorrowByteImm as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn sub_borrow_byte_imm_carry_set() {
            let mut cpu = Cpu::new();
            cpu.set_flag(Cpu::CARRY_FLAG, true);
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubBorrowByteImm as u8, 49]);
            cpu.run();

            assert_eq!(cpu.registers[0], 50);
        }
        #[test]
        fn sub_borrow_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubBorrowShortImm as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn sub_borrow_short_imm_carry_set() {
            let mut cpu = Cpu::new();
            cpu.set_flag(Cpu::CARRY_FLAG, true);
            cpu.registers[0] = 0xFFFF;
            cpu.load_program(&[Opcode::SubBorrowShortImm as u8, 0xFF, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0xFEFF);
        }
        #[test]
        fn sub_borrow_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubBorrowLongImm as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn sub_borrow_long_imm_carry_set() {
            let mut cpu = Cpu::new();
            cpu.set_flag(Cpu::CARRY_FLAG, true);
            cpu.registers[0] = 0xFFFFFFFF;
            cpu.load_program(&[Opcode::SubBorrowLongImm as u8, 0xFF, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0xFFFFFEFF);
        }
        #[test]
        fn sub_borrow_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;
            cpu.load_program(&[Opcode::SubBorrowByteReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 50);
        }
        #[test]
        fn sub_borrow_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;
            cpu.load_program(&[Opcode::SubBorrowShortReg as u8, 1]);
            cpu.run();

            assert_eq!(cpu.registers[0], 50);
        }
        #[test]
        fn sub_borrow_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 50;
            cpu.load_program(&[Opcode::SubBorrowLongReg as u8, 1]);
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
        #[test]
        fn singed_mul_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = -2_i32 as u32;
            cpu.load_program(&[Opcode::SignedMulByteImm as u8, 10]);
            cpu.run();

            assert_eq!(cpu.registers[0] as i32, -20);
        }
        #[test]
        fn singed_mul_short() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = -2_i32 as u32;
            cpu.load_program(&[Opcode::SignedMulShortImm as u8, 0, 10]);
            cpu.run();

            assert_eq!(cpu.registers[0] as i32, -5120);
        }
        #[test]
        fn singed_mul_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = -2_i32 as u32;
            cpu.load_program(&[Opcode::SignedMulLongImm as u8, 0, 0, 0, 10]);
            cpu.run();

            assert_eq!(cpu.registers[0] as i32, -335544320);
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

        #[test]
        fn singed_div_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = -10_i32 as u32;
            cpu.load_program(&[Opcode::SignedDivByteImm as u8, 2]);
            cpu.run();

            assert_eq!(cpu.registers[0] as i32, -5);
            assert_eq!(cpu.registers[1], 0);
        }
        #[test]
        fn singed_div_short() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = -10_i32 as u32;
            cpu.load_program(&[Opcode::SignedDivShortImm as u8, 2, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0] as i32, -5);
            assert_eq!(cpu.registers[1], 0);
        }
        #[test]
        fn singed_div_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = -10_i32 as u32;
            cpu.load_program(&[Opcode::SignedDivLongImm as u8, 2, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0] as i32, -5);
            assert_eq!(cpu.registers[1], 0);
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
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], jmp_addr);
        }

        #[test]
        fn jne_negative() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 10;
            let mut program = vec![];
            program.push(Opcode::JumpNotEqual as u8);
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
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
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], jmp_addr);
        }

        #[test]
        fn je_negative() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 11;
            let mut program = vec![];
            program.push(Opcode::JumpEqual as u8);
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
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
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], jmp_addr);
        }
        #[test]
        fn jl_when_eq() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 10;
            let mut program = vec![];
            program.push(Opcode::JumpLess as u8);
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
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
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
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
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
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
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
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
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], jmp_addr);
        }

        #[test]
        fn jle_when_less() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 11;
            let mut program = vec![];
            program.push(Opcode::JumpLessEqual as u8);
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], jmp_addr);
        }
        #[test]
        fn jle_when_eq() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 10;
            let mut program = vec![];
            program.push(Opcode::JumpLessEqual as u8);
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], jmp_addr);
        }
        #[test]
        fn jle_when_gr() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 9;
            let mut program = vec![];
            program.push(Opcode::JumpLessEqual as u8);
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], no_jmp_addr);
        }

        #[test]
        fn jge_when_less() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 11;
            let mut program = vec![];
            program.push(Opcode::JumpGreaterEqual as u8);
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], no_jmp_addr);
        }
        #[test]
        fn jge_when_eq() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 10;
            let mut program = vec![];
            program.push(Opcode::JumpGreaterEqual as u8);
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            let no_jmp_addr = program.len() as u32;
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], jmp_addr);
        }
        #[test]
        fn jge_when_gr() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.registers[1] = 9;
            let mut program = vec![];
            program.push(Opcode::JumpGreaterEqual as u8);
            let jmp_addr = 10 as u32;
            program.extend_from_slice(&jmp_addr.to_le_bytes());
            cpu.load_program(program.as_slice());
            cpu.cycle();
            assert_eq!(cpu.registers[crate::cpu::IP], jmp_addr);
        }
    }

    mod compare {
        use crate::{cpu::Cpu, opcodes::Opcode};
        #[test]
        fn compare_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::CompareByteImm as u8, 100]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 1);
        }
        #[test]
        fn compare_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 511;
            cpu.load_program(&[Opcode::CompareShortImm as u8, 255, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 1);
        }
        #[test]
        fn compare_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFF_FF_FF_FF;
            cpu.load_program(&[Opcode::CompareLongImm as u8, 0xFF, 0xFF, 0xFF, 0xFF]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 1);
        }
        #[test]
        fn compare_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 100;
            cpu.load_program(&[Opcode::CompareByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 1);
        }
        #[test]
        fn compare_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 511;
            cpu.registers[1] = 511;
            cpu.load_program(&[Opcode::CompareShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 1);
        }
        #[test]
        fn compare_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFF_FF_FF_FF;
            cpu.registers[1] = 0xFF_FF_FF_FF;
            cpu.load_program(&[Opcode::CompareLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 1);
        }
        #[test]
        fn compare_byte_imm_neg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0;
            cpu.load_program(&[Opcode::CompareByteImm as u8, 100]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn compare_short_imm_neg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0;
            cpu.load_program(&[Opcode::CompareShortImm as u8, 255, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn compare_long_imm_neg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0;
            cpu.load_program(&[Opcode::CompareLongImm as u8, 0xFF, 0xFF, 0xFF, 0xFF]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn compare_byte_reg_neg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.registers[1] = 0;
            cpu.load_program(&[Opcode::CompareByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn compare_short_reg_neg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 511;
            cpu.registers[1] = 0;
            cpu.load_program(&[Opcode::CompareShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn compare_long_reg_neg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFF_FF_FF_FF;
            cpu.registers[1] = 0;
            cpu.load_program(&[Opcode::CompareLongReg as u8, 0]);
            cpu.cycle();
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
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xCC);
        }

        #[test]
        fn and_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFFFFFF;
            cpu.load_program(&[Opcode::AndLongImm as u8, 0xCC, 0xCC, 0xCC, 0xCC]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xCCCCCCCC);
        }

        #[test]
        fn and_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFF;
            cpu.load_program(&[Opcode::AndShortImm as u8, 0xCC, 0xCC]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xCCCC);
        }

        #[test]
        fn and_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFF;
            cpu.registers[1] = 0xCC;
            cpu.load_program(&[Opcode::AndByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xCC);
        }

        #[test]
        fn and_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFFFFFF;
            cpu.registers[1] = 0xCCCCCCCC;
            cpu.load_program(&[Opcode::AndLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xCCCCCCCC);
        }

        #[test]
        fn and_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFF;
            cpu.registers[1] = 0xCCCC;
            cpu.load_program(&[Opcode::AndShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xCCCC);
        }
    }

    mod or {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn or_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFA;
            cpu.registers[1] = 0x55;
            cpu.load_program(&[Opcode::OrByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xFF);
        }

        #[test]
        fn or_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFAFA;
            cpu.registers[1] = 0x5555;
            cpu.load_program(&[Opcode::OrShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xFFFF);
        }

        #[test]
        fn or_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFAFAFAFA;
            cpu.registers[1] = 0x55555555;
            cpu.load_program(&[Opcode::OrLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xFFFFFFFF);
        }

        #[test]
        fn or_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFA;
            cpu.load_program(&[Opcode::OrByteImm as u8, 0x55]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xFF);
        }

        #[test]
        fn or_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFAFA;
            cpu.load_program(&[Opcode::OrShortImm as u8, 0x55, 0x55]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xFFFF);
        }

        #[test]
        fn or_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFAFAFAFA;
            cpu.load_program(&[Opcode::OrLongImm as u8, 0x55, 0x55, 0x55, 0x55]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xFFFFFFFF);
        }
    }

    mod xor {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn xor_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFA;
            cpu.registers[1] = 0x55;
            cpu.load_program(&[Opcode::XorByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xAF);
        }

        #[test]
        fn xor_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFAFA;
            cpu.registers[1] = 0x5555;
            cpu.load_program(&[Opcode::XorShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xAFAF);
        }

        #[test]
        fn xor_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFAFAFAFA;
            cpu.registers[1] = 0x55555555;
            cpu.load_program(&[Opcode::XorLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xAFAFAFAF);
        }

        #[test]
        fn xor_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFA;
            cpu.load_program(&[Opcode::XorByteImm as u8, 0x55]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xAF);
        }

        #[test]
        fn xor_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFAFA;
            cpu.load_program(&[Opcode::XorShortImm as u8, 0x55, 0x55]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xAFAF);
        }

        #[test]
        fn xor_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFAFAFAFA;
            cpu.load_program(&[Opcode::XorLongImm as u8, 0x55, 0x55, 0x55, 0x55]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xAFAFAFAF);
        }
    }

    mod logical_shift {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn left_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0F;
            cpu.load_program(&[Opcode::LogShiftLeftByteImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1E);
        }

        #[test]
        fn left_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0FFF;
            cpu.load_program(&[Opcode::LogShiftLeftShortImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1FFE);
        }

        #[test]
        fn left_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0FFFFFFF;
            cpu.load_program(&[Opcode::LogShiftLeftLongImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1FFFFFFE);
        }

        #[test]
        fn left_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0F;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::LogShiftLeftByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1E);
        }

        #[test]
        fn left_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0FFF;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::LogShiftLeftShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1FFE);
        }

        #[test]
        fn left_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0FFFFFFF;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::LogShiftLeftLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1FFFFFFE);
        }

        #[test]
        fn right_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1E;
            cpu.load_program(&[Opcode::LogShiftRightByteImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0F);
        }

        #[test]
        fn right_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1FFE;
            cpu.load_program(&[Opcode::LogShiftRightShortImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0FFF);
        }

        #[test]
        fn right_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1FFFFFFE;
            cpu.load_program(&[Opcode::LogShiftRightLongImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0FFFFFFF);
        }

        #[test]
        fn right_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1E;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::LogShiftRightByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0F);
        }

        #[test]
        fn right_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1FFE;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::LogShiftRightShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0FFF);
        }

        #[test]
        fn right_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1FFFFFFE;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::LogShiftRightLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0FFFFFFF);
        }
    }
    mod arith_shift {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn left_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0F;
            cpu.load_program(&[Opcode::ArithShiftLeftByteImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1E);
        }

        #[test]
        fn left_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0FFF;
            cpu.load_program(&[Opcode::ArithShiftLeftShortImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1FFE);
        }

        #[test]
        fn left_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0FFFFFFF;
            cpu.load_program(&[Opcode::ArithShiftLeftLongImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1FFFFFFE);
        }

        #[test]
        fn left_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0F;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::ArithShiftLeftByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1E);
        }

        #[test]
        fn left_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0FFF;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::ArithShiftLeftShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1FFE);
        }

        #[test]
        fn left_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x0FFFFFFF;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::ArithShiftLeftLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x1FFFFFFE);
        }

        #[test]
        fn right_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1E;
            cpu.load_program(&[Opcode::ArithShiftRightByteImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0F);
        }

        #[test]
        fn right_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1FFE;
            cpu.load_program(&[Opcode::ArithShiftRightShortImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0FFF);
        }

        #[test]
        fn right_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1FFFFFFE;
            cpu.load_program(&[Opcode::ArithShiftRightLongImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0FFFFFFF);
        }

        #[test]
        fn right_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1E;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::ArithShiftRightByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0F);
        }

        #[test]
        fn right_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1FFE;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::ArithShiftRightShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0FFF);
        }

        #[test]
        fn right_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x1FFFFFFE;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::ArithShiftRightLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0FFFFFFF);
        }
    }
    mod rotate {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn left_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x85;
            cpu.load_program(&[Opcode::RotateLeftByteImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0B);
        }

        #[test]
        fn left_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x8001;
            cpu.load_program(&[Opcode::RotateLeftShortImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0003);
        }

        #[test]
        fn left_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x80000001;
            cpu.load_program(&[Opcode::RotateLeftLongImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x00000003);
        }

        #[test]
        fn left_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x85;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::RotateLeftByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0B);
        }

        #[test]
        fn left_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x8001;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::RotateLeftShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x0003);
        }

        #[test]
        fn left_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x80000001;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::RotateLeftLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0x00000003);
        }

        #[test]
        fn right_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x85;
            cpu.load_program(&[Opcode::RotateRightByteImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xC2);
        }

        #[test]
        fn right_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x8001;
            cpu.load_program(&[Opcode::RotateRightShortImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xC000);
        }

        #[test]
        fn right_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x80000001;
            cpu.load_program(&[Opcode::RotateRightLongImm as u8, 0x01]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xC0000000);
        }

        #[test]
        fn right_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x85;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::RotateRightByteReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xC2);
        }

        #[test]
        fn right_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x8001;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::RotateRightShortReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xC000);
        }

        #[test]
        fn right_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0x80000001;
            cpu.registers[1] = 0x01;
            cpu.load_program(&[Opcode::RotateRightLongReg as u8, 1]);
            cpu.cycle();
            assert_eq!(cpu.registers[0], 0xC0000000);
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
            // TODO: we need to improve this test. when I try to make it more comprehensive
            // it just totally fails. its probably me just writing it out wrong, but still.
            let mut cpu = Cpu::new();

            // address of isr_table.
            cpu.registers[IDT] = 1;

            cpu.load_program(&[
                Opcode::Interrupt as u8,         // 0  ; int 0 
                Opcode::InterruptReturn as u8,  // 1  ; iret
            ]);

            cpu.run();
            assert_eq!(cpu.ip(), 3);
            assert_eq!((cpu.registers[FLAGS] & Cpu::INTERRUPT_FLAG as u32), 0);
        }
    }

    mod mov {
        const SRC_VAL: u32 = 0xCAFEC0DE;
        const DST_REG: usize = 0;
        const SRC_REG: usize = 1;
        const DST_ADR: usize = 10;
        const SRC_ADR: usize = 20;
        use std::marker::PhantomData;

        use crate::{
            cpu::Cpu,
            opcodes::Opcode,
            test::le_bytes_junk::{OurFrom, ToLeTrait},
        };
        struct TestBuilder<T> {
            cpu: Cpu,
            op: u8,
            src: Vec<u8>,
            dst: Vec<u8>,
            _marker: PhantomData<T>,
        }
        impl<T> TestBuilder<T>
        where
            T: Copy + std::fmt::Debug + OurFrom<u32> + ToLeTrait + Eq + Into<u32>,
        {
            fn new(op: Opcode) -> TestBuilder<T> {
                TestBuilder {
                    cpu: Cpu::new(),
                    op: op as u8,
                    src: vec![],
                    dst: vec![],
                    _marker: PhantomData,
                }
            }
            fn run(&mut self) -> &mut Self {
                let mut progam = vec![self.op];
                progam.extend_from_slice(&self.dst);
                progam.extend_from_slice(&self.src);
                self.cpu.load_program(&progam);
                self.cpu.cycle();
                self
            }
            fn assert_reg_eq(&mut self) {
                let dst_val = T::our_from(self.cpu.registers[DST_REG]);
                assert_eq!(dst_val, T::our_from(SRC_VAL));
            }
            fn assert_adr_eq(&mut self) {
                let dst_val = T::our_from(self.cpu.memory.long(DST_ADR));
                assert_eq!(dst_val, T::our_from(SRC_VAL));
            }
            fn set_dst_reg(&mut self) -> &mut Self {
                self.dst.clear();
                self.dst.push(DST_REG as u8);
                self
            }
            fn set_dst_abs(&mut self) -> &mut Self {
                self.dst.clear();
                self.dst.extend_from_slice(&(DST_ADR as u32).to_le_bytes());
                self
            }
            fn set_dst_mem(&mut self, rel_to: usize) -> &mut Self {
                self.dst.clear();
                self.dst
                    .extend_from_slice(&((DST_ADR - rel_to) as u32).to_le_bytes());
                self
            }
            fn set_dst_ind(&mut self) -> &mut Self {
                self.dst.clear();
                self.dst.push(DST_REG as u8);
                self.cpu.registers[DST_REG] = DST_ADR as u32;
                self
            }
            fn set_src_imm(&mut self) -> &mut Self {
                let val = T::our_from(SRC_VAL);
                self.src.clear();
                self.src.extend_from_slice(&val.to_le_bytes());
                self
            }
            fn set_src_reg(&mut self) -> &mut Self {
                self.src.clear();
                self.src.push(SRC_REG as u8);
                self.cpu.registers[SRC_REG] = T::our_from(SRC_VAL).into();
                self
            }
            fn set_src_abs(&mut self) -> &mut Self {
                self.src.clear();
                self.src.extend_from_slice(&(SRC_ADR as u32).to_le_bytes());
                let val_bytes = T::our_from(SRC_VAL).to_le_bytes();
                self.cpu
                    .memory
                    .buffer
                    .splice(SRC_ADR..SRC_ADR + val_bytes.len(), val_bytes);
                self
            }
            fn set_src_mem(&mut self, rel_to: usize) -> &mut Self {
                self.src.clear();
                self.src
                    .extend_from_slice(&((SRC_ADR - rel_to) as u32).to_le_bytes());
                let val_bytes = T::our_from(SRC_VAL).to_le_bytes();
                self.cpu
                    .memory
                    .buffer
                    .splice(SRC_ADR..SRC_ADR + val_bytes.len(), val_bytes);
                self
            }
            fn set_src_ind(&mut self) -> &mut Self {
                self.src.clear();
                self.src.push(SRC_REG as u8);
                self.cpu.registers[SRC_REG] = SRC_ADR as u32;
                let val_bytes = T::our_from(SRC_VAL).to_le_bytes();
                self.cpu
                    .memory
                    .buffer
                    .splice(SRC_ADR..SRC_ADR + val_bytes.len(), val_bytes);
                self
            }
        }
        mod to_reg {
            use super::TestBuilder;
            use crate::opcodes::Opcode;

            #[test]
            fn from_imm_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveImmRegByte);
                builder.set_dst_reg().set_src_imm().run().assert_reg_eq();
            }
            #[test]
            fn from_imm_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveImmRegShort);
                builder.set_dst_reg().set_src_imm().run().assert_reg_eq();
            }
            #[test]
            fn from_imm_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveImmRegLong);
                builder.set_dst_reg().set_src_imm().run().assert_reg_eq();
            }
            #[test]
            fn from_reg_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveRegRegByte);
                builder.set_dst_reg().set_src_reg().run().assert_reg_eq();
            }
            #[test]
            fn from_reg_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveRegRegShort);
                builder.set_dst_reg().set_src_reg().run().assert_reg_eq();
            }
            #[test]
            fn from_reg_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveRegRegLong);
                builder.set_dst_reg().set_src_reg().run().assert_reg_eq();
            }
            #[test]
            fn from_abs_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveAbsRegByte);
                builder.set_dst_reg().set_src_abs().run().assert_reg_eq();
            }
            #[test]
            fn from_abs_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveAbsRegShort);
                builder.set_dst_reg().set_src_abs().run().assert_reg_eq();
            }
            #[test]
            fn from_abs_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveAbsRegLong);
                builder.set_dst_reg().set_src_abs().run().assert_reg_eq();
            }
            #[test]
            fn from_mem_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveMemRegByte);
                builder.set_dst_reg().set_src_mem(6).run().assert_reg_eq();
            }
            #[test]
            fn from_mem_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveMemRegShort);
                builder.set_dst_reg().set_src_mem(6).run().assert_reg_eq();
            }
            #[test]
            fn from_mem_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveMemRegLong);
                builder.set_dst_reg().set_src_mem(6).run().assert_reg_eq();
            }
            #[test]
            fn from_ind_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveIndirectRegByte);
                builder.set_dst_reg().set_src_ind().run().assert_reg_eq();
            }
            #[test]
            fn from_ind_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveIndirectRegShort);
                builder.set_dst_reg().set_src_ind().run().assert_reg_eq();
            }
            #[test]
            fn from_ind_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveIndirectRegLong);
                builder.set_dst_reg().set_src_ind().run().assert_reg_eq();
            }
        }
        mod to_abs {
            use super::TestBuilder;
            use crate::opcodes::Opcode;

            #[test]
            fn from_imm_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveImmAbsByte);
                builder.set_dst_abs().set_src_imm().run().assert_adr_eq();
            }
            #[test]
            fn from_imm_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveImmAbsShort);
                builder.set_dst_abs().set_src_imm().run().assert_adr_eq();
            }
            #[test]
            fn from_imm_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveImmAbsLong);
                builder.set_dst_abs().set_src_imm().run().assert_adr_eq();
            }
            #[test]
            fn from_reg_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveRegAbsByte);
                builder.set_dst_abs().set_src_reg().run().assert_adr_eq();
            }
            #[test]
            fn from_reg_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveRegAbsShort);
                builder.set_dst_abs().set_src_reg().run().assert_adr_eq();
            }
            #[test]
            fn from_reg_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveRegAbsLong);
                builder.set_dst_abs().set_src_reg().run().assert_adr_eq();
            }
            #[test]
            fn from_abs_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveAbsAbsByte);
                builder.set_dst_abs().set_src_abs().run().assert_adr_eq();
            }
            #[test]
            fn from_abs_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveAbsAbsShort);
                builder.set_dst_abs().set_src_abs().run().assert_adr_eq();
            }
            #[test]
            fn from_abs_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveAbsAbsLong);
                builder.set_dst_abs().set_src_abs().run().assert_adr_eq();
            }
            #[test]
            fn from_mem_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveMemAbsByte);
                builder.set_dst_abs().set_src_mem(9).run().assert_adr_eq();
            }
            #[test]
            fn from_mem_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveMemAbsShort);
                builder.set_dst_abs().set_src_mem(9).run().assert_adr_eq();
            }
            #[test]
            fn from_mem_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveMemAbsLong);
                builder.set_dst_abs().set_src_mem(9).run().assert_adr_eq();
            }
            #[test]
            fn from_ind_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveIndirectAbsByte);
                builder.set_dst_abs().set_src_ind().run().assert_adr_eq();
            }
            #[test]
            fn from_ind_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveIndirectAbsShort);
                builder.set_dst_abs().set_src_ind().run().assert_adr_eq();
            }
            #[test]
            fn from_ind_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveIndirectAbsLong);
                builder.set_dst_abs().set_src_ind().run().assert_adr_eq();
            }
        }
        mod to_mem {
            use super::TestBuilder;
            use crate::opcodes::Opcode;

            #[test]
            fn from_imm_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveImmMemByte);
                builder.set_dst_mem(6).set_src_imm().run().assert_adr_eq();
            }
            #[test]
            fn from_imm_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveImmMemShort);
                builder.set_dst_mem(7).set_src_imm().run().assert_adr_eq();
            }
            #[test]
            fn from_imm_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveImmMemLong);
                builder.set_dst_mem(9).set_src_imm().run().assert_adr_eq();
            }
            #[test]
            fn from_reg_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveRegMemByte);
                builder.set_dst_mem(6).set_src_reg().run().assert_adr_eq();
            }
            #[test]
            fn from_reg_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveRegMemShort);
                builder.set_dst_mem(6).set_src_reg().run().assert_adr_eq();
            }
            #[test]
            fn from_reg_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveRegMemLong);
                builder.set_dst_mem(6).set_src_reg().run().assert_adr_eq();
            }
            #[test]
            fn from_abs_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveAbsMemByte);
                builder.set_dst_mem(9).set_src_abs().run().assert_adr_eq();
            }
            #[test]
            fn from_abs_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveAbsMemShort);
                builder.set_dst_mem(9).set_src_abs().run().assert_adr_eq();
            }
            #[test]
            fn from_abs_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveAbsMemLong);
                builder.set_dst_mem(9).set_src_abs().run().assert_adr_eq();
            }
            #[test]
            fn from_mem_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveMemMemByte);
                builder.set_dst_mem(9).set_src_mem(9).run().assert_adr_eq();
            }
            #[test]
            fn from_mem_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveMemMemShort);
                builder.set_dst_mem(9).set_src_mem(9).run().assert_adr_eq();
            }
            #[test]
            fn from_mem_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveMemMemLong);
                builder.set_dst_mem(9).set_src_mem(9).run().assert_adr_eq();
            }
            #[test]
            fn from_ind_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveIndirectMemByte);
                builder.set_dst_mem(6).set_src_ind().run().assert_adr_eq();
            }
            #[test]
            fn from_ind_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveIndirectMemShort);
                builder.set_dst_mem(6).set_src_ind().run().assert_adr_eq();
            }
            #[test]
            fn from_ind_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveIndirectMemLong);
                builder.set_dst_mem(6).set_src_ind().run().assert_adr_eq();
            }
        }
        mod to_ind {
            use super::TestBuilder;
            use crate::opcodes::Opcode;

            #[test]
            fn from_imm_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveImmIndirectByte);
                builder.set_dst_ind().set_src_imm().run().assert_adr_eq();
            }
            #[test]
            fn from_imm_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveImmIndirectShort);
                builder.set_dst_ind().set_src_imm().run().assert_adr_eq();
            }
            #[test]
            fn from_imm_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveImmIndirectLong);
                builder.set_dst_ind().set_src_imm().run().assert_adr_eq();
            }
            #[test]
            fn from_reg_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveRegIndirectByte);
                builder.set_dst_ind().set_src_reg().run().assert_adr_eq();
            }
            #[test]
            fn from_reg_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveRegIndirectShort);
                builder.set_dst_ind().set_src_reg().run().assert_adr_eq();
            }
            #[test]
            fn from_reg_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveRegIndirectLong);
                builder.set_dst_ind().set_src_reg().run().assert_adr_eq();
            }
            #[test]
            fn from_abs_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveAbsIndirectByte);
                builder.set_dst_ind().set_src_abs().run().assert_adr_eq();
            }
            #[test]
            fn from_abs_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveAbsIndirectShort);
                builder.set_dst_ind().set_src_abs().run().assert_adr_eq();
            }
            #[test]
            fn from_abs_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveAbsIndirectLong);
                builder.set_dst_ind().set_src_abs().run().assert_adr_eq();
            }
            #[test]
            fn from_mem_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveMemIndirectByte);
                builder.set_dst_ind().set_src_mem(6).run().assert_adr_eq();
            }
            #[test]
            fn from_mem_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveMemIndirectShort);
                builder.set_dst_ind().set_src_mem(6).run().assert_adr_eq();
            }
            #[test]
            fn from_mem_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveMemIndirectLong);
                builder.set_dst_ind().set_src_mem(6).run().assert_adr_eq();
            }
            #[test]
            fn from_ind_byte() {
                let mut builder = TestBuilder::<u8>::new(Opcode::MoveIndirectIndirectByte);
                builder.set_dst_ind().set_src_ind().run().assert_adr_eq();
            }
            #[test]
            fn from_ind_short() {
                let mut builder = TestBuilder::<u16>::new(Opcode::MoveIndirectIndirectShort);
                builder.set_dst_ind().set_src_ind().run().assert_adr_eq();
            }
            #[test]
            fn from_ind_long() {
                let mut builder = TestBuilder::<u32>::new(Opcode::MoveIndirectIndirectLong);
                builder.set_dst_ind().set_src_ind().run().assert_adr_eq();
            }
        }
    }
}
