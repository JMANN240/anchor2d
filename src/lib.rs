use glam::{DVec2, FloatExt, dvec2};

#[derive(Debug, Clone, Copy)]
pub struct Anchor2D {
    horizontal: HorizontalAnchor,
    vertical: VerticalAnchor,
}

impl Anchor2D {
    pub fn get_horizontal(&self) -> HorizontalAnchor {
        self.horizontal
    }

    pub fn get_vertical(&self) -> VerticalAnchor {
        self.vertical
    }

    pub fn anchor(&self, rect: Rect) -> DVec2 {
        dvec2(
            self.get_horizontal().anchor(rect.get_range_x()),
            self.get_vertical().anchor(rect.get_range_y()),
        )
    }
}

fn anchor(range: Range, t: f64) -> f64 {
    range.get_start() + 0.0f64.lerp(range.get_size(), t)
}

trait Anchor {
    fn get_t(&self) -> f64;

    fn anchor(&self, range: Range) -> f64 {
        anchor(range, self.get_t())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAnchor {
    Left,
    Center,
    Right,
}

impl Anchor for HorizontalAnchor {
    fn get_t(&self) -> f64 {
        match self {
            Self::Left => 0.0,
            Self::Center => 0.5,
            Self::Right => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalAnchor {
    context: VerticalAnchorContext,
    value: VerticalAnchorValue,
}

impl VerticalAnchor {
    pub fn new(context: VerticalAnchorContext, value: VerticalAnchorValue) -> Self {
        Self { context, value }
    }

    pub fn get_value(&self) -> VerticalAnchorValue {
        self.value
    }

    pub fn get_context(&self) -> VerticalAnchorContext {
        self.context
    }
}

impl Anchor for VerticalAnchor {
    fn get_t(&self) -> f64 {
        self.get_context().get_t(self.get_value())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAnchorContext {
    Math,
    Graphics,
}

impl VerticalAnchorContext {
    fn get_t(&self, value: VerticalAnchorValue) -> f64 {
        match (self, value) {
            (Self::Graphics, VerticalAnchorValue::Top) => 0.0,
            (Self::Math, VerticalAnchorValue::Top) => 1.0,
            (_, VerticalAnchorValue::Center) => 0.5,
            (Self::Graphics, VerticalAnchorValue::Bottom) => 1.0,
            (Self::Math, VerticalAnchorValue::Bottom) => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAnchorValue {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    start: f64,
    size: f64,
}

impl Range {
    pub fn new(value_1: f64, value_2: f64) -> Self {
        Self {
            start: value_1.min(value_2),
            size: (value_1 - value_2).abs(),
        }
    }

    pub fn get_start(&self) -> f64 {
        self.start
    }

    pub fn get_size(&self) -> f64 {
        self.size
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    range_x: Range,
    range_y: Range,
}

impl Rect {
    pub fn new(corner_1: DVec2, corner_2: DVec2) -> Self {
        Self {
            range_x: Range::new(corner_1.x, corner_2.x),
            range_y: Range::new(corner_1.y, corner_2.y),
        }
    }

    pub fn get_range_x(&self) -> Range {
        self.range_x
    }

    pub fn get_range_y(&self) -> Range {
        self.range_y
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Anchor, HorizontalAnchor, Range, VerticalAnchor, VerticalAnchorContext, VerticalAnchorValue
    };

    #[test]
    fn test_horizontal() {
        let range = Range::new(0.0, 100.0);

        assert_eq!(HorizontalAnchor::Left.get_t(), 0.0);
        assert_eq!(HorizontalAnchor::Center.get_t(), 0.5);
        assert_eq!(HorizontalAnchor::Right.get_t(), 1.0);

        assert_eq!(HorizontalAnchor::Left.anchor(range), 0.0);
        assert_eq!(HorizontalAnchor::Center.anchor(range), 50.0);
        assert_eq!(HorizontalAnchor::Right.anchor(range), 100.0);
    }

    #[test]
    fn test_vertical() {
        let range = Range::new(0.0, 100.0);

        assert_eq!(
            VerticalAnchor::new(
                VerticalAnchorContext::Graphics,
                VerticalAnchorValue::Top,
            )
            .get_t(),
            0.0
        );
        assert_eq!(
            VerticalAnchor::new(
                VerticalAnchorContext::Graphics,
                VerticalAnchorValue::Center,
            )
            .get_t(),
            0.5
        );
        assert_eq!(
            VerticalAnchor::new(
                VerticalAnchorContext::Graphics,
                VerticalAnchorValue::Bottom,
            )
            .get_t(),
            1.0
        );

        assert_eq!(
            VerticalAnchor::new(
                VerticalAnchorContext::Graphics,
                VerticalAnchorValue::Top,
            )
            .anchor(range),
            0.0
        );
        assert_eq!(
            VerticalAnchor::new(
                VerticalAnchorContext::Graphics,
                VerticalAnchorValue::Center,
            )
            .anchor(range),
            50.0
        );
        assert_eq!(
            VerticalAnchor::new(
                VerticalAnchorContext::Graphics,
                VerticalAnchorValue::Bottom,
            )
            .anchor(range),
            100.0
        );
    }
}
