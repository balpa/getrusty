use std::time::Instant;

fn main() {
    let start = Instant::now();

    fn expensive_loop() -> i64 {
        let mut sum: i64 = 0;

        for n in 0x0..0xffffff {
            sum = sum + n;
        }

        return sum;
    }

    println!("Sum is: {:?}", expensive_loop());
    let duration = start.elapsed();
    println!("Time elapsed in the fn is:  {:?}", duration);
}