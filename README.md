# location-list.kak

In kakoune, there is no standard for location lists. Many scripts / plugins (including the built-in `grep.kak`) implement a goto kind of functionality, but it is not standardized whatsoever and is easily prone to breakage. `location-list.kak` aims to standardize these lists and provide a common interface for navigating and manipulating location lists.

## What is a location list?

A location list is a list of positions in files. These positions can be jumped to by pressing `enter` on the corresponding line in the location list buffer, or iterated with the various available commands.

Similar to vim, there will be a global location list and window location lists. I am also considering custom location lists, though the utility of these remain to be seen.

## How will they work?

Each location list is kept in a `range-specs` option and is formatted as `linestart.columnstart,lineend.columnend|FILENAME⤬⤬⤬preview` (e.g. `1.1,1.7|src/main.rs⤬⤬⤬kakoune`). Some tools might only output lines, in which case the ranges are defined as starting at column zero with a length equal to the line length. Kakoune automatically adjusts these ranges based on edits made, which will work to ensure that locations are not invalidated when file contents change.
