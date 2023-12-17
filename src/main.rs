use std::time::Instant;

use query_problem::{Order, Query, query_naive, query_binsearch};
use rand::Rng;

const MIN_QUERIES: usize = 1_000;
const MAX_QUERIES: usize = 100_000;
const MIN_ORDERS: usize = 1_000;
const MAX_ORDERS: usize = 100_000;

const MIN_TIME: u64 = 0;
const MAX_TIME: u64 = 10_000;
const MIN_DURATION: u64 = 1;
const MAX_DURATION: u64 = 1_000;

const MIN_SHARES: u64 = 1;
const MAX_SHARES: u64 = 50_000;

fn main() {
    let mut rng = rand::thread_rng();
    let num_orders = rng.gen_range(MIN_ORDERS..MAX_ORDERS);
    let num_queries = rng.gen_range(MIN_QUERIES..MAX_QUERIES);

    println!("will generate {num_orders} orders and {num_queries} queries");

    let mut orders = Vec::with_capacity(num_orders);
    for _ in 0..num_orders {
        let start = rng.gen_range(MIN_TIME..MAX_TIME - MIN_DURATION);
        let end = rng.gen_range(start+MIN_DURATION..(start+MAX_DURATION).min(MAX_TIME));
        orders.push(Order {
            created_at: start,
            executed_or_cancelled_at: end,
            number_of_shares: rng.gen_range(MIN_SHARES..=MAX_SHARES),
        });
    }
    println!("Generated all orders");

    let mut queries = Vec::with_capacity(num_queries);
    for _ in 0..num_queries {
        queries.push(Query {
            time: rng.gen_range(MIN_TIME..MAX_TIME),
        });
    }

    println!("Generated all queries");

    println!("Running binsearch computation");
    let start_binsearch = Instant::now();
    let binsearch_results = query_binsearch(&orders, &queries);
    let end_binsearch = Instant::now();
    let binsearch_duration = end_binsearch - start_binsearch;
    println!("Binsearch computation done in {} seconds", binsearch_duration.as_secs_f64());

    println!("Running naive computation");
    let start_naive = Instant::now();
    let naive_results = query_naive(&orders, &queries);
    let end_naive = Instant::now();
    let naive_duration = end_naive - start_naive;
    println!("Naive computation done in {} seconds", naive_duration.as_secs_f64());

    assert_eq!(naive_results , binsearch_results);
}
