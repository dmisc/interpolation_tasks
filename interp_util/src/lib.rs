#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn linspace(min: f64, max: f64, num: usize) -> Vec<f64> {
    let dt = (max - min) / ((num - 1) as f64);
    let mut pts = Vec::with_capacity(num);
    for i in 0..num {
        pts.push(min + dt * i as f64);
    }

    pts
}

