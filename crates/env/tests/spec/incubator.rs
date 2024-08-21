use crate::spec::run_test;

macro_rules! test {
    ($file: ident) => {
        #[test]
        fn $file(){
            run_test("incubator", stringify!($file));
        }
    };
}


test!(multi_module);
