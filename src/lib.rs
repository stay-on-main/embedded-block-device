#![no_std]

pub trait BlockDevice {
    /// An enumeration of storage errors
    type Error;

    fn read(&mut self, block: u32, offset: usize, buf: &mut [u8]) -> Result<(), Self::Error>;
    fn write(&mut self, block: u32, offset: usize, buf: &[u8]) -> Result<(), Self::Error>;
    fn block_size(&self) -> usize;
    fn block_count(&self) -> u32;
}