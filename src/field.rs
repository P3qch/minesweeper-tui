use rand::Rng;


#[derive(Clone, Debug, PartialEq)]
pub enum CellType {
    Empty,
    Mine,
    Num(u8),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellState {
    Open,
    Closed,
    Flagged
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub cell: CellType,
    pub state: CellState,
}

pub struct Field {
    pub grid: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    pub num_of_flags_left: isize,
}

impl Field {
    pub fn new(width: usize, height: usize, num_of_mines: usize, start_row: usize, start_column: usize) -> Self {
        let mut result = Field {
            grid: vec![vec![Cell { cell: CellType::Empty, state: CellState::Closed }; width]; height],
            height,
            width,
            num_of_flags_left: num_of_mines as isize,

        };

        result.put_mines(num_of_mines);
        result.put_nums();
        let (row, column) = (start_row as isize, start_column as isize);

        for r in row-1..=row+1 {
            if r < 0 { continue; }
            if let Some(rt) = result.grid.clone().get((r) as usize) {
                for c in column-1..=column+1 {
                    if c < 0 { continue; }
                    if let Some(ct) = rt.get(c as usize) {
                        if ct.cell == CellType::Mine {
                            result.replace_mine(r as usize, c as usize);
                        }
                    }
                }
            }
        }
        

        result.open_at(start_row as isize, start_column as isize).unwrap();

        result
    }

    pub fn is_valid(&self, row: isize, column: isize) -> bool {
        row < self.height as isize && row >= 0 && column < self.width as isize && column >= 0
    }

    pub fn uncover_mines(&mut self) {
        for row in self.grid.iter_mut() {
            for elem in row.iter_mut() {
                if elem.cell == CellType::Mine {
                    elem.state = CellState::Open;
                }
            }
        }
    }


    pub fn flag_at(&mut self,row: usize, column: usize) {
        if self.grid[row][column].state == CellState::Closed {
            self.grid[row][column].state = CellState::Flagged;
            self.num_of_flags_left -= 1;
        } else if self.grid[row][column].state == CellState::Flagged {
            self.grid[row][column].state = CellState::Closed;
            self.num_of_flags_left += 1;
        }
    }

    pub fn open_at(&mut self, row: isize, column: isize) -> Result<(), ()> {
        if self.grid[row as usize][column  as usize].cell == CellType::Mine { return Err(()); }


        self.grid[row as usize][column as usize].state = CellState::Open;


        if  let CellType::Num(n) = self.grid[row as usize][column  as usize].cell { 
            if self.count_adjacent_flags(row, column) != n as usize { 
                return Ok(()); 
            } 
        }

        for r in row-1..=row+1 {
            if r < 0 { continue; }
            if let Some(rt) = self.grid.clone().get((r) as usize) {
                for c in column-1..=column+1 {
                    if c < 0 { continue; }
                    if let Some(ct) = rt.get(c as usize) {
                        if ct.cell == CellType::Empty && ct.state == CellState::Closed {
                            let mut result = false;
                            self.open_at(r, c).unwrap_or_else(|()| result = true);
                            if result {
                                return Err(());
                            }
                        }
                        if let CellType::Num(_) = ct.cell {
                            if ct.state == CellState::Closed {self.grid[r as usize][c as usize].state = CellState::Open;}
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn count_adjacent_flags(&self, row: isize, column: isize) -> usize {
        let mut num_of_flags_around = 0usize;

        for r in row-1..=row+1 {
            if r < 0 { continue; }
            if let Some(r) = self.grid.get((r) as usize) {
                for c in column-1..=column+1 {
                    if c < 0 { continue; }
                    if let Some(c) = r.get(c as usize) {
                        if c.state == CellState::Flagged {
                            num_of_flags_around += 1;
                        }
                    }
                }
            }
        }

        num_of_flags_around
    }

    pub fn game_over(&self) -> bool {
        self.num_of_mines_left() == 0 && self.num_of_closed_left() == 0 && self.num_of_flags_left == 0 
    }

    pub fn num_of_mines_left(&self) -> usize {
        let mut result: usize = 0;

        for row in &self.grid {
            result += row
                .iter()
                .filter(|cell| cell.cell == CellType::Mine && cell.state != CellState::Flagged)
                .count();
        }

        result
    }

    pub fn num_of_closed_left(&self) -> usize {
        let mut result: usize = 0;
        

        for row in &self.grid {
            result += row
                .iter()
                .filter(|cell| cell.state == CellState::Closed)
                .count();
        }

        result
    }

    fn put_mines(&mut self, num_of_mines: usize) {
        for _ in 0..num_of_mines {
            loop {
                
                let (x, y) = (rand::thread_rng().gen_range(0..self.width), rand::thread_rng().gen_range(0..self.height));

                if self.grid[y][x].cell == CellType::Empty {
                    self.grid[y][x].cell = CellType::Mine;
                    break;
                }
            }
        }
    }
    
    fn put_nums(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                self.set_num_for_square(row as isize, column as isize);
            }
        }
    }

    fn set_num_for_square(&mut self, row: isize, column: isize) {
        if self.grid[row as usize][column as usize].cell == CellType::Mine {
            return;
        }

        let mut num_of_mines_around: u8 = 0;

        for r in row-1..=row+1 {
            if r < 0 { continue; }
            if let Some(r) = self.grid.get((r) as usize) {
                for c in column-1..=column+1 {
                    if c < 0 { continue; }
                    if let Some(c) = r.get(c as usize) {
                        if c.cell == CellType::Mine {
                            num_of_mines_around += 1;
                        }
                    }
                }
            }
        }

        if num_of_mines_around == 0 {
            self.grid[row as usize][column as usize].cell = CellType::Empty;
        } else {
            self.grid[row as usize][column as usize].cell = CellType::Num(num_of_mines_around);
        }
    }

    fn replace_mine(&mut self, row: usize, column: usize) {
        self.grid[row][column].cell = CellType::Empty;
        

        'a: for row in self.grid.iter_mut() {
            for elem in row {
                if elem.cell == CellType::Empty {
                    elem.cell = CellType::Mine;
                    break 'a;
                }
            }
        }

        self.put_nums();
    }
}