/*
 * Copyright 2006 The Android Open Source Project
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE.skia file.
 */
use crate::types::Point;

pub fn Sk2ScalarDiv(a: f32, b: f32) -> f32 {
    return a / b;
}

pub fn Sk2ScalarMul(a: f32, b: f32) -> f32 {
    return a * b;
}

pub fn Sk2ScalarAbs(a: f32) -> f32 {
    if a < 0. {
        return -a;
    }
    return a;
}

pub fn Sk2ScalarIsNaN(a: f32) -> bool
{
    return a != a;
}


// we can do this 
pub fn valid_unit_divide(mut numer: f32, mut denom: f32, ratio: &mut f32) -> bool
{
    if (numer < 0.)
    {
        numer = -numer;
        denom = -denom;
    }

    if (denom == 0. || numer == 0. || numer >= denom) {
        return false;
    }

    let r = Sk2ScalarDiv(numer, denom);
    if Sk2ScalarIsNaN(r) {
        return false;
    }
    debug_assert!(r >= 0. && r < 1.);
    if r == 0. { // catch underflow if numer <<<< denom
        return false;
    }
    *ratio = r;
    return true;
}


pub fn is_not_monotonic(a: f32, b: f32, c: f32) -> bool {
    let ab = a - b;
    let mut bc = b - c;
    if ab < 0. {
        bc = -bc;
    }
    return ab == 0. || bc < 0.;
}

fn Sk2ScalarInterp(A: f32, B: f32, t: f32) -> f32 {
    debug_assert!(t >= 0. && t <= 1.);
    return A + Sk2ScalarMul(B - A, t);
}


// Skia does a weird thing where it treats arrays of points as castable to array of floats
fn interp_quad_x_coords(src: &[Point; 3], dst: &mut [Point; 5], t: f32)
{
    let ab = Sk2ScalarInterp(src[0].x, src[1].x, t);
    let bc = Sk2ScalarInterp(src[1].x, src[2].x, t);

    dst[0].x = src[0].x;
    dst[1].x = ab;
    dst[2].x = Sk2ScalarInterp(ab, bc, t);
    dst[3].x = bc;
    dst[4].x = src[2].x;
}

fn interp_quad_y_coords(src: &[Point; 3], dst: &mut [Point; 5], t: f32)
{
    let ab = Sk2ScalarInterp(src[0].y, src[1].y, t);
    let bc = Sk2ScalarInterp(src[1].y, src[2].y, t);

    dst[0].y = src[0].y;
    dst[1].y = ab;
    dst[2].y = Sk2ScalarInterp(ab, bc, t);
    dst[3].y = bc;
    dst[4].y = src[2].y;
}

pub fn chop_quad_at(src: &[Point; 3], dst: &mut [Point; 5], t: f32)
{
    debug_assert!(t > 0. && t < 1.);

    interp_quad_x_coords(src, dst, t);
    interp_quad_y_coords(src, dst, t);
}

// ensures that the y values are contiguous
// dst[1].fY = dst[3].fY = dst[2].fY
// I'm not sure why we need this
pub fn flatten_double_quad_extrema(dst: &mut [Point; 5])
{
    dst[1].y = dst[2].y;
    dst[3].y = dst[2].y;
}