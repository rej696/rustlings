// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec = Vec::new();

    // Borrow a mutable reference to vec
    fill_vec_borrow(&mut vec);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);
   
    // take ownership of vec, and return it into the shadowed variable
    let mut vec = fill_vec_ret(vec);

    vec.push(88);

    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);
}

fn fill_vec_ret(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}

fn fill_vec_borrow(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
