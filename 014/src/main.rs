fn next_collatz(n: u64) -> u64 {
    if n % 2 == 0 { 
        n /2 
    } 
    else {
        3 * n + 1
    }
}


fn collatz_iter<'a, F>(start: u64, stop: F) -> impl Iterator<Item = u64> + 'a
where F: Fn(u64) -> bool + 'a {
    std::iter::successors(Some(start),  move | &n| {
        if (stop)(n) || n == 1 { 
            None
        } else {
            let next = next_collatz(n);
            Some(next)
        }
    })
}

struct Memo {
    data: Vec<usize>,
}

impl Memo {
    fn new(limit: u64) -> Self {
        let mut data = vec![0_usize; (limit + 1) as usize];
        data[1] = 1;
        Self { data }
    }

    fn get(&self, n: u64) -> Option<usize> {
        let i = n as usize;
        self.data.get(i).and_then(
            |&v| if v == 0 { None } else { Some(v) }
        )
    }

    fn set(&mut self, n: u64, len: usize) {
        let i = n as usize;
        if let Some(slot) = self.data.get_mut(i) {
            *slot = len;
        }
    }
}

fn get_len(start: u64, memo: &mut Memo) -> usize {
    if let Some(v) = memo.get(start) {
        return v;
    }

    let mut iter = collatz_iter(start, |_| false);
    let mut path: Vec<u64> = iter
        .by_ref()
        .take_while(|&n| memo.get(n).is_none())
        .collect();

    let tail = iter.next().unwrap_or(1);
    let mut len = memo.get(tail).unwrap_or(1);
    for &v in path.iter().rev() {
        len += 1;
        memo.set(v, len);
    }

    len
}

fn find_longest_chain(limit: u64, memo: &mut Memo) -> u64 {
    let mut best_start: u64 = 1;
    let mut best_len: usize = 1;

    for n in 1..limit {
        let len = get_len(n, memo);
        if len > best_len {
            best_len = len;
            best_start = n;
        }
    }
    best_start
}
fn main() {
    let limit = 1_000_000;
    let mut memo = Memo::new(limit);
    let result = find_longest_chain(limit, &mut memo);
    println!("{result}");
}
