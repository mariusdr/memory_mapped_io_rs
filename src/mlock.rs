use crate::MMap;
use libc::{mlock, munlock, mincore};

/// Locks a memory region in physical memory.
pub struct MLock<'a, M: MMap> {
    handle: &'a M,
}

impl<'a, M: MMap> MLock<'a, M> {
    pub fn new(handle: &'a M) -> std::io::Result<Self> {
        unsafe {
            let rc = mlock(handle.as_ptr() as *const libc::c_void, handle.len());
            if rc != 0 {
                return Err(std::io::Error::last_os_error());
            }
        }
        Ok(Self { handle })
    }

    /// Wraps the munlock syscall and returns a result value which gives the 
    /// caller the opportunity to recognize and handle erros (as opposed to Drop which
    /// this type also implements).
    pub fn unlock(self) -> std::io::Result<()> {
        unsafe {
            let rc = munlock(self.handle.as_ptr() as *const libc::c_void, self.handle.len());
            if rc != 0 {
                let err = std::io::Error::last_os_error();
                return Err(err);
            }
        }
        Ok(())
    }
}

impl<'a, M: MMap> Drop for MLock<'a, M> {
    fn drop(&mut self) {
        unsafe {
            let _ = munlock(self.handle.as_ptr() as *const libc::c_void, self.handle.len());
        }
    }
}

struct IncoreInfo {
    pinfo: *const libc::c_char,
    plen: usize,
}

impl IncoreInfo {
    pub fn read<M: MMap>(mmap: &M) -> std::io::Result<Self> {
        unsafe {
            let pinfo: *mut libc::c_char = std::ptr::null_mut();
            if !crate::base::ptr_is_page_aligned(mmap.as_ptr()) {
                let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, "mmap address is not page aligned");
                return Err(err);
            }
            let rc = mincore(mmap.as_ptr() as *const libc::c_void, mmap.len(), pinfo);
            if rc != 0 {
                let err = std::io::Error::last_os_error();
                return Err(err);
            }
            let page_size = crate::base::get_page_size();
            if page_size <= 0 {
                let err = std::io::Error::new(std::io::ErrorKind::Other, "page size is invalid");
                return Err(err);
            }
            let plen = (mmap.len() + page_size as usize + 1) / page_size as usize;
            Ok(Self { pinfo, plen })
        }
    }

    #[inline]
    unsafe fn view_vec(&self) -> &[i8] {
        std::slice::from_raw_parts(self.pinfo, self.plen)
    }

    pub fn flagvec_len(&self) -> usize {
        self.plen
    }

    pub fn page_flagbyte(&self, pageidx: usize) -> Option<i8> {
        if self.plen >= pageidx {
            return None;
        }
        unsafe {
            let view = self.view_vec();
            Some(view[pageidx])
        }
    }
}


#[cfg(test)] 
mod tests {
    use super::*;
    use std::fs::{OpenOptions, File};
    use crate::{config::MMapConfig, AddrHint};

    #[test]
    fn atest() {
        let ahint = AddrHint::None;
        let conf = MMapConfig::new().map_private();
        let mut mmap = crate::AnonMMapMut::new(ahint, 999999, conf).unwrap();
        let icinfo = IncoreInfo::read(&mmap).unwrap();
        assert_eq!(icinfo.flagvec_len(), 0);
    }
}