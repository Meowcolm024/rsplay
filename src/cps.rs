type Cont<R, A> = dyn Fn(dyn Fn(A) -> R) -> R;

fn add_k<T>(a: i32, b: i32, k: &dyn Fn(i32) -> T) -> T {
    k(a + b)
}

fn mul_k<T>(a: i32, b: i32, k: &dyn Fn(i32) -> T) -> T {
    k(a * b)
}

fn if_k<S, T>(p: bool, t: &dyn Fn(()) -> S, e: &dyn Fn(()) -> S, k: &dyn Fn(S) -> T) -> T {
    if p {
        k(t(()))
    } else {
        k(e(()))
    }
}

fn id<T>(x: T) -> T {
    x
}

fn fact_k<T>(n: i32, k: &dyn Fn(i32) -> T) -> T {
    if n <= 1 {
        k(1)
    } else {
        fact_k(n - 1, &(|x| k(x * n)))
    }
}

fn fib_k<T>(n: i32, k: &dyn Fn(i32) -> T) -> T {
    if n <= 1 {
        k(1)
    } else {
        fib_k(n - 1, &|x| fib_k(n - 2, &|y| add_k(x, y, k)))
    }
}

pub fn cps_main() {
    let res = add_k(1, 2, &|x| if_k(x > 2, &|_| 1, &|_| 0, &id));
    println!("res = {}", res);
    let out = fact_k(5, &id);
    println!("fact 5 = {}", out);
    let fx = fib_k(10, &id);
    println!("fib 10 = {}", fx);
}
