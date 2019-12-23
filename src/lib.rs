pub struct Repeat<I: Iterator> {
    iter: I,
    item: Option<I::Item>,
    n: usize,
    counter: usize,
}

impl<I> Iterator for Repeat<I>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 0 {
            self.counter = self.n;
            self.item = self.iter.next();
        } else {
            self.counter -= 1;
        }

        match self.item {
            Some(item) => Some(item),
            None => None,
        }
    }
}

impl<I> Repeat<I>
where
    I: Iterator,
{
    fn new(iter: I, n: usize) -> Self {
        Self {
            iter,
            item: None,
            n: n - 1,
            counter: 0,
        }
    }
}

pub trait RepeatIteratorAdapter
where
    Self: Sized + Iterator,
{
    fn repeat(self, n: usize) -> Repeat<Self>;
}

impl<I> RepeatIteratorAdapter for I
where
    I: Iterator,
{
    fn repeat(self, n: usize) -> Repeat<Self> {
        Repeat::new(self, n)
    }
}

#[cfg(test)]
mod tests {
    use super::RepeatIteratorAdapter;

    #[test]
    fn it_works() {
        let input = vec![-1, 0, 1];
        let expected = vec![-1, -1, -1, 0, 0, 0, 1, 1, 1];
        let actual: Vec<i32> = input
            .into_iter()
            .repeat(3)
            .collect();
        assert_eq!(actual, expected);
    }
}
