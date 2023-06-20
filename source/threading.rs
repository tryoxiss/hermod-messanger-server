use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        for id in 0..threads
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
        self.sender.send(job).unwrap();
        warning!("threading.rs:41:8 uses an unwrap method!! Don't unwrap!!");
    }
}

struct Worker
{
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker 
{
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker
    {
        let thread = thread::spawn(move || loop
        {
            // this is absolutely disgusting.

            // Also these unwraps are also apparently safe?
            // Can someone confirm? 
            // source: https://youtu.be/1AamFJGAE8E?t=1063
            let job = reciever
                .lock()
                .expect("Getting Mutex<> failed on threading.rs:78")
                .recv()
                .expect(".recv() failed on threading.rs:80");
            
            log!(format!("Worker #{id} got a job!"))
        });

        Worker { id: id, thread: thread }
    }
}