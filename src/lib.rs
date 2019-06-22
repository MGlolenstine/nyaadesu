extern crate html5ever;
extern crate reqwest;

use html5ever::tendril::TendrilSink;
use std::rc::Rc;

/// Struct that represents a torrent and contains some of its basic information
/// like category, name, size, date, number of seeders, number of leechers and
/// number of completed downloads.
#[derive(Debug)]
pub struct Torrent {
    /// The category of the torrent.
    pub category: Category,
    /// The name of the torrent.
    pub name: String,
    /// Links for the torrent (magnet and .torrent) as a pair.
    links: (Option<String>, Option<String>),
    /// Total size of the torrent's files in bytes.
    pub size: f64,
    /// Date of pubblication of the torrent as unix timestamp.
    pub date: String,
    /// Number of seeders.
    pub seeders: i32,
    /// Number of leechers.
    pub leechers: i32,
    /// Number of completed downloads.
    pub completed_downloads: i32,
}

/// Enum that encodes a torrent's category.
#[derive(Debug)]
pub enum Category {
    Anime(Anime),
    Audio(Audio),
    Literature(Literature),
    LiveAction(LiveAction),
    Pictures(Pictures),
    Software(Software),
}

/// Enum that encodes variants of anime torrents.
#[derive(Debug)]
pub enum Anime {
    AnimeMusicVideo,
    EnglishTranslated,
    NonEnglishTranslated,
    Raw,
}

/// Enum that encodes variants of audio torrents.
#[derive(Debug)]
pub enum Audio {
    Lossless,
    Lossy,
}

/// Enum that encodes variants of literature torrents.
#[derive(Debug)]
pub enum Literature {
    EnglishTranslated,
    NonEnglishTranslated,
    Raw,
}

/// Enum that encodes variants of live action torrents.
#[derive(Debug)]
pub enum LiveAction {
    EnglishTranslated,
    IdolPromotionalVideo,
    NonEnglishTranslated,
    Raw,
}

/// Enum that encodes variants of pictures torrents.
#[derive(Debug)]
pub enum Pictures {
    Graphics,
    Photos,
}

/// Enum that encodes variants of software torrents.
#[derive(Debug)]
pub enum Software {
    Applications,
    Games,
}

/// Emun that encodes possible errors
#[derive(Debug)]
pub enum Error {
    /// Error with the request e.g. the server do not respond or you don't have
    /// access to it.
    Request,
    /// Scraping error. Since all the data is obtained via scraping of html
    /// pages if somehting in how Nyaa.si's pages are generated is changed
    /// searching for torrents may fail. This should hopefully never happen but
    /// I highly suggest you to always check for scaping errors and, if you ever
    /// record one, to report it as an issue on GitHub so that the library can
    /// be updated.
    Scraping,
}

// Enum that encodes diffent types of pages: page with torrents, page with no
// results (searching for somehting that is not on Nyaa.si) and page with no
// more results (e.g. page 6 of a query that return 4 pages of results).
enum Page {
    Torrents,
    NoTorrents,
    NoMoreTorrents,
}

/// Search Nyaa.si for torrents by name.
pub fn torrent_search(s: &str) -> Result<Vec<Torrent>, Error> {
    let client = reqwest::Client::new();

    let mut all_the_torrents = vec![];

    for n in 0.. {
        let mut torrents = torrent_search_page(&client, s, n)?;
        if torrents.is_empty() {
            break;
        } else {
            all_the_torrents.append(&mut torrents);
        }
    }

    Ok(all_the_torrents)
}

// Get torrents from the n-th result page of the query by name.
fn torrent_search_page(c: &reqwest::Client, s: &str, n: i32) -> Result<Vec<Torrent>, Error> {
    let page = fetch_page(&c, s, n);
    match page {
        Ok(mut req) => {
            let dom = html5ever::driver::parse_document(
                html5ever::rcdom::RcDom::default(),
                html5ever::driver::ParseOpts::default(),
            )
            .from_utf8()
            .read_from(&mut req)
            .unwrap();

            match identify_page(&dom.document) {
                Page::Torrents => {
                    let torrents = parse_page(&dom.document);
                    match torrents {
                        Some(ts) => Ok(ts),
                        None => Err(Error::Scraping),
                    }
                }
                Page::NoMoreTorrents => Ok(vec![]),
                Page::NoTorrents => Ok(vec![]),
            }
        }
        Err(_) => Err(Error::Request),
    }
}

// Check if the page contains any torrent or not.
fn identify_page(handle: &html5ever::rcdom::Handle) -> Page {
    // Get the eventual table body.
    match get_table_body(handle) {
        Some(t) => {
            if t.children.borrow().iter().count() > 1 {
                Page::Torrents
            } else {
                Page::NoMoreTorrents
            }
        }
        None => Page::NoTorrents,
    }
}

// Using the pHTTP client c return the p-th page of results obtained by searching for s.
fn fetch_page(c: &reqwest::Client, s: &str, p: i32) -> reqwest::Result<reqwest::Response> {
    let params = [("f", "0"), ("c", "0_0"), ("q", s), ("p", &p.to_string())];
    c.get("https://nyaa.si").query(&params).send()
}

// Parse an entire html page for torrents.
fn parse_page(handle: &html5ever::rcdom::Handle) -> Option<Vec<Torrent>> {
    let mut torrents = vec![];

    let table_body = get_table_body(handle)?;
    for row in table_body
        .children
        .borrow()
        .iter()
        .filter(|r| is_element(r))
    {
        let category = get_category(row)?;
        let name = get_name(row)?;
        let links = get_links(row)?;
        let size = get_size(row)?;
        let date = get_date(row)?;
        let seeders = get_seeders(row)?;
        let leechers = get_leechers(row)?;
        let completed_downloads = get_completed_downloads(row)?;

        torrents.push(Torrent {
            category,
            name,
            links,
            size,
            date,
            seeders,
            leechers,
            completed_downloads,
        });
    }

    Some(torrents)
}

// Dive deep in the webpage and extract the table body (i.e. <tbody>) of the main table.
fn get_table_body(handle: &html5ever::rcdom::Handle) -> Option<Rc<html5ever::rcdom::Node>> {
    let document_children = &handle.children.borrow();
    let html = document_children.get(1)?;
    let html_children = &html.children.borrow();
    let body = html_children.get(2)?;
    let body_children = &body.children.borrow();
    let container = body_children.get(5)?;
    let container_children = &container.children.borrow();
    let table_div = container_children.get(3)?;
    let table_div_children = &table_div.children.borrow();
    let table = table_div_children.get(1)?;
    let table_children = &table.children.borrow();
    let table_body = table_children.get(3)?;

    Some(table_body.clone())
}

// Identifies an Element node.
fn is_element(n: &html5ever::rcdom::Node) -> bool {
    match n.data {
        html5ever::rcdom::NodeData::Element { .. } => true,
        _ => false,
    }
}

// Extract torrent name from a table row.
fn get_name(row: &html5ever::rcdom::Node) -> Option<String> {
    let mut result = None;

    // Get to the cell that contains the title.
    let row_children = &row.children.borrow();
    let name_cell = row_children.get(3)?;
    let name_cell_children = &name_cell.children.borrow();

    // Get the title nodes (we should always have only one).
    let title_nodes = name_cell_children
        .iter()
        .filter(|link| is_title(&link))
        .collect::<Vec<_>>();
    let title_node = &title_nodes.get(0)?;

    // Extract the name from the title node.
    let title_node_children = &title_node.children.borrow();
    let title = title_node_children.get(0)?;
    if let html5ever::rcdom::NodeData::Text { contents } = &title.data {
        result = Some(contents.borrow().to_string());
    }

    result
}

// Identifies a link node (i.e. <a>) that contains the title of a torrent.
fn is_title(node: &html5ever::rcdom::Node) -> bool {
    match &node.data {
        html5ever::rcdom::NodeData::Element { attrs, .. } => {
            attrs.borrow().iter().all(|att| !is_class(att))
        }
        _ => false,
    }
}

// Identifies the attribute "class".
fn is_class(att: &html5ever::tree_builder::Attribute) -> bool {
    &att.name.local == "class"
}

// Extract torrent category from a table row.
fn get_category(row: &html5ever::rcdom::Node) -> Option<Category> {
    // Get to the node that contains the category.
    let row_children = &row.children.borrow();
    let category_cell = row_children.get(1)?;
    let category_cell_children = &category_cell.children.borrow();
    let category_node = category_cell_children.get(1)?;

    // Extract the category from the node as a String.
    let mut category = String::new();
    if let html5ever::rcdom::NodeData::Element { attrs, .. } = &category_node.data {
        attrs
            .borrow()
            .iter()
            .filter(|att| &att.name.local == "title")
            .for_each(|att| category = att.value.to_string());
    }

    match category.as_ref() {
        "Anime - Anime Music Video" => Some(Category::Anime(Anime::AnimeMusicVideo)),
        "Anime - English-translated" => Some(Category::Anime(Anime::EnglishTranslated)),
        "Anime - Non-English-translated" => Some(Category::Anime(Anime::NonEnglishTranslated)),
        "Anime - Raw" => Some(Category::Anime(Anime::Raw)),
        "Audio - Lossless" => Some(Category::Audio(Audio::Lossless)),
        "Audio - Lossy" => Some(Category::Audio(Audio::Lossy)),
        "Literature - English-translated" => {
            Some(Category::Literature(Literature::EnglishTranslated))
        }
        "Literature - Non-English-translated" => {
            Some(Category::Literature(Literature::NonEnglishTranslated))
        }
        "Literature - Raw" => Some(Category::Literature(Literature::Raw)),
        "Live Action - English-translated" => {
            Some(Category::LiveAction(LiveAction::EnglishTranslated))
        }
        "Live Action - Idol/Promotional Video" => {
            Some(Category::LiveAction(LiveAction::IdolPromotionalVideo))
        }
        "Live Action - Non-English-translated" => {
            Some(Category::LiveAction(LiveAction::NonEnglishTranslated))
        }
        "Live Action - Raw" => Some(Category::LiveAction(LiveAction::Raw)),
        "Pictures - Graphics" => Some(Category::Pictures(Pictures::Graphics)),
        "Pictures - Photos" => Some(Category::Pictures(Pictures::Photos)),
        "Software - Applications" => Some(Category::Software(Software::Applications)),
        "Software - Games" => Some(Category::Software(Software::Games)),
        _ => None,
    }
}

// Extract torrent links: .torrent file and magnet link.
fn get_links(row: &html5ever::rcdom::Node) -> Option<(Option<String>, Option<String>)> {
    let mut torrent_file = None;
    let mut magnet_link = None;

    // Get to the node that contains the links.
    let row_children = &row.children.borrow();
    let link_cell = row_children.get(5)?;
    let link_cell_children = &link_cell.children.borrow();
    let torrent_file_node = link_cell_children.get(1);
    let magnet_link_node = link_cell_children.get(3);

    // Extract the torrent file link and the magnet link from the nodes (if they
    // exist) as Strings.
    match &torrent_file_node {
        Some(node) => {
            if let html5ever::rcdom::NodeData::Element { attrs, .. } = &node.data {
                attrs
                    .borrow()
                    .iter()
                    .filter(|att| &att.name.local == "href")
                    .for_each(|att| torrent_file = Some(att.value.to_string()));
            }
        }
        None => (),
    }

    match &magnet_link_node {
        Some(node) => {
            if let html5ever::rcdom::NodeData::Element { attrs, .. } = &node.data {
                attrs
                    .borrow()
                    .iter()
                    .filter(|att| &att.name.local == "href")
                    .for_each(|att| magnet_link = Some(att.value.to_string()));
            }
        }
        None => (),
    }

    Some((torrent_file, magnet_link))
}

// Extract torrent's size.
fn get_size(row: &html5ever::rcdom::Node) -> Option<f64> {
    // Extract size as a String and split it.
    let size_raw = get_text(row, 7)?;
    let size_raw_split = size_raw.split(' ').collect::<Vec<_>>();

    // Extract the coefficient and unit of measure.
    let coefficient = size_raw_split.get(0)?.parse::<f64>().unwrap();

    let unit_raw = size_raw_split.get(1)?;
    let unit: i64 = match *unit_raw {
        "Bytes" => 1,
        "KiB" => 1_024,
        "MiB" => 1_048_576,
        "GiB" => 1_073_741_824,
        "TiB" => 1_099_511_627_776,
        _ => 0,
    };

    // Return torrent's size in bytes.
    if unit == 0 {
        None
    } else {
        Some(coefficient * unit as f64)
    }
}

// Extract torrent's date.
fn get_date(row: &html5ever::rcdom::Node) -> Option<String> {
    get_text(row, 9)
}

// Extract torrent's seeders.
fn get_seeders(row: &html5ever::rcdom::Node) -> Option<i32> {
    let seeders_raw = get_text(row, 11)?;
    let seeders = seeders_raw.parse::<i32>();

    match seeders {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

// Extract torrent's leechers.
fn get_leechers(row: &html5ever::rcdom::Node) -> Option<i32> {
    let leechers_raw = get_text(row, 13)?;
    let leechers = leechers_raw.parse::<i32>();

    match leechers {
        Ok(l) => Some(l),
        Err(_) => None,
    }
}

// Extract torrent's completed downloads.
fn get_completed_downloads(row: &html5ever::rcdom::Node) -> Option<i32> {
    let completed_downloads_raw = get_text(row, 15)?;
    let completed_downloads = completed_downloads_raw.parse::<i32>();

    match completed_downloads {
        Ok(c) => Some(c),
        Err(_) => None,
    }
}

// Get the text in the nth cell of a row. Can be used "only" to extract size,
// date, seeders, leechers and completed downloads.
fn get_text(row: &html5ever::rcdom::Node, n: usize) -> Option<String> {
    let mut result = None;

    // Get to the node that contains the wanted text.
    let row_children = &row.children.borrow();
    let node = row_children.get(n)?;
    let node_children = &node.children.borrow();
    let text = node_children.get(0)?;

    // Extract the wanted text from the node as a String.
    if let html5ever::rcdom::NodeData::Text { contents } = &text.data {
        result = Some(contents.borrow().to_string());
    }

    result
}
