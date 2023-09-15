use std::marker::PhantomData;

trait G<T> {
    fn get(&self) -> &T;
}

fn haha<'a, T>(g: &dyn G<&'a T>)
where
    T: std::fmt::Debug,
{
    println!("{:?}", g.get());
}

#[derive(Debug)]
struct Tese<T>(T);

impl<T> G<T> for Tese<T> {
    fn get(&self) -> &T {
        &self.0
    }
}

pub fn ge_main() {
    let x = 114;
    let t = Tese(&x);
    haha(&t);

    let l1 = Cons(1, Box::new(Cons(2, Box::new(Nil(PhantomData)))));
    println!("{}", l1.tail().head());
}

trait List {
    type A;
    fn head(&self) -> &Self::A;
    fn tail(&self) -> &Box<dyn List<A = Self::A>>;
}

struct Nil<A>(PhantomData<A>);

impl<A> List for Nil<A> {
    type A = A;
    fn head(&self) -> &Self::A {
        unimplemented!()
    }
    fn tail(&self) -> &Box<dyn List<A = Self::A>> {
        unimplemented!()
    }
}

struct Cons<A>(A, Box<dyn List<A = A>>);

impl<A> List for Cons<A> {
    type A = A;
    fn head(&self) -> &Self::A {
        &self.0
    }
    fn tail(&self) -> &Box<dyn List<A = Self::A>> {
        &self.1
    }
}
