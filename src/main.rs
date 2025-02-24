use std::collections::HashSet;
use std::fmt;
use std::time::SystemTime;
use std::vec::Vec;

const MAX_DIGITS: u32 = 9;

const MAX_FACTOR: u32 = 987_654_321;
// const MAX_FACTOR: u32 = 999_999_999;

struct Factor {
    base: u32,
    exponent: u32,
}

impl Factor {
    fn new(base: u32, exponent: u32) -> Factor {
        Factor { base, exponent }
    }
    fn value(&self) -> u32 {
        self.base.pow(self.exponent)
    }
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    Number(u8),
    Unknown,
}

#[derive(Clone, Copy)]
struct Row {
    // raw_value: Option<u32>,
    values: [Cell; 9],
}

impl Cell {
    fn from(number: u8) -> Cell {
        Cell::Number(number)
    }
}

impl Row {
    fn from(number: u32) -> Row {
        let mut number_str = number.to_string();
        // TODO: only add one zero and return Optional<Row> if len is < 9 with zero
        number_str = format!("000000000{}", number_str);
        let last_nine_digits_str = &number_str[number_str.len() - 9..];
        let cells: [Cell; 9] = last_nine_digits_str
            .chars()
            .map(|c| Cell::from(c.to_digit(10).unwrap() as u8))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Row {
            // raw_value: number,
            values: cells,
        }
    }
    fn is_valid(&self) -> bool {
        let set: HashSet<u8> = self
            .values
            .into_iter()
            .filter_map(|cell| match cell {
                Cell::Number(value) => Some(value),
                _ => None,
            })
            .collect();
        set.len() == self.values.len()
    }
    fn get_missing(&self) -> u8 {
        let all_numbers: HashSet<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
        let set: HashSet<u8> = self
            .values
            .into_iter()
            .filter_map(|cell| match cell {
                Cell::Number(value) => Some(value),
                _ => None,
            })
            .collect();
        let dif: Vec<u8> = all_numbers.difference(&set).cloned().collect();
        dif[0]
    }
    fn empty() -> Row {
        Row {
            values: [Cell::Unknown; 9],
        }
    }
    fn set(&mut self, col: usize, cell: Cell) {
        self.values[col] = cell
    }
    fn does_row_fit(&self, row: &Row) -> bool {
        self.values.into_iter().zip(row.values.into_iter()).fold(
            true,
            |acc, (current, requested)| match current {
                Cell::Unknown => acc,
                Cell::Number(value) => match requested {
                    Cell::Number(value2) => acc && value == value2,
                    _ => false,
                },
            },
        )
    }
}

#[derive(Clone, Copy)]
struct Grid {
    rows: [Row; 9],
}

impl Grid {
    fn empty() -> Grid {
        Grid {
            rows: [Row::empty(); 9],
        }
    }
    fn set(&mut self, row: usize, col: usize, cell: Cell) {
        self.rows[row].set(col, cell);
    }
    // fn does_row_fit(&self, row: Row) -> bool {
    //     true
    //
    // }
    fn solve_with(&mut self, rows: Vec<Row>) -> bool {
        let rows_clone = rows.clone();
        for new_row in rows {
            for row in self.rows {
                if row.does_row_fit(&new_row) {
                    rows_clone.index
                }
            }
        }
        false
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Cell::Unknown => ".".to_string(),
            Cell::Number(n) => n.to_string(),
        };
        write!(f, "{}", text)
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} {} {} {} {} {} {} {} {}",
            self.values[0],
            self.values[1],
            self.values[2],
            self.values[3],
            self.values[4],
            self.values[5],
            self.values[6],
            self.values[7],
            self.values[8]
        )
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            self.rows[0],
            self.rows[1],
            self.rows[2],
            self.rows[3],
            self.rows[4],
            self.rows[5],
            self.rows[6],
            self.rows[7],
            self.rows[8]
        )
    }
}

fn main() {
    let x = Factor::new(2, 5);
    println!("2^5 = {}", x.value());
    let mut initial_grid = Grid::empty();
    initial_grid.set(0, 7, Cell::Number(2));
    initial_grid.set(1, 8, Cell::Number(5));
    initial_grid.set(2, 1, Cell::Number(2));
    initial_grid.set(3, 2, Cell::Number(0));
    initial_grid.set(5, 3, Cell::Number(2));
    initial_grid.set(6, 4, Cell::Number(0));
    initial_grid.set(7, 5, Cell::Number(2));
    initial_grid.set(8, 6, Cell::Number(5));
    println!("{}", initial_grid);

    let start_time = SystemTime::now();
    let mut numbers_checked: u32 = 0;
    for candidate in (1..=MAX_FACTOR / 9).rev() {
        // check if a candidate is valid
        // candidate must be an odd number
        if candidate % 2 == 0 {
            continue;
        }

        let mut valid_rows: Vec<Row> = Vec::new();
        for i in 1..(MAX_FACTOR / candidate) + 1 {
            let n = candidate * i;
            if n > MAX_FACTOR {
                break;
            }
            let row = Row::from(n);
            if row.is_valid() {
                valid_rows.push(row)
            }
        }
        // let valid_rows: Vec<Row> = factors.into_iter().map(Row::from).filter(Row::isValid).collect();
        let mut groups: [Vec<Row>; 10] = Default::default();
        for row in valid_rows {
            let group = row.get_missing();
            groups[group as usize].push(row)
        }
        for group in groups {
            // for (group_index, group) in groups.iter().enumerate() {
            //     if group_index==0 || group_index == 5 || group_index ==2 {
            //         continue;
            //     }
            if initial_grid.clone().solve_with(group) {
                println!("not yet: {candidate}!");
            }
        }

        numbers_checked += 1;
        let speed = (numbers_checked as f32)
            / SystemTime::now()
                .duration_since(start_time)
                .unwrap()
                .as_secs_f32();
        if numbers_checked % 1000000 == 0 {
            println!(
                "Candidate {}: {} numbers checked at {}",
                candidate, numbers_checked, speed
            );
        }
    }
}
