use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
};

pub trait Produce: Send {
    type Output;
    fn run(&mut self) -> Self::Output;
}

pub struct ProduceBlock {
    handle: Option<thread::JoinHandle<()>>,
    continue_flag: Arc<Mutex<bool>>,
}

impl Drop for ProduceBlock {
    fn drop(&mut self) {
        log::info!("Stopping block");

        log::info!("Stoping loop");
        *self.continue_flag.lock().unwrap() = false;

        if let Some(handle) = self.handle.take() {
            handle.join().expect("Unable to join thread");
            log::info!("Joined block thread");
        }
    }
}

impl ProduceBlock {
    pub fn new<T>(mut runnable: T, sender: Sender<T::Output>) -> Self
    where
        T: Produce + 'static,
        T::Output: Send,
    {
        let continue_flag = Arc::new(Mutex::new(true));
        let continue_flag_clone = Arc::clone(&continue_flag);

        let handle = Some(thread::spawn(move || {
            while *continue_flag_clone.lock().unwrap() {
                let output = runnable.run();
                let _ = sender.send(output);
            }
        }));

        Self { handle, continue_flag }
    }
}
