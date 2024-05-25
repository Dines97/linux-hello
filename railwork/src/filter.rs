use crossbeam_channel::{Receiver, Sender};
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct FilterBlock {
    handle: Option<thread::JoinHandle<()>>,
    continue_flag: Arc<Mutex<bool>>,
}

impl FilterBlock {
    pub fn new<T, U>(receiver: Receiver<U>, sender: Sender<T>) -> Self
    where
        T: Send + 'static,
        U: Into<Option<T>> + Send + 'static,
    {
        let continue_flag = Arc::new(Mutex::new(true));
        let continue_flag_clone = Arc::clone(&continue_flag);

        let handle = Some(thread::spawn(move || {
            while *continue_flag_clone.lock().unwrap() {
                if let Ok(input) = receiver.recv() {
                    if let Some(x) = input.into() {
                        let _ = sender.send(x);
                    }
                }
            }
        }));

        Self { handle, continue_flag }
    }
}
