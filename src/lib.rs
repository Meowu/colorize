#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: Option<f64>,
    h: f64,
    s: f64,
    l: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let color = Color {
            r: 0.5,
            g: 0.2,
            b: 0.6,
            a: None,
            h: 0.0,
            s: 0.0,
            l: 0.0,
        };
        assert_eq!(color.r, 0.5);
        assert_eq!(color.g, 0.2);
        assert_eq!(color.b, 0.8);
        assert_eq!(color.a, None);
    }
}
