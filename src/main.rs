use quorigen::{board::Board, perft};

fn main() {
    let start = std::time::Instant::now();
    let mut map = std::collections::HashMap::new();
    for depth in 0.. {
        let count = perft::perft_cached(Board::default(), depth, &mut map);
        println!("perft({}) = {} in {:.3}s", depth, count, start.elapsed().as_secs_f64());
    }
}