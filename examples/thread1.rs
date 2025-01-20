use anyhow::Result;
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;
#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    //创建Producer
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx); //关闭发送端
              //创建Consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received message: {:?}", msg);
        }
        println!("Consumer is exiting");
        42
    });
    let result = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Thread Join Error: {:?}", e))?;

    println!("Hello, world!{}", result);
    Ok(())
}
fn producer(id: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value: usize = rand::random::<usize>();
        tx.send(Msg::new(id, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        let sleep_duration = Duration::from_millis(sleep_time);
        thread::sleep(sleep_duration);
        if rand::random::<u8>() % 5 == 0 {
            println!("Producer {} is exiting", id);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}
