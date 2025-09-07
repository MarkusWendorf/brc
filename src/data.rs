#[derive(Default, Debug, Clone)]
pub struct Data {
    pub min: i16,
    pub max: i16,
    pub total: u32,
    pub count: u32,
}

impl Data {
    pub fn update(&mut self, temp: i16) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.total += temp as u32;
        self.count += 1;
    }

    pub fn merge(&mut self, other: &Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.total += other.total;
        self.count += other.count;
    }
}
