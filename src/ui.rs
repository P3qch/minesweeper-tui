use std::io;
use std::time::Duration;

use crossterm::event::{poll, read, KeyCode, Event};

use tui::{
    Terminal,
    style::{Style, Color},
    backend::CrosstermBackend,
    layout::{Layout, Rect, Constraint, Direction, Alignment},
    widgets::{Clear, Block, Borders, Paragraph},
    text::{Span, Spans},
};


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
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.clear().unwrap();

        let mut run = true;

        let (mut current_x, mut current_y) = (0_usize,0_usize);
        while run {
            terminal.draw(| f | {
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
                                    crate::field::CellType::Num(n) => Span::styled(format!("[ {}  ]", n), color),
                                    crate::field::CellType::Mine => Span::styled("[    ]", color),
                                }
                            }
                        } );
                    }
                    text.push(Spans::from(row_vec));
                }

                let area = centered_rect(90, 60, size);
                let paragraph = Paragraph::new(text.clone())
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Center);

                f.render_widget(paragraph, area);


                if poll(Duration::from_millis(self.ticks)).unwrap() {
                    if let Event::Key(event) = read().unwrap() {
                        {
                            use KeyCode::*;
                            match event.code {
                            Char(c) => {
                                match c {
                                    'q' => run = false,
                                    'f' => self.field.flag_at(current_y, current_x),
                                    _ => ()
                                }
                            },
                            Enter => { self.field.open_at(current_y as isize, current_x as isize).unwrap(); },
                            Up => { if self.field.is_valid(current_y as isize - 1, 0) { current_y -= 1; } },
                            Down => { if self.field.is_valid(current_y as isize + 1, 0) { current_y += 1; } },
                            Left => { if self.field.is_valid(0, current_x as isize - 1) { current_x -= 1; } },
                            Right => { if self.field.is_valid(0, current_x as isize + 1) { current_x += 1; } },
                            _ => ()
                            }
                        }
                    }
                }

            }).unwrap();
        }
    }
}