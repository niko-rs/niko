pub struct Metrics {
    draw_calls: usize,
    sprites_drawn: usize,
    sprites_skipped: usize,

    update_start: f64,
    update_end: f64,
    
    draw_start: f64,
    draw_end: f64,

    ticks: usize,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            draw_calls: 0,
            sprites_drawn: 0,
            sprites_skipped: 0,

            update_start: 0.0,
            update_end: 0.0,
            
            draw_start: 0.0,
            draw_end: 0.0,

            ticks: 99,
        }
    }

    pub fn reset(&mut self) {
        self.draw_calls = 0;
        self.sprites_drawn = 0;
        self.sprites_skipped = 0;
    }

    pub fn start_update(&mut self) {
        self.update_start = web_sys::window().unwrap().performance().unwrap().now();
    }

    pub fn end_update(&mut self) {
        self.update_end = web_sys::window().unwrap().performance().unwrap().now();
    }

    pub fn start_draw(&mut self) {
        self.draw_start = web_sys::window().unwrap().performance().unwrap().now();
    }

    pub fn end_draw(&mut self) {
        self.draw_end = web_sys::window().unwrap().performance().unwrap().now();
    }

    pub fn add_draw_calls(&mut self, draw_calls: usize) {
        self.draw_calls += draw_calls;
    }

    pub fn add_sprites_drawn(&mut self, sprites_drawn: usize) {
        self.sprites_drawn += sprites_drawn;
    }

    pub fn add_sprites_skipped(&mut self, sprites_skipped: usize) {
        self.sprites_skipped += sprites_skipped;
    }

    // TODO rust-analyzer gets confused
    #[allow(unused_unsafe)]
    pub fn debug_print(&mut self) {
        self.ticks += 1;

        if self.ticks >= 99 {
            self.ticks = 0;

            unsafe {
                web_sys::console::log_1(&format!("update time: {:.2}ms", self.update_end - self.update_start).into());
                web_sys::console::log_1(&format!("draw time: {:.2}ms", self.draw_end - self.draw_start).into());
                web_sys::console::log_1(&format!("sprites drawn: {}, draw calls: {}, skipped: {}", self.sprites_drawn, self.draw_calls, self.sprites_skipped).into());
            }
        }

    }
}
