//! Query Nyaa.si and print results in a table.

extern crate nyaadesu;

fn main() {
    // Fetch torrents.
    let torrents = nyaadesu::torrent_search("little witch academia enchanted parade");

    match torrents {
        Ok(ts) => print_torrents(ts),
        Err(e) => handle_error(e),
    }
}

fn print_torrents(ts: Vec<nyaadesu::Torrent>) {
    if ts.is_empty() {
        println!("There are no torrents!");
    } else {
        // table header
        print!("Name                                     ");
        print!("Link ");
        print!("Date             ");
        print!("Seeders/Leechers/Completed");
        print!("\n");

        for t in ts.iter().take(20) {
            // Print torrent's name.
            let mut name = t.name.clone();
            name.truncate(40);
            
            print!("{} ", name);

            // Print "M" and "T" respectively if magnet and .torrent are
            // available
            match t.magnet_link() {
                Some(_) => print!("M"),
                None => print!(" "),
            }

            match t.torrent_file() {
                Some(_) => print!("T   "),
                None => print!("    "),
            }

            // Print torrent's date.
            print!("{} ", t.date);

            // Print seeders/leechers/completed
            print!("{}/{}/{}", t.seeders, t.leechers, t.completed_downloads);

            // Final newline.
            print!("\n");
        }
    }
}

fn handle_error(e: nyaadesu::Error) {
    match e {
        nyaadesu::Error::Request => println!("There was a problem contacting Nyaa.si"),
        nyaadesu::Error::Scraping => panic!("There was a scraping problem. Please report this bug at https//github.com/gRastello/nyaadesu"),
    }
}
