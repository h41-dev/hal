use crate::spec::run_test;

macro_rules! test {
    ($file: ident) => {
        #[test]
        fn $file(){
            run_test("core", stringify!($file));
        }
    };
}

// test!(f32);
test!(i32);
test!(i64);