decl str loli_cmd
# FIXME: This is temporary
face global loli_highlight "default+r"

# HOOKS

# # Highlight newly opened buffers
# hook global WinDisplay .* %{
#     evaluate-commands %sh{
#         $kak_opt_loli_cmd highlight $kak_bufname $kak_client
#     }
# }

# Delete store file when the session ends
hook global KakEnd .* %{
    evaluate-commands %sh{ $kak_opt_loli_cmd clean }
}

# GREP

define-command -params .. -file-completion ggrep %{
    evaluate-commands %sh{
        if [ $# -eq 0 ]; then
            set -- "${kak_selection}"
        fi

        output=$(mktemp -d "${TMPDIR:-/tmp}"/kak-grep.XXXXXXXX)/fifo
        rg --vimgrep --trim  "$@" | tr -d '\r' > ${output} 2>&1

        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo grep "${output}"
    }
}

define-command -params .. -file-completion cgrep %{
    evaluate-commands %sh{
        if [ $# -eq 0 ]; then
            set -- "${kak_selection}"
        fi

        output=$(mktemp -d "${TMPDIR:-/tmp}"/kak-grep.XXXXXXXX)/fifo
        rg --vimgrep --trim  "$@" | tr -d '\r' > ${output} 2>&1

        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo -c $kak_client grep "${output}"
    }
}

# TEST

define-command gtest %{
    evaluate-commands %sh{
        $kak_opt_loli_cmd -i $kak_command_fifo -o $kak_response_fifo test
    }
}
