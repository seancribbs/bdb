use std::env;

mod database;
mod entry;
mod page;

use database::DB;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let db = DB::open(filename);
    for page in db.pages() {
        println!("{:#?}", page.header);
    }
    println!("Key-Value Data:");
    for (key, value) in db.walk() {
        println!("   Key: {key}");
        println!("   Value: {value}");
    }
}
