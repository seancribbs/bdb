use std::env;

mod database;
mod entry;
mod page;

use database::DB;
// use entry::Entry;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let db = DB::open(filename);
    for page in db.pages() {
        println!("{:#?}", page.header);
        for (idx, entry) in page.entries().enumerate() {
            if page.is_leaf() {
                let key_or_value = if idx % 2 == 0 { "Key" } else { "Value" };
                println!("{key_or_value}: {:?}", entry);
            } else {
                println!("{:?}", entry);
            }
        }
    }
}
