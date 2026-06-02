#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RoutingKey {
    BGround,
    BAnchor,
    BSmell,
    BRatch,
    BWatch,
    BSpector,
}

impl RoutingKey {
    pub const ALL: [Self; 6] = [
        Self::BGround,
        Self::BAnchor,
        Self::BSmell,
        Self::BRatch,
        Self::BWatch,
        Self::BSpector,
    ];

    pub const fn bground() -> Self {
        Self::BGround
    }

    pub const fn banchor() -> Self {
        Self::BAnchor
    }

    pub const fn bsmell() -> Self {
        Self::BSmell
    }

    pub const fn bratch() -> Self {
        Self::BRatch
    }

    pub const fn bwatch() -> Self {
        Self::BWatch
    }

    pub const fn bspector() -> Self {
        Self::BSpector
    }

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::BGround => "bground",
            Self::BAnchor => "banchor",
            Self::BSmell => "bsmell",
            Self::BRatch => "bratch",
            Self::BWatch => "bwatch",
            Self::BSpector => "bspector",
        }
    }
}
