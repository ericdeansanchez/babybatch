use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::comm::Message;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect(
                    r#"
baby-batch: Worker::new failed to acquire lock it is possible that another
      user of the given mutex panicked while holding the mutex.
"#,
                )
                .recv()
                .expect(
                    r#"
baby-batch: Worker::new failed to `recv` from lock it is possible that the
      Sender has been disconnected.
"#,
                );

            match message {
                Message::NewJob(job) => {
                    job.call_box();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn take_thread(&mut self) -> Option<thread::JoinHandle<()>> {
        self.thread.take()
    }
}
