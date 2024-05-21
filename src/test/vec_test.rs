use super::Vec3;

#[test]
fn vec_add() {
    let v1 = Vec3::new(1, 2, 3);
    let v2 = Vec3::new(3, 2, 1);

    let v3 = Vec3::new(1.0, 2.0, 3.0);
    let v4 = Vec3::new(0.0, 1.0, 2.0);

    assert_eq!(Vec3::new(4, 4, 4), v1 + v2);
    assert_eq!(Vec3::new(1.0, 3.0, 5.0), v3 + v4);
}

#[test]
fn vec_sub() {
    let v1 = Vec3::new(1, 2, 3);
    let v2 = Vec3::new(3, 2, 1);

    let v3 = Vec3::new(1.0, 2.0, 3.0);
    let v4 = Vec3::new(0.0, 1.0, 2.0);

    assert_eq!(Vec3::new(-2, 0, 2), v1 - v2);
    assert_eq!(Vec3::new(1.0, 1.0, 1.0), v3 - v4);
}

#[test]
fn vec_mul() {
    let v1 = Vec3::new(1, 2, 3);
    let v2 = Vec3::new(3, 2, 1);

    let v3 = Vec3::new(1.0, 2.0, 3.0);
    let v4 = Vec3::new(0.0, 1.0, 2.0);

    assert_eq!(Vec3::new(3, 4, 3), v1 * v2);
    assert_eq!(Vec3::new(0.0, 2.0, 6.0), v3 * v4);
}

#[test]
fn vec_div() {
    let v1 = Vec3::new(2, 3, 4);
    let v2 = Vec3::new(2, 3, 4);

    let v3 = Vec3::new(1.0, 2.0, 3.0);
    let v4 = Vec3::new(1.0, 1.0, 2.0);

    assert_eq!(Vec3::new(1, 1, 1), v1 / v2);
    assert_eq!(Vec3::new(1.0, 2.0, 3.0 / 2.0), v3 / v4);
}

#[test]
#[should_panic(expected = "Division by zero")]
fn vec_div_by_zero() {
    let v1 = Vec3::new(2, 3, 4);
    let v2 = Vec3::new(0, 0, 0);

    let v3 = Vec3::new(1.0, 2.0, 3.0);
    let v4 = Vec3::new(0.0, 1.0, 2.0);

    let _ = v3 / v4;
    let _ = v1 / v2;
}

#[test]
fn vec_negate() {
    let v1 = Vec3::new(2, 3, 4);
    let v2 = Vec3::new(2.0, 3.0, 4.0);

    assert_eq!(Vec3::new(-2, -3, -4), -v1);
    assert_eq!(Vec3::new(-2.0, -3.0, -4.0), -v2);
}
