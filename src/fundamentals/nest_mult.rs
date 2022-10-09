/// Nest Multiplication
pub fn nest_mult(depth: usize, a: Vec<f64>, x: f64, c: Vec<f64>) -> f64 {
    a.into_iter().rev().zip(c.into_iter().rev()).fold(1., |y, aa| y * (x * aa.0 + 1.))
}


mod tests {
    use super::*;

    #[test]
    fn nest_test() {
        assert_eq!(1, 1);
        assert_ne!(nest_mult(0, vec![1.], 1./2., vec![1.]), -1.);
    }
}