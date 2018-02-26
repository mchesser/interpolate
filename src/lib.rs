pub trait Interpolate: Clone {
    #[inline]
    fn bilerp(v: [[Self; 2]; 2], x: f64, y: f64) -> Self {
        let v0 = Interpolate::lerp([v[0][0].clone(), v[0][1].clone()], y);
        let v1 = Interpolate::lerp([v[1][0].clone(), v[1][1].clone()], y);
        Interpolate::lerp([v0, v1], x)
    }

    #[inline]
    fn lerp(v: [Self; 2], x: f64) -> Self;
}

impl Interpolate for f64 {
    fn bilerp(v: [[f64; 2]; 2], x: f64, y: f64) -> f64 {
        v[0][0]*(1.0-x)*(1.0-y) + v[1][0]*x*(1.0-y) + v[0][1]*(1.0-x)*y + v[1][1]*x*y
    }

    fn lerp(v: [f64; 2], x: f64) -> f64 {
        v[0] + x * (v[1] - v[0])
    }
}

impl Interpolate for f32 {
    fn bilerp(v: [[f32; 2]; 2], x: f64, y: f64) -> f32 {
        let x = x as f32;
        let y = y as f32;
        (v[0][0]*(1.0-x)*(1.0-y) + v[1][0]*x*(1.0-y) + v[0][1]*(1.0-x)*y + v[1][1]*x*y)
    }

    fn lerp(v: [f32; 2], x: f64) -> f32 {
        let x = x as f32;
        v[0] + x * (v[1] - v[0])
    }
}

macro_rules! impl_interpolate {
    ($src_type:ty, $dest_type:ty) => (
        impl Interpolate for $src_type {
            fn lerp(v: [$src_type; 2], x: f64) -> $src_type {
                Interpolate::lerp([v[0] as $dest_type, v[1] as $dest_type], x) as $src_type
            }
        }
    )
}

impl_interpolate!(u8, f32);
impl_interpolate!(u16, f32);
impl_interpolate!(u32, f64);
impl_interpolate!(u64, f64);

impl_interpolate!(i8, f32);
impl_interpolate!(i16, f32);
impl_interpolate!(i32, f64);
impl_interpolate!(i64, f64);

/// Performs linear interpolation between the elements of two arrays.
pub fn interpolate_array<T: Interpolate>(input: [&[T]; 2], output: &mut [T], x: f64) {
    for (output, (a, b)) in output.iter_mut().zip(input[0].iter().zip(input[1].iter())) {
        *output = Interpolate::lerp([a.clone(), b.clone()], x);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Rgb {
        r: u8,
        g: u8,
        b: u8,
    }

    impl Interpolate for Rgb {
        fn lerp(v: [Rgb; 2], x: f64) -> Rgb {
            Rgb {
                r: u8::lerp([v[0].r, v[1].r], x),
                g: u8::lerp([v[0].g, v[1].g], x),
                b: u8::lerp([v[0].b, v[1].b], x)
            }
        }
    }

    // Test that we get the right result for a basic type
    #[test]
    fn basic_lerp_test() {
        let result = f32::lerp([0.0, 100.0], 0.5);
        assert!(49.9 < result && result < 50.1);
    }

    // Test that an implementation for a custom type works
    #[test]
    fn custom_type_lerp_test() {
        let start = Rgb { r: 0, g: 255, b: 0 };
        let end = Rgb { r: 100, g: 0, b: 200 };
        assert_eq!(Rgb::lerp([start, end], 0.5), Rgb { r: 50, g: 127, b: 100 });
    }

    // Test that interpolating arrays works correctly
    #[test]
    fn interpolate_array_test() {
        let start = [0, 255, 0];
        let end = [100, 0, 200];

        let mut output = [0; 3];
        interpolate_array([&start, &end], &mut output, 0.5);

        assert_eq!(output, [50, 127, 100]);
    }
}
