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
    block: TuiBlock<'a>,
}

impl<'a> Block<'a> {
    pub fn new(frame: &mut Frame<'a, CrosstermBackend<Stdout>>) -> Block<'a> {
        let block = TuiBlock::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Double);

        return Block { frame, block };
    }

    fn internal_render(
        &mut self,
        dimensions: Rect,
        title: Option<&str>,
        render: &dyn Fn(TuiBlock),
    ) {
        if let Some(title_string) = title {
            self.block
                .title(title_string)
                .title_alignment(tui::layout::Alignment::Center);
        }

        self.frame.render_widget(self.block, dimensions);
        render(self.block);
    }

    pub fn show_block(&mut self, dimensions: Rect, render: &dyn Fn(TuiBlock)) {
        self.internal_render(dimensions, None, render)
    }

    pub fn show_block_with_title(
        &mut self,
        dimensions: Rect,
        title: &str,
        render: &dyn Fn(TuiBlock),
    ) {
        self.internal_render(dimensions, Some(title), render)
    }
}
