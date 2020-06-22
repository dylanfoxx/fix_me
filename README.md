
A very simple macro that lets you write temporary code that wont build in release mode. You can still run debug and test --release without issue. 

fix_me has no overhead on any release or debug code, instead it provides a compile error if any fix_me code is still in the project at release time. 



Simple rust hello world with fix_me

use fix_me::fix_me;

fn main() {
    fix_me!({
        println!("Hello, world!");
    });   
}

This is cargo expand output with fix_me.

fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Hello, world!\n"],
            &match () {
                () => [],
            },
        ));
    };
}


this is Cargo expand output without fix_me.

fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["Hello, world!\n"],
            &match () {
                () => [],
            },
        ));
    };
}