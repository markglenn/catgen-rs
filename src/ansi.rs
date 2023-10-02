use anyhow::Result;

pub fn color_to_ansi(color: &str) -> Result<String> {
    if color.len() != 2 {
        return Err(anyhow::format_err!("Invalid color code: {}", color));
    }

    let mut color_chars = color.chars();

    let bg = color_chars.nth(0).unwrap().to_digit(16);
    let fg = color_chars.nth(0).unwrap().to_digit(16);

    Ok(format!(
        "\x1B[{}m\x1B[1;{};{}m",
        color_to_blink(bg)?,
        color_to_ansi_color(fg, false)?,
        color_to_ansi_color(bg, true)?,
    ))
}

const DOS_COLORS: [u32; 8] = [0, 4, 2, 6, 1, 5, 3, 7];

// Convert a DOS color code to the ANSI equivalent
fn color_to_ansi_color(color: Option<u32>, is_bg: bool) -> Result<u32> {
    let color_code = match (color, is_bg) {
        (Some(color), false) if color < 8 => 30 + DOS_COLORS[color as usize],
        (Some(color), false) if color < 16 => 90 + DOS_COLORS[color as usize % 8],
        (Some(color), true) if color < 16 => 40 + DOS_COLORS[color as usize >> 1],
        _ => return Err(anyhow::format_err!("Invalid color code: {:#?}", color)),
    };

    Ok(color_code)
}

fn color_to_blink(color: Option<u32>) -> Result<u32> {
    let blink_code = match color {
        Some(color) if color % 2 == 0 => 25,
        Some(color) if color % 2 == 1 => 5,
        _ => return Err(anyhow::format_err!("Invalid color code: {:#?}", color)),
    };

    Ok(blink_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_to_ansi() -> Result<()> {
        assert_eq!(color_to_ansi("00")?, "\x1B[25m\x1B[1;30;40m");
        assert_eq!(color_to_ansi("0f")?, "\x1B[25m\x1B[1;97;40m");
        assert_eq!(color_to_ansi("1f")?, "\x1B[5m\x1B[1;97;40m");
        assert_eq!(color_to_ansi("ff")?, "\x1B[5m\x1B[1;97;47m");

        assert!(color_to_ansi("0").is_err());
        assert!(color_to_ansi("000").is_err());
        assert!(color_to_ansi("gg").is_err());

        Ok(())
    }
}
