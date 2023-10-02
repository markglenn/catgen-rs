pub struct State {
    pub current_line: usize,
    pub running: bool,
    pub width: u16,
    pub height: u16,
    pub document_length: usize,
    pub is_dirty: bool,
}

impl State {
    pub fn new(size: (u16, u16), document_length: usize) -> Self {
        Self {
            current_line: 0,
            running: true,
            width: size.0,
            height: size.1,
            document_length,
            is_dirty: true,
        }
    }

    pub fn drawing_height(&self) -> usize {
        self.height as usize - 2
    }

    pub fn scroll_down(&mut self, amount: usize) {
        self.current_line += amount;

        if self.current_line > self.document_length - self.drawing_height() {
            self.current_line = self.document_length - self.drawing_height();
        }
    }

    pub fn scroll_up(&mut self, amount: usize) {
        match self.current_line.checked_sub(amount) {
            Some(line) => self.current_line = line,
            None => self.current_line = 0,
        }
    }

    pub fn scroll_to(&mut self, pos: usize) {
        self.current_line = pos as usize;

        if self.current_line > self.document_length - self.drawing_height() {
            self.current_line = self.document_length - self.drawing_height();
        }
    }

    pub fn scrollbar_position(&self) -> u16 {
        ((self.current_line as f32 / (self.document_length - self.height as usize) as f32)
            * (self.height - 5) as f32) as u16
            + 1
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
}
