use std::{
    io::{self, stdout, Stdout},
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
    DefaultTerminal,
    Frame,
    layout::{Rect, Layout, Constraint},
    style::Stylize,
    symbols::border,
    text::{Line, Text, Span},
    Terminal,
    prelude::Direction,
    widgets::{Block, Paragraph, Widget, Borders, Table, Row},
};

use color_eyre::eyre::WrapErr;

// --------------------------------------------------------------------

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum RunningState {
    #[default]
    Initial,
    GeneratorPuzzle,
    GeneratorSolution,
    Done
}

#[derive(PartialEq)]
pub enum Message {
    GenerateNewPuzzle,
    Increment,
    Decrement,
    Quit,
    Reset,
    ShowSolution,
}

#[derive(Debug)]
pub struct App {
    generator: NaiveGenerator,
    puzzle: Option<Board>,
    app_state: RunningState,
}

impl App {
    pub fn new(generator: NaiveGenerator) -> Self {
        Self {
            generator: generator,
            puzzle: None,
            app_state: RunningState::Initial
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key(&self, key_event: KeyEvent) -> Option<Message> {
        match key_event.code {
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('n') => Some(Message::GenerateNewPuzzle),
            KeyCode::Char('+') => Some(Message::Increment),
            KeyCode::Char('-') => Some(Message::Decrement),
            KeyCode::Char('s') => Some(Message::ShowSolution),
            KeyCode::Char('r') => Some(Message::Reset),
            _ => None,
        }
    }

    fn handle_event(&mut self) -> color_eyre::Result<Option<Message>> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    return Ok(self.handle_key(key));
                }
            }
        }
        Ok(None)
    }

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
                self.generator.num_givens = 50;
                return Some(Message::GenerateNewPuzzle);
            },
            Message::GenerateNewPuzzle => {
                self.puzzle = Some(self.generator.generate_puzzle());
                self.app_state = RunningState::GeneratorPuzzle;
            },
            Message::Quit => {
                self.app_state = RunningState::Done;
            },
            Message::ShowSolution => {
                match self.app_state {
                    RunningState::GeneratorPuzzle => {
                        self.app_state = RunningState::GeneratorSolution;
                    },
                    RunningState::GeneratorSolution => {
                        self.app_state = RunningState::GeneratorPuzzle;
                    },
                    _ => {},
                };
            }
        }

        None
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> color_eyre::Result<()> {
        // application loop
        while self.app_state != RunningState::Done {
            terminal.draw(|f| self.view(f))?;

            let mut current_msg = self.handle_event()?;

            while current_msg.is_some() {
                current_msg = self.update(current_msg.unwrap());
            }
        }

        Ok(())
    }
}




impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" ku - The ultimate sudoku app, eventually ".bold());

        let instructions = Line::from(vec![
            " New puzzle ".into(), "<n>".blue().bold(),
            " Increment ".into(), "<+>".blue().bold(),
            " Decrement ".into(), "<->".blue().bold(),
            " Reset ".into(), "<r>".blue().bold(),
            " Show solution ".into(), "<s>".blue().bold(),
            " Quit ".into(), "<q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .borders(Borders::ALL)
            .border_set(border::THICK);

        block.render(area, buf);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(4),
                Constraint::Min(13)
            ])
            .margin(1)
            .split(area);

        let generator_info = Paragraph::new(
            Line::from(vec![Span::raw(format!("{:?}", self.generator))])
        ).render(main_layout[0], buf);

        if self.puzzle.is_none() { return; }

        match self.app_state {
            RunningState::GeneratorPuzzle => {
                PuzzleBoardWidget::new(
                    &self.puzzle.as_ref().unwrap()).render(main_layout[1],
                    buf
                );
            },
            RunningState::GeneratorSolution => {
                FullBoardWidget::new(
                    &self.puzzle.as_ref().unwrap()).render(main_layout[1],
                    buf
                );
            },
            _ => {},
        }

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
