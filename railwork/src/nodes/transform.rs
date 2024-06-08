use crate::nodes::cycle::CycledNode;
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use std::time::Duration;

pub struct TransformNode {
    inner: CycledNode,
}

impl TransformNode {
    pub fn new<F, Input, Output>(mut func: F, receiver: Receiver<Input>, sender: Sender<Output>) -> Self
    where
        F: FnMut(Input) -> Output + Send + 'static,
        Input: Send + 'static,
        Output: Send + 'static,
    {
        let inner = CycledNode::new(move |x| match receiver.try_recv() {
            Ok(input) => {
                let output = func(input);
                if let Err(e) = sender.send(output) {
                    log::error!("Unable to send: {}", e);
                    *x.lock().unwrap() = false;
                };
            }
            Err(e) => match e {
                TryRecvError::Empty => std::thread::sleep(Duration::from_millis(10)),
                TryRecvError::Disconnected => {
                    log::error!("Unable to receive: {}", e);
                    *x.lock().unwrap() = false;
                }
            },
        });

        Self { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct A {
        acc: i32,
    }

    impl A {
        fn run(&mut self, x: i32) -> i32 {
            self.acc += x;
            self.acc
        }
    }

    #[test]
    fn it_works() {
        let mut a = A { acc: 2 };

        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();

        let _ = tx1.send(2);
        let _ = tx1.send(4);
        let _ = tx1.send(8);

        let _block = TransformNode::new(move |x| a.run(x), rx1, tx2);

        assert_eq!(rx2.recv().unwrap(), 4);
        assert_eq!(rx2.recv().unwrap(), 8);
        assert_eq!(rx2.recv().unwrap(), 16);
    }
}
