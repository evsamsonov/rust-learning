use std::thread;
use std::time::Duration;

// see https://doc.rust-lang.ru/book/ch16-01-threads.html

fn main() {
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

    handle.join().unwrap()
}
