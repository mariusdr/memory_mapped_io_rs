use libc;

#[derive(Clone, Copy, Debug)]
pub struct MMapConfig {
    flags: i32,    
}

#[derive(Clone, Copy, Debug)]
pub struct MAdviseConfig {
    flags: i32,    
}

impl MMapConfig {
    /// Create empty option set.
    pub fn new() -> Self {
        Self { flags: 0 }
    }

    pub fn value(&self) -> i32 {
        self.flags
    }

    /// Utility function.
    fn set_flag(mut self, f: i32) -> Self {
        self.flags = self.flags | f;
        self
    }

    /// Shared mapping, writes are visible to other processes mapping the 
    /// same region and (if not anon) are carried through to the file 
    /// system (the timing of this is subject to OS buffering).
    pub fn map_shared(self) -> Self {
        // TODO: on linux we should maybe use MAP_SHARED_VALIDATE instead?
        self.set_flag(libc::MAP_SHARED)
    }

    /// Private copy-on-write mapping. Writes are not visible to other 
    /// processes or the file system.
    pub fn map_private(self) -> Self {
        self.set_flag(libc::MAP_PRIVATE)
    }

    /// Do not permit the system to choose any other address as the one 
    /// provided. If the given address cannot be used mmap will fail. 
    /// If set, the caller must ensure that the address is page aligned.
    /// 
    /// If any other currently existing mapping contains pages starting at the
    /// given address, the overlapping region between the address and 
    /// address + length is removed from the other mapping!
    pub fn map_fixed(self) -> Self {
        self.set_flag(libc::MAP_FIXED)
    }

    /// Allow mapping to be both, writable and executable, when the hardened 
    /// runtime is enabled. 
    #[cfg(target_os = "macos")] 
    pub fn map_jit(self) -> Self {
        self.set_flag(libc::MAP_JIT)
    }

    /// Mapped pages are not retained in the kernel's page cache. If the system's memory 
    /// runs low, MAP_NOCACHE pages will be the first to be reclaimed.
    #[cfg(target_os = "macos")] 
    pub fn map_nocache(self) -> Self {
        self.set_flag(libc::MAP_NOCACHE)
    }

    /// Notify the kernel that this region contains a semaphore.
    #[cfg(target_os = "macos")] 
    pub fn map_hassemaphore(self) -> Self {
        self.set_flag(libc::MAP_HASSEMAPHORE)
    }
}

impl MAdviseConfig {
    /// Create empty option set.
    pub fn new() -> Self {
        Self { flags: 0 }
    }

    pub fn value(&self) -> i32 {
        self.flags
    }

    /// Utility function.
    fn set_flag(mut self, f: i32) -> Self {
        self.flags = self.flags | f;
        self
    }

    /// Indicate to the kernel that application expects to access memory
    /// in a sequential manner.
    pub fn madv_sequential(self) -> Self {
        self.set_flag(libc::MADV_SEQUENTIAL);
        self
    } 

    /// Indicate to the kernel that application expects to access memory
    /// in a random manner.
    pub fn madv_random(self) -> Self {
        self.set_flag(libc::MADV_RANDOM);
        self
    } 

    /// Indicate to the kernel that the applications intends to access this 
    /// address soon. 
    pub fn madv_willneed(self) -> Self {
        self.set_flag(libc::MADV_WILLNEED);
        self
    }

    /// Indicate to the kernel that the applications does not need this 
    /// address range any time soon. 
    /// 
    /// On Linux this will result in the freeing of the underlying pages
    /// and subsequent accesses will accesses to the address range will 
    /// succeed but the memory will be repopulated with the underlying file 
    /// data, if this is a file mapping, or zero mapped pages for anonymous
    /// and private mappings.
    pub fn madv_dontneed(self) -> Self {
        self.set_flag(libc::MADV_DONTNEED);
        self
    }

    /// Indicate to the kernel that this address range is not needed any 
    /// more and the mapped pages can be reused right away. The mapped
    /// memory will remain valid. (Like madv_dontneed on Linux).
    #[cfg(target_os = "macos")] 
    pub fn madv_free(self) -> Self {
        self.set_flag(libc::MADV_FREE);
        self
    }

    /// Tell the kernel that the mapped pages need to be zeroed out if the 
    /// address range is deallocated without first unwiring the pages. E.g.
    /// when the munmap syscall is called without a preceeding munlock or
    /// the application quits.
    #[cfg(target_os = "macos")] 
    pub fn madv_zero_wired_pages(self) -> Self {
        self.set_flag(libc::MADV_ZERO_WIRED_PAGES);
        self
    }
}