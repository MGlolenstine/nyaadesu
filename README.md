# Nyaadesu [![Build Status](https://travis-ci.org/gRastello/nyaadesu.svg?branch=master)](https://travis-ci.org/gRastello/nyaadesu)
Non-official rust library to search [Nyaa.si](https://nyaa.si) for torrents.

## Rationale
As far as I know [Nyaa.si](https://nyaa.si) does not provide any APIs so I thought it would be cool to have a way to do so in Rust and that's why this library exists.

## Documentation, examples
`nyaadesu` is now hosted on [creates.io](https://crates.io) so you can use it your projects by including it as a dependency.

Documentation is available at [docs.rs](https://docs.rs/nyaadesu/0.1.0/nyaadesu/) and you can look at some simple examples under the `examples` directory.

## Build it yourself
If for some reason you want to play with the git version good old approach will work:

```
git clone https://github.com/grastello/nyaadesu.git
cd nyaadesu
cargo build
cargo test
```

## Todo
List of project's goals:

- [X] Search [Nyaa.si](https://nyaa.si) for torrents.
- [X] Cringe Japanese-ish name.
- [X] Better type for torrents' sizes.
- [X] Better type for torrents' dates.
- [X] Way to extract magnet and .torrent link.

List of features that seems cool to have:
- [ ] Get more information about a torrent (e.g. by parsing a page like [this](https://nyaa.si/view/644786))
- [ ] Order results by number of seeders, leechers, completed downloads, size or date.
- [ ] Filter by category.
