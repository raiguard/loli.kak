decl str loli_cmd
# FIXME: This is temporary
face global loli_highlight "default+r"

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
