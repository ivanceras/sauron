use js_sys::Date;

#[derive(Debug, Clone)]
pub enum Msg {
    Click,
    Clock,
}

pub struct Store {
    click_count: u32,
    time: Date,
    listeners: Vec<Box<Fn()>>,
}

impl Store {
    pub fn new(count: u32) -> Store {
        Store {
            click_count: count,
            time: Date::new_0(),
            listeners: vec![],
        }
    }

    /*
    pub fn subscribe(&mut self, callback: Box<Fn()>) {
        self.listeners.push(callback)
    }
    */

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::Click => self.increment_click(),
            Msg::Clock => self.update_time(),
        };

        // Whenever we update state we'll let all of our state listeners know that state was
        // updated
        for callback in self.listeners.iter() {
            callback();
        }
    }

    pub fn click_count(&self) -> u32 {
        self.click_count
    }

    fn increment_click(&mut self) {
        self.click_count += 1;
    }

    fn update_time(&mut self) {
        self.time = Date::new_0();
    }

    pub fn time(&self) -> &Date {
        &self.time
    }
}
