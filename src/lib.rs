#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
//! A library for the game "Quoridor".

pub mod board;
mod types;
mod squareset;
pub mod perft;

#[cfg(test)]
mod tests {
}
