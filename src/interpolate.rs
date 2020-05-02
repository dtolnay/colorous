use crate::math::floor;
use crate::Color;

fn basis(colors: &[Color], component: fn(&Color) -> u8, t: f32) -> u8 {
    let n = colors.len() - 1;
    let i = (floor(t * n as f32) as usize).min(n - 1);

    let v1 = component(&colors[i]) as f32;
    let v2 = component(&colors[i + 1]) as f32;
    let v0 = if i > 0 {
        component(&colors[i - 1]) as f32
    } else {
        2.0 * v1 - v2
    };
    let v3 = if i < n - 1 {
        component(&colors[i + 2]) as f32
    } else {
        2.0 * v2 - v1
    };

    let t1 = (t - i as f32 / n as f32) * n as f32;
    let t2 = t1 * t1;
    let t3 = t2 * t1;

    (((1.0 - 3.0 * t1 + 3.0 * t2 - t3) * v0
        + (4.0 - 6.0 * t2 + 3.0 * t3) * v1
        + (1.0 + 3.0 * t1 + 3.0 * t2 - 3.0 * t3) * v2
        + t3 * v3)
        / 6.0) as u8
}

// 0.0 <= t <= 1.0
pub fn spline(colors: &[Color], t: f32) -> Color {
    Color {
        r: basis(colors, |c| c.r, t),
        g: basis(colors, |c| c.g, t),
        b: basis(colors, |c| c.b, t),
    }
}
