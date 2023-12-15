use rand;
use rand::seq::SliceRandom; 

use iced::widget::{Text, row};
use iced::{executor, Application, Command, Length, Settings};
use iced::window;
use widget::{Row, Column, Renderer, Button, Container, Radio};

use self::theme::Theme;
use self::widget::Element;

const BUTTON_SIZE: u16 = 200;
const TEXT_SIZE: u16 = ((BUTTON_SIZE as f64) * 0.8) as u16;

fn main() {
    let settings = Settings {
        window: iced::window::Settings {
            resizable: true,
            size: (1200, 1200),
            ..Default::default()
        },
        ..Default::default()
    };

    TicTacToe::run(settings).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
    Human,
    AI,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Occupied(Player),
}

#[derive(Debug, Clone, Copy)]
struct  Cell {
    state: CellState,
    background: theme::Background,
    color: theme::Color
}

impl Default for Cell {
    fn default() -> Self {
        Cell { 
            state: CellState::Empty, 
            background: theme::Background::default(),
            color: theme::Color::default() 
        }
    }
}


#[derive(Debug, Clone)]
struct  Board {
    cells: [Cell; 9],
}

impl Default for Board {
    fn default() -> Self {
        Board { 
            cells: [Cell::default(); 9], 
        }
    }
}

impl Board {
    fn make_move(&mut self, position: usize, player: Player) {
        self.cells[position].state = CellState::Occupied(player);
    }

    fn available_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, &cell)| if cell.state == CellState::Empty { Some(i) } else { None })
            .collect()
    }

    #[allow(dead_code)]    
    fn ai_played_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, &cell)| if cell.state == CellState::Occupied(Player::AI) { Some(i) } else { None })
            .collect()
    }

    fn hu_played_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, &cell)| if cell.state == CellState::Occupied(Player::Human) { Some(i) } else { None })
            .collect()
    }

    fn check_win(&self, player: Player) -> bool {
        let win_combos = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [6, 4, 2],
        ];

        win_combos
            .iter()
            .any(|&combo| combo.iter().all(|&pos| self.cells[pos].state == CellState::Occupied(player)))
    }


    
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Mode {
    OnePlayer,
    #[default]   
    TwoPlayers,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Level {
    #[default]
    Easy,
    Medium,
    Hard,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Status {
    #[default]
    Playing,
    Draw,
    Winner
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct  Game {
    mode: Mode,
    level: Option<Level>,
    status: Status,
    playing_count: usize
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            mode: Mode::TwoPlayers, 
            level: Some(Level::Easy), 
            status: Default::default(), 
            playing_count: Default::default() 
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Move {
    score: i32,
    index: usize,
}


#[derive(Debug, Clone)]
struct TicTacToe {
    board: Board,
    game: Game,
    message: String,
    player: Player,
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe {
            board: Board::default(),
            game: Game::default(),
            message: "X turn.".to_string(),
            player: Player::X,
        }
    }
}

impl TicTacToe {    
    fn  check_for_winner(&mut self) -> bool {
        if self.board.check_win(self.player) {
            self.message = format!("{} is the winner!!!!", match self.player {
                Player::X => "X",
                Player::O => "O",
                Player::AI => "AI",
                Player::Human => "Human",
            });
    
            self.game.status = Status::Winner;
            return true;
        }

        if self.game.playing_count == 9 {
            self.message = "We have a draw.".to_string();
            self.game.status = Status::Draw;
            return true
        }

        self.message = format!("{} turn.", match self.player {
            Player::X => "O",
            Player::O => "X",
            Player::AI => "AI",
            Player::Human => "Human",
            
        });

        return  false;

        
    }

    fn make_ai_move(&mut self, index: usize) {

        if self.board.cells[index].state == CellState::Empty && Status::Playing == self.game.status {
            self.board.cells[index].color = theme::Color::AI;             
            self.board.cells[index].state = CellState::Occupied(Player::AI);
            self.game.playing_count += 1;
            
            if !self.check_for_winner() {
                self.player = Player::Human;
                self.check_for_winner();
    
            }
        }
    }

    fn button_handler(&mut self, index: usize) {
        if self.board.cells[index].state == CellState::Empty && Status::Playing == self.game.status {
            let cell_state = match self.player {
                Player::X => { 
                    self.board.cells[index].color = theme::Color::PlayerX;
                    CellState::Occupied(Player::X) 
                },
                Player::O => { 
                    self.board.cells[index].color = theme::Color::PlayerO;
                    CellState::Occupied(Player::O) 
                },
                Player::AI => { 
                    self.board.cells[index].color = theme::Color::AI;
                    CellState::Occupied(Player::AI) 
                },
                Player::Human => { 
                    self.board.cells[index].color = theme::Color::Human;
                    CellState::Occupied(Player::Human) 
                },
            };
            self.board.cells[index].state = cell_state;
            self.game.playing_count += 1;

            self.check_for_winner();
            self.player = match self.player {
                Player::X => Player::O,
                Player::O => Player::X,
                Player::AI => Player::Human,
                Player::Human => Player::AI,
            };

            if self.game.mode == Mode::OnePlayer && self.game.status == Status::Playing {
                let ai_move: usize;
                match self.game.level  {
                    Some(Level::Easy) => {
                        ai_move = Self::free_spot(&self.board);                        
                    }
                    Some(Level::Medium) => {
                        ai_move = Self::closest_spot(&self.board, &self.game);                        
                    }
                    Some(Level::Hard) => {
                        ai_move = Self::best_spot(&self.board);                        
                    }
                    _ => todo!(),
                }

                self.make_ai_move(ai_move);

            }

            
        }
    }

    fn free_spot(board: &Board) -> usize {
        let available_spots = board.available_moves();
        let free_spot: Vec<_> = available_spots
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();
        *free_spot[0]        
    }

    fn play_block(board: &Board) -> Option<usize> {
        let win_combos = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [6, 4, 2],
        ];
    
        let played_two = win_combos.iter().position(|combo| {
            combo.iter().filter(|&&pos| board.cells[pos].state == CellState::Occupied(Player::Human)).count() == 2
        });
    
        let result = match played_two {
            Some(0..=8) => win_combos[played_two.unwrap()]
                .iter()
                .find(|&&pos| board.cells[pos].state == CellState::Empty)
                .cloned(),
            _ => None,
        };
        
        result
    }
    

    fn closest_spot(board: &Board, game: &Game) -> usize {
        let available_spots = board.available_moves();
        let hu_played_spots = board.hu_played_moves();
        let play_block = Self::play_block(board);

        if play_block != None {
            return play_block.unwrap();
        }

        if hu_played_spots.contains(&4) && game.playing_count == 1 {
                let free_spot: Vec<_> =  [0, 2, 6, 8].choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
                return *free_spot[0];

        }
        if hu_played_spots.contains(&4) && game.playing_count == 3 {
            if hu_played_spots.contains(&2) {
                return  6;
            } else if hu_played_spots.contains(&6){
                return  2;
            } else if hu_played_spots.contains(&0){
                return  8;
                
            } else if hu_played_spots.contains(&8){
                return  0;
                
            }
        }
        if !hu_played_spots.contains(&4) && game.playing_count == 1 {
            return  4;
        }
        if !hu_played_spots.contains(&4) && game.playing_count == 3 {
            if !hu_played_spots.contains(&1) {
                return  1;
            } else if !hu_played_spots.contains(&7) {
                return  7;
            } else if !hu_played_spots.contains(&3) {
                return  3;
            } else if !hu_played_spots.contains(&5) {
                return  5;
            } 
        }
        let free_spot: Vec<_> = available_spots
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();
        *free_spot[0]        
    }


    fn best_spot(board: &Board) -> usize {
        TicTacToe::minimax(board, Player::AI).index
    }
    
    fn minimax(board: &Board, player: Player) -> Move {
        let available_spots = board.available_moves();
    
        if board.check_win(Player::Human) {
            return Move { score: -10, index: 0 };
        } else if board.check_win(Player::AI) {
            return Move { score: 10, index: 0 };
        } else if available_spots.is_empty() {
            return Move { score: 0, index: 0 };
        }
    
        let mut moves = Vec::new();
    
        for &spot in &available_spots {
            let mut new_board = board.clone();
            new_board.make_move(spot, player);
    
            let result = Self::minimax(&new_board, match player {
                Player::AI => Player::Human,
                Player::Human => Player::AI,
                _ => todo!(),
            });
    
            moves.push(Move { score: result.score, index: spot });
        }
    
        if player == Player::AI {
            let best_move = moves.iter().max_by_key(|&&m| m.score).unwrap();
            *best_move
        } else {
            let best_move = moves.iter().min_by_key(|&&m| m.score).unwrap();
            *best_move
        }
    }
    
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPress(usize),
    Restart,
    LevelChanged(Level),
    ModeChanged(Mode),
}

impl Application for TicTacToe {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (TicTacToe::default(), window::change_mode(iced::window::Mode::Fullscreen))
    }

    fn title(&self) -> String {
        "Tic Tac Toe with Iced".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Restart => {
                let mode = self.game.mode.clone();
                let level = self.game.level.clone();
                *self = TicTacToe::default();
                self.game.mode = mode;
                self.game.level = level;
                if self.game.mode == Mode::OnePlayer {
                    self.player = Player::Human;
                }
                self.check_for_winner();
            }
            Message::ButtonPress(index) => {
                self.button_handler(index);
            }
            Message::ModeChanged(mode) => {
                *self = TicTacToe::default();
                self.player = Player::Human;
                self.message = format!("{} turn.", "Human".to_string());
                self.game.mode = mode;
            }
            Message::LevelChanged(level) => {
                let mode = self.game.mode.clone();
                *self = TicTacToe::default();
                self.game.mode = mode;
                self.player = Player::Human;
                self.message = format!("{} turn.", "Human".to_string());
                self.game.level = Some(level);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let board_button = |state: &CellState, index: usize| -> Button<'_, Message, Renderer> {
            let bt_text = match state {
                CellState::Occupied(Player::X) => "X",
                CellState::Occupied(Player::O) => "O",
                CellState::Occupied(Player::AI) => "O",
                CellState::Occupied(Player::Human) => "X",
                CellState::Empty => "",
            };
    
            Button::new(
                Text::new(bt_text)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center)
                    .size(TEXT_SIZE)
                    .style(self.board.cells[index].color),
            )
            .width(BUTTON_SIZE)
            .height(BUTTON_SIZE)
            .style(self.board.cells[index].background)
            .on_press(Message::ButtonPress(index))
        };
    
        let restart_button = Button::new(
            Text::new("Restart")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
                .size(TEXT_SIZE),
        )
        .width(620)
        .height(BUTTON_SIZE)
        .on_press(Message::Restart);
    
        let message = Text::new(&self.message)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .size(30);
    
        let board: Vec<Element<Message>> = self.board.cells.chunks_exact(3).enumerate().map(|(row, row_board)| {
            let row_buttons: Vec<Element<Message>> = row_board.iter().enumerate().map(|(col, cell_state)| {
                let button = board_button(&cell_state.state, (row * 3 + col + 1) - 1);
                button.into() // Convert Button to Element
            }).collect();
    
            Row::new()
                .spacing(10)
                .align_items(iced::Alignment::Center)
                .push(Row::with_children(row_buttons)) // Wrap the buttons in another Row
                .into() // Convert Row to Element
        }).collect();

        let mode: Row<'_, Message, Renderer>  =
            [Mode::OnePlayer, Mode::TwoPlayers]
                .iter()
                .fold(
                    row![Text::new("Mode:")].spacing(10),
                    |mode, mode_sel| {
                        mode.push(Radio::new(
                            format!("{mode_sel:?}"),
                            *mode_sel,
                            Some(match self.game.mode {
                                Mode::OnePlayer => Mode::OnePlayer,
                                Mode::TwoPlayers => Mode::TwoPlayers,
                            }),
                            Message::ModeChanged,
                        ))
                    },
                );



        let level: Row<'_, Message, Renderer>  =
            [Level::Easy, Level::Medium, Level::Hard]
                .iter()
                .fold(
                    row![Text::new("Level:")].spacing(10),
                    |level, level_sel| {
                        level.push(Radio::new(
                            format!("{level_sel:?}"),
                            *level_sel,
                            Some(match self.game.level {
                                Some(Level::Easy) => Level::Easy,
                                Some(Level::Medium) => Level::Medium,
                                Some(Level::Hard) => Level::Hard,
                                None => Level::Easy
                            }),
                            Message::LevelChanged,
                        ))
                    },
                );

    
        let content = Column::new()
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .push(message)
            .push(Column::with_children(board)) // Convert Vec<Element> to a single widget element
            .push(mode)
            .push(if self.game.mode == Mode::OnePlayer { level } else {Row::new()
                .spacing(10)
                .align_items(iced::Alignment::Center)
                .into() })
            .push(restart_button);
    
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    

}

mod widget {
    #![allow(dead_code)]
    use crate::theme::Theme;

    pub type Renderer = iced::Renderer<Theme>;
    pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;
    pub type Container<'a, Message> = iced::widget::Container<'a, Message, Renderer>;
    pub type Button<'a, Message, Renderer> = iced::widget::Button<'a, Message, Renderer>;
    pub type Column<'a, Message, Renderer> = iced::widget::Column<'a, Message, Renderer>;
    pub type Row<'a, Message, Renderer> = iced::widget::Row<'a, Message, Renderer>;
    pub type Radio<'a, Message, Renderer> = iced::widget::Radio<Message, Renderer>;
}

mod theme {
    use iced::widget::{button, container, text, radio};
    use iced::{application, color, Background as Theme_Background, BorderRadius, Color as Theme_Color};

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Theme;

    impl application::StyleSheet for Theme {
        type Style = ();

        fn appearance(&self, _style: &Self::Style) -> application::Appearance {
            application::Appearance {
                background_color: color!(0x28, 0x28, 0x28),
                text_color: color!(0xeb, 0xdb, 0xb2),
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Color {
        #[default]
        Primary,
        PlayerX,
        PlayerO,
        Human,
        AI,
        #[allow(dead_code)]
        Winner
    }

    impl text::StyleSheet for Theme {
        type Style = Color;

        fn appearance(&self, style: Self::Style) -> text::Appearance {
            match style {
                Color::Primary => text::Appearance {color: color!(0xeb, 0xdb, 0xb2).into()},
                Color::PlayerX => text::Appearance {color: Some(Theme_Color::from_rgb(250.0, 0.0, 0.0))},
                Color::PlayerO => text::Appearance {color: Some(Theme_Color::from_rgb(0.0, 51.0, 0.0))},
                Color::Human => text::Appearance {color: Some(Theme_Color::from_rgb(250.0, 0.0, 0.0))},
                Color::AI => text::Appearance {color: Some(Theme_Color::from_rgb(0.0, 51.0, 0.0))},
                Color::Winner => text::Appearance {color: color!(0xeb, 0xdb, 0xb2).into()},            
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Container {
        #[default]
        Default,
        #[allow(dead_code)]
        Bordered,
    }

    impl container::StyleSheet for Theme {
        type Style = Container;

        fn appearance(&self, style: &Self::Style) -> container::Appearance {
            match style {
                Container::Default => container::Appearance::default(),
                Container::Bordered => container::Appearance {
                    border_color: color!(0x45, 0x85, 0x88),
                    border_width: 1.0,
                    border_radius: BorderRadius::from(4.0),
                    ..Default::default()
                },
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Background {
        #[default]
        Primary,
        #[allow(dead_code)]
        Secondary,
    }

    impl button::StyleSheet for Theme {
        type Style = Background;
        fn active(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Background::Primary => button::Appearance {
                    border_radius: BorderRadius::from(4.0),
                    border_width: 1.0,
                    border_color: color!(0x45, 0x85, 0x88),
                    ..Default::default()
                },
                Background::Secondary => button::Appearance {
                    border_radius: BorderRadius::from(4.0),
                    border_width: 1.0,
                    border_color: color!(0x45, 0x85, 0x88),
                    background: Some(Theme_Background::from(Theme_Color::from_rgb(60.0, 56.0, 54.0))),
                    ..Default::default()
                },
            }
        }
        fn pressed(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Background::Primary => button::Appearance {
                    background: Some(Theme_Background::from(Theme_Color::from_rgb(255.0, 255.0, 255.0))),
                    border_radius: BorderRadius::from(4.0),
                    border_width: 1.0,
                    border_color: color!(0x45, 0x85, 0x88),
                    ..Default::default()
                },
                Background::Secondary => button::Appearance {
                    background: Some(Theme_Background::from(Theme_Color::from_rgb(60.0, 56.0, 54.0))),
                    border_radius: BorderRadius::from(4.0),
                    border_width: 1.0,
                    border_color: color!(0x45, 0x85, 0x88),
                    ..Default::default()
                },
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Radio {
        #[default]
        Primary,
        #[allow(dead_code)]
        Secondary,
    }

    impl radio::StyleSheet for Theme {
        type Style = Radio;

        fn active(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
            radio::Appearance { 
                background: iced::Color::TRANSPARENT.into(),
                dot_color: color!(0xeb, 0xdb, 0xb2).into(),
                border_width: 1.0,
                border_color: color!(0xeb, 0xdb, 0xb2).into(),
                text_color: None,                
             }
        }

        fn hovered(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
            radio::Appearance { 
                background: iced::Color::TRANSPARENT.into(),
                dot_color: color!(0xeb, 0xdb, 0xb2).into(),
                border_width: 1.0,
                border_color: color!(0xeb, 0xdb, 0xb2).into(),
                text_color: None,                
             }
        }



    }

}
