fn programmer_wrote(a: &mut i32, b: &mut i32) {
    *a += 1;
    *b += 1;
    *a += 1;
}

fn may_be_optimized_to(a: &mut i32, b: &mut i32) {
    *a += 2;
    *b += 1;
}

fn main() {
    let mut a = 0;
    let mut b = 0;
    programmer_wrote(&mut a, &mut b);
    let first_results = (a, b);
    a = 0;
    b = 0;
    may_be_optimized_to(&mut a, &mut b);
    let second_results = (a, b);
    assert_eq!(first_results, second_results);
}
