// Here are some easier Clippy fixes so you can see its utility.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Assume that you don't know the value of `my_option`.
    // In the case of `Some`, we want to print its value.
    if let Some(value) = my_option {
        println!("{}", value);
    }

    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear();
    println!("This Vec is empty, see? {my_vec:?}");

    let mut value_a = 45;
    let value_b = 66;
    // Let's swap these two!
    let _ = std::mem::replace(&mut value_a, value_b);
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {value_a}; value b: {value_b}");
}
