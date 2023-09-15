trait Expr<A> {
    fn eval(&self) -> A;
}

struct Lit(i32);

impl Expr<i32> for Lit {
    fn eval(&self) -> i32 {
        self.0
    }
}

struct Plus(i32, i32);

impl Expr<i32> for Plus {
    fn eval(&self) -> i32 {
        self.0 + self.1
    }
}

struct Equals<'a>(&'a dyn Expr<i32>, &'a dyn Expr<i32>);

impl Expr<bool> for Equals<'_> {
    fn eval(&self) -> bool {
        self.0.eval() == self.1.eval()
    }
}

struct Cond<'a, T>(&'a dyn Expr<bool>, &'a dyn Expr<T>, &'a dyn Expr<T>);

impl<T> Expr<T> for Cond<'_, T> {
    fn eval(&self) -> T {
        if self.0.eval() {
            self.1.eval()
        } else {
            self.2.eval()
        }
    }
}

struct Tuple<'a, A, B>(&'a dyn Expr<A>, &'a dyn Expr<B>);

impl<A, B> Expr<(A, B)> for Tuple<'_, A, B> {
    fn eval(&self) -> (A, B) {
        (self.0.eval(), self.1.eval())
    }
}

struct Fst<'a, A, B>(&'a dyn Expr<(A, B)>);

impl<A, B> Expr<A> for Fst<'_, A, B> {
    fn eval(&self) -> A {
        self.0.eval().0
    }
}

struct Snd<'a, A, B>(&'a dyn Expr<(A, B)>);

impl<A, B> Expr<B> for Snd<'_, A, B> {
    fn eval(&self) -> B {
        self.0.eval().1
    }
}

pub fn gadt_main() {
    let e0 = Equals(&Lit(1), &Lit(2));
    let e1 = Fst(&Tuple(&Lit(3), &Equals(&Lit(1), &Lit(1))));
    let e2 = Snd(&Tuple(&Lit(5), &Lit(6)));
    let p = Cond(&e0, &e1, &e2);
    let res = p.eval();
    println!("{}", res);
}
