#![no_std]
//! Fix me is useful for writing temporary code that will be fixed later.
//! It replaces comments like 
//! ```text
//! //FIXME:Rework error handling 
//!    or
//! //TODO:Add logging
//! ```
//!
//! Unlike comments fix_me is enforced by the compiler.
//!   
//! fix_me is a simple macro that lets you write temporary code that WILL NOT build in release mode.
//! - You can still compile debug builds and run test --release without issue. 
//! - fix_me has no overhead on any release or debug code, instead it provides a compile error if any fix_me code is still in the project at release time.
//!
//! Simple fix_me hello world
//!
//! ```
//! //Use on functions
//! fix_me::fix_me!(
//!     fn only_false() -> bool {
//!         use fix_me::fix_me;
//!         //Or in functions
//!         fix_me!(
//!            return true;
//!         );
//!     }
//! );
//!
//! fn main() {
//!     match only_false() {
//!         false => println!("Hello"),
//!         _ => {}
//!     }       
//! }
//! ```
//! 
//!
//! It is recommended to add it to the dependencies as you are making changes then remove it as you finish your work.
//!
//! Feature flag unfixed_code will allow you to compile release code even with fix_me still in your code base.

/// A simple macro that errors(::core::compile_error!) if not in test, debug mode or feature unfixed_code is not set

// We "should" be able to use debug in the default profile. However for some reason we need to use debug_assertions.
// This seems to be the correct way but may need to be changed.
#[macro_export]
macro_rules! fix_me {
    ( $( $token:tt )* ) => {
        #[cfg(not(any(debug_assertions, test, feature = "unfixed_code")))]
        ::core::compile_error!("Fix the code or use the 'unfixed_code' feature");
        
        $( $token )*
    };
    () => {
        #[cfg(not(any(debug_assertions, test, feature = "unfixed_code")))]
        ::core::compile_error!("Fix the code or use the 'unfixed_code' feature");
    };
}