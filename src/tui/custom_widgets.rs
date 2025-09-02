use crate::game::*;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span, Text},
    style::Stylize,
    symbols::border,
    prelude::Direction,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// BoardWidget
pub struct BoardWidget {
    board_values: Vec<Vec<String>>,
}

impl BoardWidget {
    pub fn new(board_values: Vec<Vec<String>>) -> Self {
        BoardWidget { board_values: board_values }
    }
}

impl Widget for &BoardWidget {
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
            let mut line = String::new();
            for (i, s) in self.board_values[r].iter().enumerate() {
                line.push_str(s);
                if i == 2 || i == 5 { line.push_str(" ┃ "); }
                else if i != 8 { line.push(' '); }
            }
            lines.push(Line::from(vec![Span::raw("┃ ".to_string() + &line + " ┃")]));

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

pub struct PuzzleBoardWidget {
    pub board: BoardWidget,
}

/// PuzzleBoardWidget
impl PuzzleBoardWidget {
    pub fn new(board: &Board) -> Self {
        let board_values: Vec<_> = board.values
            .into_iter()
            .map(|row| row.map(|cell| cell.to_string(false)).to_vec())
            .collect();

        Self { board: BoardWidget::new(board_values)  }
    }
}

impl Widget for &PuzzleBoardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.board.render(area, buf);
    }
}

pub struct FullBoardWidget {
    pub board: BoardWidget,
}

/// FullBoardWidget
impl FullBoardWidget {
    pub fn new(board: &Board) -> Self {
        let board_values: Vec<_> = board.values
            .into_iter()
            .map(|row| row.map(|cell| cell.to_string(true)).to_vec())
            .collect();

        Self { board: BoardWidget::new(board_values)  }
    }
}

impl Widget for &FullBoardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.board.render(area, buf);
    }
}

/// MainWindowFrameWidget
pub struct MainWindowFrameWidget {
    pub title: String,
    pub instructions: String,
}

impl MainWindowFrameWidget {
    pub fn new(title: String, instructions: String) -> Self {
        Self { title, instructions }
    }
}

impl Widget for &MainWindowFrameWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_line = Line::from(vec![self.title.clone().into()]);
        let instructions_line = Line::from(vec![self.instructions.clone().into()]);

        Block::bordered()
            .title(title_line.centered())
            .title_bottom(instructions_line.centered())
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .render(area, buf);
    }
}
