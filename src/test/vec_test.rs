use super::Vec3;

#[test]
fn vec_add() {
    let v1 = Vec3::new(1, 2, 3);
    let v2 = Vec3::new(3, 2, 1);

    assert_eq!(Vec3::new(4, 4, 4), v1 + v2);
}

#[test]
fn vec_add_float() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(0.0, 1.0, 2.0);

    assert_eq!(Vec3::new(1.0, 3.0, 5.0), v1 + v2);
}

#[test]
fn vec_sub() {
    let v1 = Vec3::new(1, 2, 3);
    let v2 = Vec3::new(3, 2, 1);

    assert_eq!(Vec3::new(-2, 0, 2), v1 - v2);
}

#[test]
fn vec_sub_float() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(0.0, 1.0, 2.0);

    assert_eq!(Vec3::new(1.0, 1.0, 1.0), v1 - v2);
}

#[test]
fn vec_mul() {
    let v1 = Vec3::new(1, 2, 3);
    let v2 = Vec3::new(3, 2, 1);

    assert_eq!(Vec3::new(3, 4, 3), v1 * v2);
    assert_eq!(Vec3::new(2, 4, 6), v1 * 2);
}

#[test]
fn vec_mul_float() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(0.0, 1.0, 2.0);

    assert_eq!(Vec3::new(0.0, 2.0, 6.0), v1 * v2);
    assert_eq!(Vec3::new(2.0, 4.0, 6.0), v1 * 2.0);
}

#[test]
fn vec_div() {
    let v1 = Vec3::new(2, 3, 4);
    let v2 = Vec3::new(2, 3, 4);

    assert_eq!(Vec3::new(1, 1, 1), v1 / v2);
}

#[test]
fn vec_div_float() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(1.0, 1.0, 2.0);

    assert_eq!(Vec3::new(1.0, 2.0, 3.0 / 2.0), v1 / v2);
}

#[test]
#[should_panic(expected = "Division by zero")]
fn vec_div_by_zero() {
    let v1 = Vec3::new(2, 3, 4);
    let v2 = Vec3::new(0, 0, 0);

    let _ = v1 / v2;
}

#[test]
#[should_panic(expected = "Division by zero")]
fn vec_div_by_zero_float() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(0.0, 1.0, 2.0);

    let _ = v1 / v2;
}

#[test]
#[should_panic(expected = "Division by zero")]

fn vec_div_by_zero2() {
    let v1 = Vec3::new(2, 3, 4);
    let _ = v1 / 0;
}

#[test]
#[should_panic(expected = "Division by zero")]

fn vec_div_by_zero2_float() {
    let v1 = Vec3::new(2.0, 3.0, 4.0);
    let _ = v1 / 0.0;
}

#[test]
fn vec_negate() {
    let v1 = Vec3::new(2, 3, 4);
    let v2 = Vec3::new(2.0, 3.0, 4.0);

    assert_eq!(Vec3::new(-2, -3, -4), -v1);
    assert_eq!(Vec3::new(-2.0, -3.0, -4.0), -v2);
}

#[test]
fn vec_negate_float() {
    let v1 = Vec3::new(2.0, 3.0, 4.0);

    assert_eq!(Vec3::new(-2.0, -3.0, -4.0), -v1);
}

#[test]
fn vec_index() {
    let v1 = Vec3::new(2, 3, 5);

    assert_eq!(2, v1[0]);
    assert_eq!(3, v1[1]);
    assert_eq!(5, v1[2]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn vec_index_out_of_bounds() {
    let v1 = Vec3::new(2, 3, 5);
    let _ = v1[3];
}

#[test]
fn vec_dot() {
    let v1 = Vec3::new(2, 3, 5);
    let v2 = Vec3::new(2, 4, 10);

    assert_eq!(4 + 12 + 50, v1.dot(v2))
}

#[test]
fn vec_dot_float() {
    let v1 = Vec3::new(2.0, 3.0, 5.0);
    let v2 = Vec3::new(2.0, 4.0, 10.0);

    assert_eq!(4.0 + 12.0 + 50.0, v1.dot(v2))
}

#[test]
fn vec_cross_product() {
    let v1 = Vec3::new(1, 2, 3);
    let v2 = Vec3::new(4, 5, 6);

    assert_eq!(Vec3::new(-3, 6, -3), v1.cross(v2))
}

#[test]
fn vec_cross_product_float() {
    let v1 = Vec3::new(1.2, 2.5, 3.2);
    let v2 = Vec3::new(10.2, 5.3, 1.2);

    assert_eq!(Vec3::new(-13.96, 31.2, -19.14), v1.cross(v2))
}

#[test]
fn vec_distance() {
    let v1 = Vec3::new(1, 0, 5);
    let v2 = Vec3::new(0, 2, 4);

    assert_eq!(6.0f64.sqrt(), v1.distance(v2))
}

#[test]
fn vec_distance_float() {
    let v1 = Vec3::new(1.0, 0.0, 5.0);
    let v2 = Vec3::new(0.0, 2.0, 4.0);

    assert_eq!(6.0f64.sqrt(), v1.distance(v2))
}

#[test]
fn vec_from_array() {
    let v1: Vec3<_> = [0, 2, 3].into();

    assert_eq!(Vec3::new(0, 2, 3), v1);
}

#[test]
fn vec_from_tuple() {
    let v1: Vec3<_> = (0, 2, 3).into();

    assert_eq!(Vec3::new(0, 2, 3), v1);
}

#[test]
fn vec_to_iter() {
    let v1 = Vec3::new(1, 2, 3);
    let mut iter = v1.into_iter();

    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(None, iter.next());
}
