use std::{borrow::{Borrow, BorrowMut}, fmt::Error, fs::File, future::Future, io::{Read, Seek, Write}, pin::Pin, sync::{Arc, Mutex}};
use crate::{log, utils::config::{PageId, BUSTUB_PAGE_SIZE}};

pub struct DiskManager {
    pub log_io : File,
    pub log_name : String,
    pub db_io : Arc<Mutex<File>>,
    pub db_name : String,
    pub num_flushes : u64,
    pub num_writes : u64,
    pub flush_log : bool,
    pub flush_log_f : Option<Pin<Box<dyn Future<Output = ()>>>>,
    // pub db_io_latch : Arc<Mutex<()>>,
}

impl DiskManager {
    // pub fn new(log_name: &str, db_name: &str) -> Self {
    //     DiskManager {
    //         log_io: Arc::new(Mutex::new(File::create_new(log_name).unwrap())),
    //         log_name: log_name.to_string(),
    //         db_io: Arc::new(Mutex::new(File::create_new(db_name).unwrap())),
    //         db_name: db_name.to_string(),
    //         num_flushes: 0,
    //         num_writes: 0,
    //         flush_log: false,
    //         flush_log_f: None,
    //         // db_io_latch: Arc::new(Mutex::new(())),
    //     }
    // }
    pub fn new (db_file : &str) -> Self {
        if !db_file.contains('.') {
            log!("Invalid file name");
        }
        let (prev, _) = db_file.split_at(db_file.find('.').unwrap());
        let log_name = format!("{}{}", prev, ".log");
        let log_file = File::open(log_name.clone()).unwrap_or_else(|_|{
            File::create(log_name.clone()).unwrap()
        });
        let db_io_flie = Arc::new(Mutex::new(File::open(db_file).unwrap_or_else(|_|{
            File::create(db_file).unwrap()
        })));
        DiskManager {
            log_io: log_file,
            log_name,
            db_io: db_io_flie,
            db_name: db_file.to_string(),
            num_flushes: 0,
            num_writes: 0,
            flush_log: false,
            flush_log_f: None,
            // db_io_latch: mutex,
        }
    }
    pub fn shutdown(&mut self) {
        self.db_io.lock().unwrap().sync_all().unwrap();
        self.log_io.sync_all().unwrap();
    }
    pub fn write_page(&mut self, page_id : PageId, page_data : &[u8] ) {
        let mut binding = self.db_io.lock().unwrap();
        let db_io = binding.borrow_mut();
        let offset = page_id as u64 * BUSTUB_PAGE_SIZE as u64;
        db_io.seek(std::io::SeekFrom::Start(offset)).unwrap();
        db_io.write_all(page_data).unwrap_or_else(|_|{
            log!("Failed to write page");
        });
        self.num_writes += 1;
        db_io.sync_all().unwrap();
    }
    pub fn read_page(&mut self, page_id : PageId, page_data : &mut [u8]) {
        let mut binding = self.db_io.lock().unwrap();
        let db_io = binding.borrow_mut();
        let offset = page_id as u64 * BUSTUB_PAGE_SIZE as u64;
        if offset >= self.get_file_size(&self.db_name) {
            log!("Invalid page id");
            // return Err(Error);
        }
        db_io.seek(std::io::SeekFrom::Start(offset)).unwrap();
        db_io.read_exact(page_data).unwrap_or_else(|_|{
            log!("Failed to read page");
        });
        if let Some(count) = db_io.read(page_data).ok() {
            if count < BUSTUB_PAGE_SIZE {
                log!("Read less than a page");
                // return Err(Error);
                page_data[count..BUSTUB_PAGE_SIZE].fill(0);
            }
        }
    }
    pub async fn write_log(&mut self, log_data : &[u8], size : usize) {
        if size == 0 {
            return ;
        }
        self.flush_log = true;
        if self.flush_log_f.is_some() {
            if let Some(future) = self.flush_log_f.as_mut() {
                future.await;
            }
        }
        self.log_io.write_all(log_data).unwrap_or_else(|_|{
            log!("Failed to write log");
        });
        self.num_writes += 1;
        self.log_io.sync_all().unwrap();
        self.flush_log = false;
    }
    pub fn read_log(&mut self, offset : u64, log_data : &mut [u8], size : usize) {
        if offset >= self.get_file_size(&self.log_name) {
            log!("Invalid offset");
            // return Err(Error);
        }
        self.log_io.seek(std::io::SeekFrom::Start(offset)).unwrap();
        self.log_io.read_exact(log_data).unwrap_or_else(|_|{
            log!("Failed to read log");
        });
        if let Some(count) = self.log_io.read(log_data).ok() {
            if count < size {
                log!("Read less than a page");
                // return Err(Error);
                log_data[count..size].fill(0);
            }
        }
    }
    pub fn get_num_flushes(&self) -> u64 {
        self.num_flushes
    }
    pub fn get_num_writes(&self) -> u64 {
        self.num_writes
    }
    pub fn get_flush_state(&self) -> bool {
        self.flush_log
    }
    
    fn get_file_size(&self, file_name : &str) -> u64 {
        let file = File::open(file_name).unwrap();
        file.metadata().unwrap().len()
    }
}