use anyhow::Result;

use crate::ansi::color_to_ansi;

pub enum PrintableLine {
    Button(usize, String),
    Text(String),
}

const BUTTON: &str = "~08>~07>~0F> Click Here <~07<~08<";

pub fn compile_lines(contents: &str, width: u16) -> Vec<PrintableLine> {
    let lines = contents
        .lines()
        .map(|line| line_to_printable_line(line, width))
        .collect::<Vec<PrintableLine>>();
    lines
}

fn line_to_printable_line(line: &str, width: u16) -> PrintableLine {
    if line.contains("þBUTTON") {
        let button_regex = regex::Regex::new(r"þBUTTON(\d{4})").unwrap();
        let caps = button_regex.captures(line).unwrap();

        // Get the line number
        let line_number = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();

        PrintableLine::Button(
            line_number,
            line_to_ansi(&button_regex.replace_all(line, BUTTON))
                .unwrap()
                .to_string(),
        )
    } else if line.contains("þLINE") {
        PrintableLine::Text(generate_line(width, 1).unwrap())
    } else {
        PrintableLine::Text(line_to_ansi(line).unwrap().to_string())
    }
}

pub fn line_to_ansi(line: &str) -> Result<String> {
    let mut result = String::new();

    // Loop through the line, converting it to ANSI escape codes
    for part in line.split("~") {
        // Empty part, so skip it
        if part.is_empty() {
            continue;
        }

        result.push_str(&color_to_ansi(&part[0..2])?);
        result.push_str(&part[2..]);
    }

    Ok(result)
}

pub fn generate_line(width: u16, padding: u16) -> Result<String> {
    let mut result = String::new();

    if padding > 0 {
        result.push_str("~00");

        for _ in 0..padding {
            result.push(' ');
        }
    }

    result.push_str("~08──~07──~0F");

    for _ in 0..width - 8 - (padding * 2) {
        result.push('─');
    }

    result.push_str("~07──~08──");

    line_to_ansi(&result)
}
