decl str-list ll_test_list "colors/one-darker.kak|11.1,11.4|decl -hidden str fg 'abb2bf'" "rc/test.kak|35.13,35.17|face global value 'rgb:%opt{darkorange}'"
decl range-specs ll_test_ranges

hook global WinCreate .* %{
    addhl window/ ranges ll_test_ranges
}
