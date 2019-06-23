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
    pub links: (Option<String>, Option<String>),
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
