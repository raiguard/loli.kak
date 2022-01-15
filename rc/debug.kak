# DEBUGGING
set-face global LoliLocation +r

set-option global loli_global_list 'rc/debug.kak|1.1,1.1|# DEBUGGING' 'rc/loli.kak|1.1,1.1|# Let us simply whatever' 'foo|bar the|1.1,1.1|LET US BE HEATHENS' 'break?|1.1,1.1|%opt{foo}' 'single quote|1.1,1.1|a fool man''s parade' 'rc/loli.kak|28.1,28.1|printf ''trigger-user-hook LoliBufChange\n''' 'rc/loli/kak|1.1,1.1|@ % @@ %% %@ @% %val{timestamp} '''''''
# set-option global loli_global_list 'rc/debug.kak|1.1,1.1|# DEBUGGING' 'rc/loli.kak|1.1,1.1|# Let us simply whatever' 'foo|bar the|1.1,1.1|LET US BE HEATHENS' 'break?|1.1,1.1|%opt{foo}' 'single quote|1.1,1.1|a fool man''s parade' 'rc/loli.kak|28.1,28.1|printf ''trigger-user-hook LoliBufChange\n'''

define-command loli-debug %{
    info -title "loli debug" \
"timestamp: %val{timestamp}
loli_global_list: %opt{loli_global_list}
loli_global_ranges: %opt{loli_global_ranges}"
}

# hook global NormalIdle .* %{
#     loli-debug
# }

# hook global GlobalSetOption loli_global_list=.* %{
#     echo -debug %opt{loli_global_list}
# }

define-command loli-add-selection -params 1 %{
    set-option -add global loli_global_list "%val{bufname}|%val{selection_desc}|%arg{1}"
}
