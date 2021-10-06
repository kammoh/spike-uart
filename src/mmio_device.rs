pub trait MmioDevice: Sized {
    fn new(args: Vec<String>) -> Result<Self, &'static str>;
    fn load(&self, addr: u64, len: usize) -> Result<Vec<u8>, &str>;
    fn store(&mut self, addr: u64, data: &[u8]) -> Result<(), &str>;
}
