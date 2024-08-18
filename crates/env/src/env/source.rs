pub mod wasm_source {
    use alloc::string::String;

    pub struct Bytes<T> where T: AsRef<[u8]> {
        data: T,
    }

    impl<T> Bytes<T> where T: AsRef<[u8]> {
        pub fn as_ref(&self) -> &[u8] {
            self.data.as_ref()
        }
    }


    pub struct File {
        path: String,
    }

    pub struct Git {
        url: String,
    }

    pub fn bytes<T: AsRef<[u8]>>(data: T) -> Bytes<T> {
        Bytes { data }
    }
}

pub mod wat_source {
    pub struct String<T> where T: AsRef<str> {
        data: T,
    }

    impl<T> String<T> where T: AsRef<str> {
        pub fn as_ref(&self) -> &str {
            self.data.as_ref()
        }
    }

    pub fn string<T: AsRef<str>>(data: T) -> String<T> {
        String { data }
    }
}