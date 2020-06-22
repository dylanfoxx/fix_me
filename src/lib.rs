// This group of tests should work during cargo test(release or debug) as well as cargo bench
// This is a simple macro but we may need to test more extensively 
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
// May need to remove doc
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


#[macro_export]
macro_rules! fix_me {
    ({ $( $token:tt )* }) => {
        #[cfg(not(any(debug_assertions, test, feature = "unfixed_code")))]
        {
            ::std::compile_error!("Fix the code or use the 'unfixed_code' feature");
        } 

        $( $token )*
    };
}
