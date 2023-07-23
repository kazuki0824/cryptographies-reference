use crate::expand_00;

#[test]
fn test_expand_00() {
    let key: u64 = 0;
    let answer = [3446828369, 103857313, 4149429448, 3104314966 as u32];
    let key = expand_00(key, 0);

    assert_eq!(key, answer);
}
