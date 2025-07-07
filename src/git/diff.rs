use std::{
    fs,
    io::Write,
    process::{Command, Output, Stdio},
};

use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
};
use regex::Regex;

pub fn get_file_diff(file_path: &str) -> Result<Text<'static>, String> {
    //get content in diff block
    let output = get_diff_output(file_path)?;

    //make it raw utf8
    let raw_output = String::from_utf8_lossy(&output.stdout);

    //transform it so it adjust in ratatui UI
    let styled_text = parse_delta_ansi(&raw_output);
    Ok(styled_text)
}

fn get_diff_output(file_path: &str) -> Result<Output, String> {
    //launch git command one the specified file
    let mut git_args = vec!["diff"];
    git_args.push(file_path);
    let git_output = match Command::new("git").args(&git_args).output() {
        Ok(value) => value,
        Err(_e) => return Err("Cannot launch git diff command".to_string()),
    };

    //if no modification. Print the file content
    if git_output.stdout.is_empty() {
        return Err(get_file_content(file_path));
    }

    //launch delta on another std
    let mut delta_process = match Command::new("delta")
        .args(["--line-numbers", "--syntax-theme=GitHub"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(value) => value,
        Err(_e) => return Err(get_file_content(file_path)),
    };

    //write git output in delta input
    if let Some(mut stdin) = delta_process.stdin.take() {
        match stdin.write_all(&git_output.stdout) {
            Ok(_v) => {}
            Err(_e) => return Err("Cannot link git and delta".to_string()),
        };
    }

    //wait for the output of delta and return the result
    match delta_process.wait_with_output() {
        Ok(value) => Ok(value),
        Err(_e) => Err("cannot get output".to_string()),
    }
}

fn get_file_content(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|_| "Cannot not read this file".to_string())
}

fn parse_delta_ansi(input: &str) -> Text<'static> {
    let ansi_regex = Regex::new(r"\x1b\[([0-9;]*[mK])").unwrap();
    let mut lines = Vec::new();

    for line in input.lines() {
        let mut spans = Vec::new();
        let mut last_end = 0;
        let mut current_style = Style::default();

        for mat in ansi_regex.find_iter(line) {
            // Text before Ansi code
            if mat.start() > last_end {
                let text = &line[last_end..mat.start()];
                if !text.is_empty() {
                    spans.push(Span::styled(text.to_string(), current_style));
                }
            }

            // Parse ANSI code
            let full_code = &line[mat.start()..mat.end()];
            current_style = parse_sgr_sequence(full_code, current_style);

            last_end = mat.end();
        }

        // text rest
        if last_end < line.len() {
            let text = &line[last_end..];
            if !text.is_empty() {
                spans.push(Span::styled(text.to_string(), current_style));
            }
        }

        if spans.is_empty() {
            lines.push(Line::from(line.to_string()));
        } else {
            lines.push(Line::from(spans));
        }
    }

    Text::from(lines)
}

fn parse_sgr_sequence(code: &str, mut current_style: Style) -> Style {
    if !code.ends_with('m') {
        return current_style; // Ignore les codes non-SGR comme \x1b[K
    }

    let params_str = &code[2..code.len() - 1]; // Retire \x1b[ et m

    if params_str.is_empty() {
        // \x1b[m = reset (comme \x1b[0m)
        return Style::default();
    }

    let params: Vec<&str> = params_str.split(';').collect();
    let mut i = 0;

    while i < params.len() {
        match params[i] {
            // Reset et attributs
            "0" | "" => current_style = Style::default(),
            "1" => current_style = current_style.bold(),
            "4" => current_style = current_style.underlined(),
            "7" => current_style = current_style.reversed(),
            "9" => current_style = current_style.crossed_out(),

            // Couleurs de premier plan standard (30-37)
            "30" => current_style = current_style.fg(Color::Black),
            "31" => current_style = current_style.fg(Color::Red),
            "32" => current_style = current_style.fg(Color::Green),
            "33" => current_style = current_style.fg(Color::Yellow),
            "34" => current_style = current_style.fg(Color::Blue),
            "35" => current_style = current_style.fg(Color::Magenta),
            "36" => current_style = current_style.fg(Color::Cyan),
            "37" => current_style = current_style.fg(Color::White),

            // Couleurs de premier plan brillantes (90-97)
            "90" => current_style = current_style.fg(Color::DarkGray),
            "91" => current_style = current_style.fg(Color::LightRed),
            "92" => current_style = current_style.fg(Color::LightGreen),
            "93" => current_style = current_style.fg(Color::LightYellow),
            "94" => current_style = current_style.fg(Color::LightBlue),
            "95" => current_style = current_style.fg(Color::LightMagenta),
            "96" => current_style = current_style.fg(Color::LightCyan),
            "97" => current_style = current_style.fg(Color::Gray),

            // Couleurs d'arrière-plan (40-47)
            "40" => current_style = current_style.bg(Color::Black),
            "41" => current_style = current_style.bg(Color::Red),
            "42" => current_style = current_style.bg(Color::Green),
            "43" => current_style = current_style.bg(Color::Yellow),
            "44" => current_style = current_style.bg(Color::Blue),
            "45" => current_style = current_style.bg(Color::Magenta),
            "46" => current_style = current_style.bg(Color::Cyan),
            "47" => current_style = current_style.bg(Color::White),

            // Séquences 256 couleurs et RGB (comme dans les tests Delta)
            "38" if i + 2 < params.len() && params[i + 1] == "5" => {
                if let Ok(color_index) = params[i + 2].parse::<u8>() {
                    current_style = current_style.fg(Color::Indexed(color_index));
                }
                i += 2;
            }

            "38" if i + 4 < params.len() && params[i + 1] == "2" => {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    params[i + 2].parse::<u8>(),
                    params[i + 3].parse::<u8>(),
                    params[i + 4].parse::<u8>(),
                ) {
                    current_style = current_style.fg(Color::Rgb(r, g, b));
                }
                i += 4;
            }

            "48" if i + 2 < params.len() && params[i + 1] == "5" => {
                if let Ok(color_index) = params[i + 2].parse::<u8>() {
                    current_style = current_style.bg(Color::Indexed(color_index));
                }
                i += 2;
            }

            "48" if i + 4 < params.len() && params[i + 1] == "2" => {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    params[i + 2].parse::<u8>(),
                    params[i + 3].parse::<u8>(),
                    params[i + 4].parse::<u8>(),
                ) {
                    current_style = current_style.bg(Color::Rgb(r, g, b));
                }
                i += 4;
            }

            _ => {}
        }
        i += 1;
    }

    current_style
}
