use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message { 
    NewJob(Job),
    Terminate,
}

impl ThreadPool 
{
    /// Create a new ThreadBook
    /// The size is the number of threads in the ThreadPool
    /// 
    /// This function will panic if threads is 0.
    pub fn new(threads: usize) -> ThreadPool
    {
        assert!(threads > 0);

        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(threads);

        for id in 0..threads + 1
        {
            // Worker numbers get printed out and written to logs,
            // so we want them to start at 1, not 0. 
            if id == 0 { continue }
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { workers, sender }
    }

    /// same interface as Thread::spawn
    pub fn run<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        // Apparently this is a safe unwrap: https://youtu.be/1AamFJGAE8E?t=1028
        // Can someone confirm?
        // 
        // My understanding is it returens a result type, which would error if a
        // thread shuts down. But since threads don't shut down unless something
        // bad happened, its fine. 
        // 
        // I still think we should type match to show that its intentional at
        // least. 
        match self.sender.send(Message::NewJob(job))
        {
            Ok(message) => { /* nothing to do */ }
            Err(message) => { error!(format!("Something went wrong in threading.rs:ThreadPool:run (line 62). 
             Here is the rust error message: \n {}", message)) }
        }
    }
}

impl Drop for ThreadPool
{
    fn drop(&mut self) 
    {
        for _ in &self.workers
        {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers
        {
            info!(format!("Shutting down worker thread #{}", worker.id));

            // This is an if statement, meaning its possible to de-nest this.
            // I couldn't figure it out.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker
{
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker 
{
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker
    {
        let thread = thread::spawn(move || loop
        {
            // this is absolutely disgusting.

            // Also these unwraps are also apparently safe?
            // Can someone confirm? 
            // source: https://youtu.be/1AamFJGAE8E?t=1063
            let message = reciever
                .lock()
                .unwrap()
                .recv()
                .unwrap();
            
            match message
            {
                Message::NewJob(job) => 
                {
                    log!(format!("Worker #{id} got a job!"));
                    job();
                    log!(format!("Worker #{id} finished its job!"))
                }
                Message::Terminate => { break; }
            }
        });

        Worker { id: id, thread: Some(thread) }
    }
}