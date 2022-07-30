use anyhow::{Ok, Result};
use tui::backend::Backend;
use tui::{layout, style, text, widgets, Terminal};

pub struct App {
    rows: usize,
    cols: usize,
    mines: usize,
    cells: usize,
}

impl App {
    pub fn new() -> App {
        App {
            rows: 10,
            cols: 10,
            mines: 10,
            cells: 4,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let width = (self.cells * self.cols + 2) as u16;

        let cons = layout::Constraint::Length(self.cells as u16);
        let row_constraints: Vec<_> = (0..self.rows).map(|_| cons).collect();
        let col_constraints: Vec<_> = (0..self.cols).map(|_| cons).collect();

        terminal.draw(|frame| {
            let terminal_rect = frame.size();

            frame.render_widget(self.outer_block(), terminal_rect);

            let height = (self.cells * self.rows + 2) as u16;
            let outer_rects = layout::Layout::default()
                .direction(layout::Direction::Vertical)
                .vertical_margin(1)
                .horizontal_margin(1)
                .constraints(vec![layout::Constraint::Min(height)])
                .split(terminal_rect);

            let mines_rect = outer_rects[0];

            let style = style::Style::default()
                .fg(style::Color::LightMagenta)
                .add_modifier(style::Modifier::BOLD);

            let block = widgets::Block::default()
                .borders(widgets::Borders::ALL)
                .title(text::Span::styled("剩余地雷数", style));
            let style = style::Style::default()
                .fg(style::Color::White)
                .bg(style::Color::Black)
                .add_modifier(style::Modifier::BOLD);
            let info_text = widgets::Gauge::default()
                .block(block)
                .gauge_style(style)
                .label(format!(
                    "{:>length$}",
                    available_flags,
                    length = available_flags
                        .to_f64()
                        .unwrap()
                        .log10()
                        .ceil()
                        .to_usize()
                        .unwrap()
                        + 1
                ))
                .ratio(available_flags.to_f64().unwrap() / mines.to_f64().unwrap());

            let horizontal_pad_block_width = (terminal_rect.width - grid_width) / 2;
            let mines_rects = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Min(horizontal_pad_block_width),
                    Constraint::Length(grid_width),
                    Constraint::Min(horizontal_pad_block_width),
                ])
                .split(mines_rect);
        })?;

        std::thread::sleep(std::time::Duration::from_millis(4 * 1000));
        Ok(())
    }

    fn outer_block(&self) -> widgets::Block {
        let title_style = style::Style::default()
            .fg(style::Color::LightYellow)
            .add_modifier(style::Modifier::BOLD);

        widgets::Block::default()
            .borders(widgets::Borders::ALL)
            .title(text::Span::styled("扫雷", title_style))
            .border_type(widgets::BorderType::Rounded)
    }
}
