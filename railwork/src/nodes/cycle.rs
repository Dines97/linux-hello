use crate::nodes::thread::ThreadNode;
use std::{
    marker::Send,
    sync::{Arc, Mutex},
};

pub(crate) struct CycledNode {
    inner: ThreadNode,
    continue_flag: Arc<Mutex<bool>>,
}

impl CycledNode {
    pub(crate) fn new<F>(mut func: F) -> Self
    where
        F: FnMut(&mut Arc<Mutex<bool>>) + Send + 'static,
    {
        let continue_flag = Arc::new(Mutex::new(true));
        let mut continue_flag_clone = Arc::clone(&continue_flag);

        let inner = ThreadNode::new(move || {
            while *continue_flag_clone.lock().unwrap() {
                func(&mut continue_flag_clone);
            }
        });

        Self { inner, continue_flag }
    }
}

impl Drop for CycledNode {
    fn drop(&mut self) {
        *self.continue_flag.lock().unwrap() = false;
    }
}
