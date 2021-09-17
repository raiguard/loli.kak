# kak-ll

In kakoune, there is no standard for location lists. Many scripts / plugins (including the built-in `grep.kak`) implement a goto kind of functionality, but it is not standardized whatsoever and is easily prone to breakage. `kak-ll` aims to standardize these lists and provide a common interface for navigating and manipulating location lists.

## What is a location list?

A location list is a list of positions in files. These positions can be jumped to by pressing `enter` on the corresponding line in the location list buffer, or iterated with the various available commands.

Similar to vim, there will be a global location list and client location lists. The global location list spans the entire project, whereas a client list is specific to a client. They are iterated via commands with the `lg` and `lc` prefixes, respectively.

## Internal mechanisms

Each location list is represented by a series of `range-specs` options - one per file in the list. Each `range-specs` has the ranges as usual, and the "arbitrary text" is used for the preview. The first entry in the `range-specs` is a dummy entry used to hold the filename for display.

There is also another `range-specs` that is used for highlighting the contents in a buffer. This one omits the dummy entry and includes the corresponding faces.

Each location list also has an `index` option that corresponds to the current index the user has selected in the list.

### List options

If a list is created in `client0` while in the file `src/main.rs`, the following options are created:

```
ll_client0_index: 0
ll_client0_list_srcmainrs: 0.0+0|src/main.rs 1.1,1.7|kakoune 2.6,2.12|lorem kakoune ipsum
ll_client0_highlights_srcmainrs: 1.1,1.7|LLHighlight 2.6,2.12|LLHighlight
```

If those locations are added to the global list, it will look like this:

```
ll_index: 0
ll_list_srcmainrs: 0.0+0|src/main.rs 1.1,1.7|kakoune 2.6,2.12|lorem kakoune ipsum
ll_highlights_srcmainrs: 1.1,1.7|LLHighlight 2.6,2.12|LLHighlight
```

### Changing the contents of a list

The `l(c|g)add` commands will allow you to add to the client list or global list respectively. These commands take two arguments:
- filename (str) - The filename that the locations correspond with.
- locations (range-specs) - The locations to add, with the preview text as the arbitrary string.

Because of the `filename` requirement, that means you can only add locations for one file at a time. This is a limitation of the system.

To remove items from the list, call `l(c|g)remove [filename] [index]`. if `filename` is omitted, all entries are removed. If `index` is omitted, all entries from that file are removed.

### Creating a list

The `l(c|g)new` command will create a new location list, overriding the old list. The old list will go into the history (TODO). The command takes a single argument, which is a kakoune option name.

The option passed is a `LocationList`, which is implemented as a `str-list` in kak. The contents of the list are similar to `range-specs`, but have an extra field at the beginning for the filename:

```
"src/main.rs|1.1,1.3|foo src/main.rs|5.3,5.6|bar"
```

`kak-ll` will convert this into multiple proper `range-specs` options to implement the list behavior.

## The buffer

You can open a location list in a special buffer. You may search and filter this buffer however you wish, but it is read-only in order to preserve line numbers (they correspond with the indices in the list). Pressing enter on a line will jump you to that location in the corresponding client.

For our above example, the buffer will look like this:

```
> src/main.rs:1:1 | kakoune
  src/main.rs:2:6 | lorem kakoune ipsum
```

By default, the preview includes the entire line. There might be options to change this in the future. The `>` in the buffer denotes the currently selected entry (subject to change).

This buffer updates whenever the list does, so you always have the latest information.

For now, the buffer will simply open in the current client. An interface for windowing will have to be set up in the future, since kak does not do windowing whatsoever.

## The commands

There are many commands available for using location lists:

- `(lg|lc)n[ext]`: Go to the next entry on the list.
- `(lg|lc)p[rev]`: Go to the previous entry on the list.
- `(lg|lc)f[irst]`: Go to the first entry on the list.
- `(lg|lc)l[ast]`: Go to the last entry on the list.
- `(lg|lc)o[pen]`: Open the location list.
- `(lg|lc)c[lose]`: Close the location list (TODO: determine how this will work with the splits).

After skimming vim docs, here are all of the things I wish to add:

above
below
before
after
file
first
last
add (add client list to global list)
do

- remember last ten lists for each client and global

## User mode

`location-list.kak` offers a `location-list` user mode for navigating the lists:

- `n`: `lgnext`
- `p`: `lgprev`
- `j`: `lgbelow`
- `k`: `lgabove`
- `h`: `lgfirst`
- `l`: `lglast`
- `o`: `lgopen`
- `c`: `lgclose`

Using capitol letters instead of lowercase will do all the same commands, but with the client list instead of the global list.

An alternate user mode is available (`location-list-alt`) that has the reverse - client list by default, global list by capitol.
