use std::ops::{Deref, DerefMut};
use libc::{mmap, munmap, mprotect, msync, madvise};
use crate::{MSyncType, MMap, MMapMut, MMapExec, MMapExecMut, MAdviseConfig};

/// Util function to determine the page size on unix operating systems.
#[cfg(unix)]
pub(crate) fn get_page_size() -> i64 {
    unsafe {
        libc::sysconf(libc::_SC_PAGESIZE)
    }
}

/// Util function to check if a given address is aligned on the page boundary.
pub(crate) fn ptr_is_page_aligned<T>(addr: *const T) -> bool {
    let ps = get_page_size();
    addr as u64 % ps as u64 == 0
}

pub(crate) struct MMapBase {
    map_len: usize,
    map_ptr: *mut u8,
}

impl MMapBase {
    pub(crate) fn new(addr_hint: *mut u8, map_len: usize, prot: i32, flags: i32, fd: i32, map_off: i64) -> std::io::Result<Self> {
        let map_ptr = unsafe {
            let addr = addr_hint as *mut libc::c_void;
            let ptr = mmap(addr, map_len, prot, flags, fd, map_off);
            if ptr == libc::MAP_FAILED {
                return Err(std::io::Error::last_os_error());
            }
            ptr
        } as *mut u8;
        Ok(Self { map_len, map_ptr })
    }

    /// Return length of the mapped region.
    pub fn len(&self) -> usize {
        self.map_len
    }

    /// Wraps the mprotect syscall which changes the protections of 
    /// the mapped pages. Note that a file descriptor opened read 
    /// only cannot made writable with this syscall, it will fail 
    /// with EACCES. 
    pub(crate) fn protect(&self, prot: i32) -> std::io::Result<()> {
        unsafe {
            let rc = mprotect(self.map_ptr as *mut libc::c_void, self.map_len, prot);
            if rc != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
        Ok(())
    }

    /// Wraps the msync syscall, which flushes the modified pages back to
    /// the file system and updates the file timestamp.
    /// There are three types of synchronization:
    /// * Async: syscall returns immediately
    /// * Sync: syscall blocks until write has finished
    /// * Invadliate: syscall invalidates all cached data 
    fn synchronize(&self, typ: MSyncType) -> std::io::Result<()> {
        unsafe {
            let rc = msync(self.map_ptr as *mut libc::c_void, self.map_len, typ.as_flag());
            if rc != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
        Ok(())
    }

    /// Wraps the madvise syscall that allows a process that has knowledge about 
    /// its memory access behavior to describe it to the system. The system may 
    /// alter its virtual memory paging strategy depending on that advice which 
    /// might improve performance.
    fn madvise(&self, flag: i32) -> std::io::Result<()> {
        unsafe {
            let rc = madvise(self.map_ptr as *mut libc::c_void, self.map_len, flag);
            if rc != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
        Ok(())
    }

    pub(crate) fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.as_ptr(), self.len())
        }
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len())
        }
    }
}

impl Deref for MMapBase {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for MMapBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl MMap for MMapBase {
    fn as_ptr(&self) -> *const u8 {
        self.map_ptr
    }
    fn sync(&self, typ: self::MSyncType) -> std::io::Result<()> {
        self.synchronize(typ)
    }
    fn unmap(self) -> std::io::Result<()> {
        unsafe {
            let rc = munmap(self.map_ptr as *mut libc::c_void, self.map_len);
            if rc != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
        Ok(())
    }
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()> {
        self.madvise(config.value())
    }
}

impl MMapMut for MMapBase {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.map_ptr as *mut u8
    }
}
impl MMapExec for MMapBase {}
impl MMapExecMut for MMapBase {}

impl Drop for MMapBase {
    fn drop(&mut self) {
        unsafe {
            // Basically the same as unmap but we do not handle the error
            // here, as drop cannot return a result. 
            //
            // We could panic but this can create a situation were we panic 
            // during stack unwinding due to another panic, i.e. a double 
            // panic which will result in the process being killed with a SIGILL signal.
            // 
            // If error handling is desired, call unmap instead of drop.
            munmap(self.map_ptr as *mut libc::c_void, self.map_len);
        }
    }
}

impl MSyncType {
    fn as_flag(&self) -> i32 {
        match *self {
            Self::Async => libc::MS_ASYNC,
            Self::Sync => libc::MS_SYNC,
            Self::SyncInvalidate => libc::MS_SYNC | libc::MS_INVALIDATE,
            Self::AsyncInvalidate => libc::MS_ASYNC | libc::MS_INVALIDATE,
            Self::Invalidate => libc::MS_INVALIDATE,
        }
    }
}
