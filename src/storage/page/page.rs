use crate::utils::config::*;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::Arc;
use std::mem::size_of;

// pub const BUSTUB_PAGE_SIZE: usize = 4096;

pub const SIZE_PAGE_HEADER: usize = 8;

pub const OFFSET_LSN: usize = 4;


/// Page is the basic unit of storage within the database system. Page provides a wrapper for actual data pages being
/// held in main memory. Page also contains book-keeping information that is used by the buffer pool manager, e.g.
/// pin count, dirty flag, page id, etc.
#[derive(Debug)]
pub struct Page {
    /// Store the actual data of the page.
    data: Vec<u8>,
    /// The page id of the page.
    page_id: PageId,
    /// The pin count of the page.
    pin_count: i32,
    /// True if the page is dirty, i.e. it is different from its corresponding page on disk.
    is_dirty: bool,
    /// Page latch.
    rwlatch: Arc<RwLock<()>>,
}

impl Page {
    /// Constructor. 
    pub fn new() -> Self {
        let mut page = Page {
            data: vec![0; BUSTUB_PAGE_SIZE],
            page_id: INVALID_PAGE_ID,
            pin_count: 0,
            is_dirty: false,
            rwlatch: Arc::new(RwLock::new(())),
        };
        page.reset_memory();
        page
    }

    /// Return the data of the page.
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// Return the id of the page.
    pub fn get_page_id(&self) -> PageId {
        self.page_id
    }

    /// Return the pin count of the page.
    pub fn get_pin_count(&self) -> i32 {
        self.pin_count
    }

    /// Return true if the page is dirty.
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    /// Acquire the write latch of the page.
    pub fn w_latch(&self) -> RwLockWriteGuard<()> {
        self.rwlatch.write().unwrap()
    }

    /// Release the write latch of the page.
    pub fn w_unlatch(&self) {
        drop(self.w_latch());
    }

    /// Acquire the read latch of the page.
    pub fn r_latch(&self) -> RwLockReadGuard<()> {
        self.rwlatch.read().unwrap()
    }

    /// Release the read latch of the page.
    pub fn r_unlatch(&self) {
        drop(self.r_latch());
    }

    /// Return the log sequence number (LSN) of the page.
    pub fn get_lsn(&self) -> Lsn {
        let lsn_bytes = &self.data[OFFSET_LSN..OFFSET_LSN + size_of::<Lsn>()];
        Lsn::from_le_bytes(lsn_bytes.try_into().unwrap())
    }

    /// Set the log sequence number (LSN) of the page.
    pub fn set_lsn(&mut self, lsn: Lsn) {
        let lsn_bytes = lsn.to_le_bytes();
        self.data[OFFSET_LSN..OFFSET_LSN + size_of::<Lsn>()].copy_from_slice(&lsn_bytes);
    }

    /// Reset the memory of the page.
    fn reset_memory(&mut self) {
        self.data.fill(0);
    }
}
