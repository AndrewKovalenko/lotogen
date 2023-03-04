use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block as TuiBlock, BorderType, Borders},
    Terminal,
};

pub struct Block<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Block<'a> {
    pub fn new(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Block {
        return Block { terminal };
    }

    fn internal_render(
        &mut self,
        dimensions: Rect,
        title: Option<&str>,
        render: &dyn Fn(&mut Terminal<CrosstermBackend<Stdout>>),
    ) {
        self.terminal
            .draw(|frame| {
                let mut block = TuiBlock::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .border_type(BorderType::Double);

                block = if let Some(title_string) = title {
                    block.title(title_string)
                } else {
                    block
                };

                frame.render_widget(block, dimensions);
            })
            .unwrap();

        render(self.terminal);
    }

    pub fn show_block(
        &mut self,
        dimensions: Rect,
        render: &dyn Fn(&mut Terminal<CrosstermBackend<Stdout>>),
    ) {
        self.internal_render(dimensions, None, render)
    }

    pub fn show_block_with_title(
        &mut self,
        dimensions: Rect,
        title: &str,
        render: &dyn Fn(&mut Terminal<CrosstermBackend<Stdout>>),
    ) {
        self.internal_render(dimensions, Some(title), render)
    }
}
