mod base;
mod config;
mod anon;
mod io;
mod filemap;
mod mlock;

use std::ops::{Deref, DerefMut};

#[allow(unused)]
use config::{MAdviseConfig, MMapConfig};
#[allow(unused)]
use anon::{AnonMMap, AnonMMapMut, AnonExecutableMMap, AnonExecutableMMapMut};
#[allow(unused)]
use filemap::{FileMMap, FileMMapMut, ExecFileMMap, ExecFileMMapMut};

/// Memory mapping with read only access.
pub trait MMap: Deref<Target=[u8]> {
    fn as_ptr(&self) -> *const u8;
    fn sync(&self, typ: MSyncType) -> std::io::Result<()>;
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()>;
    fn unmap(self) -> std::io::Result<()>;
}

/// Memory mapping with read and write permission.
pub trait MMapMut: MMap + DerefMut<Target=[u8]> {
    fn as_mut_ptr(&mut self) -> *mut u8;
}

/// Executable read only memory mapping.
pub trait MMapExec: MMap {}

/// Executable and writable memory mapping. Note that on MacOS the 
/// MAP_JIT flag must be set when the mapping has exec and write. 
/// The process must also have the JIT security entitlement.
pub trait MMapExecMut: MMapExec + MMapMut {}

/// The system uses the address hint in the mmap call to determine the 
/// start address of the mapped region if it would not overlap with any
/// existing mapping. If it does, the kernel will find a region on its 
/// own. 
/// If no address is specified the kernel will find the start address on its
/// own.
/// If MAP_FIXED is requested the system must map the region at the requested 
/// address, possibly stealing pages from existing mappings.
pub enum AddrHint {
    None,
    Addr(*mut u8),
}

impl AddrHint {
    pub(crate) fn as_ptr(self) -> *mut u8 {
        match self {
            Self::None => std::ptr::null_mut(),
            Self::Addr(p) => p,
        }
    }
}

/// Synchronize modes for msync syscall.
pub enum MSyncType {
    /// Msync call returns immediately.
    Async,
    /// Msync call blocks until write has been performed.
    Sync,
    /// Msync invalidates all cached data and performs a synchronous flush.
    SyncInvalidate,
    /// Msync invalidates all cached data and performs a asynchronous flush.
    AsyncInvalidate,
    /// Msync invalidetes only cached data, not sync flag is set.
    Invalidate,
}
