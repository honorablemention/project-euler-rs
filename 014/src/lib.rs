pub fn next_collatz(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn collatz_iter<F>(start: u64, stop: F) -> impl Iterator<Item = u64>
where
    F: Fn(u64) -> bool,
{
    let mut current = Some(start);
    let mut resume = None;

    std::iter::from_fn(move || {
        let n = current.take()?;

        if stop(n) || n == 1 {
            current = None;
            resume = None;
        } else if let Some(next) = resume.take() {
            current = Some(next);
        } else if n % 2 == 0 {
            current = Some(n / 2);
        } else {
            let next = 3 * n + 1;
            current = Some(next);
            resume = Some(next / 2);
        }

        Some(n)
    })
}

pub struct Memo {
    data: Vec<u32>,
}

impl Memo {
    pub fn new(limit: u64) -> Self {
        let mut data = vec![0_u32; (limit + 1) as usize];
        data[1] = 1;
        Self { data }
    }

    pub fn get(&self, n: u64) -> Option<usize> {
        let i = n as usize;
        if i < self.data.len() {
            let v = self.data[i];
            if v == 0 { None } else { Some(v as usize) }
        } else {
            None
        }
    }

    pub fn set(&mut self, n: u64, len: usize) {
        let i = n as usize;
        if i < self.data.len() {
            self.data[i] = len as u32;
        }
    }
}

fn get_len_iter_with_path(start: u64, memo: &mut Memo, path: &mut Vec<u64>) -> usize {
    if let Some(v) = memo.get(start) {
        return v;
    }

    path.clear();
    path.extend(collatz_iter(start, |n| memo.get(n).is_some()));
    let tail = path.pop().unwrap_or(1);
    let mut len = memo.get(tail).unwrap_or(1);

    for &v in path.iter().rev() {
        len += 1;
        memo.set(v, len);
    }

    len
}

pub fn get_len_iter(start: u64, memo: &mut Memo) -> usize {
    let mut path = Vec::with_capacity(256);
    get_len_iter_with_path(start, memo, &mut path)
}

fn get_len_loop_with_path(start: u64, memo: &mut Memo, path: &mut Vec<u64>) -> usize {
    if let Some(v) = memo.get(start) {
        return v;
    }

    let mut n = start;
    path.clear();

    while memo.get(n).is_none() && n != 1 {
        if n % 2 == 0 {
            path.push(n);
            n /= 2;
        } else {
            let next = 3 * n + 1;
            path.push(n);
            path.push(next);
            n = next / 2;
        }
    }

    let mut len = memo.get(n).unwrap_or(1);
    for &v in path.iter().rev() {
        len += 1;
        memo.set(v, len);
    }

    len
}

pub fn get_len_loop(start: u64, memo: &mut Memo) -> usize {
    let mut path = Vec::with_capacity(256);
    get_len_loop_with_path(start, memo, &mut path)
}

pub fn find_longest_chain(limit: u64, memo: &mut Memo, get_len: fn(u64, &mut Memo) -> usize) -> u64 {
    let mut best_start = 1;
    let mut best_len = 1;

    for n in 1..limit {
        let len = get_len(n, memo);
        if len > best_len {
            best_len = len;
            best_start = n;
        }
    }

    best_start
}

fn find_longest_chain_with_path(
    limit: u64,
    memo: &mut Memo,
    get_len: fn(u64, &mut Memo, &mut Vec<u64>) -> usize,
) -> u64 {
    let mut best_start = 1;
    let mut best_len = 1;
    let mut path = Vec::with_capacity(256);

    for n in 1..limit {
        let len = get_len(n, memo, &mut path);
        if len > best_len {
            best_len = len;
            best_start = n;
        }
    }

    best_start
}

pub fn solve_with_iter(limit: u64) -> u64 {
    let mut memo = Memo::new(limit);
    find_longest_chain_with_path(limit, &mut memo, get_len_iter_with_path)
}

pub fn solve_with_loop(limit: u64) -> u64 {
    let mut memo = Memo::new(limit);
    find_longest_chain_with_path(limit, &mut memo, get_len_loop_with_path)
}
