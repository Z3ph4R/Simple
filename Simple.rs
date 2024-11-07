fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 16, 17];
    
    for &num in &numbers {
        if is_prime(num) {
            println!("{} is prime", num);
        } else {
            println!("{} is not prime", num);
        }
    }
}
