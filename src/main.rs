#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  //Remove the console box.

use iced::{widget, window};

const CELL_COLUMNS: usize = 4;
const CELL_ROWS: usize = 4;

fn generate_rand(rng: &mut rand::rngs::ThreadRng) -> u8 {
  let rand_int: u8 = rand::Rng::gen_range(rng, 1..=255);

  match rand_int {
    1 => 128,
    2..=3 => 64,
    4..=7 => 32,
    8..=15 => 16,
    16..=31 => 8,
    32..=63 => 4,
    _ => 2,
  }
}


pub fn main() -> iced::Result {
  let settings = iced::Settings {
    window: window::Settings {
      size: (61 * CELL_COLUMNS as u32, 51 + 51 * CELL_ROWS as u32),
      resizable: false,
      ..Default::default()
    },
    ..Default::default()
  };
  <Game as iced::Sandbox>::run(settings)
}

#[derive(Clone, Copy, Debug)]
enum Message {
  NewGame,
  Press(usize, usize),
}

struct Game {
  grid: [[u8; CELL_ROWS]; CELL_COLUMNS],
  rng: rand::rngs::ThreadRng,
  next_num: u8,
}

fn get_padding(num: u8) -> [u16; 2] {
  if num < 10 {
    [0, 20]
  } else if num < 100 {
    [0, 10]
  } else {  //It's a star
    [0, 20]
  }
}

fn check_neighbors(grid: &mut [[u8; CELL_ROWS]; CELL_COLUMNS], x: usize, y: usize, num: u8) -> bool {
  let first_y = y == 0;
  let last_y = y == CELL_ROWS - 1;
  let first_x = x == 0;
  let last_x = x == CELL_COLUMNS - 1;
  
  let mut found = false;
  if !first_y && grid[x][y - 1] == num {
    grid[x][y - 1] = 0;
    found = true;
  }
  if !first_x && grid[x - 1][y] == num {
    grid[x - 1][y] = 0;
    found = true;
  }
  if !last_y && grid[x][y + 1] == num {
    grid[x][y + 1] = 0;
    found = true;
  }
  if !last_x && grid[x + 1][y] == num {
    grid[x + 1][y] = 0;
    found = true;
  }
  found
}

fn clear_block(grid: &mut [[u8; CELL_ROWS]; CELL_COLUMNS], x: usize, y: usize) {
  grid[x][y] = 0;
  if x > 0 { grid[x - 1][y] = 0; }
  if y > 0 { grid[x][y - 1] = 0; }
  if x > 0 && y > 0 { grid[x - 1][y - 1] = 0; }
  if x < CELL_COLUMNS { grid[x + 1][y] = 0; }
  if y < CELL_ROWS { grid[x][y + 1] = 0; }
  if x < CELL_COLUMNS && y < CELL_ROWS { grid[x + 1][y + 1] = 0; }
  if x > 0 && y < CELL_ROWS { grid[x - 1][y + 1] = 0; }
  if x < CELL_COLUMNS && y > 0 { grid[x + 1][y - 1] = 0; }
}

fn game_over(grid: &[[u8; CELL_ROWS]; CELL_COLUMNS]) -> bool {
  for y in 0..CELL_ROWS {
    for x in 0..CELL_COLUMNS {
      if grid[x][y] == 0 {
        return false;
      }
    }
  }
  true
}

impl iced::Sandbox for Game {
  type Message = Message;

  fn new() -> Self {
    let mut game = Game {
      grid: [[0; CELL_ROWS]; CELL_COLUMNS],
      rng: rand::thread_rng(),
      next_num: 0,
    };
    game.next_num = generate_rand(&mut game.rng);
    game
  }

  fn title(&self) -> String {
    String::from("Double to 128")
  }
  
  fn update(&mut self, message: Message) {
    match message {
      Message::NewGame => self.grid = [[0; CELL_ROWS]; CELL_COLUMNS],
      Message::Press(x, y) => {
        let mut new_num = self.next_num;
        for _ in 0..4 {
          if check_neighbors(&mut self.grid, x, y, new_num) {
            new_num *= 2;
          } else {
            break;
          }
        }
        if new_num == 128 {
          clear_block(&mut self.grid, x, y);
        } else {
          self.grid[x][y] = new_num;
        }
        self.next_num = generate_rand(&mut self.rng);
      },
    }
  }
  
  fn view(&self) -> iced::Element<Message> {
    let mut column = widget::Column::new().spacing(1).align_items(iced::Alignment::Center);
    let mut top_row = widget::Row::new().align_items(iced::Alignment::Center);
    
    let button = if game_over(&self.grid) {
      widget::Button::new(widget::Text::new("*").size(36)).padding([0, 20]).on_press(Message::NewGame)
    } else if self.next_num == 128 {
      widget::Button::new(widget::Text::new("*").size(36)).padding([0, 20])
    } else {
      widget::Button::new(widget::Text::new(self.next_num.to_string()).size(36)).padding(get_padding(self.next_num))
    }.width(60).height(50);
    top_row = top_row.push(button);
    
    column = column.push(top_row);
    
    for y in 0..CELL_ROWS {
      let mut row = widget::Row::new().spacing(1);
      for x in 0..CELL_COLUMNS {
        let button = if self.grid[x][y] == 0 {
          widget::Button::new("").width(60).height(50).on_press(Message::Press(x, y))
        } else {
          let num_text = if self.grid[x][y] == 128 {
            widget::Text::new("*")
          } else {
            widget::Text::new(self.grid[x][y].to_string())
          }.size(36);
          widget::Button::new(num_text).padding(get_padding(self.grid[x][y])).width(60).height(50)
        };
        row = row.push(button);
      }
      column = column.push(row);
    }
    column.into()
  }
}