# Let us simply focus on the global (quickfix) list for now
# Then we can focus on the client lists

# FACES

# Visualize the locations in the current buffer
set-face global LoliLocation ""

set-face global LoliSelectedLine default+b

# HIGHLIGHTERS

hook -group loli-highlight global WinSetOption filetype=loli %{
    add-highlighter window/loli group
    add-highlighter window/loli/ regex "^(.*?)\|(\d+:\d+)\|?" 1:blue 2:comment
    add-highlighter window/loli/ line %{%opt{loli_global_index}} LoliSelectedLine
    hook -once -always window WinSetOption filetype=.* %{ remove-highlighter window/loli }
}

# OPTIONS

# Str-list to hold the master copy of the list
declare-option -hidden str-list loli_global_list

# Range-specs for updating the ranges
# We can use this for every client by setting it at the window level
declare-option -hidden range-specs loli_global_ranges

# The current selected index in the list
declare-option -hidden int loli_global_index 1

# Timestamp to signal when the list positions need to be updated
declare-option -hidden int loli_prev_timestamp 0

# HOOKS

# # LoliBufChange
# # This hook fires whenever a window's timestamp changes, indicating that the buffer contents were modified
# hook global NormalIdle .* %{
#     evaluate-commands %sh{
#         if [ "$kak_timestamp" -gt "$kak_opt_loli_prev_timestamp" ]; then
#             printf 'trigger-user-hook LoliBufChange\n'
#             printf 'set-option buffer loli_prev_timestamp %s\n'  "$kak_timestamp"
#         fi
#     }
# }

# # Add highlighter for the ranges
# hook global WinCreate .* %{
#     hook -once -always global WinDisplay .* %{
#         add-highlighter window/ ranges loli_global_ranges
#     }
# }

# # Update ranges that might have changed when the window was dormant
# hook global WinDisplay .* %{
#     loli-update-ranges
# }

# # Update the master list based on updates to the range-specs
# hook global User LoliBufChange %{
#     evaluate-commands %sh{
#         # Save the current list
#         eval "set -- $kak_quoted_opt_loli_global_list"
#         list_len=$#
#         i=0
#         while [ "$#" -gt 0 ]; do
#             i=$((i+1))
#             eval "list_$i=\"$1\""
#             shift
#         done

#         # Set the current ranges to the environment
#         eval "set -- $kak_quoted_opt_loli_global_ranges"
#         # And skip the timestamp
#         shift

#         # Begin set-option command
#         echo -n "set-option global loli_global_list "

#         for i in $(seq 1 $list_len); do
#             eval "location=\$list_$i"
#             location=$(echo "$location" | sed "s/@/@@/g")
#             location_original=$location

#             # TODO: Figure out how to handle pipes in bufnames
#             bufname=${location%%|*}
#             right=${location#*|}
#             current_range=${right%%|*}
#             preview=${right#*|}

#             new_range=${1%%|*}

#             if [ "$bufname" = "$kak_bufname" ] && [ "$new_range" != "$current_range" ]; then
#                 # Add the updated range
#                 echo -n "%@$bufname|$new_range|$preview@ "
#             else
#                 # Add the original range
#                 echo -n "%@$location_original@ "
#             fi
#             if [ "$bufname" = "$kak_bufname" ]; then
#                 # Move to the next range
#                 shift
#             fi
#         done
#     }
# }

# hook global GlobalSetOption loli_global_list=.* %{
#     loli-update-all-ranges
# }

# # COMMANDS

# # Create a range-specs for the current window
# define-command -hidden loli-update-ranges %{
#     evaluate-commands %sh{
#         # Loop over the list
#         eval set -- "$kak_quoted_opt_loli_global_list"
#         # Begin the command
#         echo -n "set-option window loli_global_ranges $kak_timestamp "
#         while [ $# -gt 0 ]; do
#             # TODO: Check for pipes in bufname and preview
#             bufname=${1%%|*}
#             right=${1#*|}
#             range=${right%|*}

#             # Check that this item is in the current buffer
#             if [ "$bufname" = "$kak_bufname" ]; then
#                 # Add the range to be displayed and/or updated
#                 echo -n "'$range|LoliLocation' "
#             fi
#             shift
#         done
#     }
# }

# # Update all currently displayed windows
# define-command -hidden loli-update-all-ranges %{
#     evaluate-commands %sh{
#         eval set -- $kak_quoted_client_list
#         while [ $# -gt 0 ]; do
#             echo "evaluate-commands -client $1 loli-update-ranges"
#             shift
#         done
#     }
# }

# define-command loli-global-open \
# -docstring "open the global location list buffer" \
# %{
#     evaluate-commands -try-client %opt{toolsclient} -save-regs '"' %sh{
#         eval set -- "$kak_quoted_opt_loli_global_list"

#         content=""
#         while [ $# -gt 0 ]; do
#             # TODO: Account for pipes in bufname and description
#             bufname=${1%%|*}
#             right=${1#*|}
#             range=${right%|*}
#             preview=${right#*|}

#             range_start=${range%,*}
#             range_start_line=${range_start%.*}
#             range_start_col=${range_start#*.}

#             content="$content$bufname|$range_start_line:$range_start_col| $preview
# "
#             shift
#         done

#         output=$(mktemp -d "${TMPDIR:-/tmp}"/kak-loli.XXXXXXXX)/fifo
#         mkfifo ${output}

#         ( printf "%s" "$content" | perl -pe "chomp if eof" | sed "s/@/@@/" | tr -d '\r' > ${output} 2>&1 & ) > /dev/null 2>&1 < /dev/null

#         echo "
#             edit! -fifo ${output} *loli-global*
#             set-option buffer filetype loli
#             set-option buffer readonly true
#             hook -always -once buffer BufCloseFifo .* %{ nop %sh{ rm -r $(dirname ${output}) } }
#             map buffer normal <ret> ': loli-global-jump %val{cursor_line}<ret>'
#         "
#     }
# }

# define-command loli-global-close \
# -docstring "close the global location list buffer" \
# %{
#     try %{
#         evaluate-commands -buffer *loli-global* delete-buffer
#     } catch %{
#         echo -markup "{Error}Global list is not open"
#     }
# }

# define-command loli-global-jump \
# -params 1 \
# -docstring "jump to the given index in the global location list" \
# %{
#     evaluate-commands -try-client %opt{jumpclient} %sh{
#         index=$1
#         eval set -- $kak_quoted_opt_loli_global_list
#         if [ $index -lt 1 ] || [ $index -gt $# ]; then
#             echo "echo -markup '{Error}Invalid index'"
#             return
#         fi

#         shift $((index-1)) || echo "echo -markup '{Error}Invalid index"

#         location=$(echo "$1" | sed "s/@/@@/g")
#         # TODO: What was this doing?
#         if [ -n "$location" ]; then
#             echo "echo '$location'"
#         fi

#         # TODO: Account for pipes in bufname and description
#         bufname=${1%%|*}
#         latter=${1#*|}
#         range=${latter%%|*}
#         preview=${latter#*|}

#         range_start=${range%,*}
#         range_start_line=${range_start%.*}
#         range_start_col=${range_start#*.}

#         echo "
#             set-option global loli_global_index $index
#             edit '$bufname' $range_start_line $range_start_col
#         "
#     }
# }

# define-command loli-global-next \
# -docstring "jump to the next location in the global list" \
# %{
#     loli-global-jump %sh{ expr $kak_opt_loli_global_index + 1 }
# }

# define-command loli-global-prev \
# -docstring "jump to the previous location in the global list" \
# %{
#     loli-global-jump %sh{ expr $kak_opt_loli_global_index - 1 }
# }

# define-command loli-global-first \
# -docstring "jump to the first location in the global list" \
# %{
#     loli-global-jump 1
# }

# define-command loli-global-last \
# -docstring "jump to the last location in the global list" \
# %{
#     loli-global-jump %sh{
#         eval set -- $kak_quoted_opt_loli_global_list
#         echo "$#"
#     }
# }

# # TODO: Perhaps find a way to deduplicate this?

# define-command loli-global-before \
# -docstring "jump to the closest location before the current selection" \
# %{
#     evaluate-commands %sh{
#         eval set -- $kak_quoted_opt_loli_global_list
#         for i in $(seq $# -1 1); do
#             eval "location=\$$i"
#             # TODO: Account for pipes in bufname and description
#             bufname=${location%%|*}

#             if [ "$bufname" = "$kak_bufname" ]; then
#                 latter=${location#*|}
#                 range=${latter%|*}
#                 preview=${latter#*|}

#                 range_start=${range%,*}
#                 range_start_line=${range_start%.*}
#                 range_start_col=${range_start#*.}

#                 if [ $range_start_line -eq $kak_cursor_line -a $range_start_col -lt $kak_cursor_column ] || [ $range_start_line -lt $kak_cursor_line ]; then
#                     echo "loli-global-jump ${i}"
#                     return
#                 fi
#             fi
#         done

#         echo "fail 'No location found'"
#     }
# }

# define-command loli-global-after \
# -docstring "jump to the closest location after the current selection" \
# %{
#     evaluate-commands %sh{
#         eval set -- $kak_quoted_opt_loli_global_list
#         for i in $(seq 1 $#); do
#             eval "location=\$$i"
#             # TODO: Account for pipes in bufname and description
#             bufname=${location%%|*}

#             if [ "$bufname" = "$kak_bufname" ]; then
#                 latter=${location#*|}
#                 range=${latter%|*}
#                 preview=${latter#*|}

#                 range_start=${range%,*}
#                 range_start_line=${range_start%.*}
#                 range_start_col=${range_start#*.}

#                 if [ $range_start_line -eq $kak_cursor_line -a $range_start_col -gt $kak_cursor_column ] || [ $range_start_line -gt $kak_cursor_line ]; then
#                     echo "loli-global-jump ${i}"
#                     return
#                 fi
#             fi
#         done

#         echo "fail 'No location found'"
#     }
# }

# define-command loli-global-vanilla-buffer \
# -docstring "create a location list from the contents of the current grep-like buffer" \
# %{
#     execute-keys <percent>
#     evaluate-commands %sh{
#         IFS='
# ' # split on newline only
#         set -o noglob
#         echo -n "set-option global loli_global_list "
#         for line in $kak_selection; do
#             bufname=${line%%:*}
#             right=${line#*:}
#             range_line=${right%%:*}
#             right=${right#*:}
#             range_col=${right%%:*}
#             right=${right#*:}
#             preview=$(echo "$right" | sed "s/@/@@/g" | xargs)

#             echo -n "%@$bufname|$range_line.$range_col,$range_line.$range_col|$preview@ "
#         done
#     }
# }

# define-command loli-add-aliases \
# -docstring "add useful command aliases for loli" \
# %{
#     alias global gopen loli-global-open
#     alias global gclose loli-global-close
#     alias global gjump loli-global-jump
#     alias global gnext loli-global-next
#     alias global gprev loli-global-prev
#     alias global gfirst loli-global-first
#     alias global glast loli-global-last
#     alias global gbefore loli-global-before
#     alias global gafter loli-global-after
# }
