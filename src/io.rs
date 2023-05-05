use std::io::{Read, Write, Seek, SeekFrom};
use crate::{MMap, MMapMut};

pub struct MMapReader<'a, M: MMap> {
    cur: u64,
    mmap: &'a M,
}

impl<'a, M: MMap> MMapReader<'a, M> {
    pub fn new(mmap: &'a M) -> Self {
        Self { cur: 0, mmap }
    }

    fn seek_from_start(&mut self, pos: u64) -> std::io::Result<u64> {
        self.cur = pos;
        Ok(self.cur)
    }
    
    fn seek_from_end(&mut self, pos: i64) -> std::io::Result<u64> {
        let c = self.mmap.len() as i64 + pos;
        if c < 0 {
            let e = std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cannot seek before byte 0");
            return Err(e);
        }
        self.cur = c as u64;
        Ok(self.cur)
    }
    
    fn seek_from_current(&mut self, pos: i64) -> std::io::Result<u64> {
        let c = self.cur as i64 + pos;
        if c < 0 {
            let e = std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cannot seek before byte 0");
            return Err(e);
        }
        self.cur = c as u64;
        Ok(self.cur)
    }
}

impl<'a, M: MMap> Seek for MMapReader<'a, M> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Current(n) => self.seek_from_current(n),
            SeekFrom::Start(n) => self.seek_from_start(n),
            SeekFrom::End(n) => self.seek_from_end(n),
        }
    }
}

impl<'a, M: MMap> Read for MMapReader<'a, M> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }
        if self.cur as usize >= self.mmap.len() {
            // reaching end of file is no error, just return that 
            // zero bytes were read.
            return Ok(0);
        }
        let top = std::cmp::min(self.cur as usize + buf.len(), self.mmap.len());
        let n = top - self.cur as usize;
        buf[0..n].copy_from_slice(&self.mmap[self.cur as usize..top]);
        self.cur = top as u64;
        Ok(n) 
    }
}

pub struct MMapWriter<'a, M: MMapMut> {
    cur: u64,
    mmap: &'a mut M,
}

impl<'a, M:MMapMut> MMapWriter<'a, M> {
    pub fn new(mmap: &'a mut M) -> Self {
        Self { cur: 0, mmap }
    }
    
    fn seek_from_start(&mut self, pos: u64) -> std::io::Result<u64> {
        self.cur = pos;
        Ok(self.cur)
    }
    
    fn seek_from_end(&mut self, pos: i64) -> std::io::Result<u64> {
        let c = self.mmap.len() as i64 + pos;
        if c < 0 {
            let e = std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cannot seek before byte 0");
            return Err(e);
        }
        self.cur = c as u64;
        Ok(self.cur)
    }
    
    fn seek_from_current(&mut self, pos: i64) -> std::io::Result<u64> {
        let c = self.cur as i64 + pos;
        if c < 0 {
            let e = std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cannot seek before byte 0");
            return Err(e);
        }
        self.cur = c as u64;
        Ok(self.cur)
    }
}

impl<'a, M: MMapMut> Seek for MMapWriter<'a, M> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Current(n) => self.seek_from_current(n),
            SeekFrom::End(n) => self.seek_from_end(n),
            SeekFrom::Start(n) => self.seek_from_start(n),
        }
    }
}

impl<'a, M: MMapMut> Write for MMapWriter<'a, M> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }
        if self.cur as usize >= self.mmap.len() {
            // reaching end of file is no error, just return that 
            // zero bytes were written.
            return Ok(0);
        }
        let top = std::cmp::min(self.cur as usize + buf.len(), self.mmap.len());
        let n = top - self.cur as usize;
        self.mmap[0..n].copy_from_slice(&buf);
        self.cur = top as u64;
        Ok(n) 
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.mmap.sync(crate::MSyncType::Sync)
    }
}

mod writer_tests {
    use super::*;
    use crate::*;
    use crate::anon::*;

    fn setup(len: usize) -> AnonMMapMut {
        let mut mmap = AnonMMapMut::new(AddrHint::None, len, MMapConfig::new().map_private()).unwrap();
        for i in 0..len {
            mmap[i] = (i % 0xff) as u8;
        }
        mmap
    }

    #[test]
    fn write_then_read() {
        let mut mmap = setup(24123);
        let patt = (0..mmap.len()).map(|x| (x % 255) as u8).collect::<Vec<u8>>();
        let mut writer = MMapWriter::new(&mut mmap);
        writer.write(patt.as_slice()).unwrap();

        let mut rbuf = vec![0xff; mmap.len()];
        let mut reader = MMapReader::new(&mmap);
        reader.read(rbuf.as_mut_slice()).unwrap();

        assert_eq!(rbuf, patt);
    }
    
    #[test]
    fn write_flush_read() {
        let mut mmap = setup(24123);
        let patt = (0..mmap.len()).map(|x| (x % 255) as u8).collect::<Vec<u8>>();
        let mut writer = MMapWriter::new(&mut mmap);
        writer.write(patt.as_slice()).unwrap();

        writer.flush().unwrap();

        let mut rbuf = vec![0xff; mmap.len()];
        let mut reader = MMapReader::new(&mmap);
        reader.read(rbuf.as_mut_slice()).unwrap();

        assert_eq!(rbuf, patt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use crate::anon::*;

    fn setup(len: usize) -> AnonMMapMut {
        let mut mmap = AnonMMapMut::new(AddrHint::None, len, MMapConfig::new().map_private()).unwrap();
        for i in 0..len {
            mmap[i] = (i % 0xff) as u8;
        }
        mmap
    }

    #[test]
    fn read_all() {
        let len = 24999;
        let mmap = setup(len);
        let mut reader = MMapReader::new(&mmap);

        let mut buf = vec![0xff; mmap.len()];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, mmap.len());
        for i in 0..n {
            assert_eq!(buf[i], (i % 255) as u8);
        }
    }

    #[test]
    fn consecutive_reads() {
        let len = 24999;
        let mmap = setup(len);
        let mut reader = MMapReader::new(&mmap);

        let mut buf = vec![0xff; 3];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 3);
        assert_eq!(buf, vec![0, 1, 2]);

        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 3);
        assert_eq!(buf, vec![3, 4, 5]);
        
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 3);
        assert_eq!(buf, vec![6, 7, 8]);
    }
    
    #[test]
    fn buffer_longer_than_mmap() {
        let len = 24999;
        let mmap = setup(len);
        let mut reader = MMapReader::new(&mmap);

        let mut buf = vec![0xff; 2 * mmap.len()];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, mmap.len()); // should only read n == mmap.len() either way!
        for i in 0..n {
            assert_eq!(buf[i], (i % 255) as u8);
        }
    }

    #[test]
    fn read_nothing_on_end_of_file() {
        let len = 24999;
        let mmap = setup(len);
        let mut reader = MMapReader::new(&mmap);

        let mut buf = vec![0xff; mmap.len()];
        let n = reader.read_to_end(&mut buf).unwrap();
        assert_eq!(n, mmap.len());

        // now, no more bytes should be read: read returns 
        // n == 0 bytes. 
        let mut buf = vec![0xff; mmap.len()];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn seek_from_start() {
        let len = 24999;
        let mmap = setup(len);
        let mut reader = MMapReader::new(&mmap);

        reader.seek(SeekFrom::Start(123)).unwrap();
        assert_eq!(123, reader.cur);

        // seek from start should always 'return' to start..
        reader.seek(SeekFrom::Start(4321)).unwrap();
        assert_eq!(4321, reader.cur);
    }

    /// Seek may go over the end of the file but any read thereafter
    /// will return that no bytes were read.
    #[test]
    fn over_seek() {
        let len = 24999;
        let mmap = setup(len);
        let mut reader = MMapReader::new(&mmap);
        reader.seek(SeekFrom::Start(2 * mmap.len() as u64)).unwrap();
        assert_eq!(2 * mmap.len() as u64, reader.cur);

        let mut buf = vec![0xff; 10];
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 0);
    }

}
