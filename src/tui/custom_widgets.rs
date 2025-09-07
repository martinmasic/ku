use crate::game::*;
#[allow(unused_imports)]
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    style::{Style, Color, Modifier},
    symbols::border,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// BoardWidget
struct BoardWidget<'a> {
    board: &'a Board,
    given_style: Option<Style>,
    non_given_style: Option<Style>,
    empty_style: Option<Style>,
    empty_value: Option<char>,
    show_non_givens: bool,
    position: Option<(usize, usize)>,
}

impl<'a> BoardWidget<'a> {
    pub fn new(board: &'a Board) -> Self {
        BoardWidget {
            board: board,
            given_style: None,
            non_given_style: None,
            empty_style: None,
            empty_value: None,
            show_non_givens: true,
            position: None,
        }
    }

    pub fn given_style(mut self, style: Style) -> Self {
        self.given_style = Some(style);
        self
    }

    pub fn non_given_style(mut self, style: Style) -> Self {
        self.non_given_style = Some(style);
        self
    }

    pub fn empty_style(mut self, style: Style) -> Self {
        self.empty_style = Some(style);
        self
    }

    pub fn show_non_givens(mut self, show: bool) -> Self {
        self.show_non_givens = show;
        self
    }

    pub fn empty_value(mut self, value: char) -> Self {
        self.empty_value = Some(value);
        self
    }

    pub fn position(mut self, position: Option<(usize, usize)>) -> Self {
        self.position = position;
        self
    }
}

impl<'a> Widget for &'a BoardWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Build the entire self.board as lines of Span
        let mut lines: Vec<Line> = Vec::new();

        let top_border = format!("{}{}{}{}{}{}{}",
            "┏".to_string(),
            &"━".repeat(7), "┳",
            &"━".repeat(7), "┳",
            &"━".repeat(7), "┓"
        );
        lines.push(Line::from(vec![Span::raw(top_border)]));

        let sep = format!("{}{}{}{}{}{}{}",
            "┣".to_string(),
            "━".repeat(7), "╋",
            "━".repeat(7), "╋",
            "━".repeat(7), "┫"
        );

        for r in 0..9 {
            let mut line: Vec<Span> = Vec::new();
            line.push(Span::raw("┃ ".to_string()));
            for (c, cell) in self.board.values[r].iter().enumerate() {
                match cell {
                    Cell::Given(ch) => {
                        let mut style = self.given_style
                            .unwrap_or(Style::default());
                        if let Some(pos) = self.position && pos == (r,c) {
                            style = style
                                .bg(Color::Magenta)
                                .fg(Color::Black);
                        }
                        line.push(Span::styled(
                            ch.to_string().clone(),
                            style
                        ));
                    },
                    Cell::NonGiven(ch) => {
                        if self.show_non_givens {
                            let mut style = self.non_given_style
                                .unwrap_or(Style::default());
                            if let Some(pos) = self.position && pos == (r,c) {
                                style = style
                                    .bg(Color::Magenta)
                                    .fg(Color::Black);
                            }
                            line.push(Span::styled(
                                ch.to_string().clone(),
                                style
                            ));
                        } else {
                            let mut style = self.empty_style
                                .unwrap_or(Style::default());
                            if let Some(pos) = self.position && pos == (r,c) {
                                style = style
                                    .bg(Color::Magenta)
                                    .fg(Color::Black);
                            }
                            line.push(Span::styled(
                                self.empty_value.unwrap_or('_').to_string().clone(),
                                style
                            ));
                        }
                    },
                    Cell::Empty => {
                        let mut style = self.empty_style
                            .unwrap_or(Style::default());
                        if let Some(pos) = self.position && pos == (r,c) {
                            style = style
                                .bg(Color::Magenta)
                                .fg(Color::Black);
                        }
                        line.push(Span::styled(
                            self.empty_value.unwrap_or('_').to_string().clone(),
                            style
                        ));
                    }
                }


                if c == 2 || c == 5 { line.push(Span::raw(" ┃ ")); }
                else if c != 8 { line.push(Span::raw(" ")); }
            }
            line.push(Span::raw(" ┃"));

            lines.push(Line::from(line));

            if r == 2 || r == 5 {
                lines.push(Line::from(vec![Span::raw(sep.clone())]));
            }
        }

        let bottom_border = format!("{}{}{}{}{}{}{}",
            "┗".to_string(),
            &"━".repeat(7), "┻",
            &"━".repeat(7), "┻",
            &"━".repeat(7), "┛"
        );
        lines.push(Line::from(vec![Span::raw(bottom_border)]));

        Paragraph::new(lines).render(area, buf);
    }

}

pub struct PuzzleBoardWidget<'a> {
    board_widget: BoardWidget<'a>,
}

/// PuzzleBoardWidget
impl<'a> PuzzleBoardWidget<'a> {
    pub fn new(board: &'a Board) -> Self {
        let board_widget = BoardWidget::new(board)
            .given_style(Style::default()
                .add_modifier(Modifier::BOLD)
            )
            .empty_value('_')
            .empty_style(
                Style::default()
                    .fg(Color::Gray)
            )
            .show_non_givens(true);
        Self { board_widget: board_widget }
    }

    pub fn set_position(mut self, position: Option<(usize, usize)>) -> Self {
        self.board_widget = self.board_widget.position(position);
        self
    }
}

impl<'a> Widget for &'a PuzzleBoardWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.board_widget.render(area, buf);
    }
}


pub struct FullBoardWidget<'a> {
    board_widget: BoardWidget<'a>,
}

/// FullBoardWidget
impl<'a> FullBoardWidget<'a> {
    pub fn new(board: &'a Board) -> Self {
        let board_widget = BoardWidget::new(board)
            .given_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
            )
            .non_given_style(
                Style::default()
                    .fg(Color::Green)
            );
        Self { board_widget: board_widget }
    }
}

impl<'a> Widget for &'a FullBoardWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.board_widget.render(area, buf);
    }
}

/// MainWindowFrameWidget
pub struct MainWindowFrameWidget<'a> {
    pub title: Line<'a>,
    pub instructions: Line<'a>,
}

impl<'a> MainWindowFrameWidget<'a> {
    pub fn new(title: Line<'a>, instructions: Line<'a>) -> Self {
        Self { title, instructions }
    }
}

impl<'a> Widget for &'a MainWindowFrameWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .title(self.title.clone().centered())
            .title_bottom(self.instructions.clone().centered())
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .render(area, buf);
    }
}

