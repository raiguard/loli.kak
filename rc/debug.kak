# DEBUGGING

hook global WinCreate .* %{
    hook -once global WinDisplay .* %{
        # These use faces so we can see them for now
        set-option global loli_global_list debug.kak|1.1,1.1|Error loli.kak|3.7,3.7|Error
        add-highlighter window/ ranges loli_global_ranges
    }
}

define-command loli-debug %{
    info -title "loli debug" \
"timestamp: %val{timestamp}
loli_global_list: %opt{loli_global_list}
loli_global_ranges: %opt{loli_global_ranges}"
}

map global user l ": loli-debug<ret>" -docstring "loli debug"

hook global NormalIdle .* %{
    loli-debug
}
hook global User LoliBufChange %{
    loli-debug
}
