use hal_core::module::{Value, ValueType};
use hal_core::module::Value::I32;
use hal_env::{Environment, LoadWasm, wat_source};

#[test]
fn i32() {
    test_method(ValueType::I32, I32(42))
}


fn test_method(vt: ValueType, value: Value) {
    let mut env = Environment::default();
    let mut state = env.load(wat_source::string(
        r#"(module
  (memory 1)
  (func $store
    ({vt}.const 0)
    ({vt}.const {value})
    ({vt}.store)
  )
  (export "store_fn" (func $store))
)"#.replace("{vt}", vt.to_str()).replace("{value}", &*value.to_string())
    )).unwrap();

    let invocation = state.invoke("store_fn", []);
    assert!(invocation.is_ok());

    let memory = state.memory(0).unwrap();
    assert_eq!(memory.data.borrow()[0], 42);
}
