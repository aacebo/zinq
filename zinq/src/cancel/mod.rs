#[derive(Default)]
pub struct Cancellation {
    cancelled: bool,
    observers: Vec<fn()>,
}

impl Cancellation {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn subscribe(&mut self, observer: fn()) {
        self.observers.push(observer);
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;

        for observer in &self.observers {
            observer()
        }
    }
}