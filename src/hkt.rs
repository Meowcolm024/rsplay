pub trait Functor<A> {
    type F<T>;
    fn map<B>(x: Self::F<A>, f: &dyn Fn(A) -> B) -> Self::F<B>;
}

pub trait Applicative<A>: Functor<A> {
    fn pure(x: A) -> Self::F<A>;
    fn ap<B>(x: Self::F<A>, f: Self::F<Box<dyn Fn(A) -> B>>) -> Self::F<B>;
}

pub trait Monad<A>: Applicative<A> {
    fn flatmap<B>(x: Self::F<A>, f: &dyn Fn(A) -> Self::F<B>) -> Self::F<B>;
}

pub trait Alternative<A>: Applicative<A> {
    fn empty() -> Self::F<A>;
    fn orelse(this: Self::F<A>, that: Self::F<A>) -> Self::F<A>;
}

pub struct VecImpl;

impl<A> Functor<A> for VecImpl {
    type F<T> = Vec<T>;
    fn map<B>(x: Self::F<A>, f: &dyn Fn(A) -> B) -> Self::F<B> {
        let mut res = Vec::new();
        for e in x {
            res.push(f(e));
        }
        res
    }
}

impl<A: Copy> Applicative<A> for VecImpl {
    fn pure(x: A) -> Self::F<A> {
        vec![x]
    }

    fn ap<B>(x: Self::F<A>, f: Self::F<Box<dyn Fn(A) -> B>>) -> Self::F<B> {
        let mut res = Vec::new();
        for g in f {
            for e in &x {
                res.push(g(*e));
            }
        }
        res
    }
}

impl<A: Copy> Monad<A> for VecImpl {
    fn flatmap<B>(x: Self::F<A>, f: &dyn Fn(A) -> Self::F<B>) -> Self::F<B> {
        let mut res = Vec::new();
        for e in x {
            for t in f(e) {
                res.push(t);
            }
        }
        res
    }
}

impl<A: Copy> Alternative<A> for VecImpl {
    fn empty() -> Self::F<A> {
        Vec::new()
    }

    fn orelse(this: Self::F<A>, that: Self::F<A>) -> Self::F<A> {
        let mut res = this.clone();
        for x in that {
            res.push(x);
        }
        res
    }
}

pub struct OptionImpl;

impl<A> Functor<A> for OptionImpl {
    type F<T> = Option<T>;
    fn map<B>(x: Self::F<A>, f: &dyn Fn(A) -> B) -> Self::F<B> {
        match x {
            Some(x) => Some(f(x)),
            None => None,
        }
    }
}

impl<A> Applicative<A> for OptionImpl {
    fn pure(x: A) -> Self::F<A> {
        Some(x)
    }

    fn ap<B>(x: Self::F<A>, f: Self::F<Box<dyn Fn(A) -> B>>) -> Self::F<B> {
        match (f, x) {
            (Some(f), Some(x)) => Some(f(x)),
            _ => None,
        }
    }
}

impl<A> Monad<A> for OptionImpl {
    fn flatmap<B>(x: Self::F<A>, f: &dyn Fn(A) -> Self::F<B>) -> Self::F<B> {
        match x {
            Some(x) => f(x),
            None => None,
        }
    }
}

impl<A> Alternative<A> for OptionImpl {
    fn empty() -> Self::F<A> {
        None
    }

    fn orelse(this: Self::F<A>, that: Self::F<A>) -> Self::F<A> {
        match this {
            Some(_) => this,
            None => that,
        }
    }
}

pub fn guard<M, A>(p: bool, dummy: A) -> M::F<A>
where
    M: Alternative<A>,
{
    if p {
        M::pure(dummy)
    } else {
        M::empty()
    }
}

fn test<M>(x: M::F<i32>, y: M::F<i32>) -> M::F<i32>
where
    M: Monad<i32> + Alternative<i32>,
    M::F<i32>: Clone,
{
    M::flatmap(x, &|a| {
        M::flatmap(y.clone(), &|b| {
            M::flatmap(guard::<M, _>(a > b, 0), &|_| M::pure(a + b))
        })
    })
}
/*
    do
        a <- x
        b <- y
        guard $ a > b
        pure $ a + b
 */

pub fn hkt_main() {
    let v1 = vec![11, 45, 14];
    let v2: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(|x| x + 1), Box::new(|x| x * 2)];
    println!("{:?}", VecImpl::ap(v1, v2));
    let r1 = test::<OptionImpl>(Some(514), Some(114));
    println!("{:?}", r1);
    let r2 = test::<VecImpl>(vec![1, 2, 3, 4, 5], vec![0, 4, 6, 2, 8]);
    println!("{:?}", r2);

    let evens: Vec<i32> = VecImpl::flatmap((0..10).collect(), &|x| {
        VecImpl::flatmap(guard::<VecImpl, _>(x % 2 == 0, ()), &|_| VecImpl::pure(x))
    });
    println!("{:?}", evens);
}
