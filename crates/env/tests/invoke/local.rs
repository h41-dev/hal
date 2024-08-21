use hal_core::module::Value;
use hal_env::{Environment, SpawnWat, wat_source};

#[test]
fn i32_i32_to_i32() {
    let mut env = Environment::default();
    let mut instance = env.spawn(wat_source::string(
        r#"(module
                      (func (export "add") (param i32 i32) (result i32)
                        (local.get 0)
                        (local.get 1)
                        i32.add
                      )
                    )"#
    )).unwrap();

    let args = vec![Value::I32(40), Value::I32(2)];
    let expected = [Value::I32(42)];
    let result = instance.invoke("add", args).unwrap();
    assert_eq!(result.as_ref(), expected);
}