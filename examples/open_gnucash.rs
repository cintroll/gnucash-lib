extern crate gnucash;

fn main() {
    let gnucash = gnucash::GnuCash;
    let _ = gnucash.open("/home/matheus/workspace/gnucash-lib/examples/account.gnucash");
}