#[derive(Copy, Clone)]
pub struct Bus {
    pub mem: [u8; 0x10000]
}
impl Bus {
    pub fn new()-> Self{
    Bus{
    mem: [2;0x10000]}
    }
    pub fn read(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
    pub fn write(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }
}
fn main()
{
    println!("hello world");
}