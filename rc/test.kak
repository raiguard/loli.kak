decl range-specs ll_test_ranges

hook global WinCreate .* %{
    set window ll_test_ranges %val{timestamp} '1.1,1.4|decl range-specs ll_test_ranges'
    set -add window ll_test_ranges '3.6,3.11|hook global WinCreate .*'
    addhl window/ ranges ll_test_ranges
}

# hook global NormalIdle .* %{
#     echo "%opt{ll_test_ranges}"
# }
