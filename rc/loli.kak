# Let us simply focus on the global (quickfix) list for now
# Then we can focus on the client lists

# FACES

# Visualize the locations in the current buffer
set-face global LoliLocation ""

# OPTIONS

# str-list to hold the master copy of the list
declare-option -hidden str-list loli_global_list

# range-specs for updating the ranges
# We can use this for every client by setting it at the window level
declare-option -hidden range-specs loli_global_ranges

# Timestamp to signal when the list positions need to be updated
declare-option -hidden int loli_prev_timestamp 0

# COMMANDS

# To get the range parts:
# regex="(.*)\|([0-9]*?)\.([0-9]*?),([0-9]*?)\.([0-9]*?)\|(.*)"

# Create a range-specs for the current buffer
define-command -hidden loli_update_ranges %{
    # Clear the current ranges
    set-option window loli_global_ranges %val{timestamp}
    evaluate-commands %sh{
        regex="(.*)\|([0-9]*?\.[0-9]*?,[0-9]*?\.[0-9]*?)\|(.*)"
        # Loop over the list
        eval set -- "$kak_quoted_opt_loli_global_list"
        while [ $# -gt 0 ]; do
            if [[ "$1" =~ $regex ]]; then
                # Check that this item is in the current buffer
                bufname=${BASH_REMATCH[1]}
                if [ "$bufname" == "$kak_bufname" ]; then
                    # Add the range to be displayed and/or updated
                    range=${BASH_REMATCH[2]}
                    preview=${BASH_REMATCH[3]}
                    echo "set-option -add window loli_global_ranges ${range}|LoliLocation"
                fi
            fi
            shift
        done
    }
}

define-command loli_update_all_ranges %{
    evaluate-commands %sh{
        eval set -- $kak_quoted_client_list
        while [ $# -gt 0 ]; do
            echo "evaluate-commands -client $1 loli_update_ranges"
            shift
        done
    }
}

# HOOKS

# LoliBufChange
# This hook fires whenever a window's timestamp changes, indicating that the buffer contents were modified
hook global NormalIdle .* %{
    evaluate-commands %sh{
        if [ "$kak_timestamp" -gt "$kak_opt_loli_prev_timestamp" ]; then
            printf 'trigger-user-hook LoliBufChange\n'
            printf 'set-option buffer loli_prev_timestamp %s\n'  "$kak_timestamp"
        fi
    }
}

# Add highlighter to update ranges if they exist
hook global WinCreate .* %{
    hook -once -always global WinDisplay .* %{
        add-highlighter window/ ranges loli_global_ranges
    }
}

# Update ranges that might have changed when the window was dormant
hook global WinDisplay .* %{
    loli_update_ranges
}

# Update the master list based on updates to the range-specs
hook global User LoliBufChange %{
    evaluate-commands %sh{
        # Save the current list to an array
        declare -a global_list
        eval set -- $kak_quoted_opt_loli_global_list
        while [ $# -gt 0 ]; do
            global_list+=($1)
            shift
        done

        # Reset the global list
        echo "set-option global loli_global_list"

        # Set the current ranges to the environment
        eval set -- $kak_quoted_opt_loli_global_ranges
        # And skip the timestamp
        shift

        list_regex="(.*)\|([0-9]*?\.[0-9]*?,[0-9]*?\.[0-9]*?)\|(.*)"
        range_regex="([0-9]*?\.[0-9]*?,[0-9]*?\.[0-9]*?)\|(.*)"

        # Creating a new list is more efficient than modifying in-place
        # Modifying in-place would require creating sub-arrays with matching...
        declare -a new_global_list

        for i in "${!global_list[@]}"; do
            location=${global_list[$i]}
            # Extract data from the location
            if [[ "$location" =~ $list_regex ]]; then
                # Check that this location is in the current buffer
                bufname=${BASH_REMATCH[1]}
                preview=${BASH_REMATCH[3]}
                if [ "$bufname" == "$kak_bufname" ]; then
                    # Extract data from the current range
                    if [[ "$1" =~ $range_regex ]]; then
                        # Add the updated range
                        range=${BASH_REMATCH[1]}
                        echo "set-option -add global loli_global_list '${bufname}|${range}|${preview}'"
                        # Move to the next range
                        shift
                    fi
                else
                    # Re-add the same location
                    echo "set-option -add global loli_global_list '$location'"
                fi
            fi
        done
    }
}
