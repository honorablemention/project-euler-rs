struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        Some(self.current)
    }
}

fn main() {
    let limit = 4_000_000;
    
    // Create instance of our Fibonacci iterator initialized with 1, 2
    let fib_iterator = Fibonacci { current: 1, next: 2 };
    
    let sum_of_evens: u32 = fib_iterator
        .take_while(|&n| n <= limit)
        .filter(|&n| n % 2 == 0)
        .sum();

    println!("Result for {} is: {}", limit, sum_of_evens);
}
