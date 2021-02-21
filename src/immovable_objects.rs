#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Immovable {
    pub type_object: ImmovableType,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImmovableType {
    Background,
    Wall,
    Emerald,
    Bag,
}