#!/bin/sh

ls -l --almost-all --no-group --time-style=long-iso --color=always | awk '
function format_size(size) {
    if (size >= 1024*1024*1024) return sprintf("%.1fG", size/(1024*1024*1024))
    if (size >= 1024*1024) return sprintf("%.1fM", size/(1024*1024))
    if (size >= 1024) return sprintf("%.1fK", size/1024)
    return sprintf("%dB", size)
}

function get_width(text) {
    gsub(/\033\[[0-9;]*m/, "", text)
    w = 0
    for (i=1; i<=length(text); i++) {
        char = substr(text, i, 1)
        if (char ~ /[]/) {
            w += 2
        } else {
            w += 1
        }
    }
    return w
}

BEGIN {
    count = 1
    headers[1] = "#"; headers[2] = "Name"; headers[3] = "Format"; headers[4] = "Type"; headers[5] = "Size"; headers[6] = "Modified";
    for (i=1; i<=6; i++) max_widths[i] = get_width(headers[i])
}
!/^total/ && NF > 0 {
    perms = $1
    size = $4
    date = $5
    time = $6
    name = ""
    for (i = 7; i <= NF; i++) {
        name = name " " $i
    }
    sub(/^ /, "", name)

    type = "File"
    glyph = ""
    if (substr(perms, 1, 1) == "d") {
        type = "Dir"
        glyph = ""
    } else if (substr(perms, 1, 1) == "l") {
        type = "Link"
        glyph = ""
    }

    format = "N/A"
    if (match(name, /\.[^.]+$/)) {
        format = substr(name, RSTART + 1)
    }

    data[count, 1] = count
    data[count, 2] = glyph " " name
    data[count, 3] = format
    data[count, 4] = type
    data[count, 5] = format_size(size)
    data[count, 6] = date " " time

    for (i=1; i<=6; i++) {
        width = get_width(data[count, i])
        if (width > max_widths[i]) max_widths[i] = width
    }
    count++
}
END {
    line = "╔"
    for (i=1; i<=6; i++) {
        line = line sprintf("%s", sprintf("%*s", max_widths[i] + 2, ""))
        if (i<6) line = line "╤"
    }
    line = line "╗"
    gsub(/ /, "═", line)
    gsub(/╤/, "╦", line)
    print line

    header = "║"
    for (i=1; i<=6; i++) {
        header = header sprintf(" %-*s ", max_widths[i], headers[i])
        if (i<6) header = header "│"
    }
    header = header "║"
    print header

    line = "╠"
    for (i=1; i<=6; i++) {
        line = line sprintf("%s", sprintf("%*s", max_widths[i] + 2, ""))
        if (i<6) line = line "╪"
    }
    line = line "╣"
    gsub(/ /, "═", line)
    gsub(/╪/, "╬", line)
    print line

    for (j=1; j < count; j++) {
        row = "║"
        for (i=1; i<=6; i++) {
            row = row sprintf(" %-*s ", max_widths[i], data[j, i])
            if (i<6) row = row "│"
        }
        row = row "║"
        print row
    }

    line = "╚"
    for (i=1; i<=6; i++) {
        line = line sprintf("%s", sprintf("%*s", max_widths[i] + 2, ""))
        if (i<6) line = line "╧"
    }
    line = line "╝"
    gsub(/ /, "═", line)
    gsub(/╧/, "╩", line)
    print line
}
'
