use speedy2d::color::Color;

pub const BACKGROUND: Color = Color::from_rgb(0.12, 0.1, 0.17);
// pub const ANT: Color = Color::from_rgb(0.1, 0.1, 0.17);
pub const FOOD: Color = Color::from_rgb(0.1, 0.5, 0.17);

pub fn food_neighbor_count(cnt: f32) -> Color {
    assert!(cnt > 0.0);
    assert!(cnt <= 8.0);

    Color::from_rgb((1.0 / cnt).powi(2), 1.0 / cnt, 1.0 / cnt)
}
