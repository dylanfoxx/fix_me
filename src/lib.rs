#![no_std]
//! A very simple macro that lets you write temporary code that WILL NOT build in release mode. You can still run debug and test --release without issue. 
//!
//! fix_me has no overhead on any release or debug code, instead it provides a compile error if any fix_me code is still in the project at release time. 
//!
//! Feature flag unfixed_code will allow you to compile release code even with fix_me still in your code base.
//!
//!
//! Simple rust hello world with fix_me
//!
//! ```
//! use fix_me::fix_me;
//!
//! fn main() {
//!     fix_me!({
//!         println!("Hello, world!");
//!     });   
//! }
//! ```

// This group of tests should work during cargo test(release or debug). cargo bench still needs to be tested 
#[cfg(test)]
mod tests {
    use super::fix_me;

    #[test]
    fn macro_test() {
        fix_me!({
            //Some code we need to fix
        });
    }

    #[test]
    fn scope_test(){
        fix_me!({
            let num_thing = 13;
        });

        assert!(num_thing == 13);
    }

    #[test]
    fn  scope_test_2(){
        let test_string = String::from("test");

        fix_me!({
            let test_string = String::from("replace");
        });

        assert!(test_string == "replace");
    }

}


// This set of tests are used to make sure we can build in debug mode but not release
#[cfg(feature = "build_tests")]
mod build_tests {
    use super::fix_me;

    fn does_it_hold(){
        fix_me!({
            let mut num = 12;
        });

        num = 15;

        assert!(num == 15)
    }

    
}

/// A simple macro that errors(::core::compile_error!) if not in test, debug mode or feature unfixed_code is not set

// We "should" be able to use debug in the default profile. However for some reason we need to use debug_assert.
// This seems to be the correct way but may need to be changed.
#[macro_export]
macro_rules! fix_me {
    ({ $( $token:tt )* }) => {
        #[cfg(not(any(debug_assertions, test, feature = "unfixed_code")))]
        {
            ::core::compile_error!("Fix the code or use the 'unfixed_code' feature");
        }

        $( $token )*
    };
}
