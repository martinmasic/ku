use std::{
    io::{self, stdout},
    fmt::Debug,
    time::Duration,
    panic,
};

mod custom_widgets;
use custom_widgets::*;

use crate::{
    generator::*,
    game::*,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::{ Backend, CrosstermBackend },
    buffer::Buffer,
    crossterm::{
        execute,
        terminal::{
            disable_raw_mode,
            enable_raw_mode,
            EnterAlternateScreen,
            LeaveAlternateScreen,
        },
        ExecutableCommand
    },
    Frame,
    layout::{Constraint, Margin, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    Terminal,
    prelude::Direction,
    widgets::{Block, Borders, Paragraph, Widget},
};

use color_eyre::eyre::WrapErr;

// --------------------------------------------------------------------

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum PuzzleScreenState {
    #[default]
    Puzzle,
    Solution
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum RunningState {
    #[default]
    InitialScreen,
    PuzzleScreen(PuzzleScreenState),
    Done
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveDirection {
    Left, Right, Up, Down
}

#[derive(Debug, PartialEq, Eq)]
pub enum SolutionStatus {
    Valid,
    NotValid
}

#[derive(PartialEq, Eq)]
pub enum Message {
    GenerateNewPuzzle,
    Increment,
    Decrement,
    Quit,
    Reset,
    ShowSolution,
    EnterDigit(char),
    Move(MoveDirection),
    DeleteDigit,
    CheckSolution,
    ShowSolutionStatus(SolutionStatus),
}

#[derive(Debug)]
pub struct App {
    generator: NaiveGenerator,
    puzzle: Option<Board>,
    solution: Option<Board>,
    running_state: RunningState,
    position: Option<(usize, usize)>,
    solution_status: Option<bool>
}

impl App {
    pub fn new(generator: NaiveGenerator) -> Self {
        Self {
            generator: generator,
            puzzle: None,
            running_state: RunningState::InitialScreen,
            solution: None,
            position: None,
            solution_status: None
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> color_eyre::Result<()> {
        while self.running_state != RunningState::Done {
            terminal.draw(|f| self.view(f))?;

            let mut current_msg = self.handle_event()?;

            while current_msg.is_some() {
                current_msg = self.update(current_msg.unwrap());
            }
        }

        Ok(())
    }

    fn view(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_on_puzzle_screen(&self, key_event: KeyEvent) -> Option<Message> {
        match key_event.code {
            KeyCode::Char('n') => Some(Message::GenerateNewPuzzle),
            KeyCode::Char('+') => Some(Message::Increment),
            KeyCode::Char('-') => Some(Message::Decrement),
            KeyCode::Char('s') => Some(Message::ShowSolution),
            KeyCode::Char('r') => Some(Message::Reset),

            KeyCode::Char('h') => Some(Message::Move(MoveDirection::Left)),
            KeyCode::Char('l') => Some(Message::Move(MoveDirection::Right)),
            KeyCode::Char('k') => Some(Message::Move(MoveDirection::Up)),
            KeyCode::Char('j') => Some(Message::Move(MoveDirection::Down)),
            KeyCode::Char('x') => Some(Message::DeleteDigit),
            KeyCode::Enter => Some(Message::CheckSolution),

            KeyCode::Char(x) => {
                if let '1'..='9' = x { return Some(Message::EnterDigit(x)) }
                None
            },
            KeyCode::Left => Some(Message::Move(MoveDirection::Left)),
            KeyCode::Right => Some(Message::Move(MoveDirection::Right)),
            KeyCode::Up => Some(Message::Move(MoveDirection::Up)),
            KeyCode::Down => Some(Message::Move(MoveDirection::Down)),
            _ => None
        }
    }

    fn handle_key_on_initial_screen(&self, key_event: KeyEvent) -> Option<Message> {
        match key_event.code {
            KeyCode::Char('n') => Some(Message::GenerateNewPuzzle),
            _ => None,
        }
    }

    fn handle_key(&self, key_event: KeyEvent) -> Option<Message> {
        match key_event.code {
            KeyCode::Char('q') => Some(Message::Quit),
            _ => match self.running_state {
                    RunningState::PuzzleScreen(_) =>
                        self.handle_key_on_puzzle_screen(key_event),
                    RunningState::InitialScreen =>
                        self.handle_key_on_initial_screen(key_event),
                    _ => None
                },
        }
    }

    /// Handle events and emit messages
    fn handle_event(&mut self) -> color_eyre::Result<Option<Message>> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.kind {
                    KeyEventKind::Press => return Ok(self.handle_key(key)),
                    // KeyEventKind::Repeat => {
                    //     if let KeyCode::Char('s') = key.code {
                    //       return Ok(Some(Message::ShowSolution));
                    //     }
                    // },
                    // KeyEventKind::Release => {
                    //     if let KeyCode::Char('s') = key.code {
                    //       return Ok(Some(Message::ShowSolution));
                    //     }
                    // },
                    _ => {}

                }
            }
        }
        Ok(None)
    }

    fn move_left(&mut self) -> &Self {
        match self.position {
            None => self.position = Some((0, 0)),
            Some(pos) => self.position = Some((pos.0, (pos.1 + 9 - 1) % 9))
        }
        self
    }

    fn move_right(&mut self) -> &Self {
        match self.position {
            None => self.position = Some((0, 0)),
            Some(pos) => self.position = Some((pos.0, (pos.1 + 1) % 9))
        }
        self
    }

    fn move_up(&mut self) -> &Self {
        match self.position {
            None => self.position = Some((0, 0)),
            Some(pos) => self.position = Some(((pos.0 + 9 - 1) % 9, pos.1))
        }
        self
    }

    fn move_down(&mut self) -> &Self {
        match self.position {
            None => self.position = Some((0, 0)),
            Some(pos) => self.position = Some(((pos.0 + 1) % 9, pos.1))
        }
        self
    }

    fn enter_digit(&mut self, digit: char) -> &Self {
        if let Some(pos) = self.position &&
            let Some(ref mut puzzle) = self.puzzle
        {
            if let Cell::Given(_) = puzzle.values[pos.0][pos.1] {
                return self;
            }
            puzzle.set_non_given(Cell::NonGiven(digit), pos);
        }

        self
    }

    fn delete_non_given(&mut self) -> &Self {
        if let Some(pos) = self.position &&
            let Some(ref mut puzzle) = self.puzzle
        {
            if let Cell::Given(_) = puzzle.values[pos.0][pos.1] {
                return self;
            }
            puzzle.set_non_given(Cell::Empty, pos);
        }

        self
    }

    fn valid_solution(&mut self) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if self.puzzle.unwrap().values[r][c]
                    != self.solution.unwrap().values[r][c]
                {
                    return false;
                }
            }
        }

        true
    }

    /// Update App state according to message received
    fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Increment => {
                if self.generator.num_givens < 80 {
                    self.generator.num_givens += 1;
                    return Some(Message::GenerateNewPuzzle);
                }
            },
            Message::Decrement => {
                if self.generator.num_givens > 17 {
                    self.generator.num_givens -= 1;
                    return Some(Message::GenerateNewPuzzle);
                }
            },
            Message::Reset => {
                self.generator.num_givens = 39;
                return Some(Message::GenerateNewPuzzle);
            },
            Message::GenerateNewPuzzle => {
                let new_puzzle = self.generator.generate_puzzle();
                self.solution = Some(new_puzzle);

                let puzzle = new_puzzle.clone().non_givens_cleared();
                self.puzzle = Some(puzzle);

                self.running_state = RunningState::PuzzleScreen(PuzzleScreenState::Puzzle);

                self.solution_status = None;
            },
            Message::Quit => {
                self.running_state = RunningState::Done;
            },
            Message::ShowSolution => {
                match &self.running_state {
                    RunningState::PuzzleScreen(x) => {
                        match x {
                            PuzzleScreenState::Puzzle => {
                                self.running_state = RunningState::PuzzleScreen(
                                    PuzzleScreenState::Solution
                                );
                            },
                            PuzzleScreenState::Solution => {
                                self.running_state = RunningState::PuzzleScreen(
                                    PuzzleScreenState::Puzzle
                                );
                            },
                        }
                    }
                    _ => {},
                };
            },
            Message::Move(direction) => {
                match direction {
                    MoveDirection::Left => self.move_left(),
                    MoveDirection::Right => self.move_right(),
                    MoveDirection::Up => self.move_up(),
                    MoveDirection::Down => self.move_down(),
                };
            },
            Message::EnterDigit(digit) => {
                self.enter_digit(digit);
            },
            Message::DeleteDigit => {
                self.delete_non_given();
            },
            Message::CheckSolution => {
                if self.valid_solution() {
                    return Some(
                        Message::ShowSolutionStatus(SolutionStatus::Valid)
                    );
                } else {
                    return Some(
                        Message::ShowSolutionStatus(SolutionStatus::NotValid)
                    );
                }
            },
            Message::ShowSolutionStatus(status) => {
                match status {
                    SolutionStatus::Valid => self.solution_status = Some(true),
                    SolutionStatus::NotValid => self.solution_status = Some(false),
                }
            },
        }

        None
    }


    fn render_puzzle_screen(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" ku - The ultimate sudoku app, eventually ".bold());

        let instructions = Line::from(vec![
            " New puzzle ".into(), "<n>".blue().bold(),
            " Set givens ".into(), "<+/->".blue().bold(),
            " Move accross the board ".into(), "<↑→↓←> ".blue().bold(),
            " Reset ".into(), "<r>".blue().bold(),
            " Show solution ".into(), "<s> ".blue().bold(),
            " Quit ".into(), "<q> ".blue().bold(),
        ]);

        MainWindowFrameWidget::new(title, instructions).render(area, buf);
        let area = area.inner(Margin::new(1, 1));

        let screen_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(4),
                Constraint::Min(13)
            ])
            .split(area);

        let mut header_lines: Vec<Line> = Vec::new();
        header_lines.push(
            Line::from(vec![Span::raw(format!("{:?}", self.generator))])
        );

        match self.solution_status {
            None => {},
            Some(true) => {
                header_lines.push(
                    Line::from(vec![Span::raw(
                        format!("Solution is valid!")
                    )])
                );
            },
            Some(false) => {
                header_lines.push(
                    Line::from(vec![Span::raw(
                        format!("Solution is not valid!")
                    )])
                );
            },
        }

        // if self.position.is_some() {
        //     header_lines.push(
        //         Line::from(vec![Span::raw(
        //             format!(
        //                 "position: {}, {}",
        //                 self.position.unwrap().0,
        //                 self.position.unwrap().1,
        //             )
        //         )]));
        // }


        Paragraph::new(header_lines).render(screen_layout[0], buf);


        if self.puzzle.is_none() { return; }

        match self.running_state {
            RunningState::PuzzleScreen(PuzzleScreenState::Puzzle) =>
                PuzzleBoardWidget::new(&self.puzzle.as_ref().unwrap())
                    .set_position(self.position.clone())
                    .render(screen_layout[1], buf),
            RunningState::PuzzleScreen(PuzzleScreenState::Solution) =>
                FullBoardWidget::new(&self.solution.as_ref().unwrap())
                    .render(screen_layout[1], buf),
            _ => {},
        }
    }

    fn render_initial_screen(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" ku - The ultimate sudoku app, eventually ".bold());

        let instructions = Line::from(vec![
            " New puzzle ".into(), "<n>".blue().bold(),
            " Quit ".into(), "<q> ".blue().bold(),
        ]);

        MainWindowFrameWidget::new(title, instructions).render(area, buf);
        let area = area.inner(Margin::new(1, 1));

        let mut welcome_msg: Vec<Line> = Vec::new();
        welcome_msg.push(Line::from("Welcome tu **ku**, the ultimate sudoku app, eventually!"));
        welcome_msg.push(Line::from(""));
        welcome_msg.push(Line::from("This is a work in progress.\n"));
        welcome_msg.push(Line::from(""));
        welcome_msg.push(Line::from("Bugs abound."));
        welcome_msg.push(Line::from(""));
        welcome_msg.push(Line::from("Sky is falling."));
        welcome_msg.push(Line::from(""));
        welcome_msg.push(Line::from("Chaos reigns."));
        welcome_msg.push(Line::from(""));
        welcome_msg.push(Line::from("Enjoy :)"));
        let vertical_margin = (area.bottom() - area.top() - welcome_msg.len() as u16) / 2 ;

        Paragraph::new(welcome_msg)
            .centered()
            .render(
                area.inner(Margin { vertical: vertical_margin, horizontal: 0 }),
                buf
            );
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.running_state {
            RunningState::PuzzleScreen(_) => self.render_puzzle_screen(area, buf),
            RunningState::InitialScreen => self.render_initial_screen(area, buf),
            _ => {}
        };
    }
}



// -------------------------------------------------------------------------


pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

pub fn run() -> color_eyre::Result<()> {
    install_panic_hook();

    // setting up the terminal
    let mut terminal = init_terminal()?;
    let mut app = App::new(NaiveGenerator::new(0));

    let app_result = app.run(&mut terminal).wrap_err("run failed");
    if let Err(err) = restore_terminal() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {err}"
        );
    }

    restore_terminal()?;
    app_result
}

fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

pub fn restore_terminal() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
