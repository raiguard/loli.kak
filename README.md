# kak-loli

An implementation of proper [location lists](http://vimdoc.sourceforge.net/htmldoc/quickfix.html) for [kakoune](https://kakoune.org). kak does have a naive `goto` functionality for its built-in `grep`, `make`, and `spell` utilities, but these duplicate a lot of code, do not work well when lines are changed, and are generally a hacked implementation. This plugin aims to provide a standard interface for kakoune tools to use, for generic global and client-specific location lists.

## Project status

This project is under heavy development, and is not ready for use quite yet. All it does at the moment is highlight the ranges specified in each list.

## Installation

### Build from source

Requires [Rust](https://www.rust-lang.org/).

```
git clone https://github.com/raiguard/kak-loli
cd kak-loli
cargo install --locked --force --path . --debug
```

This will compile `kak-loli` and put the `kak-loli` executable on your `path`. If you wish to compile in release move, remove the `--debug` flag.

Next, put this at the top of your `kakrc`:

```kakounescript
evaluate-commands %sh{
  kak-loli -s $kak_session init
}
```

This will add the appropriate hooks and commands to your kakoune session.

## Usage

Right now, the `lggrep` and `lcgrep` commands are available. Running one of them and providing a search query (regex) will highlight all matches.

`lggrep` adds to the "global list" which means matches will highlight in all clients. `lcgrep` will add to the "client list" and matches will only be highlighted in the client it was called from.
