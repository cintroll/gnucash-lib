use std::fs::File;
use std::io::BufReader;
use std::io::copy;
use flate2::read::GzDecoder;

fn main() -> Result<(), std::io::Error> {
    let gnucash = File::open("/home/matheus/workspace/gnucash-lib/samples/account.gnucash")?;
    let mut xml = BufReader::new(GzDecoder::new(gnucash));

    let mut gnucash_xml = File::create("/home/matheus/workspace/gnucash-lib/samples/account.xml")?;

    let _ = copy(&mut xml, &mut gnucash_xml);
   
    Ok(())
}
