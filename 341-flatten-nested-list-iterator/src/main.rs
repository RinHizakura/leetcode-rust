#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    collect: Vec<i32>,
    idx: usize,
    len: usize,
}

impl NestedIterator {
    fn flatten(list: &Vec<NestedInteger>, collect: &mut Vec<i32>) {
        for e in list {
            match e {
                NestedInteger::Int(i) => {
                    collect.push(*i);
                }
                NestedInteger::List(v) => {
                    Self::flatten(&v, collect);
                }
            };
        }
    }

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut collect = vec![];
        Self::flatten(&nested_list, &mut collect);
        let len = collect.len();
        NestedIterator {
            collect: collect,
            len: len,
            idx: 0,
        }
    }

    fn next(&mut self) -> i32 {
        let result = self.collect[self.idx];
        self.idx += 1;
        result
    }

    fn has_next(&self) -> bool {
        self.idx < self.len
    }
}

fn main() {
    let nested_list = vec![NestedInteger::Int(1)];
    let mut obj = NestedIterator::new(nested_list);
    if obj.has_next() {
        let ret = obj.next();
        println!("{}", ret);
    }
}
