
#[derive(Clone, Debug)]
pub struct Perm {
    pub(crate) orig2new: Vec<usize>,
    pub(crate) new2orig: Vec<usize>,
}


#[derive(Clone, Debug)]
struct Slice {
    begin: usize,
    end: usize,
}

impl Slice {
    fn elems<'a>(&self, storage: &'a [usize]) -> &'a [usize] {
        &storage[self.begin..self.end]
    }
}

#[derive(Debug)]
struct ColsQueue {
    score2head: Vec<Option<usize>>,
    prev: Vec<usize>,
    next: Vec<usize>,
    min_score: usize,
    len: usize,
}


impl ColsQueue {
    fn new(num_cols: usize) -> ColsQueue {
        ColsQueue {
            score2head: vec![None; num_cols],
            prev: vec![0; num_cols],
            next: vec![0; num_cols],
            min_score: num_cols,
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn pop_min(&mut self) -> Option<usize> {
        let col = loop {
            if self.min_score >= self.score2head.len() {
                println!("None branch");
                return None;
            }
            if let Some(col) = self.score2head[self.min_score] {
                break col;
            }
            self.min_score += 1;
        };

        self.remove(col, self.min_score);
        Some(col)
    }

    fn add(&mut self, col: usize, score: usize) {
        self.min_score = std::cmp::min(self.min_score, score);
        self.len += 1;

        if let Some(head) = self.score2head[score] {
            self.prev[col] = self.prev[head];
            self.next[col] = head;
            self.next[self.prev[head]] = col;
            self.prev[head] = col;
        } else {
            self.prev[col] = col;
            self.next[col] = col;
            self.score2head[score] = Some(col);
        }
    }

    fn remove(&mut self, col: usize, score: usize) {
        self.len -= 1;
        if self.next[col] == col {
            self.score2head[score] = None;
        } else {
            self.next[self.prev[col]] = self.next[col];
            self.prev[self.next[col]] = self.prev[col];
            if self.score2head[score].unwrap() == col {
                self.score2head[score] = Some(self.next[col]);
            }
        }
    }
}


/// Lower block triangular form of a matrix.
#[derive(Clone, Debug)]
pub struct BlockDiagForm {
    /// Row permutation: for each original row its new row number so that diag is nonzero.
    pub row2col: Vec<usize>,
    /// For each block its set of columns (the order of blocks is lower block triangular)
    pub block_cols: Vec<Vec<usize>>,
}


pub fn order_simple<'a>(size: usize, get_col: impl Fn(usize) -> &'a [usize]) -> Perm {
    let mut cols_queue = ColsQueue::new(size);
    for c in 0..size {
        cols_queue.add(c, get_col(c).len() - 1);
    }

    let mut new2orig = Vec::with_capacity(size);
    while new2orig.len() < size {
        let min = cols_queue.pop_min();
        println!("min: {:?}", min);
        //TODO panic happens here
        new2orig.push(min.unwrap());
    }

    let mut orig2new = vec![0; size];
    for (new, &orig) in new2orig.iter().enumerate() {
        orig2new[orig] = new;
    }

    Perm { orig2new, new2orig }
}



fn main() {
    order_simple(4, |c| {
        match c {
            0 => &[0, 1, 2, 3],
            1 => &[2],
            2 => &[0, 1],
            3 => &[1, 2, 3],
            _ => unreachable!(),
        }
    });
    println!("All ok! Try running in release mode")
}