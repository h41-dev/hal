use std::fmt::Display;

use hal_core::module::{Value, ValueType};
use hal_env::{Environment, SpawnWat, wat_source};
use Value::{I32, I64};

#[test]
fn i32() {
    test_method(ValueType::I32, [I32(1), I32(2)], I32(3))
}

#[test]
fn i64() {
    test_method(ValueType::I64, [I64(1), I64(2)], I64(3))
}


fn test_method(vt: ValueType, args: impl AsRef<[Value]>, expected: Value) {
    let args = &args;
    let expected = expected;

    let mut env = Environment::default();
    let mut instance = env.spawn(wat_source::string(
        r#"(module
                      (func (export "add") (param {vt} {vt}) (result {vt})
                        (local.get 0)
                        (local.get 1)
                        {vt}.add
                      )
                    )"#.replace("{vt}", vt.to_str())
    )).unwrap();

    let result = instance.invoke("add", args).unwrap();
    assert_eq!(result.as_ref(), [expected], "{}", vt);
}
