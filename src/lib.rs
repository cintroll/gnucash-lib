use std::fs::File;
use std::io::BufReader;
use flate2::read::GzDecoder;

mod book;

pub fn open(path: &str) {
    let gnucash = File::open(path).unwrap();
    let xml = BufReader::new(GzDecoder::new(gnucash));

    book::Book::open(xml)
}
