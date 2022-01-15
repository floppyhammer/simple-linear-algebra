use rand::prelude::*;
use std::time::Duration;

mod matrix;

fn main() {
    let mut rng = rand::thread_rng();
    let mut a = matrix::Matrix::new(10, 10);

    // Set random elements.
    for i in 0..a.height {
        for j in 0..a.width {
            a.set_element(i, j, rng.gen());
        }
    }
    
    let t = matrix::transpose(&a);

    let c = matrix::matmul(&a, &a);

    a.display();
    t.display();
    c.display();
}
