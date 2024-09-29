use std::{fs::File, future::Future, pin::Pin, sync::{Arc, Mutex}};


pub struct DiskManager {
    pub log_io : Arc<Mutex<File>>,
    pub log_name : String,
    pub db_io : Arc<Mutex<File>>,
    pub db_name : String,
    pub num_flushes : u64,
    pub num_writes : u64,
    pub flush_log : bool,
    pub flush_log_f : Option<Pin<Box<dyn Future<Output = ()>>>>,
    //pub db_io_latch : Arc<Mutex<()>>,
}

impl DiskManager {
    pub fn new(log_name: &str, db_name: &str) -> Self {
        DiskManager {
            log_io: Arc::new(Mutex::new(File::create_new(log_name).unwrap())),
            log_name: log_name.to_string(),
            db_io: Arc::new(Mutex::new(File::create_new(db_name).unwrap())),
            db_name: db_name.to_string(),
            num_flushes: 0,
            num_writes: 0,
            flush_log: false,
            flush_log_f: None,
            // db_io_latch: Arc::new(Mutex::new(())),
        }
    }
}