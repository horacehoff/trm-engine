use std::{io::{stdout, Write}, thread, time};
use crossterm::{cursor::MoveTo, queue, style::{Color, SetForegroundColor, ResetColor}};

fn draw_to_terminal(row: u16, column: u16, color:Color) {
    if row > 50 || column > 150 {
        panic!("Invalid pixel position -> x: {}; y: {}", row, column)
    }
    queue!(
        stdout(),
        MoveTo(column, row),
        SetForegroundColor(color),
        crossterm::style::Print("█"),
        ResetColor
    ).expect("Failed to draw to terminal");

    stdout().flush().unwrap()
}

fn clear_terminal() {
    queue!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
}


fn main() {
    queue!(stdout(), crossterm::terminal::SetSize(150, 300)).unwrap();
    clear_terminal();
    // let now = time::Instant::now();
    // draw_to_terminal(10, 10);
    // println!("{}", now.elapsed().as_secs_f64());
    // draw_to_terminal(1, 1, Color::Red);
    // loop {

    // }
    let mut i = 0;

    loop {
        draw_to_terminal(i,i, Color::DarkGreen);
        i += 1;
        thread::sleep(time::Duration::from_millis(100));
    }
}
