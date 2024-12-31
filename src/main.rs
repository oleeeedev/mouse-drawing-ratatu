use color_eyre::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseEvent,
        MouseEventKind,
    },
    execute,
};
use ratatui::{
    layout::{Position, Rect, Size},
    style::{Color, Stylize},
    symbols,
    text::Line,
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = MouseDrawingApp::default().run(terminal);
    ratatui::restore();
    result
}

#[derive(Default)]
struct MouseDrawingApp {
    pub should_exit: bool,
    pub mouse_position: Option<Position>,
    pub points: Vec<(Position, Color)>,
    pub current_color: Color,
}

impl MouseDrawingApp {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        execute!(std::io::stdout(), EnableMouseCapture)?;
        while !self.should_exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        execute!(std::io::stdout(), DisableMouseCapture)?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event) => self.on_key_event(event),
            Event::Mouse(event) => self.on_mouse_event(event),
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(' ') => {
                self.current_color = Color::Rgb(rand::random(), rand::random(), rand::random());
            }
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            _ => {}
        }
    }

    fn on_mouse_event(&mut self, event: MouseEvent) {
        let position = Position::new(event.column, event.row);
        match event.kind {
            MouseEventKind::Down(_) => self.points.push((position, self.current_color)),
            MouseEventKind::Drag(_) => self.draw_line(position),
            _ => {}
        }
        self.mouse_position = Some(position);
    }

    fn draw_line(&mut self, position: Position) {
        if let Some(start) = self.points.last() {
            let (x0, y0) = (i32::from(start.0.x), i32::from(start.0.y));
            let (x1, y1) = (i32::from(position.x), i32::from(position.y));
            for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y1)) {
                let point = (Position::new(x as u16, y as u16), self.current_color);
                self.points.push(point);
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        self.render_points(frame);
        self.render_mouse_cursor(frame);
        let value = "Mouse Example ('Esc' to quit. Click / drag to draw. 'Space' to change color)";
        let title = Line::from(value).centered();
        frame.render_widget(title, frame.area());
    }

    fn render_points(&self, frame: &mut Frame<'_>) {
        for (position, color) in &self.points {
            let area = Rect::from((*position, Size::new(1, 1))).clamp(frame.area());
            frame.render_widget(symbols::block::FULL.fg(*color), area);
        }
    }

    fn render_mouse_cursor(&self, frame: &mut Frame<'_>) {
        if let Some(position) = self.mouse_position {
            let area = Rect::from((position, Size::new(1, 1))).clamp(frame.area());
            frame.render_widget("╳".bg(self.current_color), area);
        }
    }
}