
use core::fmt::Debug;

use crate::Kvs;

pub struct MockKvs<D> {
    data: D,
}

impl <D> MockKvs<D> 
where 
    D: AsRef<[u8]> + AsMut<[u8]> + Debug,
{
    pub fn new(data: D) -> Self {
        let m = MockKvs{ data };

        // TODO: erase all memory

        m
    }
}

impl <D> Flash for MockKvs<D>
where 
    D: AsRef<[u8]> + AsMut<[u8]> + Debug,
{
    const PAGE_SIZE: usize = 2048;

    fn read(&mut self, addr: usize, data: &mut [u8]) -> Result<(), Self::Error> {
        let d = self.data.as_ref();
        data.copy_from_slice(&d[addr..addr+data.len()]);
        Ok(())
    }

    fn write(&mut self, addr: usize, data: &[u8]) -> Result<(), Self::Error> {
        let d = self.data.as_mut();
        (&mut d[addr..addr+data.len()]).copy_from_slice(data);
        Ok(())
    }

    fn erase_page(&mut self, addr: usize) -> Result<(), Self::Error> {
        let d = self.data.as_mut();

        for i in 0..Self::PAGE_SIZE {
            d[addr + i] = 0xFF;
        }
    }
}