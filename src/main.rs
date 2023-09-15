#![feature(never_type)]

// use crate::fu::fu_main;

// mod cps;
// mod gadts;
// mod hkt;
// mod fu;
// mod ge;
// mod lk;
// mod st;
// mod trie;
mod re;
mod hs;
// mod gr;
mod bc;
mod rrc;
mod lru;
mod lfu;

#[inline(always)]
fn absurd<T>(x: !) -> T {
    match x {}
}

fn safeunwrap<T>(e: Result<T, !>) -> T {
    match e {
        Ok(x) => x,
        Err(x) => absurd(x),
    }
}

trait IO<T> {
    fn print(&self, s: &str) -> T;
    fn read(&self) -> String;
}

trait Bar {
    fn bar(&self) -> i32;
}

fn foo<T, IOimpl: IO<T> + Bar>(greet: &str, io: &IOimpl) -> T {
    let name = io.read();
    io.print(format!("{} {} {} times!", greet, name.as_str(), io.bar()).as_str())
}

struct RawIO;
impl IO<()> for RawIO {
    fn print(&self, s: &str) -> () {
        println!("{}", s);
    }
    fn read(&self) -> String {
        String::from("Ben")
    }
}
impl Bar for RawIO {
    fn bar(&self) -> i32 {
        114514
    }
}

struct StrIO;
impl IO<String> for StrIO {
    fn print(&self, s: &str) -> String {
        String::from(s)
    }
    fn read(&self) -> String {
        String::from("Tom")
    }
}
impl Bar for StrIO {
    fn bar(&self) -> i32 {
        1919810
    }
}

fn main() {
    // let raw = RawIO;
    // let strio = StrIO;
    // foo("Hello", &raw);
    // println!("{}", foo("Hi", &strio));
    // println!("Hello, world!");
    // cps::cps_main();
    // hkt::hkt_main();
    // gadts::gadt_main();
    // let a = safeunwrap(Ok(114514));
    // println!("{}", a);

    // ge::ge_main();
    
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(fu_main());
    // lk::lk_main();
    // st::st_main();
    // trie::trie_main();
    // re::re_main();
    // hs::hs_main();
    // gr::gr_main();
    // bc::bc_main();
    // rrc::rrc_main();
    // lru::lru_main();
    lfu::lfu_main();
}
