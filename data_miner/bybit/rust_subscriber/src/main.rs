use std::collections::HashMap;
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

mod algo;
mod fs_cache;
mod market_data;
mod objects;
mod updater;
extern crate bybit;
const PAIRS_PATH: &str = "../routes.txt";
const LOOP_PATH: &str = "../symbol_loops.csv";

fn main() -> Result<(), Error> {
    let possible_pairs = fs_cache::read_pairs(PAIRS_PATH)?;
    let sym_to_pair = Arc::new(
        possible_pairs
            .iter()
            .map(|p| (p.to_bybit_symbol(), p.clone()))
            .collect::<HashMap<_, _>>(),
    );

    let _loops = fs_cache::read_loops(LOOP_PATH)?;

    let market_data = Arc::new(Mutex::new(market_data::MarketData::new(&possible_pairs)));

    let _threads =
        updater::run_trades_population(&market_data, &sym_to_pair, &possible_pairs);

    loop {
        let rt_locked = market_data.lock().unwrap();
        println!(
            "recent_trades size: total={}, filled={}, empty={}",
            rt_locked.trades_zero_cnt,
            rt_locked.trades_total_cnt - rt_locked.trades_zero_cnt,
            rt_locked.trades_zero_cnt
        );
        sleep(Duration::from_secs(2));
    }

    // for t in threads {
    //     let _ = t.join();
    // }

    Ok(())
}
