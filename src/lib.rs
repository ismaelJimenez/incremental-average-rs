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
        let mut t = IncrementalAverage::new();

        assert!(t.get().is_none());

        t.add(1.0);
        t.get();

        assert!(t.get().is_some());
        assert_eq!(t.get().unwrap(), 1.0);

        t.add(1.0);
        t.get();

        assert!(t.get().is_some());
        assert_eq!(t.get().unwrap(), 1.0);

        t.add(4.0);
        t.get();

        assert!(t.get().is_some());
        assert_eq!(t.get().unwrap(), 2.0);

        t.add(0.0);
        t.get();

        assert!(t.get().is_some());
        assert_eq!(t.get().unwrap(), 1.5);

        t.remove(0.0);
        t.get();

        assert!(t.get().is_some());
        assert_eq!(t.get().unwrap(), 2.0);

        t.remove(4.0);
        t.get();

        assert!(t.get().is_some());
        assert_eq!(t.get().unwrap(), 1.0);

        t.remove(1.0);
        t.get();

        assert!(t.get().is_some());
        assert_eq!(t.get().unwrap(), 1.0);
    }
}
