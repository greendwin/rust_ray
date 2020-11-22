use rust_ray::math::*;

#[test]
fn clamp_indentity() {
    assert_eq!(clamp(0.0, 0.0, 1.0), 0.0);
    assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
    assert_eq!(clamp(1.0, 0.0, 1.0), 1.0);
}

#[test]
fn clamp_minmax() {
    assert_eq!(clamp(-10.0, 0.0, 1.0), 0.0);
    assert_eq!(clamp(100.0, 0.0, 10.0), 10.0);
    assert_eq!(clamp(100.0, 0.0, 10.0), 10.0);
}

#[test]
fn lerm_interpolate() {
    assert_eq!(lerp(0.0, 10.0, 20.0), 10.0);
    assert_eq!(lerp(0.5, 10.0, 20.0), 15.0);
    assert_eq!(lerp(1.0, 10.0, 20.0), 20.0);
}

#[test]
fn lerm_minmax() {
    assert_eq!(lerp(-1.0, 10.0, 20.0), 10.0);
    assert_eq!(lerp(2.0, 10.0, 20.0), 20.0);
}

#[test]
fn lerp_unclamped_interpolate() {
    assert_eq!(lerp_unclamped(0.0, 10.0, 20.0), 10.0);
    assert_eq!(lerp_unclamped(0.5, 10.0, 20.0), 15.0);
    assert_eq!(lerp_unclamped(1.0, 10.0, 20.0), 20.0);
}

#[test]
fn lerp_unclamped_minmax() {
    assert_eq!(lerp_unclamped(-1.0, 10.0, 20.0), 0.0);
    assert_eq!(lerp_unclamped(2.0, 10.0, 20.0), 30.0);
}

#[test]
fn inv_lerp_calc() {
    assert_eq!(inv_lerp(10.0, 10.0, 20.0), 0.0);
    assert_eq!(inv_lerp(20.0, 10.0, 20.0), 1.0);
    assert_eq!(inv_lerp(15.0, 10.0, 20.0), 0.5);
    assert_eq!(inv_lerp(30.0, 10.0, 20.0), 2.0);
    assert_eq!(inv_lerp(0.0, 10.0, 20.0), -1.0);
}

#[test]
fn inv_lerp_eq_ab() {
    assert_eq!(inv_lerp(10.0, 10.0, 10.0), 0.0);
    assert_eq!(inv_lerp(20.0, 10.0, 10.0), 0.0);
}