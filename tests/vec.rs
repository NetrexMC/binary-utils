use binary_utils::{varint::VarInt, Streamable, LE};

#[test]
fn test_varint() {
    let v = VarInt::<u32>(25565);
    let _val: Vec<u8> = vec![221, 199, 1];
    dbg!(VarInt::<u32>::from_be_bytes(&[255, 255, 255, 1][..]));
    dbg!(&v.to_be_bytes());
}

// test a string
#[test]
fn test_le_vec() {
    // LE bytes for "Netrex"
    let le_bytes_netrex: Vec<u8> = vec![120, 101, 114, 116, 101, 78, 6, 0];
    let str_bytes = LE("Netrex".to_string());
    println!("{:?}", str_bytes.fparse());

    assert_eq!(str_bytes.fparse(), le_bytes_netrex);

    let mut test: Vec<LE::<String>> = Vec::new();
    test.push(str_bytes.clone());

    // Vectors store length {stream, stream }
    // where "stream" in this case is [length, string bytes]
    let vector = test.fparse();
    println!("{:?}", vector);
    let restored = Vec::<LE::<String>>::fcompose(&vector[..], &mut 0);
    assert_eq!(restored[0].clone().inner(), str_bytes.inner())
}
