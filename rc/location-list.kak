# --------------------------------------------------
# COMMANDS


def -docstring "(global) next location below current line" lgbelow %{
    echo "(global) next location below current line"
}

def -docstring "(global) next location" lgnext %{
    echo "(global) next location"
}

def -docstring "(global) previous location above current line" lgabove %{
    echo "(global) previous location above current line"
}

def -docstring "(global) previous location" lgprev %{
    echo "(global) previous location"
}

def -docstring "(global) first location" lgfirst %{
    echo "(global) first location"
}

def -docstring "(global) last location" lglast %{
    echo "(global) last location"
}

def -docstring "(global) open list buffer" lgopen %{
    echo "(global) open list buffer"
}

def -docstring "(global) close list buffer" lgclose %{
    echo "(global) close list buffer"
}

def -docstring "(client) next location below current line" lcbelow %{
    echo "(client) next location below current line"
}

def -docstring "(client) next location" lcnext %{
    echo "(client) next location"
}

def -docstring "(client) previous location above current line" lcabove %{
    echo "(client) previous location above current line"
}

def -docstring "(client) previous location" lcprev %{
    echo "(client) previous location"
}

def -docstring "(client) first location" lcfirst %{
    echo "(client) first location"
}

def -docstring "(client) last location" lclast %{
    echo "(client) last location"
}

def -docstring "(client) open list buffer" lcopen %{
    echo "(client) open list buffer"
}

def -docstring "(client) close list buffer" lcclose %{
    echo "(client) close list buffer"
}

# --------------------------------------------------
# USER MODES

# Default user mode
declare-user-mode location-list
map global location-list j ": lgbelow<ret>" -docstring "(global) next location below current line"
map global location-list n ": lgnext<ret>" -docstring "(global) next location"
map global location-list k ": lgabove<ret>" -docstring "(global) previous location above current line"
map global location-list p ": lgprev<ret>" -docstring "(global) previous location"
map global location-list h ": lgfirst<ret>" -docstring "(global) first location"
map global location-list l ": lglast<ret>" -docstring "(global) last location"
map global location-list o ": lgopen<ret>" -docstring "(global) open list buffer"
map global location-list c ": lgclose<ret>" -docstring "(global) close list buffer"
map global location-list J ": lcbelow<ret>" -docstring "(client) next location below current line"
map global location-list N ": lcnext<ret>" -docstring "(client) next location"
map global location-list K ": lcabove<ret>" -docstring "(client) previous location above current line"
map global location-list P ": lcprev<ret>" -docstring "(client) previous location"
map global location-list H ": lcfirst<ret>" -docstring "(client) first location"
map global location-list L ": lclast<ret>" -docstring "(client) last location"
map global location-list O ": lcopen<ret>" -docstring "(client) open list buffer"
map global location-list C ": lcclose<ret>" -docstring "(client) close list buffer"

# Alternate user mode
declare-user-mode location-list-alt
map global location-list-alt j ": lcbelow<ret>" -docstring "(client) next location below current line"
map global location-list-alt n ": lcnext<ret>" -docstring "(client) next location"
map global location-list-alt k ": lcabove<ret>" -docstring "(client) previous location above current line"
map global location-list-alt p ": lcprev<ret>" -docstring "(client) previous location"
map global location-list-alt h ": lcfirst<ret>" -docstring "(client) first location"
map global location-list-alt l ": lclast<ret>" -docstring "(client) last location"
map global location-list-alt o ": lcopen<ret>" -docstring "(client) open list buffer"
map global location-list-alt c ": lcclose<ret>" -docstring "(client) close list buffer"
map global location-list-alt J ": lgbelow<ret>" -docstring "(global) next location below current line"
map global location-list-alt N ": lgnext<ret>" -docstring "(global) next location"
map global location-list-alt K ": lgabove<ret>" -docstring "(global) previous location above current line"
map global location-list-alt P ": lgprev<ret>" -docstring "(global) previous location"
map global location-list-alt H ": lgfirst<ret>" -docstring "(global) first location"
map global location-list-alt L ": lglast<ret>" -docstring "(global) last location"
map global location-list-alt O ": lgopen<ret>" -docstring "(global) open list buffer"
map global location-list-alt C ": lgclose<ret>" -docstring "(global) close list buffer"
