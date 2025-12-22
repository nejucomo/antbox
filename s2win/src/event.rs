use derive_more::IsVariant;

#[derive(Debug, IsVariant)]
pub enum ButtonPosition {
    Up,
    Down,
}
