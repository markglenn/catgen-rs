use anyhow::Result;
use crossterm::{
    cursor::{MoveTo, MoveToColumn},
    style::{Color, Colors, Print, SetColors, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

use crate::parser::generate_line;

pub fn draw_footer(mut stdout: &Stdout, width: u16, height: u16) -> Result<()> {
    stdout
        .queue(MoveTo(0, height - 2))?
        .queue(Clear(ClearType::UntilNewLine))?
        .queue(Print(generate_line(width, 0)?))?;

    stdout
        .queue(MoveTo(0, height - 1))?
        .queue(SetColors(Colors::new(Color::Green, Color::Black)))?
        .queue(Print("Catgen v3.0"))?;

    // Draw company logo in the center
    stdout
        .queue(MoveToColumn((width - 22) / 2))?
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
        .queue(MoveToColumn(width - 21))?
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
