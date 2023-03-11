use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block as TuiBlock, BorderType, Borders},
    Frame,
};

pub struct Block<'a> {
    frame: &'a mut Frame<'a, CrosstermBackend<Stdout>>,
}

impl<'a> Block<'a> {
    pub fn new(frame: &mut Frame<CrosstermBackend<Stdout>>) -> Block<'a> {
        return Block { frame };
    }

    fn internal_render(
        &mut self,
        dimensions: Rect,
        title: Option<&str>,
        render: &dyn Fn(&mut Frame<CrosstermBackend<Stdout>>),
    ) {
        let mut block = TuiBlock::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Double);

        block = if let Some(title_string) = title {
            block
                .title(title_string)
                .title_alignment(tui::layout::Alignment::Center)
        } else {
            block
        };

        self.frame.render_widget(block, dimensions);

        render(self.frame);
    }

    pub fn show_block(
        &mut self,
        dimensions: Rect,
        render: &dyn Fn(&mut Frame<CrosstermBackend<Stdout>>),
    ) {
        self.internal_render(dimensions, None, render)
    }

    pub fn show_block_with_title(
        &mut self,
        dimensions: Rect,
        title: &str,
        render: &dyn Fn(&mut Frame<CrosstermBackend<Stdout>>),
    ) {
        self.internal_render(dimensions, Some(title), render)
    }
}
