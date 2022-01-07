use std::io::Read;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;

pub struct Account {

}

impl Account {
    pub fn open<R: Read>(reader: &mut Reader<BufReader<R>>) {
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                   match e.name() {
                       _ => (),
                   } 
                },

                Ok(Event::End(ref e)) => {
                    if let b"gnc:account" = e.name() {
                        break;
                    }
                },

                Ok(Event::Eof) => panic!("Unexpected EOF before closing account tag!"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }

    }
}