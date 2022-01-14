# loli.kak

An implementation of proper [location lists](http://vimdoc.sourceforge.net/htmldoc/quickfix.html) for [Kakoune](https://kakoune.org).

Kakoune does have a basic goto implementation for its built-in `grep`, `make`, and `spell` tools, but these are very naive and are easily prone to breakage. Additionally, a plugin or script that wishes to implement this kind of functionality has to do it all again from scratch, resulting in a fractured ecosystem and lots of duplicate code.

`loli.kak` aims to provide a standard interface for scripts to populate a location list, then provide a single set of commands for iterating, viewing, and navigating these lists.

## Project status

This project is under heavy development, and is not ready for use quite yet.

## Installation

Source `rc/loli.kak` manually. If you use a plugin manager at the moment, it will load debug code that you probably do not want.

## Usage

The plugin doesn't do much right now. You can manually update the `loli_global_list` option and then call `loli_update_all_ranges` show them in your buffers. See `rc/debug.kak` for the formatting, etc.
