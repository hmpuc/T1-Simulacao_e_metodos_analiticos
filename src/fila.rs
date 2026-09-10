pub struct Fila {
    servidores: i32,
    capacity: i32,
    min_arrival: f64,
    max_arrival: f64,
    min_service: f64,
    max_service: f64,
    customers: i32,
    loss: i32,
    timers: Vec<f64>
}

impl Fila {
    pub fn new(servidores: i32, capacity: i32, min_arrival: f64, max_arrival: f64, min_service: f64, max_service: f64) -> Self {
        return Fila {
            servidores: servidores,
            capacity: capacity,
            min_arrival: min_arrival,
            max_arrival: max_arrival,
            min_service: min_service,
            max_service: max_service,
            customers: 0,
            loss: 0,
            timers: vec![0.0; 1]
        }
    }

    pub fn status(&self) -> i32 {
        self.customers
    }

    pub fn capacity(&self) -> i32 {
        self.capacity
    }

    pub fn servers(&self) -> i32 {
        self.servidores
    }

    pub fn loss(&mut self) {
        self.loss += 1;
    }

    pub fn entrada(&mut self) {
        self.customers += 1;
    }

    pub fn saida(&mut self) {
        self.customers -= 1;
    }
}