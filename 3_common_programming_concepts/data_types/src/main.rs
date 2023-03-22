fn main() {
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    let _a = [3; 5];
}
