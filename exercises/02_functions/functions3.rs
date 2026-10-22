fn main() {
    // TODO: Fix the function call.
    call_me(10);
}

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

