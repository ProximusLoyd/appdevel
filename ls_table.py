#!/usr/bin/env python3

import os
import sys
import stat
from datetime import datetime
import unicodedata

try:
    from wcwidth import wcswidth
except ImportError:
    def wcswidth(text):
        width = 0
        for char in text:
            if unicodedata.east_asian_width(char) in ('W', 'F', 'A'):
                width += 2
            else:
                width += 1
        return width

COLORS = {
    "dir": "\033[32m", 
    "file": "\033[34m",
    "link": "\033[36m",
    "header": "\033[92m", 
    "reset": "\033[0m",
}

GLYPHS = {
    "dir": "",
    "file": "",
    "link": "",
    "music": "",
    "video": "",
    "image": "",
    "archive": "",
    "text": "",
    "code": "",
    "config": "",
}

FILE_TYPE_MAP = {
    "music": [".mp3", ".wav", ".flac", ".ogg"],
    "video": [".mp4", ".mkv", ".avi", ".mov"],
    "image": [".jpg", ".jpeg", ".png", ".gif", ".bmp"],
    "archive": [".zip", ".tar", ".gz", ".rar", ".7z"],
    "text": [".txt", ".md", ".doc", ".docx", ".pdf"],
    "code": [".py", ".js", ".html", ".css", ".rs", ".c", ".cpp", ".java", ".sh"],
    "config": [".toml", ".json", ".yaml", ".yml", ".ini", ".cfg", ".lock"],
}

def display_width(text):
    clean_text = text
    for color in COLORS.values():
        clean_text = clean_text.replace(color, "")
    return wcswidth(clean_text)

def format_size(size_bytes):
    if size_bytes >= 1024**3:
        return f"{size_bytes / (1024**3):.1f}GB"
    elif size_bytes >= 1024**2:
        return f"{size_bytes / (1024**2):.1f}MB"
    elif size_bytes >= 1024:
        return f"{size_bytes / 1024:.1f}KB"
    else:
        return f"{size_bytes}B"

def get_file_info(entry, show_perms):
    try:
        stats = entry.stat(follow_symlinks=False)
        mode = stats.st_mode
        size = stats.st_size
        modified = datetime.fromtimestamp(stats.st_mtime).strftime('%Y-%m-%d %H:%M:%S')
        perms = stat.filemode(mode) if show_perms else ""

        _, ext = os.path.splitext(entry.name)
        
        if stat.S_ISDIR(mode):
            file_type = "dir"
            glyph = GLYPHS["dir"]
            color = COLORS["dir"]
            format = "dir"
        elif stat.S_ISLNK(mode):
            file_type = "link"
            glyph = GLYPHS["link"]
            color = COLORS["link"]
            format = "link"
        else:
            ext_str = ext[1:].lower() 
            if ext_str:
                format = ext_str[:6] 
            else:
                format = "N/A" 
            
            ext = ext.lower() 
            file_type = "file"
            glyph = GLYPHS["file"]
            color = COLORS["file"]
            for type_name, extensions in FILE_TYPE_MAP.items():
                if ext in extensions:
                    file_type = type_name
                    glyph = GLYPHS[type_name]
                    break

        return {
            "name": f"{color}{glyph}{COLORS['reset']}  {entry.name}",
            "format": format,
            "type": file_type.capitalize(),
            "perms": perms,
            "size": format_size(size),
            "modified": modified,
            "is_dir": stat.S_ISDIR(mode),
        }
    except OSError:
        return None

def print_row(row_data, headers, col_widths, is_header=False):
    row_str = "║" 
    for i, h in enumerate(headers):
        cell = row_data[h.lower()] if not is_header else row_data[i]
        cell_width = display_width(cell)
        padding = col_widths[i] - cell_width
        
        if headers[i] == "Name" and padding < 0:
            truncated_cell = ""
            current_display_width = 0
            for char in cell:
                char_width = display_width(char)
                if current_display_width + char_width <= col_widths[i]:
                    truncated_cell += char
                    current_display_width += char_width
                else:
                    break
            cell = truncated_cell
            padding = col_widths[i] - display_width(cell) 

        cell_content = cell
        if is_header:
            cell_content = f"{COLORS['header']}{cell}{COLORS['reset']}"
        row_str += f" {cell_content}{' ' * padding} "
        if i == 0: 
            row_str += "║" 
        elif i < len(col_widths) - 1: 
            row_str += "│" 
        else:
            row_str += "║" 
    print(row_str)

def print_separator_line(left, mid_chars, right, h_line, col_widths):
    separator = left
    for i, w in enumerate(col_widths):
        separator += h_line * (w + 2)
        if i < len(col_widths) - 1:
            separator += mid_chars[i] 
    separator += right
    print(separator)

def main():
    target_dir = "."
    show_perms = False

    args = sys.argv[1:]
    if "-p" in args:
        show_perms = True
        args.remove("-p")
    if args:
        target_dir = args[0]

    try:
        entries = list(os.scandir(target_dir))
    except FileNotFoundError:
        print(f"Error: Directory not found: {target_dir}")
        sys.exit(1)

    entries.sort(key=lambda e: (not e.is_dir(), e.name))

    table_data = [get_file_info(e, show_perms) for e in entries]
    table_data = [info for info in table_data if info is not None] 

    for i, info in enumerate(table_data):
        info["#"] = str(i + 1)

    headers = ["#", "Name", "Format", "Type"]
    if show_perms:
        headers.append("Perms")
    headers.extend(["Size", "Modified"])
    col_widths = [display_width(h) for h in headers]

    for info in table_data:
        for i, h in enumerate(headers):
            cell_width = display_width(info[h.lower()])
            if cell_width > col_widths[i]:
                col_widths[i] = cell_width

    MAX_NAME_COL_WIDTH = 40
    if col_widths[1] > MAX_NAME_COL_WIDTH:
        col_widths[1] = MAX_NAME_COL_WIDTH

    top_mid_chars = ["╤"] * (len(col_widths) - 1)
    top_mid_chars[0] = "╦"
    print_separator_line("╔", top_mid_chars, "╗", "═", col_widths)
    print_row(headers, headers, col_widths, is_header=True)

    middle_mid_chars = ["╪"] * (len(col_widths) - 1)
    middle_mid_chars[0] = "╬"
    print_separator_line("╠", middle_mid_chars, "╣", "═", col_widths)
    for info in table_data:
        print_row(info, headers, col_widths)

    bottom_mid_chars = ["╧"] * (len(col_widths) - 1)
    bottom_mid_chars[0] = "╩"
    print_separator_line("╚", bottom_mid_chars, "╝", "═", col_widths)

if __name__ == "__main__":
    main()