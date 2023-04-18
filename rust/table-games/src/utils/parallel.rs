use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub enum Job {
    SimulateHour(u128),
    CalculateMetrics(Vec<i128>),
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

pub enum PoolCreationError {
    InvalidSize,
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::InvalidSize);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let workers = 0..size;
        let workers: Vec<_> = workers
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();

        Ok(ThreadPool { workers, sender: Some(sender) })
    }

    pub fn execute(&self, job: Job) {
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .expect("Could not send job to thread pool");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        drop(self.sender.take());

        for worker in &mut self.workers {

            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            let job = match message {
                Ok(job) => job,
                Err(_) => break,
            };

            match job {
                Job::SimulateHour(hour) => {
                    println!("Worker {} simulated hour {}", id, hour);
                }
                Job::CalculateMetrics(results) => {
                    println!("Worker {} calculated metrics for hour {}", id, id);
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}