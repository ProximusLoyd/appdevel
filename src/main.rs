use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::{UNIX_EPOCH, SystemTime};
use chrono::{DateTime, Local};
use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};
use unicode_segmentation::UnicodeSegmentation;



const COLORS: &[(&str, &str)] = &[
    ("dir", "\x1b[95m"), 
    ("file", "\x1b[94m"),
    ("link", "\x1b[96m"),
    ("header", "\x1b[92m"), 
    ("reset", "\x1b[0m"),
];

const GLYPHS: &[(&str, &str)] = &[
    ("dir", ""),
    ("file", ""),
    ("link", ""),
    ("music", ""),
    ("video", ""),
    ("image", ""),
    ("archive", ""),
    ("text", ""),
    ("code", ""),
    ("config", ""),
];

const FILE_TYPE_MAP: &[(&str, &[&str])] = &[
    ("music", &["mp3", "wav", "flac", "ogg"]),
    ("video", &["mp4", "mkv", "avi", "mov"]),
    ("image", &["jpg", "jpeg", "png", "gif", "bmp"]),
    ("archive", &["zip", "tar", "gz", "rar", "7z"]),
    ("text", &["txt", "md", "doc", "docx", "pdf"]),
    ("code", &["py", "js", "html", "css", "rs", "c", "cpp", "java", "sh"]),
    ("config", &["toml", "json", "yaml", "yml", "ini", "cfg", "lock"]),
];



fn get_color(name: &str) -> &str {
    COLORS.iter().find(|(key, _)| *key == name).map(|(_, value)| *value).unwrap_or("")
}

fn get_glyph(name: &str) -> &str {
    GLYPHS.iter().find(|(key, _)| *key == name).map(|(_, value)| *value).unwrap_or("")
}

fn format_size(size: u64) -> String {
    const KILO: u64 = 1024;
    const MEGA: u64 = KILO * 1024;
    const GIGA: u64 = MEGA * 1024;

    if size >= GIGA {
        format!("{:.1}GB", size as f64 / GIGA as f64)
    } else if size >= MEGA {
        format!("{:.1}MB", size as f64 / MEGA as f64)
    } else if size >= KILO {
        format!("{:.1}KB", size as f64 / KILO as f64)
    } else {
        format!("{}B", size)
    }
}


fn display_width(text: &str) -> usize {
    let mut width = 0;
    let mut in_escape = false;
    for ch in text.chars() {
        if in_escape {
            if ch == 'm' {
                in_escape = false;
            }
        } else if ch == '\x1b' {
            in_escape = true;
        } else {
            width += UnicodeWidthChar::width(ch).unwrap_or(0);
        }
    }
    width
}

fn get_file_info(entry: &fs::DirEntry, show_perms: bool) -> Option<Vec<String>> {
    let path = entry.path();
    if let Ok(metadata) = fs::symlink_metadata(&path) {
        let mode = metadata.mode();
        let size = metadata.len();
        let modified_time: SystemTime = metadata.modified().unwrap_or(UNIX_EPOCH);
        let datetime: DateTime<Local> = modified_time.into();
        let file_name = entry.file_name().to_string_lossy().to_string();

        let (file_type, glyph, color) = if metadata.is_dir() {
            ("Dir".to_string(), get_glyph("dir"), get_color("dir"))
        } else if metadata.file_type().is_symlink() {
            ("Link".to_string(), get_glyph("link"), get_color("link"))
        } else {
            let ext = Path::new(&file_name).extension().and_then(|s| s.to_str()).unwrap_or("");
            let mut file_type = "File".to_string();
            let mut glyph = get_glyph("file");

            if file_name == ".gitignore" {
                file_type = "text".to_string();
                glyph = get_glyph("text");
            } else {
                for (name, exts) in FILE_TYPE_MAP.iter() {
                    if exts.contains(&ext) {
                        file_type = name.to_string();
                        glyph = get_glyph(name);
                        break;
                    }
                }
            }
            (file_type, glyph, get_color("file"))
        };

        let format = if metadata.is_dir() {
            "dir".to_string()
        } else {
            Path::new(&file_name)
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| {
                    let s_str = s.to_string();
                    if s_str.len() > 6 {
                        s_str[..6].to_string()
                    } else {
                        s_str
                    }
                })
                .unwrap_or("N/A".to_string())
        };

        let mut info_vec = vec![
            format!("{}{}{}  {}{}", color, glyph, get_color("reset"), file_name, get_color("reset")),
            format,
            file_type,
        ];

        if show_perms {
            info_vec.push(perms_to_string(mode));
        }

        info_vec.push(format_size(size));
        info_vec.push(datetime.format("%Y-%m-%d %H:%M:%S").to_string());

        Some(info_vec)
    } else {
        None
    }
}

fn perms_to_string(mode: u32) -> String {
    let mut perms = String::new();
    perms.push(if (mode & 0o170000) == 0o040000 { 'd' } else { '-' });
    perms.push(if (mode & 0o400) != 0 { 'r' } else { '-' });
    perms.push(if (mode & 0o200) != 0 { 'w' } else { '-' });
    perms.push(if (mode & 0o100) != 0 { 'x' } else { '-' });
    perms.push(if (mode & 0o040) != 0 { 'r' } else { '-' });
    perms.push(if (mode & 0o020) != 0 { 'w' } else { '-' });
    perms.push(if (mode & 0o010) != 0 { 'x' } else { '-' });
    perms.push(if (mode & 0o004) != 0 { 'r' } else { '-' });
    perms.push(if (mode & 0o002) != 0 { 'w' } else { '-' });
    perms.push(if (mode & 0o001) != 0 { 'x' } else { '-' });
    perms
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";
    let mut show_perms = false;

    let mut args_iter = args.iter().skip(1);
    while let Some(arg) = args_iter.next() {
        if arg == "-p" {
            show_perms = true;
        } else {
            target_dir = arg;
        }
    }

    let mut entries: Vec<_> = match fs::read_dir(target_dir) {
        Ok(entries) => entries.filter_map(|entry| entry.ok()).collect(),
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    entries.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        b_is_dir.cmp(&a_is_dir).then_with(|| a.file_name().cmp(&b.file_name()))
    });

    let mut table_data: Vec<_> = entries.iter().filter_map(|e| get_file_info(e, show_perms)).collect();

    for (i, row) in table_data.iter_mut().enumerate() {
        row.insert(0, (i + 1).to_string());
    }
    let mut headers = vec!["#", "Name", "Format", "Type"];
    if show_perms {
        headers.push("Perms");
    }
    headers.push("Size");
    headers.push("Modified");
    let mut col_widths: Vec<usize> = headers.iter().map(|h| h.width()).collect();

    for row in &table_data {
        for (i, cell) in row.iter().enumerate() {
            let width = display_width(cell.as_str());
            if width > col_widths[i] {
                col_widths[i] = width;
            }
        }
    }

    
    const MAX_NAME_COL_WIDTH: usize = 40;
    if col_widths[1] > MAX_NAME_COL_WIDTH {
        col_widths[1] = MAX_NAME_COL_WIDTH;
    }

    

    fn print_row(row_data: &[String], col_widths: &[usize], is_header: bool, headers: &[&str]) {
        let total_width: usize = col_widths.iter().sum::<usize>() + (col_widths.len() * 3) + 1;
        let mut row_str = String::with_capacity(total_width);
        row_str.push('║'); 
        for (i, cell) in row_data.iter().enumerate() {
            let width = col_widths[i];
            let cell_width = display_width(cell.as_str());

            let mut final_cell = cell.clone();
            if headers[i] == "Name" && cell_width > width { 
                let mut truncated = String::new();
                let mut current_width = 0;
                for grapheme in UnicodeSegmentation::graphemes(cell.as_str(), true) {
                    let grapheme_width = display_width(grapheme);
                    if current_width + grapheme_width > width {
                        break;
                    }
                    truncated.push_str(grapheme);
                    current_width += grapheme_width; 
                }
                final_cell = truncated;
            }

            
            let final_cell_display_width = display_width(&final_cell);
            let padding = if width > final_cell_display_width { width - final_cell_display_width } else { 0 };

            let cell_content = if is_header {
                format!("{}{}{}", get_color("header"), final_cell, get_color("reset"))
            } else {
                final_cell
            };
            row_str.push_str(&format!(" {}{} ", cell_content, " ".repeat(padding)));
            if i == 0 { // Right border of the '#' column
                row_str.push_str("║"); // Double vertical border
            } else if i < col_widths.len() - 1 { // Other inner vertical borders
                row_str.push_str("│"); // Single vertical border
            } else {
                row_str.push_str("║"); // Rightmost outer border is double
            }
        }
        println!("{}", row_str);
    }

    fn print_separator_line(col_widths: &[usize], left: &str, mid_chars: &[&str], right: &str, h_line: &str) {
        let mut separator = left.to_string();
        for (i, w) in col_widths.iter().enumerate() {
            separator.push_str(&h_line.repeat(w + 2));
            if i < col_widths.len() - 1 {
                separator.push_str(mid_chars[i]); // Use specific mid char for each column
            }
        }
        separator.push_str(right);
        println!("{}", separator);
    }

    let mut top_mid_chars = vec!["╤"; col_widths.len() - 1];
    top_mid_chars[0] = "╦";
    print_separator_line(&col_widths, "╔", &top_mid_chars, "╗", "═");
    print_row(&headers.iter().map(|s| s.to_string()).collect::<Vec<String>>(), &col_widths, true, &headers);

    let mut middle_mid_chars = vec!["╪"; col_widths.len() - 1];
    middle_mid_chars[0] = "╬";
    print_separator_line(&col_widths, "╠", &middle_mid_chars, "╣", "═");
    for row in &table_data {
        print_row(row, &col_widths, false, &headers);
    }

    let mut bottom_mid_chars = vec!["╧"; col_widths.len() - 1];
    bottom_mid_chars[0] = "╩";
    print_separator_line(&col_widths, "╚", &bottom_mid_chars, "╝", "═");
}