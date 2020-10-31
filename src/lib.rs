#![no_std]

use core::fmt::Debug;

use bitflags::bitflags;

/// Flash trait describes page-erasable flash
pub trait Flash {
    /// Flash page size (minimum erasable chunk)
    const PAGE_SIZE: usize;

    /// Flash operation error
    type Error: Debug;

    /// Read data from flash
    fn read(&mut self, addr: usize, data: &mut [u8]) -> Result<(), Self::Error>;

    /// Write data to flash
    ///
    /// Note that flash can usually only be cleared (0xFF -> 0x00) so write
    /// data may not be correct if the sector is not already erased
    fn write(&mut self, addr: usize, data: &[u8]) -> Result<(), Self::Error>;

    /// Erase a flash page by address
    fn erase_page(&mut self, addr: usize) -> Result<(), Self::Error>;
}

/// Options for Key Value Store configuration
#[derive(Clone, PartialEq, Debug)]
pub struct Options {
    /// Flash KVS start address
    start_addr: usize,
    /// Number of pages available to the KVS
    num_pages: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Error<E> {
    /// Underlying flash error
    Flash(E),
}


impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Error::Flash(e)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum PageKind {
  /// Standard K:V data page
  Standard = 0x00,
}

bitflags!(
  struct PageFlags: u16 {
    /// Default to all bits set for FLASH erased
    const DEFAULT = 0xFFFF;


  }
);

/// PageHeader identifies a flash pages in the NVS
#[derive(Debug, Clone, PartialEq)]
struct PageHeader {
    /// File system version ID, MUST be 1
    version: u8,
    /// Page kind, specifies how the page should be read
    kind: PageKind,
    /// Page index, wrapping monotonic count
    index: u32,
    /// Page usage flags
    flags: PageFlags,
}

bitflags!(
  struct EntryFlags: u16 {
    /// Default to all bits set for FLASH erased
    const DEFAULT = 0xFFFF;


  }
);

struct EntryHeader {
    /// Entry index
    ///
    /// Monotonically increases from 0 as the file system runs,
    /// wrapping at u32::MAX
    index: u32,
    /// Entry usage flags
    flags: PageFlags,
}


pub struct Kvs<F: Flash> {
  flash: F,
  opts: Options,
}


impl<F, E> Kvs<F>
where
    F: Flash<Error = E>,
    E: Debug,
{
    pub fn new(flash: F, opts: Options) -> Result<Self, Error<E>> {
        let s = Self { flash, opts };

        // TODO: attempt to load existing KVS / create a new KVS

        Ok(s)
    }

    pub fn read(&mut self, key: &[u8], value: &mut [u8]) -> Result<usize, Error<E>> {
      // TODO: locate (latest) existing entry

      // TODO: read out header

      // TODO: read out entry data

      unimplemented!()
    }

    pub fn write(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error<E>> {
      // TODO: locate (latest) existing entry

      // TODO: find space for new entry

      // TODO: write new entry

      // TODO: invalidate previous entry

      unimplemented!()
    }

    /// Erase all (available) pages
    fn erase_all(&mut self) -> Result<(), Error<E>> {
        for i in 0..self.opts.num_pages {
            self.flash.erase_page(i * F::PAGE_SIZE)?;
        }

        Ok(())
    }
}
