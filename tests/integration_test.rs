// This group of tests should work during cargo test(release or debug).
// Cargo bench still needs support.

#[test]
fn macro_test() {
    fix_me::fix_me!(
        //Some code we need to fix
    );
}

#[test]
fn scope_test(){
    fix_me::fix_me!(
        let num_thing = 13;
    );

    assert!(num_thing == 13);
}

#[test]
fn scope_test_2(){
    let test_string = String::from("test");

    fix_me::fix_me!(
        let test_string = String::from("replace");
    );

    assert!(test_string == "replace");
}

#[cfg(test)]
mod test {
    
    fix_me::fix_me!(
        fn to_fix() -> bool{
            let fix = false;
            fix
        }
    );

    #[test]
    fn test_1(){
        let val = to_fix();
    }
}