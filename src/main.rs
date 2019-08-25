extern crate hypercore;

use hypercore::{Feed, Storage, Store};
use random_access_disk::{RandomAccessDisk};
use random_access_memory::{RandomAccessMemory};
use std::result::Result;
use failure::Error;

fn create_feed(page_size: usize) -> Result<Feed<RandomAccessMemory>, Error> {
    let create = |_store: Store| Ok(RandomAccessMemory::new(page_size));
    let storage = Storage::new(create)?;
    Ok(Feed::with_storage(storage)?)
}
fn main() -> Result<(), String> {
    let mut feed = create_feed(1024).map_err(|e| format!("{}", e))?;
    // let mut feed = Feed::open(std::path::PathBuf::from("/tmp/test")).expect("cannot create feed");
    for i in 0..10000 { 
        println!("{}", i);
        feed.append("hellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohellohello".as_bytes()).map_err(|e| format!("{}", e))?
    }
    println!("byte length: {}", feed.byte_len());
    Ok(())
}