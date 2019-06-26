//! Non official rust library to search [Nyaa.si] for torrents.
//!
//! [Nyaa.si] does not provie any official API to interact with the website so
//! every information that this library can provide is obtained by scraping
//! HTML. This means that if something changes in [Nyaa.si] there is a
//! possibility for `nyaadesu` to stop functioning so be sure to always check
//! for an [`Error`] while using the [`search_torrent`] function (and maybe
//! report it on [GitHub]).
//!
//! See [examples] for a basic understanding of how to use this library.
//!
//! [Nyaa.si]: https://nyaa.si
//! [examples]: https://github.com/gRastello/nyaadesu/tree/master/examples
//! [`Error`]: ./enum.Error.html
//! [`search_torrent`]: ./fn.torrent_search.html
//! [GitHub]: https://github.com/grastello/nyaadesu

mod types;
mod scraping;

pub use self::types::*;
pub use self::scraping::*;
