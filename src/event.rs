#[derive(Clone, Copy)]
pub enum EventType {
    Arrival,
    Departure,
}

#[derive(Clone, Copy)]
pub struct Event {
    event_type: EventType,
    time: f64,
}

impl Event {
    pub fn new(event_type: EventType, time: f64) -> Self {
        Self { event_type, time }
    }

    pub const fn time(&self) -> f64 {
        self.time
    }

    pub const fn event_type(&self) -> EventType {
        self.event_type
    }
}
