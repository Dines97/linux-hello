use crate::nodes::cycle::CycledNode;
use crossbeam_channel::Sender;
use cycle_controller::CycleController;

pub struct ProduceNode {
    inner: CycledNode,
}

impl ProduceNode {
    pub fn new<F, Output>(mut func: F, sender: Sender<Output>, enable_cycle_controller: bool) -> Self
    where
        F: FnMut() -> Output + Send + 'static,
        Output: Send + 'static,
    {
        let mut cycle_controller = enable_cycle_controller.then(CycleController::default);

        let inner = CycledNode::new(move |x| {
            let output = func();
            if let Some(x) = cycle_controller.as_mut() {
                log::trace!("{}", x);
                x.update();
            }
            if let Err(e) = sender.send(output) {
                log::error!("Unable to send: {}", e);
                *x.lock().unwrap() = false;
            };
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
        fn run(&mut self) -> i32 {
            self.acc = (2 * self.acc) % 2048;
            self.acc
        }
    }

    #[test]
    fn it_works() {
        let mut a = A { acc: 2 };

        let (tx, rx) = crossbeam_channel::unbounded();

        let _block = ProduceNode::new(move || a.run(), tx, false);
        // rx.recv().unwrap();

        assert_eq!(rx.recv().unwrap(), 4);
        assert_eq!(rx.recv().unwrap(), 8);
        assert_eq!(rx.recv().unwrap(), 16);
    }
}
