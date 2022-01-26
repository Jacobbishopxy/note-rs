/// TRPL Chapter 17.1's example

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests_oe {
    use super::*;

    #[test]
    fn auto_update_average() {
        let mut foo = AveragedCollection {
            list: vec![],
            average: 0 as f64,
        };

        foo.add(10);
        foo.add(20);

        assert_eq!(foo.average(), 15f64);

        foo.remove();
        assert_eq!(foo.average(), 10f64);
    }
}
