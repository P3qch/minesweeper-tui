/* 
MIT License

Copyright (c) 2021 P3qch

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::io;
use std::time::{SystemTime, Duration};


use crossterm::{
    event::{poll, read, KeyCode, Event},
    terminal::{enable_raw_mode, disable_raw_mode}
};

use tui::{
    Terminal,
    style::{Style, Color},
    backend::CrosstermBackend,
    layout::{Layout, Rect, Constraint, Direction, Alignment},
    widgets::{Clear, Block, Borders, Paragraph},
    text::{Span, Spans},
};


fn human_time(mut secs: u64) -> String {
    let hours = secs / 3600;
    secs %= 3600;
    let minutes = secs / 60;
    secs %= 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn color_for_number(n: u8) -> Color {
    use Color::*;
    match n {
        1 => Rgb(2, 0, 253),
        2 => Rgb(1, 126, 0),
        3 => Rgb(254, 0, 1),
        4 => Rgb(1, 1, 128),
        5 => Rgb(127, 3, 0),
        6 => Rgb(0, 128, 128),
        7 => Rgb(0, 0, 0),
        8 => Rgb(128, 128, 128),

        _ => Color::Reset,
    }
}

pub struct GameUI {
    pub field: crate::field::Field,
    ticks: u64,
}

impl GameUI {
    pub fn new(field: crate::field::Field,  ticks: u64) -> Self {
        GameUI {
            field,
            ticks,
        }
    }
    pub fn game_loop(&mut self) {
        let stdout = io::stdout();

        enable_raw_mode().unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.clear().unwrap();

        let mut run = true;
        let mut lost=  false;
        let mut emoji = '😃';

        let mut started_game = false;
        let mut timer = SystemTime::now();
        let mut time_result = 0;
        let (mut current_x, mut current_y) = (0_usize,0_usize);
        while run  {
            terminal.draw(| f | {
                //println!("{}", self.field.game_over());
                if self.field.game_over() && !lost { 
                    lost = true; 
                    started_game = false;
                    emoji = '😎';
                    time_result = timer.elapsed().unwrap().as_secs();
                }
                let size = f.size();
                f.render_widget(Clear, size);

                let default_style = Style::default()
                    .bg(Color::Gray)
                    .fg(Color::Black);
                let highlighterd_style = Style::default()
                    .bg(Color::Blue)
                    .fg(Color::Black);

                let mut text = vec![];
    
                for (y, row) in self.field.grid.iter().enumerate() {
                    let mut row_vec: Vec<Span> = vec![];
                    for (x, elem) in row.iter().enumerate() {
                        let color = if y == current_y && x == current_x { highlighterd_style } else { default_style };
                        row_vec.push( match elem.state { 
                            crate::field::CellState::Closed => Span::styled( "[ #  ]", color),
                            crate::field::CellState::Flagged => Span::styled("[ 🚩 ]", color),
                            crate::field::CellState::Open => {
                                match elem.cell {
                                    crate::field::CellType::Empty => Span::styled("[ .  ]", color),
                                    crate::field::CellType::Num(n) => Span::styled(format!("[ {}  ]", n), color.fg(color_for_number(n))),
                                    crate::field::CellType::Mine => Span::styled("[ 💣 ]", color),
                                }
                            }
                        } );
                    }
                    text.push(Spans::from(row_vec));
                }
                text.push(Spans::from(""));
                text.push(Spans::from("f - flag"));
                text.push(Spans::from("Enter - open cell"));
                text.push(Spans::from("Up/Down/Left/Right - navigation"));
                text.push(Spans::from("q - exit"));

                let area = centered_rect(90, 60, size);
                let paragraph = Paragraph::new(text.clone())
                    .block(Block::default())
                    .alignment(Alignment::Center);

                f.render_widget(paragraph, area);


                let text = vec![
                    Spans::from(vec![Span::raw(format!("       Flags used: {}  {}  Closed cells left: {}",self.field.num_of_flags_left, emoji, self.field.num_of_closed_left() ))]),
                    Spans::from(vec![Span::raw(if started_game  {human_time(timer.elapsed().unwrap().as_secs())}  else { human_time(time_result) })]),
                ];

                let paragraph = Paragraph::new(text.clone())
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Center);

                f.render_widget(paragraph, size);
                if poll(Duration::from_millis(self.ticks)).unwrap() {
                    if !lost {
                        if let Event::Key(event) = read().unwrap() {
                            {   
                                use KeyCode::*;
                                if event.code != Null && !started_game {started_game = true; timer = SystemTime::now();}

                                match event.code {
                                    Char(c) => {
                                        match c {
                                            'q' => run = false,
                                            'f' => self.field.flag_at(current_y, current_x),
                                            _ => ()
                                        }
                                    },
                                    Enter => { self.field.open_at(current_y as isize, current_x as isize).unwrap_or_else(|()| { 
                                        self.field.uncover_mines(); 
                                        lost = true; 
                                        started_game = false;
                                        time_result = timer.elapsed().unwrap().as_secs();
                                        emoji = '🙁';
                                    }); },
                                    Up => { if self.field.is_valid(current_y as isize - 1, 0) { current_y -= 1; } },
                                    Down => { if self.field.is_valid(current_y as isize + 1, 0) { current_y += 1; } },
                                    Left => { if self.field.is_valid(0, current_x as isize - 1) { current_x -= 1; } },
                                    Right => { if self.field.is_valid(0, current_x as isize + 1) { current_x += 1; } },
                                    _ => ()
                                }
                            }
                        }
                    } else if let Event::Key(event) = read().unwrap() {
                        if let KeyCode::Char(_) = event.code  {
                            run = false;
                        }
                    }
                    
                }

            }).unwrap();
        }

        terminal.clear().unwrap();
        disable_raw_mode().unwrap();
    }
}