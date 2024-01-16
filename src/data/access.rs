use std::thread;
use crate::data::DATA;

/// To access data, create a thread, spawn it, then get the lock.
/// When you're done, then join the thread with its parent thread.
pub async fn print_data() {
  thread::spawn(move || {
    let data = DATA.lock().unwrap();
    println!("data: {:#?}", data);
  })
  .join()
  .unwrap()
}
