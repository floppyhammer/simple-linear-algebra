pub(crate) struct Matrix {
    pub(crate) height: usize,
    pub(crate) width: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub(crate) fn new(height: usize, width: usize) -> Matrix {
        Matrix {
            height,
            width,
            data: vec![0.0; height * width],
        }
    }

    fn get_element(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.width + col]
    }

    pub(crate) fn set_element(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.width + col] = value;
    }

    pub(crate) fn display(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{0: <7.6} ", self.get_element(i, j));
            }
            println!();
        }
    }
}

pub(crate) fn transpose(a: &Matrix) -> Matrix {
    let mut t = Matrix::new(a.height, a.width);
    for i in 0..a.height {
        for j in 0..a.width {
            t.set_element(j, i, a.get_element(i, j));
        }
    }
    t
}

pub(crate) fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = Matrix::new(a.height, b.width);
    for i in 0..a.height {
        for j in 0..b.width {
            for k in 0..a.width {
                c.set_element(i, j, a.get_element(i, k) * b.get_element(k, j));
            }
        }
    }
    c
}
