use super::{Claim, ClaimType};
use core::fmt::{self, Display, Formatter};
use serde::{Deserialize, Serialize};
use yeti::knox::{accumulator::vb20::Element, bls12_381_plus::Scalar};

/// A claim used for revocation
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct RevocationClaim {
    /// The revocation id
    pub value: String,
}

impl Display for RevocationClaim {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RevocationClaim {{ {} }}", self.value)
    }
}

impl From<String> for RevocationClaim {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl Claim for RevocationClaim {
    type Value = String;

    fn get_type(&self) -> ClaimType {
        ClaimType::Revocation
    }

    fn to_scalar(&self) -> Scalar {
        Element::hash(self.value.as_bytes()).0
    }

    fn get_value(&self) -> Self::Value {
        self.value.clone()
    }
}