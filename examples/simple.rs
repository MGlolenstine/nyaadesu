//! Fetch torrents for "Puella Magi Madoka Magica" and print their names.

extern crate nyaadesu;

fn main() {
    println!("Fetching torrents...");
    let torrents = nyaadesu::torrent_search("little witch academia enchanted parade");

    match torrents {
        Ok(ts) => print_torrents(ts),
        Err(e) => handle_error(e),
    }
}

fn print_torrents(ts: Vec<nyaadesu::Torrent>) {
    if ts.is_empty() {
        println!("There are no torrents!")
    } else {
        ts.iter().for_each(|t| println!("{:?}", t.name))
    }
}

fn handle_error(e: nyaadesu::Error) {
    match e {
        nyaadesu::Error::Request => println!("There was a problem contacting Nyaa.si"),
        nyaadesu::Error::Scraping => panic!("There was a scraping problem. Please report this bug at https//github.com/gRastello/nyaadesu"),
    }
}
