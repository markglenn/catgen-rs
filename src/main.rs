mod ansi;
mod parser;
mod ui;

use anyhow::Result;
use parser::PrintableLine;
use std::io::{stdout, Stdout, Write};
use ui::draw_footer;

use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

type PrintableLines = Vec<PrintableLine>;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;

    // Get the size of the terminal
    let (width, height) = crossterm::terminal::size()?;

    crossterm::terminal::enable_raw_mode()?;

    let lines = parser::compile_lines(DATA, width);

    let mut stdout = stdout();

    stdout.execute(crossterm::cursor::Hide)?;

    let mut current_line = 0;
    draw_footer(&stdout, width, height)?;

    let drawing_height = height as usize - 2;

    loop {
        // Loop through all the lines that fit on the screen
        draw_doc(&stdout, &lines, current_line, drawing_height)?;

        match crossterm::event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => {
                if current_line > 0 {
                    current_line -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                state: _,
                modifiers: _,
            }) => {
                current_line = std::cmp::min(current_line + 1, lines.len() - drawing_height);
            }
            r => println!("{:?}", r),
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    stdout.execute(crossterm::cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}

fn draw_doc(
    mut stdout: &Stdout,
    lines: &PrintableLines,
    line: usize,
    drawing_height: usize,
) -> Result<(), anyhow::Error> {
    for y in 0..drawing_height {
        if line + y as usize >= lines.len() {
            break;
        }

        // Move the cursor to the start of the line
        stdout.queue(MoveTo(0, y as u16))?;

        // Print the line
        print_line(stdout, &lines[line + y as usize])?;
    }

    stdout.flush()?;

    Ok(())
}

fn print_line(mut stdout: &Stdout, line: &PrintableLine) -> Result<()> {
    let text = match line {
        PrintableLine::Text(text) | PrintableLine::Button(_, text) => text,
    };

    stdout
        .queue(Print(text))?
        .queue(Clear(ClearType::UntilNewLine))?;

    Ok(())
}

// Original contents
const DATA: &str = r#"
08                        ___ ___ ___________ ___ ___
08                       Y   Y   Y   _   _   Y   Y   |
08                       |   l   l___|   |___|   l   |
08                       l____   |   |   |   |   _   |
07                           |   |   |   |   |   |   |
07                           l___|   |   l   l___|   |
07                                   `---'       `---'
07      ______   ___ ___ ___ _______ ______  _______ ___ _______ ______
07     Y   _  \ Y   Y   Y   Y   _   Y   _  \Y   _   Y   Y   _   Y   _  \
07     |   |   \|   |       |   l___|   |   |   l___l   |   |   |   |   |
0F     |   |    \   |  \_/  |   __)_    |   l____   |   |   |   |   |   |
0F     |   l    /   |   |   |   l   |   |   |   l   |   |   l   |   |   |
0F     l_______/|   l___|   l_______l___|   l_______|   l_______l___|   l
0F              `---'   `---'           `---'       `---'           `---'
0F     _______ _______ _______ ___________ ___ ___ _______ _______ _______
07    Y   _   Y   _   Y   _   Y   _   _   Y   Y   Y   _   Y   _   \   _   Y
07    |   l___l   |   |   l___l___|   |___l   |   |   l   |   l   /   l___|
07    l____   |   |   |   __)     |   |   |  / \  |   _   |   _   l   __)_
08    |   l   |   l   |   |       |   |   |       |   |   |   |   |   l   |
08    |_______|_______l   |       |   l   l___l___l___|   l___|   l_______|
08                    `---'       `---'               `---'   `---'
0F           CatGen ~04v~093~04.~090 ~04(~0FC~04) ~0F1997 ~0E4th Dimension Software, ~0AMark Glenn
03þLINE1
03                             þBUTTON0250
04    INTRODUCTION
01    ------------
03
0F    CatGen ~04v~093~04.~090 ~03is a program to allow programmers of QB or QBASIC to have a
03    useful file to BAS converter.
03
03    Only compiling is required to create these catalogs.
03
04    REQUIREMENTS
01    ------------
02      1~09) ~0FColor Monitor
02      2~09) ~0FQB or QBasic
02      3~09) ~0FHard drive
02      4~09) ~0FQB for compilation
02      5~09) ~0FKeyboard
03
04    FEATURES included
01    -----------------
02      1~09) ~0FColor capability
02      2~09) ~0FSpeed (YEAH SPEEEEEED)
02      3~09) ~0F5000 line maximum
02      4~09) ~0FScrolling feature
02      5~09) ~0FConvert TEXT to BAS to be compiled
02      6~09) ~0FSearch Feature
02      7~09) ~0FMulti colored lines
02      8~09) ~0FBackground colors
02      9~09) ~0FNo flickering
02     10~09) ~0FOne file only (Doc inside BAS file)
02     11~09) ~0FMouse (even in QBasic)
02     12~09) ~0FScroll Bar
02     13~09) ~0FFast Code
02     14~09) ~0FButtons
02     15~09) ~0FPreprogrammed lines
03
04    TABLE OF CONTENTS
01    -----------------
02      1~09) ~0FWhat is CatGen v3.0
02      2~09) ~0FWhat is included
02      3~09) ~0FCommand line perimeters
02      4~09) ~0FWhat is it doing while running.
02      5~09) ~0FHow does it do all this
02      6~09) ~0FI've run it, now what?
02      7~09) ~0FColor
02      8~09) ~0FModifying
02      9~09) ~0FHow do I use the actual compiled work.
02     10~09) ~0FFinal touches
03
04    APPENDIXES
01    ----------
02      A~09) ~0FTrouble Shooting
02      B~09) ~0FUpcoming Features
02      C~09) ~0FFeatures of each version.
02      D~09) ~0FInformation about our company
02      E~09) ~0FOther programs
03
04  1~02) ~0FWhat is CatGen v3.0?
01     --------------------
0F     CatGen ~04v~093~04.~090 ~03is a program that takes your TEXT files and converts them
03  into self running file viewers.  Not only that, but it is in color.
03  Believe it or not, this file was created with ~0FCatGen ~04v~093~04.~090~03.  It's
03  capabilities are outrages.  It has absolutely no flicker and has incredible
03  speed.  It is based on 25x80 screen size and will look quite weird in 50x80.
03  It has scrolling features that will be used by using the up, down, PGUP, PGDN
03  keys.  The best feature though is that you can distribute your file.
03
04  2~02) ~0FWhat is included?
01     -----------------
03     CatGen comes with 9 files
02      1~04: ~0FCATGEN.EXE   ~03- The catalog generator
02      2~04: ~0FCATDOC.BAS   ~03- This file source code
02      3~04: ~0FCATDOC.EXE   ~03- This file
02      4~04: ~0FCATGEN.DOC   ~03- This file in TEXT format
02      5~04: ~0FORDER.FRM    ~03- Order and Registration file for other programs
02      6~04: ~0FFILE_ID.DIZ  ~03- File Info
02      7~04: ~0FCATALOG.EXE  ~03- Catalog for Inventive Software
02      8~04: ~0FANSI2CAT.EXE ~03- Ansi converter
02      9~04: ~0FFONTSET.TXT  ~03- ASCII Fonts
03
04  3~02) ~0FCommand line perimeters
01     -----------------------
03     There really is only one command line perimeter.
0F       : CATGEN Infile.ext (Outfile.ext)
03        The Infile.exe must be include.  Outfile.ext is something that is
03        optional.  If you decide not to use this, it will output to CAT1.OUT.
03
04  4~02) ~0FWhat is it doing while running?
01     -------------------------------
03     It does multiple things.  It first creates a temp file called TEMP1.TMP
03  that holds your DOC with the DATA statements in front of it.  Then it goes
03  in and loads all the BASIC statements to run it.  A file is created called
03  (filename) and you have a finished product ready to compile.
03
04  5~02) ~0FHow does it do all this?
01     ------------------------
03     It takes your file and process it and creates BASIC text.  Take a look at
03  the file sometime and see how it is really done.
03
04  6~02) ~0FI've run it, now what?
01     ----------------------
03     Okay, you have created your file and now it is time to modify.  Look at
03  Chapter 8 for that.  Chapter 7 shows how to change the color of each line.
03  After that, you want to compile it.  Load the program into QB by typing this:
0F    QB filename /L QB.QLB   or
0F    QBASIC filename
03  Now go to run and compile.  Make it Stand Alone EXE and click on Make EXE
03  and Exit.  If you only have qbasic, do the following.  Create a file called
03  filename.BAT and edit it.
0F  @ECHO OFF
0F  QBASIC filename /RUN
03
04  7~02) ~0FColor
01     -----
03     Wow, I can change color?  Yep, you can.  If you look at the end, you will
03  see that is says the DATA statement and then the numbers '03'.  This is the
03  color that it changes it all to.  You can change this number to anything.
03  It must be two digits long like:  03 or C5.  In 2.0, the numbers were
03  changed to hex so you can now do backgrounds.  The colors are as following:
00  00) BLACK       ~0808) GREY             ~2010) BLACK ON BLUE~03
01  01) BLUE        ~0909) LIGHT BLUE       ~EFEF) BRIGHT WHITE ON WHITE~03
02  02) GREEN       ~0A0A) LIGHT GREEN
03  03) CYAN        ~0B0B) LIGHT CYAN
04  04) RED         ~0C0C) LIGHT RED
05  05) MAGENTA     ~0D0D) LIGHT MAGENTA
06  06) BROWN       ~0E0E) YELLOW
07  07) WHITE       ~0F0F) BRIGHT WHITE
03
04  8~02) ~0FModifying
01     ---------
03     For people who want to make a big logo on the doc, I included a ANSI to
03  CATGEN converter.  All you do is add it to your doc and it will look exactly
03  like the ansi file.  (Sorry, no animations and only 25 lines long).
03    Also, you can add buttons and lines:
03                            þBUTTON0001
03    CHR$(254) + BUTTON0001  (4 digit line number)
03    þLINE1
03    CHR$(254) + LINE1 (1 digit line type)
03
03
04  9~02) ~0FHow do I use the actual compiled work?
01     --------------------------------------
03     Well, you first run the program.  You see the file and information at the
03  bottom.  You use the arrow keys or the PGUP or PGDN key.  These move the DOC
03  up or down.  The new feature is the ÿSÿ feature.  It searches the file for
03  whatever you want.
03
04 10~02) ~0FFinal Touches
01     -------------
03     Since I don't really know any final touches, but all great DOCs have this
03   area.  You can modify the generated file all you want.  The generated viewer
03   is freeware and maybe modified and distributed without permission.  Think of
03   it as your program you and only you made.
03
04  A~02) ~0FTrouble Shooting
01     ----------------
04     1) While compiling, it says an out of memory error
02       A:Try freeing more memory.  If you don't want to do this, if you look
02          at the begining of the file, it shows DIM statements.  Change this
02          to the length of your file.  If you don't know the length, look at
02          the first DATA statement.  It will say something like this:
02           DATA 253
02          Change the DIM statements to:
02           DIM text$(255) 'add 2 just in case
02           DIM ColorVal(255) 'add 2 just in case
02           You shouln't have to do this because it is automatic now.
04     2) While compiling, it says an error
02       A:Try trouble shooting it.  If that doesn't work, re-generate the file.
02          if that still doesn't work, contact me about it.
04     3) Doesn't generate
02       A:You have a corrupted CatGen v3.0.  Please reinstall.  You may also
02          have a bad output file.  DOS only allows certain characters in
02          filenames.  Try a filename with only letters:
02           CATGEN MY.DOC MYDOC.BAS
04     4) Bombs
02       A:You have a corrupted CatGen v3.0.  Please reinstall.
04     5) Subscript out of range
02       A:Your file is more than your memory can handle.  Cut it down.
04     6) Unexpected error, please recompile
02       A:For some odd reason, your DOC got edited so it can not be used.  You
02         might have edited it and messed it up.  If you know what you are
02         doing, take out the ON ERROR statement and try to debug it.  If not,
02         re compile it.  If the same error occurs, contact me.
04     7) I lost my cursor.
02       A:When the Help is invoked, it turns off the cursor.  It doesn't just
02         disappear.  Just turn it back on:
02         LOCATE , , 1, 7, 8
04     8) I don't have a mouse and start-up is sooo slow.
02        You can easily turn off the mouse.  Look for the lines that say
02        >>>>>> START HERE <<<<<<
04     9) I edit the basic file and it doesn't have any ÿ's in the DOC.  How
04        do I add them?
02       A:Well, since DATA statements are surrounded by ÿ's, all ÿ's in your
02         DOC were converted to character 255's.  To add ÿ's, go to the
02         position where you want them and hold down ALT.  Then press 255 on the
02         key pad and let go of ALT.  It is an invisible character, but it is
02         there.
03
04  B~02) ~0FUpcoming Features
01     -----------------
03        Editor
03        Multicolored search
03
04  C~02) ~0FFeatures of Each Version
01     ------------------------
03      v1.0b - The first official version.  Never distributed.
03
03      v1.0 - Added color feature.  PGUP and PGDN keys added.  Fixed
03             Arrow key bug.  Finally distributed
03
03      v1.1 - Search feature added.  Also added the ON ERROR function so
03             (Just in case) if it has an error it won't show that ugly error
03             screen.  Added the registration file(oops, sorry to everyone
03             with version 1.0).
03
03      v1.2 - Added 'S' at the bottom.  Fixed the scrolling up one line on
03             search bug.  Fixed bottom text on no search bug.  Man did I have
03             more trouble with that search feature than anything else.  Mainly
03             a bug fix version.
03
03      v1.3 - This would have been the one with EMS support but found it would
03             only work in QuickBasic without rewriting the whole code.  Maybe
03             later.
03
03      v1.4 - Added online help support.  With this new option, you won't see
03             all the ugly code because it has been put in SUBS.  Also I added
03             a change in the DIM statements.  No need to change them this time
03             because it automaticly changes them.  Also added limited edition
03             source code.
03
03      v2.0 - Limited edition source code taken away because of people without
03             respect.  Scrolls sideways.  Multiple colored lines.  Background
03             colors.  Improved search.  Now only supports HELP programs
03             because the two options were identical except HELP was cleaner.
03
0F      v3.0~03 - New and improved version.  CatGen is now 15x faster, cleaner code
03             and mouse/scroll bar/buttons.  Also includes preprogrammed lines
03             and buttons to press.  Better search, ANSI converter, and ASCII
03             fonts document.
03
04  D~02) ~0FInformation about our company
01     -----------------------------
03    As of the summer of 1996, our company changed its name to:
08            4~07t~0Fh Dimension Softwa~07r~08e
03    If you see any programs with the company Inventive software or 4th Dim.
03    then you can feel safe that they are from our company.
03
12    Please visit us at out web-site.
0D    http://4ds.simplenet.com/
03    and click on the big 4th Dimension image.
"#;
