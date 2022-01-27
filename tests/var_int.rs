use binary_utils::Streamable;
use binary_utils::*;

#[test]
fn read_write_var_int() {
    let one = VarInt::<u32>(2147483647);
    let two = VarInt::<u32>(255);
    let buf_one = one.parse().unwrap();
    let buf_two = two.parse().unwrap();

    assert_eq!(buf_one, vec![255, 255, 255, 255, 7]);
    assert_eq!(buf_two, vec![255, 1]);

    let buf_long_one = VarInt::<u64>(9223372036854775807).parse().unwrap();
    assert_eq!(
        buf_long_one,
        vec![255, 255, 255, 255, 255, 255, 255, 255, 127]
    );

    assert_eq!(
        one.0,
        VarInt::<u32>::compose(&buf_one[..], &mut 0).unwrap().0
    );

    assert_eq!(
        two.0,
        VarInt::<u32>::compose(&buf_two[..], &mut 0).unwrap().0
    );

    // test reading
    let buf_game_id: Vec<u8> = vec![2, 0, 0, 0, 5];
    let int_game_id = VarInt::<u32>::compose(&buf_game_id[..], &mut 0).unwrap();
    assert_eq!(int_game_id.0, 2);
}

#[test]
fn var_int_test_middle() {
    // false, false, byte, varint (255), varint (1)
    let buffer = vec![0, 0, 0, 255, 1, 0, 0];
    let mut position = 0;

    assert_eq!(u24::compose(&buffer[..], &mut position).unwrap().inner(), 0);

    assert_eq!(
        VarInt::<u32>::compose(&buffer[..], &mut position)
            .unwrap()
            .0,
        255
    );
}
