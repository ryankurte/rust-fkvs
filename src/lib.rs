#![no_std]

use core::fmt::Debug;

use log::debug;
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

    /// Indicates a page is not in use (clear to activeate)
    const INACTIVE = (1 << 0);

    /// Indicates a page is valid (clear to invalidate)
    const VALID = (1 << 1);
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

    /// Indicates an entry is not in use (clear to activate)
    const INACTIVE = (1 << 0);

    /// Indicates an entry is valid (clear to invalidate)
    const VALID = (1 << 1);
  }
);

#[derive(Debug, Clone, PartialEq)]
struct EntryHeader {
    /// Entry index, per-key wrapping monotonic count
    index: u16,

    flags: EntryFlags,

    key_len: u16,

    val_len: u16,
}

pub struct Kvs<F: Flash> {
    flash: F,
    opts: Options,

    page_active: u32,
    page_offset: u32,
}

impl<F, E> Kvs<F>
where
    F: Flash<Error = E>,
    E: Debug,
{
    pub fn new(flash: F, opts: Options) -> Result<Self, Error<E>> {
        let mut s = Self { flash, opts, page_active: 0, page_offset: 0 };

        s.init()?;        

        Ok(s)
    }

    fn init(&mut self) -> Result<usize, Error<E>> {
      // Attempt to find existing / latest KVS page
      let mut current_index = None;
      for i in 0..self.opts.num_pages {
          // Read page header
          let h = self.get_page_header(i * F::PAGE_SIZE)?;

          // Skip inactive pages
          if h.flags.contains(PageFlags::INACTIVE) {
            continue;
          }

          // Skip expired pages
          if !h.flags.contains(PageFlags::VALID) {
            continue;
          }

          // Track current indez
          match current_index {
            Some(ref mut c) if *c < h.index => *c = h.index,
            Some(_) => (),
            None => current_index = Some(h.index),
          }
      }

      match current_index {
        Some(i) => {
          debug!("FKVS Initialising with current index: {}", i);

          self.page_active = i;

          unimplemented!()
        },
        None => {
          debug!("FKVS no index found, re-formatting");

          self.format()?;
        }
      }
    }

    /// Format the file system, erasing all content and resetting to the initial state
    fn format(&mut self) -> Result<usize, Error<E>> {
      unimplemented!()
    }

    /// Read a chunk of data from the file system
    pub fn read(&mut self, key: &[u8], value: &mut [u8]) -> Result<usize, Error<E>> {
        // TODO: locate (latest) existing entry

        // TODO: read out header

        // TODO: read out entry data

        unimplemented!()
    }

    /// Write a chunk of data to the file system
    pub fn write(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error<E>> {
        // TODO: locate (latest) existing entry

        // TODO: check values do not already match

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

    fn get_page_header(&self, addr: usize) -> Result<PageHeader, Error<E>> {
        unimplemented!()
    }

    fn set_page_header(&mut self, addr: usize, ph: PageHeader) -> Result<(), Error<E>> {
        unimplemented!()
    }

    fn get_entry_header(&self, addr: usize) -> Result<EntryHeader, Error<E>> {
        unimplemented!()
    }

    fn set_entry_header(&mut self, addr: usize, ph: EntryHeader) -> Result<(), Error<E>> {
        unimplemented!()
    }
}
