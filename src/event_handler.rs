use std::collections::VecDeque;

use crate::event::Event;

pub struct EventHandler {
    events: VecDeque<Event>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    pub fn schedule(&mut self, event: Event) {
        let mut position = 0;
        while position < self.events.len() && self.events[position].time() <= event.time() {
            position += 1;
        }
        self.events.insert(position, event);
    }

    pub fn next(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}
