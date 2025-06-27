pub mod flag_registers;
pub mod instructions;
pub mod memory_bus;
pub mod registers;

use flag_registers::FlagsRegister;
use instructions::Instruction;
use memory_bus::MemoryBus;
use registers::Registers;

pub struct CPU {
  registers: Registers,
  sp: u16,
  pc: u16,
  flags_register: FlagsRegister,
  bus: MemoryBus,
}

impl CPU {
  pub fn step(&mut self) {
    let mut instruction_byte = self.bus.read_byte(self.pc);
    let prefixed = instruction_byte == 0xCB;
    if prefixed {
      instruction_byte = self.bus.read_byte(self.pc + 1);
    }
		println!("Running instruction {:x}", instruction_byte);
    let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
      self.execute(instruction)
    } else {
      let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
      panic!("Unkown instruction found for: {}", description)
    };

    self.pc = next_pc;
  }
  
  fn execute(&mut self, instruction: Instruction) -> u16 {
    match instruction {
    	Instruction::BIT(number, source) => {
    		let source_value = match source {
            instructions::BitSource::H => self.registers.h,
            _ => { panic!("TODO: implement other sources") }
          };
        self.flags_register.zero = (source_value << number) & 1 == 0;
        self.flags_register.subtract = false;
        self.flags_register.half_carry = true;
        self.pc + 2
    	}
    	Instruction::LD(target, source) => {
    		let source_value = match source {
            instructions::LoadSource::A => self.registers.a,
            _ => { panic!("TODO: implement other sources") }
          };
          match target {
            instructions::LoadTarget::HLD => {
            	let hl = self.registers.get_hl();
            	self.bus.write_byte(hl, source_value);
            	println!("Setting {:x}={}", hl, source_value);
            	self.registers.set_hl(hl - 1);
            }
            _ => { panic!("TODO: implement other targets") }
          };
          self.pc + 1
    	}
    	Instruction::LDN16(target) => {
    		let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
      	let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
      	let value = (most_significant_byte << 8) | least_significant_byte;
    	
    		match target {
    			instructions::LoadTypeN16::SP => {
    				self.sp = value;
    				self.pc + 3
    			}
    			instructions::LoadTypeN16::HL => {
    				self.registers.set_hl(value);
    				self.pc + 3
    			}
    			_ => { panic!("Unknown target."); }
    		}
    	}
    	Instruction::XOR(target, source) => {
    		let source_value = match source {
            instructions::ArithmeticSource::A => self.registers.a,
            _ => { panic!("TODO: implement other sources") }
          };
          match target {
            instructions::ArithmeticTarget::A => self.registers.a ^= source_value,
            _ => { panic!("TODO: implement other targets") }
          };
          self.pc + 1
    	}
      _ => { /* TODO: support more instructions */ self.pc }
    }
  }
  
  pub fn load_bootstrap(&mut self, bootstrap_bin: &[u8]) {
  	self.bus.write_array(0, bootstrap_bin);
  }
}

impl Default for CPU {
  fn default() -> Self {
  	Self {
  		registers: Registers::default(),
  		flags_register: FlagsRegister::default(),
  		sp: 0,
  		pc: 0,
  		bus: MemoryBus::default(),
  	}
  }
}
