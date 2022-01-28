mod bus;

#[derive(Copy, Clone)]
pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub reg_pc: u16,
    pub reg_p: u8,
    pub reg_s: u8,
    pub fetch: u8,
    pub abs_address: u16,
    pub cycles: u8,
    pub bus: bus::Bus,
}
impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_a: 0x00,
            reg_x: 0x00,
            reg_y: 0x00,
            reg_pc: 0x0000,
            reg_p: 0x00,
            reg_s: 0x00,
            fetch: 0x00,
            abs_address: 0x0000,
            cycles: 0,
            bus: bus::Bus::new(),
        }
    }
    fn fetchnow(&mut self) {
        self.fetch = self.bus.read(self.abs_address);
    }

    pub fn load(&mut self, a: Vec<u8>, start_pc: u16) {
        self.reg_pc = start_pc;
        for i in 0..a.len()-1 {
            self.bus.mem[self.reg_pc as usize] = a[i];
            self.reg_pc += 1;
        }
        self.reg_pc = start_pc;
    }

    pub fn start(&mut self) {
        loop {
            let opcode: u8 = self.bus.mem[self.reg_pc as usize];
            match opcode {
                0x69 => {
                    self.imm();
                    self.ADC();
                }
                0x65 => {
                    self.zpg();
                    self.ADC();

                }
                _ => {
                    break;
                }
            }
        }
    }

    fn imp(&mut self) {
        //does nothing.Exists for side effects
        self.reg_pc += 1;
    }

    fn imm(&mut self) {
        self.reg_pc += 1;
        self.abs_address = self.reg_pc;
        self.fetchnow(); //Takes the byte immediate to the opcode as argument
        self.reg_pc += 1;
    }

    fn zpg(&mut self) {
        self.reg_pc += 1;
        self.abs_address = self.bus.read(self.reg_pc) as u16; //Takes the byte immediately after opcode as address for op
        self.reg_pc += 1;
        self.fetchnow();
    }

    fn zpgx(&mut self) {
        self.reg_pc += 1; //fetches the byte immediately after opcode and adds value of x register(wrapping it around if sum >0xFF) to get the address for op
        self.abs_address = ((self.bus.read(self.reg_pc) + self.reg_x) as u16) & 0x00FF;
        self.reg_pc += 1;
        self.fetchnow();
    }
    fn zpgy(&mut self) {
        self.reg_pc += 1; //same as zpgx except with y register
        self.abs_address = ((self.bus.read(self.reg_pc) + self.reg_y) as u16) & 0x00FF;
        self.reg_pc += 1;
        self.fetchnow();
    }

    fn abs(&mut self) {
        //fetches 2 bytes(L and H) after opcode and constructs a 16 bit address (HL) as argument for op
        self.reg_pc += 1;
        let lo: u8 = self.bus.read(self.reg_pc);
        self.reg_pc += 1;
        let hi: u16 = self.bus.read(self.reg_pc) as u16;
        self.reg_pc += 1;
        self.abs_address = hi << 8 + lo as u16;
        self.fetchnow();
    }
    fn absx(&mut self) {
        //fetches 2 bytes(L and H) after opcode and constructs a 16 bit address (HL) adds value of x register to get address for op
        self.reg_pc += 1;
        let lo: u8 = self.bus.read(self.reg_pc);
        self.reg_pc += 1;
        let hi: u16 = self.bus.read(self.reg_pc) as u16;
        self.reg_pc += 1;
        self.abs_address = hi << 8 + (lo + self.reg_x) as u16;
        self.fetchnow();
    }

    fn absy(&mut self) {
        //same as absx but with y register
        self.reg_pc += 1;
        let lo: u8 = self.bus.read(self.reg_pc);
        self.reg_pc += 1;
        let hi: u16 = self.bus.read(self.reg_pc) as u16;
        self.reg_pc += 1;
        self.abs_address = hi << 8 + (lo + self.reg_y) as u16;
        self.fetchnow();
    }

    fn rel(&mut self) {
        //fetches 1 byte after op,which is treated as a signed int. This value is then added to pc to get address for operand
        self.reg_pc += 1;
        let x: i8 = self.bus.read(self.reg_pc) as i8;
        self.reg_pc += 1;
        self.abs_address = self.reg_pc + x as u16;
        self.fetchnow();
    }

    fn acc(&mut self) {
        //accumulator is the argument for op
        self.reg_pc += 1;
        self.fetch = self.reg_a;
    }

    fn xind(&mut self) {
        self.reg_pc += 1;
        let ind_address = (self.bus.read(self.reg_pc) + self.reg_x) as u16;
        self.reg_pc += 1;
        let lo: u8 = self.bus.read(ind_address);
        let hi: u16 = self.bus.read(ind_address + 1) as u16;
        self.abs_address = hi << 8 + lo;
        self.fetchnow();
    }

    fn indy(&mut self) {
        self.reg_pc += 1;
        let ind_address = self.bus.read(self.reg_pc) as u16;
        self.reg_pc += 1;
        let lo: u8 = self.bus.read(ind_address);
        let hi: u16 = self.bus.read(ind_address + 1) as u16;
        self.abs_address = hi << 8 + lo + self.reg_y;
        self.fetchnow();
    }
    fn ind(&mut self) {
        self.reg_pc += 1;
        let lo: u8 = self.bus.read(self.reg_pc);
        self.reg_pc += 1;
        let hi: u16 = self.bus.read(self.reg_pc) as u16;
        self.reg_pc += 1;
        let ind_address: u16 = hi << 8 + lo;
        let lo: u8 = self.bus.read(ind_address);
        let hi: u16 = self.bus.read(ind_address + 1) as u16;
        self.abs_address = hi << 8 + lo;
        self.fetchnow();
    }

    fn ADC(&mut self) {
        self.reg_a += self.fetch;
    }
    fn STA(&mut self) {}
}

fn main() {
    let mut a = CPU::new();
    let prog: Vec<u8> = vec![0x69, 0x21, 0x65, 0x23];
    a.load(prog, 0x12);
    a.start();
    println!("{}", a.reg_a);
}
