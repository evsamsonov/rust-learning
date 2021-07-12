use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    thread_join_and_move();
    thread_channel();
}

// see https://doc.rust-lang.ru/book/ch16-01-threads.html
fn thread_join_and_move() {
    println!("Thread join and move");
    println!("===");

    let v = vec![1, 2, 3];

    // move позволяет замыканию забрать v во владение
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread! {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Ошибка компиляции. Основной поток больше не владеет v
    // drop(v);

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    println!();
}

// see https://doc.rust-lang.ru/book/ch16-02-message-passing.html
fn thread_channel() {
    println!("Thread channel");
    println!("===");

    let (tx, rx) = mpsc::channel();

    let handle_sender = thread::spawn(move || {
        loop {
            let received = rx.recv();
            match received {
                Ok(i) => println!("Got {} from channel", i),
                Err(_e) => {
                    break;
                }
            }
        }
    });

    let handle_receiver = thread::spawn(move || {
        for i in 1..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle_sender.join().unwrap();
    handle_receiver.join().unwrap();
    println!();
}
