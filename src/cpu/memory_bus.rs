pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_signed_byte(&self, address: u16) -> i8 {
        self.memory[address as usize] as i8
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn write_array(&mut self, address: u16, value: &[u8]) {
        for i in 0..value.len() - 1 {
            self.memory[address as usize + i] = value[i as usize];
        }
    }

    pub fn read_ly(&self) -> u8 {
        self.memory[0xff44]
    }

    pub fn write_ly(&mut self, value: u8) {
        self.memory[0xff44] = value;
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }
}
