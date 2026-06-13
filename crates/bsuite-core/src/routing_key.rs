use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::fmt;

const ROUTING_KEY_VARIANTS: &[&str] = &[
    "bground", "banchor", "bsmell", "bratch", "bwatch", "bspector",
];

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

    pub fn from_stable_name(value: &str) -> Option<Self> {
        match value {
            "bground" => Some(Self::BGround),
            "banchor" => Some(Self::BAnchor),
            "bsmell" => Some(Self::BSmell),
            "bratch" => Some(Self::BRatch),
            "bwatch" => Some(Self::BWatch),
            "bspector" => Some(Self::BSpector),
            _ => None,
        }
    }
}

impl fmt::Display for RoutingKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.stable_name())
    }
}

impl Serialize for RoutingKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.stable_name())
    }
}

impl<'de> Deserialize<'de> for RoutingKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_stable_name(&value)
            .ok_or_else(|| de::Error::unknown_variant(&value, ROUTING_KEY_VARIANTS))
    }
}
