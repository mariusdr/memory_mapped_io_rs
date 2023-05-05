use crate::{MMap, MMapMut, MSyncType, MMapConfig, base::MMapBase, MAdviseConfig, AddrHint};
use std::ops::{Deref, DerefMut};

pub struct AnonMMap {
    inner: MMapBase,
}

impl AnonMMap {
    pub fn new(addr_hint: AddrHint, map_len: usize, conf: MMapConfig) -> std::io::Result<Self> {
        let flags = conf.value() | libc::MAP_ANON;
        let inner = MMapBase::new(addr_hint.as_ptr(), map_len, Self::prot(), flags, -1, 0)?;
        Ok(Self{ inner })
    }

    #[inline]
    fn prot() -> i32 {
        libc::PROT_READ
    }
}

impl Deref for AnonMMap {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl MMap for AnonMMap {
    fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
    fn sync(&self, typ: MSyncType) -> std::io::Result<()> {
        self.inner.sync(typ)
    } 
    fn unmap(self) -> std::io::Result<()> {
        self.inner.unmap()
    }
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()> {
        self.inner.advise(config)
    }
}

impl TryFrom<AnonMMapMut> for AnonMMap {
    type Error = std::io::Error;
    fn try_from(mmap: AnonMMapMut) -> std::io::Result<Self> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<AnonExecutableMMap> for AnonMMap {
    type Error = std::io::Error;
    fn try_from(mmap: AnonExecutableMMap) -> std::io::Result<Self> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<AnonExecutableMMapMut> for AnonMMap {
    type Error = std::io::Error;
    fn try_from(mmap: AnonExecutableMMapMut) -> std::io::Result<Self> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}




pub struct AnonMMapMut {
    inner: MMapBase,
}

impl AnonMMapMut {
    pub fn new(addr_hint: AddrHint, map_len: usize, conf: MMapConfig) -> std::io::Result<Self> {
        let flags = conf.value() | libc::MAP_ANON;
        let prot = Self::prot();
        let inner = MMapBase::new(addr_hint.as_ptr(), map_len, prot, flags, -1, 0)?;
        Ok(Self{ inner })
    }

    #[inline]
    fn prot() -> i32 {
        libc::PROT_READ | libc::PROT_WRITE
    }
}

impl Deref for AnonMMapMut {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl DerefMut for AnonMMapMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl MMap for AnonMMapMut {
    fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
    fn sync(&self, typ: MSyncType) -> std::io::Result<()> {
        self.inner.sync(typ)
    } 
    fn unmap(self) -> std::io::Result<()> {
        self.inner.unmap()
    }
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()> {
        self.inner.advise(config)
    }
}

impl MMapMut for AnonMMapMut {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }
}

impl TryFrom<AnonMMap> for AnonMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: AnonMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<AnonExecutableMMap> for AnonMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: AnonExecutableMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<AnonExecutableMMapMut> for AnonMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: AnonExecutableMMapMut) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}


pub struct AnonExecutableMMap {
    inner: MMapBase,
}

impl AnonExecutableMMap {
    pub fn new(addr_hint: AddrHint, map_len: usize, conf: MMapConfig) -> std::io::Result<Self> {
        let flags = conf.value() | libc::MAP_ANON;
        let prot = Self::prot();
        let inner = MMapBase::new(addr_hint.as_ptr(), map_len, prot, flags, -1, 0)?;
        Ok(Self{ inner })
    }

    #[inline]
    fn prot() -> i32 {
        libc::PROT_READ | libc::PROT_EXEC
    }
}

impl Deref for AnonExecutableMMap {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl MMap for AnonExecutableMMap {
    fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
    fn sync(&self, typ: MSyncType) -> std::io::Result<()> {
        self.inner.sync(typ)
    }
    fn unmap(self) -> std::io::Result<()> {
        self.inner.unmap()
    }
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()> {
        self.inner.advise(config)
    }
}

impl TryFrom<AnonExecutableMMapMut> for AnonExecutableMMap {
    type Error = std::io::Error;
    fn try_from(mmap: AnonExecutableMMapMut) -> std::io::Result<Self> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<AnonMMap> for AnonExecutableMMap {
    type Error = std::io::Error;
    fn try_from(mmap: AnonMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<AnonMMapMut> for AnonExecutableMMap {
    type Error = std::io::Error;
    fn try_from(mmap: AnonMMapMut) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}


pub struct AnonExecutableMMapMut {
    inner: MMapBase,
}

impl AnonExecutableMMapMut {
    pub fn new(addr_hint: AddrHint, map_len: usize, conf: MMapConfig) -> std::io::Result<Self> {
        let flags = conf.value() | libc::MAP_ANON;
        let prot = Self::prot();
        let inner = MMapBase::new(addr_hint.as_ptr(), map_len, prot, flags, -1, 0)?;
        Ok(Self{ inner })
    }

    #[inline]
    fn prot() -> i32 {
        libc::PROT_READ | libc::PROT_EXEC | libc::PROT_EXEC
    }
}

impl Deref for AnonExecutableMMapMut {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl DerefMut for AnonExecutableMMapMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl MMap for AnonExecutableMMapMut {
    fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
    fn sync(&self, typ: MSyncType) -> std::io::Result<()> {
        self.inner.sync(typ)
    }
    fn unmap(self) -> std::io::Result<()> {
        self.inner.unmap()
    }
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()> {
        self.inner.advise(config)
    }
}

impl MMapMut for AnonExecutableMMapMut {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }
}

impl TryFrom<AnonExecutableMMap> for AnonExecutableMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: AnonExecutableMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<AnonMMap> for AnonExecutableMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: AnonMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<AnonMMapMut> for AnonExecutableMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: AnonMMapMut) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn make(config: MMapConfig) {
        let r = AnonMMap::new(AddrHint::None, 12000, config);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn expect_no_os_error() {
        make(MMapConfig::new().map_shared());
        make(MMapConfig::new().map_private());
        make(MMapConfig::new().map_shared().map_private());
    }

    #[cfg(target_os = "macos")] 
    #[test]
    fn expect_no_os_error_macos() {
        make(MMapConfig::new().map_private().map_jit());
        make(MMapConfig::new().map_private().map_nocache());
        make(MMapConfig::new().map_shared().map_nocache());
    }

    #[test]
    fn memory_is_zeroed_mmap() {
        let m = AnonMMap::new(AddrHint::None, 12000, MMapConfig::new().map_private()).unwrap();
        let mut s = vec![0xff; 12000];
        s.copy_from_slice(&m[0..12000]);
        assert_eq!(s, vec![0x00; 12000]);
    }

    #[test]
    fn memory_is_zeroed_mmap_mut() {
        let m = AnonMMapMut::new(AddrHint::None, 12000, MMapConfig::new().map_private()).unwrap();
        let mut s = vec![0xff; 12000];
        s.copy_from_slice(&m[0..12000]);
        assert_eq!(s, vec![0x00; 12000]);
    }

    #[test]
    fn read_write_anon_mapped_memory() {
        let mut m = AnonMMapMut::new(AddrHint::None, 12000, MMapConfig::new().map_private()).unwrap();
        let w = vec![0xff; 12000];
        m.copy_from_slice(&w);
        
        let mut r = vec![0x00; 12000];
        r.copy_from_slice(&m);
        assert_eq!(r, w);
    }
}