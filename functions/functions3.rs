// functions3.rs

// I AM DONE

fn main() {
    call_me(12);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
