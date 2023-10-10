//! A library for the game "Quoridor".
//! 
//! # Examples
//! 
//! ```
//! use quoridor::add;
//! 
//! let result = add(2, 2);
//! assert_eq!(result, 4);
//! ```

mod board;
mod types;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
