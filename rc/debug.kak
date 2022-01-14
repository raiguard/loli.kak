# DEBUGGING
set-face global LoliLocation +r

set-option global loli_global_list debug.kak|1.1,1.1|foo loli.kak|2.7,2.7|bar loli.kak|11.10,11.10|bar

define-command loli-debug %{
    info -title "loli debug" \
"timestamp: %val{timestamp}
loli_global_list: %opt{loli_global_list}
loli_global_ranges: %opt{loli_global_ranges}"
}

hook global NormalIdle .* %{
    loli-debug
}
hook global User LoliBufChange %{
    loli-debug
}

hook global GlobalSetOption loli_global_list=.* %{
    echo -debug %opt{loli_global_list}
}
