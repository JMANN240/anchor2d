pub struct Anchor2D {
    pub horizontal: HorizontalAnchor,
    pub vertical: VerticalAnchor,
}

pub enum HorizontalAnchor {
    Left,
    Center,
    Right,
}

pub enum VerticalAnchor {
    Top,
    Center,
    Bottom,
}
