declare-option str loli_cmd
# FIXME: This is temporary
set-face global loli_highlight "default+r"

# HOOKS

# Highlight newly opened buffers
hook global WinDisplay .* %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp -c $kak_client highlight $kak_bufname
    }
}

# Delete store file when the session ends
hook global KakEnd .* %{
    evaluate-commands %sh{ $kak_opt_loli_cmd clean }
}

# Update ranges any time a buffer changes
declare-option -hidden int loli_prev_timestamp 0
hook -group loli-buf-change global NormalIdle .* %{
    evaluate-commands %sh{
        if [ "$kak_timestamp" -gt "$kak_opt_loli_prev_timestamp" ]; then
            printf 'trigger-user-hook LoliBufChange\n'
            printf 'set-option buffer loli_prev_timestamp %s\n' "$kak_timestamp"
        fi
    }
}
hook global User LoliBufChange %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp update "$kak_bufname"
    }
}

# CLEAR

define-command gclear %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp clear
    }
}

define-command cclear %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp -c $kak_client clear
    }
}

# GREP

define-command -params .. -file-completion ggrep %{
    evaluate-commands %sh{
        if [ $# -eq 0 ]; then
            set -- "${kak_selection}"
        fi

        output=$(mktemp -d "${TMPDIR:-/tmp}"/kak-grep.XXXXXXXX)/fifo
        rg --vimgrep --trim  "$@" | tr -d '\r' > ${output} 2>&1

        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp grep "${output}"
    }
}

define-command -params .. -file-completion cgrep %{
    evaluate-commands %sh{
        if [ $# -eq 0 ]; then
            set -- "${kak_selection}"
        fi

        output=$(mktemp -d "${TMPDIR:-/tmp}"/kak-grep.XXXXXXXX)/fifo
        rg --vimgrep --trim  "$@" | tr -d '\r' > ${output} 2>&1

        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp -c $kak_client grep "${output}"
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp -c $kak_client highlight "$kak_bufname"
    }
}

# STR-LIST

define-command -params 1 gnew %{
    evaluate-commands %sh{
        echo "evaluate-commands %sh{
            \$kak_opt_loli_cmd -i \$kak_command_fifo -o \$kak_response_fifo -t \$kak_timestamp list \"\$kak_quoted_opt_$1\"
        }" >> $kak_command_fifo
    }
}

define-command -params 1 cnew %{
    evaluate-commands %sh{
        echo "evaluate-commands %sh{
            $kak_opt_loli_cmd -i \$kak_command_fifo -o \$kak_response_fifo -t \$kak_timestamp -c $kak_client list \"\$kak_quoted_opt_$1\"
            $kak_opt_loli_cmd -i \$kak_command_fifo -o \$kak_response_fifo -t \$kak_timestamp -c $kak_client highlight \"\$kak_bufname\"
        }" >> $kak_command_fifo
    }
}

# NAVIGATION

define-command gfirst %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp first
    }
}

define-command cfirst %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp -c $kak_client first
    }
}

define-command glast %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp last
    }
}

define-command clast %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp -c $kak_client last
    }
}

define-command gnext %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp next
    }
}

define-command cnext %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp -c $kak_client next
    }
}

define-command gprev %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp prev
    }
}

define-command cprev %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp -c $kak_client prev
    }
}

define-command gopen %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp open
    }
}

define-command copen %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp -c $kak_client open
    }
}

# FILETYPE

hook global WinSetOption filetype=loli %{
    add-highlighter window/loli group
    add-highlighter window/loli/ regex "^(.) ((?:\w:)?[^:\n]+):(\d+):(\d+)? (\|)" 1:green 2:cyan 3:green 4:green 5:comment
    # add-highlighter window/loli/ line %{%opt{loli_current_line}} default+b
    hook -once -always window WinSetOption filetype=.* %{ remove-highlighter window/loli }
    hook buffer -group loli-hooks NormalKey <ret> loli-jump
}

define-command -hidden loli-jump %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -t $kak_timestamp buf-jump $kak_bufname $kak_selection_desc
    }
}
