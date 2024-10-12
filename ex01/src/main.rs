use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

const BUFFER_SIZE: usize = 5;
const THREAD_AMOUNT: usize = 10;
const MSG_AMOUNT: usize = 10;

struct Logger<W> {
    buffer: Box<[u8]>,
    writer: W,
}

impl<W> Logger<W> {
    pub fn new(threshold: usize, writer: W) -> Self {
        Logger {
            buffer: Vec::into_boxed_slice(vec![0; threshold]),
            writer,
        }
    }
}

impl<W: io::Write> Logger<W> {
    pub fn log(&mut self, message: &str) -> io::Result<()> {
        self.buffer.len()
    }
    pub fn flush(&mut self) {
        writeln!(&self.writer, "{}", self.buffer);
    }
}

/**
* allowed symbols:
   std::sync::Mutex
   std::thread::spawn
   std::io::Write
   std::vec::Vec::into_boxed_slice
*/
fn main() {
    let mut logger = Arc::new(Mutex::new(Logger::new(BUFFER_SIZE, io::stdout())));
    let mut threads = vec![];

    // let mut logger_ref;
    for i in 0..THREAD_AMOUNT {
        let logger_ref = logger.clone();
        let handle = thread::spawn(move || {
            for j in 0..MSG_AMOUNT {
                logger_ref
                    .lock()
                    .unwrap()
                    .log(&["hello", &j.to_string(), "from thread", &i.to_string()].join(" "));
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}

// fn main() {
//     let counter = Arc::new(Mutex::new(0));

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }
