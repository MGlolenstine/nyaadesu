# Nyaadesu
Non-official rust library to search [Nyaa.si](https://nyaa.si) for torrents.

## Rationale
As far as I know [Nyaa.si](https://nyaa.si) does not provide any APIs so I thought it would be cool to have a way to do so in Rust and that's why this library exists.

## Building and Using it
I'm planning to upload `nyaadesu` to [crates.io](https://crates.io) in the future but for now, if you want to play with it, I suggest you the good old approach:

```
git clone https://github.com/grastello/nyaadesu.git
cd nyaadesu
cargo build
cargo test
```

You can then look at the example in the `examples` directory or run it directly with `./target/debug/examples/example` to get a feel of how the library works. Optionally you may want to browse the documentation; to do that a `cargo doc --open` will suffice.

## Todo
List of project's goals:

- [X] Search [Nyaa.si](https://nyaa.si) for torrents
- [X] Cringe Japanese-ish name.
- [X] Better type for torrents' sizes.
- [ ] Better type for torrents' dates.
- [ ] Way to extract magnet and .torrent link.

List of features that seems cool to have:
- [ ] Get more information about a torrent (e.g. by parsing a page like [this](https://nyaa.si/view/644786))
- [ ] Order results by number of seeders, leechers, completed downloads, size or date.
- [ ] Filter by category.
