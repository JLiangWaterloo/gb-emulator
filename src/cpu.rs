pub mod flag_registers;
pub mod instructions;
pub mod memory_bus;
pub mod registers;

use instructions::Instruction;
use memory_bus::MemoryBus;
use registers::Registers;

struct CPU {
  registers: Registers,
  pc: u16,
  bus: MemoryBus,
}

impl CPU {
  fn step(&mut self) {
    let mut instruction_byte = self.bus.read_byte(self.pc);
    let prefixed = instruction_byte == 0xCB;
    if prefixed {
      instruction_byte = self.bus.read_byte(self.pc + 1);
    }

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
      _ => { /* TODO: support more instructions */ self.pc }
    }
  }
}
