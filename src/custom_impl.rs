pub trait Magnitude {
    fn magnitude(&self) -> u8;
}

impl Magnitude for u8 {
    fn magnitude(&self) -> u8 {
        if *self == 0 {
            return 1;
        }
        let mut value = self.clone();
        let mut counter = 0;
        while value > 0 {
            value /= 10;
            counter += 1;
        }
        return counter;
    }
}

impl Magnitude for usize {
    fn magnitude(&self) -> u8 {
        if *self == 0 {
            return 1;
        }
        let mut value = self.clone() as i8;
        let mut counter = 0;
        while value > 0 {
            value /= 10;
            counter += 1;
        }
        return counter;
    }
}
