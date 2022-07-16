use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use sha1::{Sha1, Digest};
use hex;

const THREADS: usize = 16;

fn main() {
    //Output
    let (o_tx, o_rx) = channel::<String>();

    let prefix = "team_name:5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8:dddddd9f51b63e325971b5d923cfc170178841e7";

    let mut threads: Vec<Sender<u32>> = Vec::new();

    // Create 8 threads
    for _ in 0..THREADS {
        // Input
        let (i_tx, i_rx) = channel::<u32>();
        let tx = o_tx.clone();

        threads.push(i_tx);

        thread::spawn(move|| {

            hash_worker(prefix, i_rx, tx);
        });
    }

    //Spawn a thread for spawning threads....
    thread::spawn(move|| {
        for i in 1..99999999 {
            threads[i % THREADS].send(i as u32).unwrap();
        }
    });

    println!("{}", o_rx.recv().unwrap());
}

fn hash_worker(prefix: &str, input_channel: Receiver<u32>, output_channel: Sender<String>) {
    loop {
        let number = input_channel.recv().unwrap();

        let to_hash = format!("{}:{}", prefix, number);

        let hash = hash(to_hash.as_bytes());

        if hash.starts_with("dddddd") {
            //println!("{}", hash);

            output_channel.send(format!("{}:{},{}", prefix, number, hash)).unwrap();
            return;
        }
    }
}

fn hash(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let mut hash: [u8; 20] = [0; 20];
    let result = hasher.finalize();

    for i in 0..20 {
        hash[i] = result[i];
    }

    return hex::encode(hash);
}
