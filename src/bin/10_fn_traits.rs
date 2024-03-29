// ------------------------------------------------------
//     - Traits
//          - General Information
//          - Default Implementation
//          - Functions within a trait implementation
// ------------------------------------------------------

struct Data {
    some_data: Vec<i32>,
}

trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum = 0;
        for i in self.some_data.iter() {
            sum = sum + *i;
        }
        sum as f32 / self.some_data.len() as f32
    }

    fn variance(&self) -> f32 {
        let mu = self.mean();
        let mut sum_squared_diff: f32 = 0.0;
        for i in self.some_data.iter() {
            sum_squared_diff += (*i as f32 - mu) * (*i as f32 - mu);
        }
        sum_squared_diff / self.some_data.len() as f32
    }
}
fn main() {
    let my_data = Data {
        some_data: vec![5, 1, 6, 7, 8, 9, 2, 3],
    };
    println!("The mean of the data is  {}", my_data.mean());
    println!("The variance of the data is  {}", my_data.variance());
}
