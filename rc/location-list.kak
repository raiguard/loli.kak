# --------------------------------------------------
# COMMANDS

def -docstring "Go to the next entry on the global list." lgnext %{
    echo "Go to the next entry on the global list."
}

def -docstring "Go to the previous entry on the global list." lgprev %{
    echo "Go to the previous entry on the global list."
}

def -docstring "Go to the first entry on the global list." lgfirst %{
    echo "Go to the first entry on the global list."
}

def -docstring "Go to the last entry on the global list." lglast %{
    echo "Go to the last entry on the global list."
}

def -docstring "Open the global location list." lgopen %{
    echo "Open the global location list."
}

def -docstring "Close the global location list." lgclose %{
    echo "Close the global location list."
}

def -docstring "Go to the next entry on the client list." lcnext %{
    echo "Go to the next entry on the client list."
}

def -docstring "Go to the previous entry on the client list." lcprev %{
    echo "Go to the previous entry on the client list."
}

def -docstring "Go to the first entry on the client list." lcfirst %{
    echo "Go to the first entry on the client list."
}

def -docstring "Go to the last entry on the client list." lclast %{
    echo "Go to the last entry on the client list."
}

def -docstring "Open the client location list." lcopen %{
    echo "Open the client location list."
}

def -docstring "Close the client location list." lcclose %{
    echo "Close the client location list."
}

# --------------------------------------------------
# USER MODES

# Default user mode
declare-user-mode location-list
map global location-list j ": lgnext<ret>" -docstring "Go to the next entry on the global list."
map global location-list n ": lgnext<ret>" -docstring "Go to the next entry on the global list."
map global location-list k ": lgprev<ret>" -docstring "Go to the previous entry on the global list."
map global location-list p ": lgnext<ret>" -docstring "Go to the next entry on the global list."
map global location-list h ": lgfirst<ret>" -docstring "Go to the first entry on the global list."
map global location-list l ": lglast<ret>" -docstring "Go to the last entry on the global list."
map global location-list o ": lgopen<ret>" -docstring "Open the global location list."
map global location-list c ": lgclose<ret>" -docstring "Close the global location list."
map global location-list J ": lcnext<ret>" -docstring "Go to the next entry on the client list."
map global location-list N ": lcnext<ret>" -docstring "Go to the next entry on the client list."
map global location-list K ": lcprev<ret>" -docstring "Go to the previous entry on the client list."
map global location-list P ": lcprev<ret>" -docstring "Go to the previous entry on the client list."
map global location-list H ": lcfirst<ret>" -docstring "Go to the first entry on the client list."
map global location-list L ": lclast<ret>" -docstring "Go to the last entry on the client list."
map global location-list O ": lcopen<ret>" -docstring "Open the client location list."
map global location-list C ": lcclose<ret>" -docstring "Close the client location list."

# Alternate (reversed) user mode
declare-user-mode location-list-alt
map global location-list-alt j ": lcnext<ret>" -docstring "Go to the next entry on the client list."
map global location-list-alt n ": lcnext<ret>" -docstring "Go to the next entry on the client list."
map global location-list-alt k ": lcprev<ret>" -docstring "Go to the previous entry on the client list."
map global location-list-alt p ": lcprev<ret>" -docstring "Go to the previous entry on the client list."
map global location-list-alt h ": lcfirst<ret>" -docstring "Go to the first entry on the client list."
map global location-list-alt l ": lclast<ret>" -docstring "Go to the last entry on the client list."
map global location-list-alt o ": lcopen<ret>" -docstring "Open the client location list."
map global location-list-alt c ": lcclose<ret>" -docstring "Close the client location list."
map global location-list-alt J ": lgnext<ret>" -docstring "Go to the next entry on the global list."
map global location-list-alt N ": lgnext<ret>" -docstring "Go to the next entry on the global list."
map global location-list-alt K ": lgprev<ret>" -docstring "Go to the previous entry on the global list."
map global location-list-alt P ": lgprev<ret>" -docstring "Go to the previous entry on the global list."
map global location-list-alt H ": lgfirst<ret>" -docstring "Go to the first entry on the global list."
map global location-list-alt L ": lglast<ret>" -docstring "Go to the last entry on the global list."
map global location-list-alt O ": lgopen<ret>" -docstring "Open the global location list."
map global location-list-alt C ": lgclose<ret>" -docstring "Close the global location list."
