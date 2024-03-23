use anyhow::{anyhow, Result};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

type Job = Box<dyn FnOnce() -> Result<()> + Send + 'static>;

enum Message {
    Work(Job),
    Exit,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> Result<ThreadPool> {
        if num_threads == 0 {
            return Err(anyhow!(
                "Tried to construct a thread pool with zero threads 🤔"
            ));
        }

        let mut workers = Vec::with_capacity(num_threads);
        let (sender, receiver) = channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..num_threads {
            workers.push(Worker::new(id, receiver.clone()));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        let job = Box::new(job);
        self.sender
            .send(Message::Work(job))
            .expect("should be able to send a job to a worker");
    }
}

/// Need to make sure that threads are cleaned up when they are dropped!
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender
                .send(Message::Exit)
                .expect("couldn't sent 'exit' message to worker");
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().expect(
                    "should be able to gracefully kill a worker thread",
                );
                if cfg!(debug_assertions) {
                    println!("successfully killed {}", worker.id);
                }
            }
        }
    }
}

struct Worker {
    thread: Option<JoinHandle<()>>,
    id: usize,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            if cfg!(debug_assertions) {
                println!(
                    "creating thread {}, {}",
                    id,
                    thread::current().id().as_u64()
                );
            }

            loop {
                let message = receiver
                    .lock()
                    .expect("should be able to lock mutex in worker")
                    .recv()
                    .expect("should be able to receive job in worker");

                match message {
                    Message::Work(job) => {
                        job().unwrap();
                    }
                    Message::Exit => {
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
