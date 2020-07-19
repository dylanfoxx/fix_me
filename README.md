 A very simple macro that lets you write temporary code that WILL NOT build in release mode. You can still run debug and test --release without issue. 

 fix_me has no overhead on any release or debug code, instead it provides a compile error if any fix_me code is still in the project at release time. 


 Feature flag unfixed_code will allow you to compile release code even with fix_me still in your code base.


 Simple rust hello world with fix_me

 ```
 use fix_me::fix_me;

 fn main() {
     fix_me!({
         println!("Hello, world!");
     });   
 }
 ```