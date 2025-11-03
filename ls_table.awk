# Function to calculate display width of a string
function display_width(str,   i, c, width) {
    width = 0
    for (i = 1; i <= length(str); i++) {
        c = substr(str, i, 1)
        if (c < "\x80") {
            width++
        } else {
            width += 2
        }
    }
    return width
}

# Function to print a separator line
function print_separator(left, sep, right) {
    printf "%s", left;
    for (i = 1; i <= 6; i++) {
        for (j = 0; j < max_widths[i] + 2; j++) printf "%s", h;
        if (i < 6) printf "%s", sep;
    }
    printf "%s\n", right;
}

BEGIN {
    # Define box-drawing characters
    h = "\u2500"; # horizontal
    v = "\u2502"; # vertical
    tl = "\u250c"; # top-left
    tr = "\u2510"; # top-right
    bl = "\u2514"; # bottom-left
    br = "\u2518"; # bottom-right
    ts = "\u252c"; # top-separator
    bs = "\u2534"; # bottom-separator
    ls = "\u251c"; # left-separator
    rs = "\u2524"; # right-separator
    s = "\u253c";  # separator

    # Initialize max widths
    max_widths[1] = 4; # Name
    max_widths[2] = 4; # Type
    max_widths[3] = 5; # Perms
    max_widths[4] = 1; # L
    max_widths[5] = 4; # Size
    max_widths[6] = 11; # Date & Time

    # Set a maximum width for the Name column to prevent overflow
    MAX_NAME_COL_WIDTH = 60;
}

# Process each line of ls -l output
NR > 1 && $1 != "total" {
    permissions[NR-1] = $1;
    links[NR-1] = $2;
    size[NR-1] = $5;
    datetime[NR-1] = $6 " " $7;
    name[NR-1] = "";
    for (i = 9; i <= NF; i++) {
        name[NR-1] = name[NR-1] (i == 9 ? "" : " ") $i;
    }
    
    # Get file type
    type[NR-1] = "File";
    if (match(name[NR-1], /\.([^.]+)$/, ext)) {
        type[NR-1] = ext[1];
    } else if (permissions[NR-1] ~ /^d/) {
        type[NR-1] = "Dir";
    }


    # Update max widths using display_width
    current_name_width = display_width(name[NR-1]);
    if (current_name_width > max_widths[1]) {
        max_widths[1] = current_name_width;
    }

    if (display_width(type[NR-1]) > max_widths[2]) max_widths[2] = display_width(type[NR-1]);
    if (display_width(permissions[NR-1]) > max_widths[3]) max_widths[3] = display_width(permissions[NR-1]);
    if (display_width(links[NR-1]) > max_widths[4]) max_widths[4] = display_width(links[NR-1]);
    if (display_width(size[NR-1]) > max_widths[5]) max_widths[5] = display_width(size[NR-1]);
    if (display_width(datetime[NR-1]) > max_widths[6]) max_widths[6] = display_width(datetime[NR-1]);
}

END {
    # Cap the Name column width
    if (max_widths[1] > MAX_NAME_COL_WIDTH) {
        max_widths[1] = MAX_NAME_COL_WIDTH;
    }

    # Build format strings
    header_format = "";
    row_format = "";
    for (i = 1; i <= 6; i++) {
        header_format = header_format v " %-" max_widths[i] "s ";
        row_format = row_format v " %-" max_widths[i] "s ";
    }
    header_format = header_format v "\n";
    row_format = row_format v "\n";

    # Print top border
    print_separator(tl, ts, tr);

    # Print header
    printf(header_format, "Name", "Type", "Perms", "L", "Size", "Date & Time");

    # Print header/content separator
    print_separator(ls, s, rs);

    # Print file info
    for (i = 1; i < NR; i++) {
        if (permissions[i] != "") {
            # Truncate name if it's too long
            display_name = name[i];
            if (display_width(display_name) > max_widths[1]) {
                # This is tricky with multi-byte characters. 
                # A simple substr might cut a character in half.
                # For now, we will just truncate based on byte length, which might not be perfect.
                display_name = substr(display_name, 1, max_widths[1] - 3) "...";
            }
            printf(row_format, display_name, type[i], permissions[i], links[i], size[i], datetime[i]);
        }
    }

    # Print bottom border
    print_separator(bl, bs, br);
}
