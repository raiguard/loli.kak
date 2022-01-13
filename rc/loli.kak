# Let us simply focus on the global (quickfix) list for now
# Then we can focus on the client lists

# str-list to hold the master copy of the list
declare-option str-list loli_global_list
# range-specs for updating the ranges
# We can use this for every client by setting it at the window level
declare-option range-specs loli_global_ranges

declare-option -hidden int loli_prev_timestamp 0

# First, make a mechanism for storing a master list and syncing it with various range-specs

# TODO: Figure out how to make requires work and make a util for parsing multiple passed lists

# Refresh all range-specs when the master list is updated
hook global GlobalSetOption loli_global_list=.* %{
    lua %opt{loli_global_list} "$&$" %val{client_list} %{
        local function parse_args(var_names, list_names)
            local vars = {}
            local i = 0
            for _, var_name in pairs(var_names) do
                i = i + 1
                vars[var_name] = arg[i]
            end

            local lists = {}
            for _, list_name in pairs(list_names) do
                i = i + 1
                local list = {}
                local item = arg[i]
                while item and item ~= "$&$" and item ~= "lua" do
                    table.insert(list, item)
                    i = i + 1
                    item = arg[i]
                end
                lists[list_name] = list
            end

            return vars, lists
        end

        local range_specs = {}
        local vars, lists = parse_args({}, {"entries", "clients"})
        for _, entry in pairs(lists.entries) do
            local _, _, filename, range, preview = string.find(entry, "(.-)%|(%d+.%d+,%d+.%d+)%|(.*)")
            if range and filename and preview then
                local specs = range_specs[filename]
                if not specs then
                    specs = {}
                    range_specs[filename] = specs
                end
                table.insert(specs, range.."|"..preview)
            end
        end

        kak.echo("-debug", "RANGES:")
        for filename, ranges in pairs(range_specs) do
            kak.echo("-debug", filename..": "..table.concat(ranges, " // "))
        end

        kak.echo("-debug", "CLIENTS:")
        for _, client_name in pairs(lists.clients) do
            kak.echo("-debug", client_name)
        end
    }
}

# LoliBufChange
# This hook fires whenever a window's timestamp changes, indicating that the buffer contents were modified
hook global NormalIdle .* %{
    evaluate-commands %sh{
        if [ "$kak_timestamp" -gt "$kak_opt_loli_prev_timestamp" ]; then
            printf 'trigger-user-hook LoliBufChange\n'
            printf 'set-option buffer loli_prev_timestamp %s\n'  "$kak_timestamp"
        fi
    }
}

hook global User LoliBufChange %{
    echo "update master list"
}
