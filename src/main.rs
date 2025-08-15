//use rand::{ Rng};
//use rayon::prelude::*;
//use std::time::Duration;

//fn compute_job(job: i64) -> i64 {
//    let mut rng = rand::rng();
//    let sleep_ms: u64 = rng.random_range(0..10);
//    std::thread::sleep(Duration::from_millis(sleep_ms));

//    job * job
//}

//fn process_result(result: i64) {
//    println!("{}", result);
//}

//fn main() {
//    let jobs = 0..100;
// By default, the thread pool has a size equal to the number of logical CPUS of machine.
//    jobs.into_par_iter().map(compute_job).for_each(process_result);
//}
//
//For I/O (Input/Output) bound jobs, we need to move to async land. More precisely, we wi//ll use Streams, which are async Iterators that can process items concurrently.
//
//use futures::{stream, StreamExt};
//use rand::{Rng};
//use std::time::Duration;

//async fn compute_job(job: i64) -> i64 {
//    let mut rng = rand::rng();
//    let sleep_ms: u64 = rng.gen_range(0..10);
//    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

//    job * job;
//}

//async fn process_result(result: i64) {
//    println!("{}", result);
//}
//#[tokio::main]
//async fn main() {
//    let jobs = 0..100;
//    let concurrency = 42;

//   stream::iter(jobs)
//        .for_each_concurrent(concurrency, |job//| async move {
//            let result = compute_job(job).await;
//            process_result(result).await;
//        })
//        .await;
//}

use futures::{stream, StreamExt};
use rand::Rng;
use std::time::Duration;

async fn compute_job(job: i64) -> i64 {
    let mut rng = rand::rng();
    let sleep_ms: u64 = rng.random_range(0..10);
    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

    job * job
}

async fn process_result(result: i64) {
    println!("{}", result);
}
#[tokio::main]
async fn main() {
    let jobs = 0..100;
    let concurrency = 42;

    // stream::iter(jobs)
    //     .map(compute_job)
    //     .buffer_unordered(concurrency)
    //     .for_each(process_result)
    //     .await;
    let _results: Vec<i64> = stream::iter(jobs)
        .map(compute_job)
        .buffer_unordered(concurrency)
        .collect()
        .await;
}
