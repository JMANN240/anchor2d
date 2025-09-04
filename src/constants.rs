use crate::{Anchor2D, HorizontalAnchor, VerticalAnchor, VerticalAnchorContext, VerticalAnchorValue};

pub const LGT: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Left,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Graphics,
        value: VerticalAnchorValue::Top,
    },
};

pub const LMT: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Left,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Math,
        value: VerticalAnchorValue::Top,
    },
};

pub const CGT: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Center,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Graphics,
        value: VerticalAnchorValue::Top,
    },
};

pub const CMT: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Center,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Math,
        value: VerticalAnchorValue::Top,
    },
};

pub const RGT: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Right,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Graphics,
        value: VerticalAnchorValue::Top,
    },
};

pub const RMT: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Right,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Math,
        value: VerticalAnchorValue::Top,
    },
};

pub const LGC: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Left,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Graphics,
        value: VerticalAnchorValue::Center,
    },
};

pub const LMC: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Left,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Math,
        value: VerticalAnchorValue::Center,
    },
};

pub const CGC: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Center,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Graphics,
        value: VerticalAnchorValue::Center,
    },
};

pub const CMC: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Center,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Math,
        value: VerticalAnchorValue::Center,
    },
};

pub const RGC: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Right,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Graphics,
        value: VerticalAnchorValue::Center,
    },
};

pub const RMC: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Right,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Math,
        value: VerticalAnchorValue::Center,
    },
};

pub const LGB: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Left,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Graphics,
        value: VerticalAnchorValue::Bottom,
    },
};

pub const LMB: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Left,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Math,
        value: VerticalAnchorValue::Bottom,
    },
};

pub const CGB: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Center,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Graphics,
        value: VerticalAnchorValue::Bottom,
    },
};

pub const CMB: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Center,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Math,
        value: VerticalAnchorValue::Bottom,
    },
};

pub const RGB: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Right,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Graphics,
        value: VerticalAnchorValue::Bottom,
    },
};

pub const RMB: Anchor2D = Anchor2D {
    horizontal: HorizontalAnchor::Right,
    vertical: VerticalAnchor {
        context: VerticalAnchorContext::Math,
        value: VerticalAnchorValue::Bottom,
    },
};
