use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn select<'a, T: std::fmt::Display>(prompt: &str, list: &'a [T]) -> Option<&'a T> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let last_item_id = list.len() - 1;

    write!(
        stdout,
        "{} ? {}{}{}{}\n\r",
        termion::color::Fg(termion::color::Green),
        termion::style::Reset,
        termion::style::Bold,
        prompt,
        termion::style::Reset
    )
    .unwrap();

    let mut curr = 0;
    let mut input = stdin.keys();

    loop {
        for (i, s) in list.iter().enumerate() {
            write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

            if curr == i {
                write!(
                    stdout,
                    "{}{} {} {}{}",
                    termion::style::Bold,
                    termion::color::Fg(termion::color::Cyan),
                    figures_rs::POINTER,
                    s,
                    termion::style::Reset
                )
                .unwrap();
            } else {
                write!(stdout, "   {}", s).unwrap();
            }

            write!(stdout, "\n\r").unwrap();
        }
        stdout.flush().unwrap();

        let next = input.next().unwrap();

        match next.unwrap() {
            Key::Char('\n') | Key::Char('l') => {
                write!(stdout, "\n\r{}", termion::cursor::Show).unwrap();
                return list.get(curr);
            }
            Key::Up | Key::Char('k') => {
                if curr == 0 {
                    curr = last_item_id;
                } else {
                    curr -= 1;
                }
            }
            Key::Down | Key::Char('j') => {
                if curr == last_item_id {
                    curr = 0;
                } else {
                    curr += 1;
                }
            }
            Key::Ctrl('c') | Key::Char('q') => {
                write!(stdout, "\n\r{}", termion::cursor::Show).unwrap();
                return None;
            }
            _ => {}
        };

        print!("{}", termion::cursor::Up(list.len() as u16));
    }
}

pub fn checkbox<'a, T: std::fmt::Display>(
    prompt: &'a str,
    list: &'a [T],
) -> Option<Vec<(&'a T, bool)>> {
    let mut list: Vec<(&T, bool)> = list.iter().map(|item| (item, false)).collect();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let last_item_id = list.len() - 1;

    write!(
        stdout,
        "{} ? {}{}{}{}\n\r",
        termion::color::Fg(termion::color::Green),
        termion::style::Reset,
        termion::style::Bold,
        prompt,
        termion::style::Reset
    )
    .unwrap();

    let mut curr = 0;
    let mut input = stdin.keys();

    fn print_check<'a, T>(s: &(&T, bool)) -> &'a str {
        if s.1 {
            figures_rs::CIRCLE_FILLED
        } else {
            figures_rs::CIRCLE
        }
    }

    loop {
        for (i, s) in list.iter_mut().enumerate() {
            write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

            if curr == i {
                write!(
                    stdout,
                    "{}{} {} {} {}{}",
                    termion::style::Bold,
                    termion::color::Fg(termion::color::Cyan),
                    figures_rs::POINTER,
                    print_check(s),
                    s.0,
                    termion::style::Reset
                )
                .unwrap();
            } else {
                write!(stdout, "   {} {}", print_check(s), s.0).unwrap();
            }

            write!(stdout, "\n\r").unwrap();
        }
        stdout.flush().unwrap();

        let next = input.next().unwrap();

        match next.unwrap() {
            Key::Char('\n') | Key::Char('l') => {
                write!(stdout, "\n\r{}", termion::cursor::Show).unwrap();
                return Some(list);
            }
            Key::Char(' ') => {
                list[curr].1 = !list[curr].1;
            }
            Key::Up | Key::Char('k') => {
                if curr == 0 {
                    curr = last_item_id;
                } else {
                    curr -= 1;
                }
            }
            Key::Down | Key::Char('j') => {
                if curr == last_item_id {
                    curr = 0;
                } else {
                    curr += 1;
                }
            }
            Key::Ctrl('c') | Key::Char('q') => {
                write!(stdout, "\n\r{}", termion::cursor::Show).unwrap();
                return None;
            }
            _ => {}
        };

        print!("{}", termion::cursor::Up(list.len() as u16));
    }
}
