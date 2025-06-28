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
    self.pc = self.pc.wrapping_add(1);
    let prefixed = instruction_byte == 0xCB;
    if prefixed {
      instruction_byte = self.bus.read_byte(self.pc);
    	self.pc = self.pc.wrapping_add(1);
    }
		println!("Running instruction {:x} @ {}", instruction_byte, self.pc);
    if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
      self.execute(instruction)
    } else {
      let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
      panic!("Unkown instruction found for: {}", description)
    };
  }
  
  fn execute(&mut self, instruction: Instruction) {
    match instruction {
    	Instruction::BIT(number, source) => {
    		let source_value = match source {
            instructions::BitSource::H => self.registers.h,
            _ => { panic!("TODO: implement other sources") }
          };
        self.flags_register.zero = (source_value >> number) & 1 == 0;
        self.flags_register.subtract = false;
        self.flags_register.half_carry = true;
    	}
    	Instruction::JR(condition) => {
    		let condition_value = match condition {
    				instructions::JumpCondition::NZ => !self.flags_register.zero,
            _ => { panic!("TODO: implement other sources") }
    		};
    		if condition_value {
    			let distance = self.bus.read_signed_byte(self.pc);
    			println!("Jumping {}", distance);
    			self.pc = self.pc.wrapping_add(1).wrapping_add_signed(distance.into());
    		} else {
    			println!("Not jumping");
    			// Optimization: skip reading the next byte since we are not jumping, still need to update pc counter though.
    			self.pc = self.pc.wrapping_add(1);
    		}
    	}
    	Instruction::LD(target, source) => {
    		let source_value = match source {
            instructions::LoadSource::A => self.registers.a,
            instructions::LoadSource::N8 => {
            	let old_pc = self.pc;
    					self.pc = self.pc.wrapping_add(1);
    					self.bus.read_byte(old_pc)
            }
            _ => { panic!("TODO: implement other sources") }
          };
          match target {
          	instructions::LoadTarget::A=> {
          		println!("Setting a={}", source_value);
          		self.registers.a = source_value;
          	}
          	instructions::LoadTarget::C=> {
          		println!("Setting c={}", source_value);
          		self.registers.c = source_value;
          	}
            instructions::LoadTarget::HLD => {
            	let hl = self.registers.get_hl();
            	self.bus.write_byte(hl, source_value);
            	println!("Setting {:x}={}", hl, source_value);
            	self.registers.set_hl(hl - 1);
            }
            _ => { panic!("TODO: implement other targets") }
          };
    	}
    	Instruction::LDN16(target) => {
    		let least_significant_byte = self.bus.read_byte(self.pc) as u16;
      	let most_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
      	let value = (most_significant_byte << 8) | least_significant_byte;
      	self.pc = self.pc.wrapping_add(2);
    	
    		match target {
    			instructions::LoadTypeN16::SP => {
    				self.sp = value;
    			}
    			instructions::LoadTypeN16::HL => {
    				self.registers.set_hl(value);
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
    	}
      _ => {}
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
