# kak-loli

An implementation of proper [location lists](http://vimdoc.sourceforge.net/htmldoc/quickfix.html) for [kakoune](https://kakoune.org).

kakoune does have a basic goto implementation for its built-in `grep`, `make`, and `spell` tools, but these are very naive and are easily prone to breakage. Additionally, a plugin or script that wishes to implement this kind of functionality has to do it all again from scratch, resulting in a fractured ecosystem and lots of duplicate code.

`kak-loli` aims to provide a standard interface for scripts to populate a location list, then provide a single set of commands for iterating, viewing, and navigating these lists.

## Project status

This project is under heavy development, and is not ready for use quite yet. All it does at the moment is highlight the ranges specified in each list.

## Installation

Requires [Rust](https://www.rust-lang.org/).

### Precompiled binaries

Coming soon to an AUR near you!

### [plug.kak](https://github.com/andreyorst/plug.kak)

```kakounescript
plug "raiguard/kak-loli" do %{ cargo install --locked --force --path . }
```

### Build from source

```
git clone https://github.com/raiguard/kak-loli
cd kak-loli
cargo install --locked --force --path .
```

This will compile and place the `kak-loli` executable on your path.

Next, put this at the top of your `kakrc`:

```kakounescript
evaluate-commands %sh{
  kak-loli -s $kak_session init
}
```

This will add the appropriate hooks and commands to your kakoune session.

## Usage

All command starting with `g` will use the global list, and `c` will use the client-specific list

- `(g|c)grep` - Run `ripgrep` on your project or the specified file(s), and pipe the results into a location list.
- `(g|c)new` - Create a location list from a `str-list` option.
- `(g|c)clear` - Clear the location list, removing the highlighters.
- `(g|c)first` - Jump to the first location in the list.
- `(g|c)last` - Jump to the last location in the list.
- `(g|c)next` - Jump to the next location in the list.
- `(g|c)prev` - Jump to the previous location in the list.
- `(g|c)open` - Open all of the locations in the toolsclient (currently has no other functions)
