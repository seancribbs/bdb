use crate::page::{EntryIterator, Page, PageHeader};
use std::fs;
use std::path::Path;

use crate::entry::Entry;

pub struct DB {
    buffer: Vec<u8>,
}

impl DB {
    pub fn open(filename: impl AsRef<Path>) -> Self {
        let buffer = fs::read(filename).unwrap();
        Self { buffer }
    }

    pub fn pages(&self) -> impl Iterator<Item = Page<'_>> {
        self.raw_pages().map(Page::new)
    }

    pub fn raw_pages(&self) -> impl Iterator<Item = &[u8]> {
        self.buffer.chunks(4096)
    }

    pub fn walk(&'a self) -> impl Iterator<Item = (Entry<'a>, Entry<'a>)> {
        Walk::new(self)
    }
}

struct Walk<'a> {
    page: usize,
    entry: usize,
    pages: Vec<Page<'a>>,
}

impl<'a> Walk<'a> {
    fn new(db: &'a DB) -> Self {
        Self {
            page: 0,
            entry: 0,
            pages: db.pages().collect(),
        }
    }

    fn move_to_next_page(&mut self) {
        let page = &self.pages[self.page];
        let PageHeader::BTree {
            level: 1,
            next_pgno,
            ..
        } = page.header
        else {
            unreachable!()
        };
        if next_pgno != 0 {
            self.page = next_pgno as usize;
            self.entry = 0;
        }
    }

    fn current_page(&self) -> &Page<'a> {
        &self.pages[self.page]
    }

    fn move_to_leaf(&mut self) {
        loop {
            let page = self.current_page();
            match page.header {
                // is_leaf()
                PageHeader::BTree { level: 1, .. } => {
                    self.entry = 0;
                    break;
                }
                PageHeader::BTree { .. } => {
                    let Entry::Internal { pgno, .. } = page.entries().next().unwrap() else {
                        unreachable!()
                    };
                    self.page = pgno as usize;
                }
                PageHeader::Metadata { root, .. } => {
                    self.page = root as usize;
                }
            }
        }
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = (Entry<'a>, Entry<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.move_to_leaf();
        let page = &self.pages[self.page];
        match (page.get_entry(self.entry), page.get_entry(self.entry + 1)) {
            (Some(k), Some(v)) => {
                self.entry += 2;
                return Some((k, v));
            }
            (None, None) => {
                if let Some(next) = page.next_page_number() {
                    self.page = next as usize;
                    self.entry = 0;
                    return self.next();
                }
            }
            _ => panic!("number of entries is odd"),
        }
        // (k, v)
        // 1. try to read k => Some(..)
        //    try to read v => Some(..) => return Some(k, v)
        // 2. try to read k => None
        //    are we at the last page? => return None
        //    otherwise: move to the next page and try to read again
        None
    }
}
