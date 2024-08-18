#[cfg(test)]
mod tests {
    use hal_core::module::Value;
    use hal_env::{SingleThreadedEnvironment, SpawnWat, wat_source};

    #[test]
    fn invoke_function() {
        let tests = vec![
            (2, 3, 5),
            (10, 5, 15),
            (1, 1, 2),
        ];

        let mut env = SingleThreadedEnvironment::default();
        let mut handle = env.spawn(wat_source::string(
            r#"(module
                      (func (export "add") (param i32 i32) (result i32)
                        (local.get 0)
                        (local.get 1)
                        i32.add
                      )
                    )"#
        )).unwrap();

        for (left, right, want) in tests {
            let args = vec![Value::I32(left), Value::I32(right)];
            let result = handle.invoke("add", args).unwrap();
            assert_eq!(result, Some(Value::I32(want)));
        }

        print!("{:?}", handle);
    }
}