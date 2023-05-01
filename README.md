# binary_utils
A panic-free way to read/write binary streams in rust.

[Documentation](https://docs.rs/binary_utils/) |
[Discord](https://discord.gg/y4aWA5MQxK)

## Generic Usage
```rust
use binary_utils::{BinaryIo, BinaryReader, BinaryWriter};

#[derive(BinaryIo)]
pub struct MyStruct {
    pub a: u32,
    pub b: u32,
}

fn main() {
    let mut writer = BinaryWriter::new();
    let my_struct = MyStruct { a: 1, b: 2 };
    if let Err(_) = writer.write(&my_struct) {
        println!("Failed to write MyStruct");
    }

    let mut reader = BinaryReader::from(writer);
    if let Ok(my_struct2) = MyStruct::read(&mut reader) {
        assert_eq!(my_struct, my_struct2);
    } else {
        println!("Failed to read MyStruct");
    }
}
```

For more examples and usage, please refer to the [Documentation](https://docs.rs/binary_utils/).