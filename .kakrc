define-command loli-log %{
    new edit /tmp/kak-loli.log
}

define-command lsp-log %{
    new edit /tmp/kak-lsp.log
}

set global lsp_cmd "kak-lsp -s %val{session} -vvv --log /tmp/kak-lsp.log"
