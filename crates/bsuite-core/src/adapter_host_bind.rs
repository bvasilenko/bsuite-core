use crate::BsuiteCoreError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HostContext {
    L2a,
    PayloadV3,
    StrapiV5,
    SanityV3,
    DirectusV10,
}

impl HostContext {
    pub const ALL: [Self; 5] = [
        Self::L2a,
        Self::PayloadV3,
        Self::StrapiV5,
        Self::SanityV3,
        Self::DirectusV10,
    ];

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::L2a => "l2a",
            Self::PayloadV3 => "payload-v3",
            Self::StrapiV5 => "strapi-v5",
            Self::SanityV3 => "sanity-v3",
            Self::DirectusV10 => "directus-v10",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdapterBinding {
    pub host: HostContext,
    pub package_name: String,
}

impl AdapterBinding {
    pub fn new(host: HostContext, package_name: impl Into<String>) -> Self {
        Self {
            host,
            package_name: package_name.into(),
        }
    }
}

pub trait AdapterHostBinder {
    fn bind(&self, host: HostContext) -> Result<AdapterBinding, BsuiteCoreError>;
}
