use crate::{MMap, MMapMut, MSyncType, MMapConfig, base::MMapBase, MAdviseConfig, AddrHint};
use std::ops::{Deref, DerefMut};
use std::fs::File;
use std::os::unix::prelude::AsRawFd;

pub struct FileMMap {
    inner: MMapBase,
}

impl FileMMap {
    pub fn new(addr_hint: AddrHint, conf: MMapConfig, file: &File, off: i64) -> std::io::Result<Self> {
        let flags = conf.value(); 
        let prot = Self::prot();
        let fd = file.as_raw_fd();
        let map_len = file.metadata()?.len() as usize;

        // without this check we could get a SIGBUS signal on the mmap call when 
        // the offset points beyond the mappable memory.
        if off as usize > map_len {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "offset points beyond file boundary"));
        }
        let map_len = map_len - off as usize;
        let inner = MMapBase::new(addr_hint.as_ptr(), map_len, prot, flags, fd, off)?;
        Ok(Self { inner })
    }

    #[inline]
    fn prot() -> i32 {
        libc::PROT_READ
    }
}

impl Deref for FileMMap {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl MMap for FileMMap {
    fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
    fn sync(&self, typ: MSyncType) -> std::io::Result<()> {
        self.inner.sync(typ)
    }
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()> {
        self.inner.advise(config)
    }
    fn unmap(self) -> std::io::Result<()> {
        self.inner.unmap()
    }
}

impl TryFrom<FileMMapMut> for FileMMap {
    type Error = std::io::Error;
    fn try_from(mmap: FileMMapMut) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<ExecFileMMap> for FileMMap {
    type Error = std::io::Error;
    fn try_from(mmap: ExecFileMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<ExecFileMMapMut> for FileMMap {
    type Error = std::io::Error;
    fn try_from(mmap: ExecFileMMapMut) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}


pub struct FileMMapMut {
    inner: MMapBase,
}

impl FileMMapMut {
    pub fn new(addr_hint: AddrHint, conf: MMapConfig, file: &File, off: i64) -> std::io::Result<Self> {
        let flags = conf.value(); 
        let prot = Self::prot();
        let fd = file.as_raw_fd();
        let map_len = file.metadata()?.len() as usize;
        if off as usize > map_len {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "offset points beyond file boundary"));
        }
        let map_len = map_len - off as usize;
        let inner = MMapBase::new(addr_hint.as_ptr(), map_len, prot, flags, fd, off)?;
        Ok(Self { inner })
    }

    #[inline]
    fn prot() -> i32 {
        libc::PROT_READ | libc::PROT_WRITE
    }
}

impl Deref for FileMMapMut {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl DerefMut for FileMMapMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl MMap for FileMMapMut {
    fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()> {
        self.inner.advise(config)
    }
    fn sync(&self, typ: MSyncType) -> std::io::Result<()> {
        self.inner.sync(typ)
    }
    fn unmap(self) -> std::io::Result<()> {
        self.inner.unmap()
    }
}

impl MMapMut for FileMMapMut {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }
}

impl TryFrom<FileMMap> for FileMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: FileMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<ExecFileMMap> for FileMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: ExecFileMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<ExecFileMMapMut> for FileMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: ExecFileMMapMut) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}



pub struct ExecFileMMap {
    inner: MMapBase,
}

impl ExecFileMMap {
    pub fn new(addr_hint: AddrHint, conf: MMapConfig, file: &File, off: i64) -> std::io::Result<Self> {
        let flags = conf.value(); 
        let prot = Self::prot();
        let fd = file.as_raw_fd();
        let map_len = file.metadata()?.len() as usize;
        if off as usize > map_len {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "offset points beyond file boundary"));
        }
        let map_len = map_len - off as usize;
        let inner = MMapBase::new(addr_hint.as_ptr(), map_len, prot, flags, fd, off)?;
        Ok(Self { inner })
    }

    #[inline]
    fn prot() -> i32 {
        libc::PROT_READ | libc::PROT_EXEC
    }
}

impl Deref for ExecFileMMap {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl MMap for ExecFileMMap {
    fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()> {
        self.inner.advise(config)
    }
    fn sync(&self, typ: MSyncType) -> std::io::Result<()> {
        self.inner.sync(typ)
    }
    fn unmap(self) -> std::io::Result<()> {
        self.inner.unmap()
    }
}

impl TryFrom<ExecFileMMapMut> for ExecFileMMap {
    type Error = std::io::Error;
    fn try_from(mmap: ExecFileMMapMut) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<FileMMapMut> for ExecFileMMap {
    type Error = std::io::Error;
    fn try_from(mmap: FileMMapMut) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<FileMMap> for ExecFileMMap {
    type Error = std::io::Error;
    fn try_from(mmap: FileMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}



pub struct ExecFileMMapMut {
    inner: MMapBase,
}

impl ExecFileMMapMut {
    pub fn new(addr_hint: AddrHint, conf: MMapConfig, file: &File, off: i64) -> std::io::Result<Self> {
        let flags = conf.value(); 
        let prot = Self::prot();
        let fd = file.as_raw_fd();
        let map_len = file.metadata()?.len() as usize;
        if off as usize > map_len {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "offset points beyond file boundary"));
        }
        let map_len = map_len - off as usize;
        let inner = MMapBase::new(addr_hint.as_ptr(), map_len, prot, flags, fd, off)?;
        Ok(Self { inner })
    }

    #[inline]
    fn prot() -> i32 {
        libc::PROT_READ | libc::PROT_EXEC | libc::PROT_WRITE
    }
}

impl Deref for ExecFileMMapMut {
    type Target = [u8]; 
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl DerefMut for ExecFileMMapMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl MMap for ExecFileMMapMut {
    fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }
    fn advise(&self, config: MAdviseConfig) -> std::io::Result<()> {
        self.inner.advise(config)
    }
    fn sync(&self, typ: MSyncType) -> std::io::Result<()> {
        self.inner.sync(typ)
    }
    fn unmap(self) -> std::io::Result<()> {
        self.inner.unmap()
    }
}

impl MMapMut for ExecFileMMapMut {
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }
}

impl TryFrom<ExecFileMMap> for ExecFileMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: ExecFileMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<FileMMapMut> for ExecFileMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: FileMMapMut) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

impl TryFrom<FileMMap> for ExecFileMMapMut {
    type Error = std::io::Error;
    fn try_from(mmap: FileMMap) -> Result<Self, Self::Error> {
        let inner = mmap.inner;
        inner.protect(Self::prot())?;
        Ok(Self { inner })
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use std::io::{Write, Read, Seek, SeekFrom};
    use std::fs::File;

    use super::*;
    use rand::Rng;

    struct TestFile {
        path: String, 
        fp: File,
    }

    impl TestFile {
        fn new(fpath: &str, cnt: usize) -> std::io::Result<Self> {
            let mut fp = std::fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(fpath).unwrap();

            let mut buf = vec![0u8; cnt];
            rand::thread_rng().fill(&mut buf[..]);
            fp.write(&buf)?;
            Ok(Self { path: String::from(fpath), fp })
        }

        fn read_to_vec(&mut self) -> std::io::Result<Vec<u8>> {
            let mut buf: Vec<u8> = Vec::new();
            self.fp.seek(SeekFrom::Start(0))?;
            self.fp.read_to_end(&mut buf)?;
            Ok(buf)
        }

        fn spawn_mmap(&self, off: i64) -> std::io::Result<FileMMap> {
            FileMMap::new(AddrHint::None, MMapConfig::new().map_private(), &self.fp, off)
        }
        
        fn spawn_mmap_mut(&self, off: i64) -> std::io::Result<FileMMapMut> {
            FileMMapMut::new(AddrHint::None, MMapConfig::new().map_private(), &self.fp, off)
        }

        fn spawn_exec_mmap(&self, off: i64) -> std::io::Result<ExecFileMMap> {
            ExecFileMMap::new(AddrHint::None, MMapConfig::new().map_private(), &self.fp, off)
        }

        #[allow(dead_code)]  
        fn spawn_exec_mmap_mut(&self, off: i64) -> std::io::Result<ExecFileMMapMut> {
            ExecFileMMapMut::new(AddrHint::None, MMapConfig::new().map_private(), &self.fp, off)
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    /// Content opened in mmap should be equal to the random bytes written to the
    /// file in the testcase setup.
    #[test]
    fn read_file_in_mmap() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/eo2gh9ogbjwoqb21obdao.txt", cnt).unwrap();
        let mmap = tf.spawn_mmap(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
    }
    
    #[test]
    fn read_file_in_mmap_mut() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/ujohoogh9879z29u2b1b9u2g3.txt", cnt).unwrap();
        let mmap = tf.spawn_mmap_mut(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
    }
    
    #[test]
    fn write_file_via_mmap_mut() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/aeqieb21v31v1rvt1313vvv1v312.txt", cnt).unwrap();
        let mut mmap = tf.spawn_mmap_mut(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
        // write all ones to file.
        for i in 0..mmap.len() {
            mmap[i] = 0xff;
        } 

        // now check file via normal file api.
        let mut freadbuf: Vec<u8> = Vec::new();
        tf.fp.read_to_end(&mut freadbuf).unwrap();
        for i in 0..freadbuf.len() {
            assert_eq!(freadbuf[i], 0xff);
        }
    }
    
    #[test]
    fn write_file_and_flush_sync() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/uuuieuqriqghifibvigbi1br1ihvthi.txt", cnt).unwrap();
        let mut mmap = tf.spawn_mmap_mut(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
        // write all ones to file.
        for i in 0..mmap.len() {
            mmap[i] = 0xff;
        } 
        mmap.sync(MSyncType::Sync).unwrap();
    }
    
    #[test]
    fn write_file_and_flush_async() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/ihbewhgvwighv232itivti1vr.txt", cnt).unwrap();
        let mut mmap = tf.spawn_mmap_mut(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
        // write all ones to file.
        for i in 0..mmap.len() {
            mmap[i] = 0xff;
        } 
        mmap.sync(MSyncType::Async).unwrap();
    }
    
    #[test]
    fn write_file_and_flush_sync_invalidate() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/bvvvqevqirvqvt123iv.txt", cnt).unwrap();
        let mut mmap = tf.spawn_mmap_mut(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
        // write all ones to file.
        for i in 0..mmap.len() {
            mmap[i] = 0xff;
        } 
        mmap.sync(MSyncType::SyncInvalidate).unwrap();
    }
    
    #[test]
    fn write_file_and_flush_async_invalidate() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/akgiqhoqebogkboeijbgi3.txt", cnt).unwrap();
        let mut mmap = tf.spawn_mmap_mut(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
        // write all ones to file.
        for i in 0..mmap.len() {
            mmap[i] = 0xff;
        } 
        mmap.sync(MSyncType::AsyncInvalidate).unwrap();
    }
    
    #[test]
    fn write_file_and_invalidate_cached_data() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/ouwhqb2ub31vbovfbov12v.txt", cnt).unwrap();
        let mut mmap = tf.spawn_mmap_mut(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
        // write all ones to file.
        for i in 0..mmap.len() {
            mmap[i] = 0xff;
        } 
        mmap.sync(MSyncType::Invalidate).unwrap();
    }
    
    #[test]
    fn read_file_in_exec_mmap() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/zzqgqi97119v1fv1zr1r3v1o1ihvt19z.txt", cnt).unwrap();
        let mmap = tf.spawn_exec_mmap(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
    }

    #[cfg(not(target_os = "macos"))] 
    #[test]
    fn read_file_in_exec_mmap_mut() {
        let cnt = 100000;
        let mut tf = TestFile::new("/tmp/uhgojebjbg3h280rb3bbg92oqn1.txt", cnt).unwrap();
        let mmap = tf.spawn_exec_mmap_mut(0).unwrap();
        assert_eq!(tf.fp.metadata().unwrap().len() as usize, mmap.len());
        assert_eq!(tf.read_to_vec().unwrap(), mmap[..]);
    }

    #[test]
    fn open_mmap_at_offset() {
        let cnt = 100000;
        let ps = crate::base::get_page_size();
        let mut tf = TestFile::new("/tmp/gh3203birb21i3b1ibhbvir.txt", cnt).unwrap();
        let mmap = tf.spawn_mmap(ps).unwrap();
        let buf = tf.read_to_vec().unwrap();
        assert_eq!(buf[ps as usize..], mmap[..]);
    }
    
    #[test]
    fn open_mmap_mut_at_offset() {
        let cnt = 100000;
        let ps = crate::base::get_page_size();
        let mut tf = TestFile::new("/tmp/hhqoebqjihrihvihev233121kb3.txt", cnt).unwrap();
        let mmap = tf.spawn_mmap_mut(ps).unwrap();
        let buf = tf.read_to_vec().unwrap();
        assert_eq!(buf[ps as usize..], mmap[..]);
    }
    
    #[test]
    fn open_exec_mmap_at_offset() {
        let cnt = 100000;
        let ps = crate::base::get_page_size();
        let mut tf = TestFile::new("/tmp/qieb1br1b1231bir1hrv.txt", cnt).unwrap();
        let mmap = tf.spawn_exec_mmap(ps).unwrap();
        let buf = tf.read_to_vec().unwrap();
        assert_eq!(buf[ps as usize..], mmap[..]);
    }

    #[cfg(not(target_os = "macos"))] 
    #[test]
    fn open_exec_mmap_mut_at_offset() {
        let cnt = 100000;
        let ps = crate::base::get_page_size();
        let mut tf = TestFile::new("/tmp/jbibgbb2i1b3vrivvouev2vvzu.txt", cnt).unwrap();
        let mmap = tf.spawn_exec_mmap_mut(ps).unwrap();
        let buf = tf.read_to_vec().unwrap();
        assert_eq!(buf[ps as usize..], mmap[..]);
    }
    
    #[test]
    fn offset_points_beyond_file() {
        let cnt = 100;
        let off = crate::base::get_page_size();
        let tf = TestFile::new("/tmp/zi82huwbfu1208bdbf201bjaaop.txt", cnt).unwrap();
        let res = tf.spawn_mmap(off);
        assert!(res.is_err());
        if let Err(e) = res {
            assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput);
        }
    }

    /// Offset must be a multiple of the systems page size. If it is not EINVAL is 
    /// returned on the mmap syscall.
    #[test]
    fn err_on_mmap_with_invalid_offset() {
        let cnt = 100000;
        let off = crate::base::get_page_size() + 1;
        let tf = TestFile::new("/tmp/o3013u013h1bjgbeb9ub39uu.txt", cnt).unwrap();
        let res = tf.spawn_mmap(off);
        assert!(res.is_err());
        if let Err(e) = res {
            assert_eq!(e.raw_os_error().unwrap(), libc::EINVAL);
        }
    }

    #[test]
    fn conversions_from_file_mmap() {
        let cnt = 10;
        let tf = TestFile::new("/tmp/ye1n232h8gw9gbu927.txt", cnt).unwrap();
        let mmap = tf.spawn_mmap(0).unwrap();
        let _: FileMMapMut = mmap.try_into().unwrap();
        let mmap = tf.spawn_mmap(0).unwrap();
        let _: ExecFileMMap = mmap.try_into().unwrap();
        // for writable and exectuable pages a macos process requires a special 
        // system permission, so disabled for now..
        #[cfg(not(target_os = "macos"))]
        {
            let mmap = tf.spawn_mmap(0).unwrap();
            let _: ExecFileMMapMut = mmap.try_into().unwrap();
        }
    }

    #[test]
    fn conversions_from_file_mmap_mut() {
        let cnt = 10;
        let tf = TestFile::new("/tmp/hqweh20g1729vfzo2zvg1.txt", cnt).unwrap();
        let mmap = tf.spawn_mmap_mut(0).unwrap();
        let _: FileMMap = mmap.try_into().unwrap();
        let mmap = tf.spawn_mmap_mut(0).unwrap();
        let _: ExecFileMMap = mmap.try_into().unwrap();
        #[cfg(not(target_os = "macos"))]
        {
            let mmap = tf.spawn_mmap_mut(0).unwrap();
            let _: ExecFileMMapMut = mmap.try_into().unwrap();
        }
    }
    
    #[test]
    fn conversions_from_exec_file_mmap() {
        let cnt = 10;
        let tf = TestFile::new("/tmp/audvi2b11v223121b13h2.txt", cnt).unwrap();
        let mmap = tf.spawn_exec_mmap(0).unwrap();
        let _: FileMMapMut = mmap.try_into().unwrap();
        let mmap = tf.spawn_exec_mmap(0).unwrap();
        let _: FileMMap = mmap.try_into().unwrap();
        #[cfg(not(target_os = "macos"))]
        {
            let mmap = tf.spawn_exec_mmap(0).unwrap();
            let _: ExecFileMMapMut = mmap.try_into().unwrap();
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    #[test]
    fn conversions_from_exec_file_mmap_mut() {
        let cnt = 10;
        let tf = TestFile::new("/tmp/a2328108gburgub3.txt", cnt).unwrap();
        let mmap = tf.spawn_exec_mmap_mut(0).unwrap();
        let _: FileMMapMut = mmap.try_into().unwrap();
        let mmap = tf.spawn_exec_mmap_mut(0).unwrap();
        let _: FileMMap = mmap.try_into().unwrap();
        let mmap = tf.spawn_exec_mmap_mut(0).unwrap();
        let _: ExecFileMMap = mmap.try_into().unwrap();
    }
}
