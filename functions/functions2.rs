// functions2.rs

// I AM DONE

fn main() {
    call_me(3);
}

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
