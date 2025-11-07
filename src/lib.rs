#![no_std]
use core::mem;

pub fn unfold<T, F>(init: T, next: F) -> Unfold<T, F>
where
    F: Fn(&T) -> T,
{
    Unfold { state: init, next }
}

pub struct Unfold<T, F> {
    state: T,
    next: F,
}

impl<T, F> Iterator for Unfold<T, F>
where
    F: Fn(&T) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.next)(&self.state);
        let prev = mem::replace(&mut self.state, next);
        Some(prev)
    }
}

pub fn try_unfold<T, F>(init: T, next: F) -> TryUnfold<T, F>
where
    F: Fn(&T) -> Option<T>,
{
    let state = Some(init);
    TryUnfold { state, next }
}

pub struct TryUnfold<T, F> {
    state: Option<T>,
    next: F,
}

impl<T, F> Iterator for TryUnfold<T, F>
where
    F: Fn(&T) -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.state.take()?;
        self.state = (self.next)(&prev);
        Some(prev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfold() {
        let mut iter = unfold(0, |&x| x + 1);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn test_try_unfold() {
        let mut iter = try_unfold(0, |&x| if x < 2 { Some(x + 1) } else { None });
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }
}
