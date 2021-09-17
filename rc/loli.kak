decl str loli_cmd

face global loli_highlight ",,rgb:c678dd+u"

# For performance reasons, kakoune does not provide all variables to all shell expansions
# Therefore, we must write the actual commands to $kak_command_fifo so kakoune can see the
# variable names directly, instead of using indirection

def -hidden -params 1 lgnew %{
    nop %sh{
        echo "eval %sh{ \$kak_opt_loli_cmd new \"\$kak_quoted_opt_$1\" }" > $kak_command_fifo
    }
}

def -hidden -params 1 lcnew %{
    nop %sh{
        echo "eval %sh{ \$kak_opt_loli_cmd -c $kak_client new \"\$kak_quoted_opt_$1\" }" > $kak_command_fifo
    }
}

# Delete store file when the session ends
hook global KakEnd .* %{
    nop %sh{ $kak_opt_loli_cmd clean }
}
