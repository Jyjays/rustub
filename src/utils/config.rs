use std::sync::atomic::AtomicBool;
//use std::sync::Arc;
use std::time::Duration;

// 定义预设常量
pub const INVALID_PAGE_ID: i32 = -1;
pub const INVALID_TXN_ID: i32 = -1;
pub const INVALID_LSN: i32 = -1;
pub const HEADER_PAGE_ID: i32 = 0;
pub const BUSTUB_PAGE_SIZE: usize = 4096;
pub const BUFFER_POOL_SIZE: usize = 10;
pub const LOG_BUFFER_SIZE: usize = (BUFFER_POOL_SIZE + 1) * BUSTUB_PAGE_SIZE;
pub const BUCKET_SIZE: usize = 50;
pub const LRUK_REPLACER_K: usize = 10;
pub const VARCHAR_DEFAULT_LENGTH: usize = 128;
pub const TXN_START_ID: i64 = 1 << 62;

// 使用类型别名
pub type FrameId = i32;
pub type PageId = i32;
pub type TxnId = i64;
pub type Lsn = i32;
pub type SlotOffset = usize;
pub type Oid = u16;

// 全局变量或动态全局变量
pub static CYCLE_DETECTION_INTERVAL: Duration = Duration::from_millis(100); // 可以根据需要进行修改
pub static ENABLE_LOGGING: AtomicBool = AtomicBool::new(false); // 用于线程安全的全局变量
pub static LOG_TIMEOUT: Duration = Duration::from_secs(30); // 日志超时时间，可以根据实际需要更改
