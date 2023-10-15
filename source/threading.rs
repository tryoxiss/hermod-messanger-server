/*
This file is a part of Hermod Messanger Server.

	Copyright (C) 2023-Present Hermod Messanger Contributers. (AUTHORS.md)
	Under The GNU Affero General Public Licence 3.0 ONLY (LICENCE.md)

	If for any reason the licence file was not provided, you may obtain a
	copy at <https://www.gnu.org/licenses/agpl-3.0.txt>.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.
*/

/*
This file was authored by:
	* Madeline "tryoxiss"
*/

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use log::trace;
use log::debug;
use log::error;

use crate::{ENDBLOCK, CODE_START, INDENT};

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
	/// Create a new ThreadPool
	/// The size is the number of threads in the ThreadPool
	///
	/// This function will panic if threads is 0.
	pub fn new(threads: u16) -> ThreadPool
	{
		assert!(threads > 0);

		let (sender, reciever) = mpsc::channel();
		let reciever = Arc::new(Mutex::new(reciever));

		// 🚩 FIXME: Unnecesary memory useage
		// We use extra bits to represent a usize when the max size is a u16,
		// this simply will not stand.
		let mut workers = Vec::with_capacity(usize::from(threads));

		for id in 0..threads + 1
		{
			// Worker numbers get printed out and written to traces,
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
			Ok(_message) => { /* nothing to do */ }
			Err(message) => { error!("Something went wrong in {CODE_START}threading.rs:ThreadPool:run{ENDBLOCK} (line 62).
{INDENT}Here is the rust error message: \n {}", message) }
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
			debug!("Shutting down worker thread #{}", worker.id);

			// This is an if statement, meaning its possible to de-nest this.
			// I couldn't figure it out.
			if let Some(thread) = worker.thread.take()
			{
				// this unwrap is psudo-safe since we don't care if it panics: 
				// we are shutting it down anyway.
				thread.join().unwrap();
			}
		}
	}
}

struct Worker
{
	id: u16,
	thread: Option<thread::JoinHandle<()>>
}

impl Worker
{
	fn new(id: u16, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker
	{
		let thread = thread::Builder::new().name(id.to_string()).spawn(move || loop
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
				Message::NewJob(job) => Worker::handle_job(job, id),
				Message::Terminate => break
			}
		});

		return Worker { id: id, thread: Some(thread.unwrap()) }
	}

	fn handle_job(job: Job, id: u16)
	{
		debug!("Worker #{id} got a job!");
		job();
		debug!("Worker #{id} finished its job!");
	}
}