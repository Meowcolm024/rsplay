use std::time::{self, Duration};

async fn foo() -> i32 {
    tokio::time::sleep(Duration::from_secs(1)).await;
    114
}

async fn bar() -> i32 {
    tokio::time::sleep(Duration::from_secs(2)).await;
    514
}

pub async fn fu_main() {
    let t = time::SystemTime::now();
    // println!("start {:?}", &t);
    let a = foo();
    let b = bar();
    let (a1, b1) = tokio::join!(a, b);
    tokio::task::spawn_blocking(move || println!("fu {} {:?}", a1 + b1, t.elapsed()))
        .await
        .unwrap()
}
