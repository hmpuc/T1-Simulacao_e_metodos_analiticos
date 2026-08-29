pub struct Queue {
    clients: usize,
    capacity: i64,
    servers: usize,
    busy_servers: usize,
}

impl Queue {
    pub fn new(capacity: i64, servers: usize) -> Self {
        Self {
            clients: 0,
            capacity,
            servers,
            busy_servers: 0,
        }
    }

    pub fn add(&mut self) -> bool {
        if self.capacity >= 0 && self.clients >= self.capacity as usize {
            return false;
        }

        self.clients += 1;
        return true;
    }

    pub fn remove(&mut self) {
        if self.clients > 0 {
            self.clients -= 1;
        }
    }

    pub fn has_available_server(&self) -> bool {
        self.busy_servers < self.servers
    }

    pub fn has_waiting_client(&self) -> bool {
        self.clients > self.busy_servers
    }

    pub fn length(&self) -> usize {
        self.clients
    }

    pub fn occupy_server(&mut self) {
        self.busy_servers += 1;
    }

    pub fn release_server(&mut self) {
        if self.busy_servers > 0 {
            self.busy_servers -= 1;
        }
    }
}
