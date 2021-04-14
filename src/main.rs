
fn main() {
    #![feature(option_result_contains)]

    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.contains(&2), true);

    let x: Result<u32, &str> = Ok(3);
    assert_eq!(x.contains(&2), false);

    let x: Result<u32, &str> = Err("Some error message");
    assert_eq!(x.contains(&2), false);
}
