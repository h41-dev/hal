use hal_core::module::Value;
use hal_env::{Environment, SpawnWat, wat_source};

#[test]
fn call_local() {
    let mut env = Environment::default();
    let mut state = env.spawn(wat_source::string(
        r#"(module
                  (func (export "call_doubler") (param i32) (result i32)
                    (local.get 0)
                    (call $double)
                  )
                  (func $double (param i32) (result i32)
                    (local.get 0)
                    (local.get 0)
                    i32.add
                  )
                )"#
    )).unwrap();

    let args = [Value::I32(21)];
    let expected = Some(Value::I32(42));
    let result = state.invoke("call_doubler", args).unwrap();
    assert_eq!(result, expected);
}
