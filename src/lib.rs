struct IncrementalAverage {
    size: u32,
    average: f64,
}

impl IncrementalAverage {
    fn new() -> Self {
        Self {
            size: 0,
            average: 0.0,
        }
    }

    fn add(&mut self, value: f64) {
        self.size += 1;
        self.average += (value - self.average) / f64::from(self.size);
    }

    fn remove(&mut self, value: f64) {
        self.average = (f64::from(self.size) * self.average - value) / (f64::from(self.size - 1));
        self.size -= 1;
    }

    fn get(&self) -> Option<f64> {
        if self.size > 0 {
            Some(self.average)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ia = IncrementalAverage::new();

        assert!(ia.get().is_none());

        ia.add(1.0);
        ia.get();

        assert!(ia.get().is_some());
        assert_eq!(ia.get().unwrap(), 1.0);

        ia.add(1.0);
        ia.get();

        assert!(ia.get().is_some());
        assert_eq!(ia.get().unwrap(), 1.0);

        ia.add(4.0);
        ia.get();

        assert!(ia.get().is_some());
        assert_eq!(ia.get().unwrap(), 2.0);

        ia.add(0.0);
        ia.get();

        assert!(ia.get().is_some());
        assert_eq!(ia.get().unwrap(), 1.5);

        ia.remove(0.0);
        ia.get();

        assert!(ia.get().is_some());
        assert_eq!(ia.get().unwrap(), 2.0);

        ia.remove(4.0);
        ia.get();

        assert!(ia.get().is_some());
        assert_eq!(ia.get().unwrap(), 1.0);

        ia.remove(1.0);
        ia.get();

        assert!(ia.get().is_some());
        assert_eq!(ia.get().unwrap(), 1.0);
    }
}
