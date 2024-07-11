// Condition and loops

// area of circle java program
// area of triangle
// area of rectangle program
// area of isosceles triangle
// area of parallelogram
// area of rhombus
// area of equilateral triangle
// perimeter of circle
// perimeter of equilateral triangle
// perimeter of parallelogram

const PI: f64 = 3.142;

pub fn area_of_circle(radius: f64) -> f64 {
    PI * radius * radius
}

pub fn area_of_triangle(base: f64, height: f64) -> f64 {
    base * height / 2.0
}

pub fn area_of_rectangle(width: f64, height: f64) -> f64 {
    width * height
}

pub fn area_of_isosceles_triangle(base: f64, height: f64) -> f64 {
    (1.0 * base * height) / 2.0
}

pub fn area_of_parallelogram(base: f64, height: f64) -> f64 {
    base * height
}

pub fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

pub fn perimeter_circle(radius: f64) -> f64 {
    2.0 * PI * radius
}

pub fn area_equilateral_triangle(side: f64) -> f64 {
    (f64::sqrt(3.0) / 4.0) * side * side
}

pub fn perimeter_equilateral_triangle(a: f64) -> f64 {
    3.0 * a
}

pub fn perimeter_of_parallelogram(length: f64, breadth: f64) -> f64 {
    2.0 * (length + breadth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_of_circlen() {
        assert_eq!(area_of_circle(5.0), 78.55);
    }

    #[test]
    fn test_area_of_triangle() {
        assert_eq!(area_of_triangle(5.0, 13.0), 32.50000);
    }

    #[test]
    fn test_area_of_rectangle() {
        assert_eq!(area_of_rectangle(5.0, 10.0), 50.0);
    }

    #[test]
    fn test_area_of_isosceles_triangle() {
        assert_eq!(area_of_isosceles_triangle(5.0, 10.0), 25.0);
    }

    #[test]
    fn test_area_of_parallelogram() {
        assert_eq!(area_of_parallelogram(5.0, 10.0), 50.0);
    }

    #[test]
    fn test_area_of_rhombus() {
        assert_eq!(area_of_rhombus(5.0, 10.0), 25.0);
    }

    #[test]
    fn test_perimeter_circle() {
        assert_eq!(perimeter_circle(5.0), 31.419999999999998);
    }

    #[test]
    fn test_area_equilateral_triangle() {
        assert_eq!(area_equilateral_triangle(5.0), 10.825317547305481);
    }

    #[test]
    fn test_perimeter_equilateral_triangle() {
        assert_eq!(perimeter_equilateral_triangle(5.0), 15.0);
    }

    #[test]
    fn test_perimeter_of_parallelogram() {
        assert_eq!(perimeter_of_parallelogram(10.0, 8.0), 36.0);
    }
}
