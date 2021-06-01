pub struct BoxplotData {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub median: f64,
    pub q1: f64,
    pub q3: f64,
}

impl BoxplotData {
    fn new(data: &Vec<u64>) -> BoxplotData {
        let mut data = data.clone();
        data.sort();
        // TODO: panic if vector is empty or set attributes as options
        BoxplotData {
            min: min(&data),
            max: max(&data),
            mean: mean(&data),
            median: median(&data),
            q1: q1(&data),
            q3: q3(&data),
        }
    }
}

/// TODO: move fns to a statistics lib

pub fn min(v: &Vec<u64>) -> u64 {
    v.iter().min().unwrap().clone()
}

pub fn max(v: &Vec<u64>) -> u64 {
    v.iter().max().unwrap().clone()
}

pub fn mean(v: &Vec<u64>) -> f64 {
    let sum: u64 = v.iter().sum();
    let n = v.len();
    sum as f64 / n as f64
}

// expects a vector sorted as param
pub fn median(v: &Vec<u64>) -> f64 {
    let n = v.len();
    if n % 2 == 0  {
        (v[(n/2) - 1] + v[n/2]) as f64 / 2.0
    } else {
        v[n/2] as f64
    }
} 

pub fn q1(v: &Vec<u64>) -> f64 {
    let n = v.len() as f64;  
    let q1 = (n * 0.25).round() as usize;
    (v[q1]) as f64
}

pub fn q3(v: &Vec<u64>) -> f64 {
    let n = v.len() as f64;  
    let q1 = (n * 0.75).round() as usize;
    (v[q1]) as f64
}

pub fn main() {
    let votes: Vec<u64> = [1, 2, 3, 4, 5, 4, 7, 56, 12, 150].to_vec();
    let boxplot = BoxplotData::new(&votes);
    println!("min: {}", boxplot.min);
    println!("max: {}", boxplot.max);
    println!("mean: {}", boxplot.mean);
    println!("median: {}", boxplot.median);
    println!("q1: {}", boxplot.q1);
    println!("q3: {}", boxplot.q3);
}
