#[cfg(test)]
mod tests {
    
    mod stack_tests {
        use crate::{cpu::{Cpu, SP}, opcodes::Opcode};
        
        fn create_cpu() -> Cpu {
            let mut cpu = Cpu::new();
            cpu.registers[SP] = 50;
            return cpu;
        }
        
        #[test]
        fn test_push_byte_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 1;
            
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::PushByteReg as u8, 0]);
            cpu.run();
            
            assert_eq!(cpu.memory.byte(expected_sp), 100);
        }
        #[test]
        fn test_push_byte_imm() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 1;
            
            cpu.load_program(&[Opcode::PushByteImm as u8, 100]);
            cpu.run();
            
            assert_eq!(cpu.memory.byte(expected_sp), 100);
        }
        #[test]
        fn test_push_byte_mem() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 1;
            cpu.memory.set_byte(100, 0xFF);
            
            cpu.load_program(&[Opcode::PushByteMem as u8, 100]);
            cpu.run();
            
            assert_eq!(cpu.memory.byte(expected_sp), 0xFF);
        }
        
        #[test]
        fn test_push_short_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 2;
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::PushShortReg as u8, 0]);
            cpu.run();

            assert_eq!(cpu.memory.short(expected_sp), 100);
        }

        #[test]
        fn test_push_short_imm() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 2;
            cpu.load_program(&[Opcode::PushShortImm as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.memory.short(expected_sp), 100);
        }

        #[test]
        fn test_push_short_mem() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 2;
            cpu.memory.set_short(100, 0xFF);
            
            cpu.load_program(&[Opcode::PushShortMem as u8, 100, 0]);
            cpu.run();
            
            assert_eq!(cpu.memory.short(expected_sp), 0xFF);
        }
        
        #[test]
        fn test_push_long_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 4;
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::PushLongReg as u8, 0, 0, 0]);
            cpu.run();
            
            assert_eq!(cpu.memory.long(expected_sp), 100);
        }
        
        #[test]
        fn test_push_long_imm() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 4;
            cpu.load_program(&[Opcode::PushLongImm as u8, 100, 0, 0, 0]);
            cpu.run();
            
            assert_eq!(cpu.memory.long(expected_sp), 100);
        }
        
        #[test]
        fn test_push_long_mem() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() - 4;
            
            cpu.memory.set_long(100, 0xFF_FF_FF_FF);
            
            cpu.load_program(&[Opcode::PushLongMem as u8, 100, 0, 0, 0]);
            cpu.run();
            
            
            assert_eq!(cpu.memory.long(expected_sp), 0xFF_FF_FF_FF);
        }
        
        
        #[test]
        fn test_pop_byte_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 1;
            cpu.memory.set_byte(cpu.sp(), 100);
            
            cpu.load_program(&[Opcode::PopByteReg as u8, 0]);
            cpu.run();
            
            assert_eq!(cpu.registers[0], 100);
            assert_eq!(cpu.sp(), expected_sp);
        }
        
        #[test]
        fn test_pop_byte_mem() {
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
        fn test_pop_short_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 2;
            cpu.memory.set_short(cpu.sp(), 100);
            
            cpu.load_program(&[Opcode::PopShortReg as u8, 0]);
            cpu.run();
            
            assert_eq!(cpu.registers[0], 100);
            assert_eq!(cpu.sp(), expected_sp);
        }
        
        #[test]
        fn test_pop_short_mem() {
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
        fn test_pop_long_reg() {
            let mut cpu = create_cpu();
            let expected_sp = cpu.sp() + 4;
            cpu.memory.set_long(cpu.sp(), 100);
            
            cpu.load_program(&[Opcode::PopLongReg as u8, 0, 0, 0]);
            cpu.run();
            
            assert_eq!(cpu.registers[0], 100);
            assert_eq!(cpu.sp(), expected_sp);
        }
        
        #[test]
        fn test_pop_long_mem() {
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
    
    mod general_tests {
        use crate::cpu::Cpu;

        #[test]
        fn test_hlt() {
            let mut cpu = Cpu::new();
            cpu.load_program(&[0]);
            cpu.run();
            
            assert_eq!(cpu.ip(), 1);
            assert_eq!((cpu.flags() & Cpu::HALT_FLAG), Cpu::HALT_FLAG);
        }
    }
    
    mod arith_tests {
        use crate::{cpu::Cpu, opcodes::Opcode};
        #[test]
        fn test_add_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddByte as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 200);
        }
        #[test]
        fn test_add_byte_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 156;
            cpu.load_program(&[Opcode::AddByte as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn test_sub_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubByte as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn test_sub_byte_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0;
            cpu.load_program(&[Opcode::SubByte as u8, 100]);
            cpu.run();

            assert_eq!(cpu.registers[0], 156);
        }
        #[test]
        fn test_mul_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2;
            cpu.load_program(&[Opcode::MulByte as u8, 10]);
            cpu.run();

            assert_eq!(cpu.registers[0], 20);
        }
        #[test]
        fn test_mul_byte_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::MulByte as u8, 100]);
            cpu.run();

            let result = 100u8.wrapping_mul(100u8);
            assert_eq!((cpu.registers[0] & 0xFF) as u8, result);
        }
        #[test]
        fn test_div_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.load_program(&[Opcode::DivByte as u8, 2]);
            cpu.run();

            assert_eq!(cpu.registers[0], 5);
            assert_eq!(cpu.registers[1], 0);
        }
        #[test]
        fn test_div_byte_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 1000;
            cpu.load_program(&[Opcode::DivByte as u8, 2]);
            cpu.run();
            // 1000 as a byte wraps to 232
            let result = 232u8.wrapping_div(2);
            assert_eq!((cpu.registers[0] & 0xFF) as u8, result);
            assert_eq!((cpu.registers[1] & 0xFF) as u8, 0);
        }
        #[test]
        fn test_div_byte_remainder() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 9;
            cpu.load_program(&[Opcode::DivByte as u8, 2]);
            cpu.run();
            assert_eq!((cpu.registers[0] & 0xFF) as u8, 4);
            assert_eq!((cpu.registers[1] & 0xFF) as u8, 1);
        }

        #[test]
        fn test_add_short() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddShort as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 200);
        }
        #[test]
        fn test_add_short_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 65535;
            cpu.load_program(&[Opcode::AddShort as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 99);
        }
        #[test]
        fn test_sub_short() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubShort as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }
        #[test]
        fn test_sub_short_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0;
            cpu.load_program(&[Opcode::SubShort as u8, 100, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 65536 - 100);
        }
        #[test]
        fn test_mul_short() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2;
            cpu.load_program(&[Opcode::MulShort as u8, 10, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 20);
        }
        #[test]
        fn test_mul_short_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 70_000;
            cpu.load_program(&[Opcode::MulShort as u8, 100, 0]);
            cpu.run();

            let result = 4_464u16.wrapping_mul(100u16);
            assert_eq!((cpu.registers[0] & 0xFFFF) as u16, result);
        }
        #[test]
        fn test_div_short() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.load_program(&[Opcode::DivShort as u8, 2, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 5);
            assert_eq!(cpu.registers[1], 0);
        }
        #[test]
        fn test_div_short_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 70_000; // Value larger than u16::MAX
            cpu.load_program(&[Opcode::DivShort as u8, 2, 0]);
            cpu.run();
            // 70,000 as a u16 wraps to 4,464
            let result = 4_464u16 / 2;
            assert_eq!((cpu.registers[0] & 0xFFFF) as u16, result);
            assert_eq!(cpu.registers[1], 0);
        }
        #[test]
        fn test_div_short_remainder() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 9;
            cpu.load_program(&[Opcode::DivShort as u8, 2, 0]);
            cpu.run();
            assert_eq!(cpu.registers[0], 4);
            assert_eq!(cpu.registers[1], 1);
        }
        
        #[test]
        fn test_add_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::AddLong as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 200);
        }

        #[test]
        fn test_add_long_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 4_294_967_295;
            cpu.load_program(&[Opcode::AddLong as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 99);
        }

        #[test]
        fn test_sub_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 100;
            cpu.load_program(&[Opcode::SubLong as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 0);
        }

        #[test]
        fn test_sub_long_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0;
            cpu.load_program(&[Opcode::SubLong as u8, 100, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 4_294_967_295 - 99);
        }

        #[test]
        fn test_mul_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2;
            cpu.load_program(&[Opcode::MulLong as u8, 10, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 20);
        }

        #[test]
        fn test_mul_long_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2_000_000_000;
            cpu.load_program(&[Opcode::MulLong as u8, 100, 0, 0, 0]);
            cpu.run();

            let result = 2_000_000_000u32.wrapping_mul(100);
            assert_eq!(cpu.registers[0], result);
        }

        #[test]
        fn test_div_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.load_program(&[Opcode::DivLong as u8, 2, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 5);
            assert_eq!(cpu.registers[1], 0);
        }

        #[test]
        fn test_div_long_wrap() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 2_000_000_000;
            cpu.load_program(&[Opcode::DivLong as u8, 2, 0, 0, 0]);
            cpu.run();

            let result = 1_000_000_000u32;
            assert_eq!(cpu.registers[0], result);
            assert_eq!(cpu.registers[1], 0);
        }
        
        #[test]
        fn test_div_long_remainder() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 9;
            cpu.load_program(&[Opcode::DivLong as u8, 2, 0, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[0], 4);
            assert_eq!(cpu.registers[1], 1);
        }
    }
    
    mod and_tests {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn test_and_byte_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFF;
            cpu.load_program(&[Opcode::AndByteImm as u8, 0xCC]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCC);
        }

        #[test]
        fn test_and_byte_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFF;
            cpu.registers[1] = 0xCC;
            cpu.load_program(&[Opcode::AndByteReg as u8, 1]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCC);
        }

        #[test]
        fn test_and_byte_mem() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFF;
            cpu.memory.buffer[100] = 0xCC;
            cpu.load_program(&[Opcode::AndByteMem as u8, 100]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCC);
        }

        #[test]
        fn test_and_long_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFFFFFF;
            cpu.load_program(&[Opcode::AndLongImm as u8, 0xCC, 0xCC, 0xCC, 0xCC]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCCCCCCCC);
        }

        #[test]
        fn test_and_long_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFFFFFF;
            cpu.registers[1] = 0xCCCCCCCC;
            cpu.load_program(&[Opcode::AndLongReg as u8, 1]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCCCCCCCC);
        }

        #[test]
        fn test_and_long_mem() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFFFFFF;
            cpu.memory.set_long(100, 0xCCCCCCCC);
            cpu.load_program(&[Opcode::AndLongMem as u8, 100]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCCCCCCCC);
        }

        #[test]
        fn test_and_short_imm() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFF;
            cpu.load_program(&[Opcode::AndShortImm as u8, 0xCC, 0xCC]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCCCC);
        }

        #[test]
        fn test_and_short_reg() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFF;
            cpu.registers[1] = 0xCCCC;
            cpu.load_program(&[Opcode::AndShortReg as u8,  1]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCCCC);
        }

        #[test]
        fn test_and_short_mem() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xFFFF;
            cpu.memory.set_short(100, 0xCCCC);
            cpu.load_program(&[Opcode::AndShortMem as u8, 100]);
            cpu.run();
            assert_eq!(cpu.registers[0], 0xCCCC);
        }
    }
    
    
    mod mov_tests {
        use crate::{cpu::Cpu, opcodes::Opcode};

        #[test]
        fn test_mov_reg_reg_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[1] = 10;
            cpu.load_program(&[Opcode::MoveRegRegByte as u8, 0, 1]);
            cpu.run();
            assert_eq!(cpu.registers[0], cpu.registers[1]);
            assert_eq!(cpu.ip(), 4);
        }
        #[test]
        fn test_mov_reg_mem_byte() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 10;
            cpu.load_program(&[Opcode::MoveRegMemByte as u8, 100, 0]);
            cpu.run();
            assert_eq!(cpu.memory.buffer[100], 10);
            assert_eq!(cpu.ip(), 7)
        }
        #[test]
        fn test_mov_mem_mem_byte() {
            let mut cpu = Cpu::new();
            cpu.memory.buffer[10] = 100;

            cpu.load_program(&[Opcode::MoveMemMemByte as u8, 255, 1, 0, 0, 10]);

            cpu.run();

            assert_eq!(100, cpu.memory.buffer[511]);
        }
        #[test]
        fn test_move_mem_reg_byte() {
            let mut cpu = Cpu::new();

            cpu.memory.buffer[511] = 250;
            cpu.load_program(&[Opcode::MoveMemRegByte as u8, 10, 255, 1, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[10], 250);
        }

        #[test]
        fn test_mov_mem_reg_short() {
            let mut cpu = Cpu::new();

            cpu.memory.set_short(511, 0xBEEF);
            cpu.load_program(&[Opcode::MoveMemRegShort as u8, 10, 255, 1, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[10], 0xBEEF);
        }
        #[test]
        fn test_mov_reg_reg_short() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 65535;
            cpu.load_program(&[Opcode::MoveRegRegShort as u8, 1, 0]);
            cpu.run();
            assert_eq!(cpu.registers[1], 65535);

            let mut cpu = Cpu::new();
            cpu.registers[0] = 65538;
            cpu.load_program(&[Opcode::MoveRegRegShort as u8, 1, 0]);
            cpu.run();
            assert_eq!(cpu.registers[1], 2);
        }
        #[test]
        fn test_mov_mem_mem_short() {
            let mut cpu = Cpu::new();
            cpu.memory.set_long(511, 0xDEADBEEF);
            cpu.load_program(&[Opcode::MoveMemMemShort as u8, 255, 2, 0, 0, 255, 1, 0, 0]);
            cpu.run();

            assert_eq!(cpu.memory.long(767), 0xBEEF);
        }
        #[test]

        // Longs
        fn test_move_mem_reg_long() {
            let mut cpu = Cpu::new();
            cpu.memory.set_long(511, 0xDEADBEEF);
            cpu.load_program(&[Opcode::MoveMemRegLong as u8, 10, 255, 1, 0, 0]);
            cpu.run();

            assert_eq!(cpu.registers[10], 0xDEADBEEF);
        }
        #[test]
        fn test_mov_reg_reg_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xDEADBEEF;
            cpu.load_program(&[Opcode::MoveRegRegLong as u8, 1, 0]);
            cpu.run();
            assert_eq!(cpu.registers[1], 0xDEADBEEF);
        }
        #[test]
        fn test_mov_reg_mem_long() {
            let mut cpu = Cpu::new();
            cpu.registers[0] = 0xDEADBEEF;
            cpu.load_program(&[Opcode::MoveRegMemLong as u8, 255, 1, 0, 0, 0]);
            cpu.run();
            assert_eq!(cpu.ip(), 7);
            assert_eq!(cpu.memory.long(511), 0xDEADBEEF);
        }
        #[test]
        fn test_mov_mem_mem_long() {
            let mut cpu = Cpu::new();
            cpu.memory.set_long(511, 0xDEADBEEF);
            cpu.load_program(&[Opcode::MoveMemMemLong as u8, 255, 2, 0, 0, 255, 1, 0, 0]);
            cpu.run();

            assert_eq!(cpu.memory.long(767), 0xDEADBEEF);
        }
    }
}
