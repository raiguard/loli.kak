decl str loli_cmd

face global loli_highlight "default+r"

# For performance reasons, kakoune does not provide all variables to all shell expansions
# Therefore, we must write the actual commands to $kak_command_fifo so kakoune can see the
# variable names directly, instead of using indirection

# def -hidden -params 1 lgnew %{
#     nop %sh{
#         echo "eval %sh{ \$kak_opt_loli_cmd -t \$kak_timestamp new -t \$kak_timestamp \"\$kak_quoted_opt_$1\" }" > $kak_command_fifo
#     }
# }

# def -hidden -params 1 lcnew %{
#     nop %sh{
#         echo "eval %sh{ \$kak_opt_loli_cmd -c $kak_client new -t \$kak_timestamp \"\$kak_quoted_opt_$1\" }" > $kak_command_fifo
#     }
# }

# hook global BufCreate .* %{
#     eval %sh{
#         $kak_opt_loli_cmd highlight $kak_bufname
#     }
# }

def gclear %{
    eval %sh{
        $kak_opt_loli_cmd clear $kak_bufname
    }
}

def cclear %{
    eval %sh{
        $kak_opt_loli_cmd -c $kak_client clear $kak_bufname
    }
}

hook global WinDisplay .* %{
    eval %sh{
        $kak_opt_loli_cmd highlight $kak_bufname $kak_client
    }
}

# Delete store file when the session ends
hook global KakEnd .* %{
    eval %sh{ $kak_opt_loli_cmd clean }
}

# Update store when the kak timestamp changes
declare-option -hidden int prev_timestamp 0

hook -group buf-change global NormalIdle .* %{
    evaluate-commands %sh{
        if [ "$kak_timestamp" -gt "$kak_opt_prev_timestamp" ]; then
            printf 'trigger-user-hook BufChange\n'
            printf 'set-option buffer prev_timestamp %s\n' "$kak_timestamp"
        fi
    }
}

# hook global User BufChange %{
#     echo -debug "%val{timestamp} UPDATE RANGES"
# }
