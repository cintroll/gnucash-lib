use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::BufReader;
use std::io::Read;

pub struct Book {

}

impl Book {
    pub fn open<R: Read>(reader: BufReader<R>) {
        let mut reader = Reader::from_reader(reader);
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    println!("{:?}", std::str::from_utf8(e.name()).unwrap());
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }

        }
    }
}