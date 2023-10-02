use anyhow::Result;
use crossterm::{
    cursor::{MoveTo, MoveToColumn},
    style::{Color, Colors, Print, SetBackgroundColor, SetColors, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

use crate::{parser::generate_line, state::State};

pub fn draw_footer(mut stdout: &Stdout, state: &State) -> Result<()> {
    stdout
        .queue(MoveTo(0, state.height - 2))?
        .queue(Clear(ClearType::UntilNewLine))?
        .queue(Print(generate_line(state.width, 0)?))?;

    stdout
        .queue(MoveTo(0, state.height - 1))?
        .queue(Clear(ClearType::UntilNewLine))?
        .queue(SetColors(Colors::new(Color::Green, Color::Black)))?
        .queue(Print("Catgen v3.0"))?;

    // Draw company logo in the center
    stdout
        .queue(MoveToColumn((state.width - 22) / 2))?
        .queue(SetForegroundColor(Color::DarkGrey))?
        .queue(Print("4"))?
        .queue(SetForegroundColor(Color::Grey))?
        .queue(Print("t"))?
        .queue(SetForegroundColor(Color::White))?
        .queue(Print("h Dimension Softwa"))?
        .queue(SetForegroundColor(Color::Grey))?
        .queue(Print("r"))?
        .queue(SetForegroundColor(Color::DarkGrey))?
        .queue(Print("e"))?;

    // Draw the helper text
    stdout
        .queue(MoveToColumn(state.width - 21))?
        .queue(SetForegroundColor(Color::Red))?
        .queue(Print("("))?
        .queue(SetForegroundColor(Color::Green))?
        .queue(Print("↑↓"))?
        .queue(SetForegroundColor(Color::Red))?
        .queue(Print(") ("))?
        .queue(SetForegroundColor(Color::Green))?
        .queue(Print("S"))?
        .queue(SetForegroundColor(Color::Red))?
        .queue(Print(")"))?
        .queue(SetForegroundColor(Color::Blue))?
        .queue(Print("earch "))?
        .queue(SetForegroundColor(Color::Red))?
        .queue(Print("("))?
        .queue(SetForegroundColor(Color::Green))?
        .queue(Print("P"))?
        .queue(SetForegroundColor(Color::Red))?
        .queue(Print(")"))?
        .queue(SetForegroundColor(Color::Blue))?
        .queue(Print("rint"))?;

    stdout.flush()?;
    Ok(())
}

pub fn draw_scollbar(mut stdout: &Stdout, state: &State) -> Result<()> {
    stdout
        .queue(SetBackgroundColor(Color::White))?
        .queue(SetForegroundColor(Color::Black))?
        .queue(MoveTo(state.width - 1, 0))?
        .queue(Print("↑"))?
        .queue(MoveTo(state.width - 1, state.height - 3))?
        .queue(Print("↓"))?;

    for y in 1..state.height - 3 {
        stdout
            .queue(MoveTo(state.width - 1, y))?
            .queue(Print("░"))?;
    }

    // Draw the scrollbar position
    stdout
        .queue(SetBackgroundColor(Color::Black))?
        .queue(SetForegroundColor(Color::White))?
        .queue(MoveTo(state.width - 1, state.scrollbar_position()))?
        .queue(Print(" "))?
        .flush()?;
    Ok(())
}

pub fn draw_closing_screen(mut stdout: &Stdout) -> Result<()> {
    stdout
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(SetBackgroundColor(Color::Black))?;

    let lines = [
        (8, r#"                        ___ ___ ___________ ___ ___"#),
        (8, r#"                       Y   Y   Y   _   _   Y   Y   |"#),
        (8, r#"                       |   l   l___|   |___|   l   |"#),
        (8, r#"                       l____   |   |   |   |   _   |"#),
        (7, r#"                           |   |   |   |   |   |   |"#),
        (7, r#"                           l___|   |   l   l___|   |"#),
        (7, r#"                                   `---'       `---'"#),
        (
            7,
            r#"      ______   ___ ___ ___ _______ ______  _______ ___ _______ ______"#,
        ),
        (
            7,
            r#"     Y   _  \ Y   Y   Y   Y   _   Y   _  \Y   _   Y   Y   _   Y   _  \"#,
        ),
        (
            7,
            r#"     |   |   \|   |       |   l___|   |   |   l___l   |   |   |   |   |"#,
        ),
        (
            15,
            r#"     |   |    \   |  \_/  |   __)_    |   l____   |   |   |   |   |   |"#,
        ),
        (
            15,
            r#"     |   l    /   |   |   |   l   |   |   |   l   |   |   l   |   |   |"#,
        ),
        (
            15,
            r#"     l_______/|   l___|   l_______l___|   l_______|   l_______l___|   l"#,
        ),
        (
            15,
            r#"              `---'   `---'           `---'       `---'           `---'"#,
        ),
        (
            15,
            r#"     _______ _______ _______ ___________ ___ ___ _______ _______ _______"#,
        ),
        (
            7,
            r#"    Y   _   Y   _   Y   _   Y   _   _   Y   Y   Y   _   Y   _   \   _   Y"#,
        ),
        (
            7,
            r#"    |   l___l   |   |   l___l___|   |___l   |   |   l   |   l   /   l___|"#,
        ),
        (
            7,
            r#"    l____   |   |   |   __)     |   |   |  / \  |   _   |   _   l   __)_"#,
        ),
        (
            8,
            r#"    |   l   |   l   |   |       |   |   |       |   |   |   |   |   l   |"#,
        ),
        (
            8,
            r#"    |_______|_______l   |       |   l   l___l___l___|   l___|   l_______|"#,
        ),
        (
            8,
            r#"                    `---'       `---'               `---'   `---'"#,
        ),
        (15, r#"                         http://4ds.simplenet.com/"#),
    ];

    let mut y = 0;
    for (dos_color, line) in lines.iter() {
        let color = match dos_color {
            7 => Color::Grey,
            8 => Color::DarkGrey,
            15 => Color::White,
            _ => Color::Red,
        };

        y = y + 1;
        stdout
            .queue(SetForegroundColor(color))?
            .queue(Print(line))?
            .queue(MoveTo(0, y))?;
    }

    // Add an extra line between the logo and the prompt
    stdout.queue(MoveTo(0, y + 1))?.flush()?;

    Ok(())
}
