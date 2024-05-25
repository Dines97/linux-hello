use crossbeam_channel::{Receiver, Sender};
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct BatchBlock {
    handle: Option<thread::JoinHandle<()>>,
    continue_flag: Arc<Mutex<bool>>,
}

impl BatchBlock {
    pub fn new_sized<T>(size: usize, receiver: Receiver<T>, sender: Sender<Vec<T>>) -> Self
    where
        T: Send + 'static,
    {
        let continue_flag = Arc::new(Mutex::new(true));
        let continue_flag_clone = Arc::clone(&continue_flag);

        let handle = Some(thread::spawn(move || {
            while *continue_flag_clone.lock().unwrap() {
                if receiver.len() >= size {
                    let output = receiver.iter().take(size).collect();
                    let _ = sender.send(output);
                }
            }
            let output = receiver.iter().collect();
            let _ = sender.send(output);
        }));

        Self { handle, continue_flag }
    }

    pub fn new_unsized<T>(receiver: Receiver<T>, sender: Sender<Vec<T>>) -> Self
    where
        T: Send + 'static,
    {
        let continue_flag = Arc::new(Mutex::new(true));
        let continue_flag_clone = Arc::clone(&continue_flag);

        let handle = Some(thread::spawn(move || {
            while *continue_flag_clone.lock().unwrap() {}
            let output = receiver.iter().collect();
            let _ = sender.send(output);
        }));

        Self { handle, continue_flag }
    }
}
