use crate::{BsuiteCoreError, RoutingKey};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TierEvidence {
    pub routing_key: RoutingKey,
    pub tier: String,
    pub evidence: String,
}

impl TierEvidence {
    pub fn new(
        routing_key: RoutingKey,
        tier: impl Into<String>,
        evidence: impl Into<String>,
    ) -> Self {
        Self {
            routing_key,
            tier: tier.into(),
            evidence: evidence.into(),
        }
    }
}

pub trait OpacityHookPublisher {
    fn publish(&self, evidence: TierEvidence) -> Result<(), BsuiteCoreError>;
}
