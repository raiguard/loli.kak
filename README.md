# loli.kak

An implementation of proper [location lists](http://vimdoc.sourceforge.net/htmldoc/quickfix.html) for [Kakoune](https://kakoune.org).

Kakoune does have a basic goto implementation for its built-in `grep`, `make`, and `spell` tools, but these are very naive and are easily prone to breakage. Additionally, a plugin or script that wishes to implement this kind of functionality has to do it all again from scratch, resulting in a fractured ecosystem and lots of duplicate code.

`loli.kak` aims to provide a standard interface for scripts to populate a location list, then provide a single set of commands for iterating, viewing, and navigating these lists.

## Project status

This project is under heavy development, and is not ready for use quite yet.

## Installation

Source `rc/loli.kak` manually. If you use a plugin manager at the moment, it will load debug code that you probably do not want.

## Usage

Updating the `loli_global_list` option with strings in the format of `{filename}|{range}|{preview}` will create a location list. This list will update as changes are made to each buffer.

Use the `LoliLocation` face to view the locations. The face is invisible by default, so you must overwrite it for the locations to be visible:

```kak
set-face global LoliLocation +r
```
