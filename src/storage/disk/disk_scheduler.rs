use std::{sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::JoinHandle};
use tokio::sync::oneshot;

use crate::utils::config::PageId;

use super::disk_manager::DiskManager;

struct DiskRequest {
    is_write: bool,
    data: Vec<u8>,
    page_id: PageId,
    callback: oneshot::Sender<bool>,
}

pub struct DiskScheduler {
    disk_manager: Arc<Mutex<DiskManager>>,
    request_sender: Sender<Option<DiskRequest>>,
}

impl DiskScheduler {
    pub fn new(disk_manager: Arc<Mutex<DiskManager>>) -> Self{
        let (rs, rr) = mpsc::channel();
        let disk_scheduler = DiskScheduler {
            disk_manager,
            request_sender: rs,
        };
        disk_scheduler.start_worker_thread(rr);
        disk_scheduler
    }
    fn start_worker_thread(&self, request_receiver: Receiver<Option<DiskRequest>>) -> JoinHandle<()> {
        let disk_manager = self.disk_manager.clone();
        std::thread::spawn(move || {
            while let Some(mut request) = request_receiver.recv().unwrap() {
                if request.is_write {
                    disk_manager.lock().unwrap().write_page(request.page_id, &request.data);
                } else {
                    disk_manager.lock().unwrap().read_page(request.page_id, &mut request.data);
                }
                request.callback.send(true).unwrap();
            }
        })
    }   
}
unsafe impl Send for DiskRequest{}
unsafe impl Sync for DiskRequest{}
unsafe impl Send for DiskManager{}
unsafe impl Sync for DiskManager{}