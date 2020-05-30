use rayon::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};

fn main() {
    let (s1, r1): (SyncSender<[u8; 4096]>, Receiver<[u8; 4096]>) = mpsc::sync_channel(4);
    let (s2, r2): (SyncSender<[u8; 4096]>, Receiver<[u8; 4096]>) = mpsc::sync_channel(4);
    let producer = std::thread::spawn(move || loop {
        use rand::random;
        let matrix: [u8; 4096] = [random(); 4096];
        s1.send(matrix);
        s2.send(matrix);
    });
    let consumer = std::thread::spawn(move || loop {
        let matrix = r1.recv().unwrap();
        let n = std::time::Instant::now();
        let sum = matrix
            .into_par_iter()
            .fold_with(0_u64, |a: u64, b: &u8| a + (*b as u64))
            .sum::<u64>();
        let n2 = std::time::Instant::now();
        println!("t1 {} in {:?}", sum, n2 - n);
    });
    let consumer2 = std::thread::spawn(move || loop {
        let matrix = r2.recv().unwrap();
        let n = std::time::Instant::now();
        let sum = matrix
            .into_par_iter()
            .fold_with(0_u64, |a: u64, b: &u8| a + (*b as u64))
            .sum::<u64>();
        let n2 = std::time::Instant::now();
        println!("t2 {} in {:?}", sum, n2 - n);
    });

    producer.join();
}
