//! Kernel contract verification library
//! 
//! Parses YAML contracts and verifies proof obligations.

use serde::{Deserialize, Serialize};

/// A kernel contract with proof obligations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub name: String,
    pub equations: Vec<Equation>,
    pub obligations: Vec<ProofObligation>,
}

/// Mathematical equation from a peer-reviewed paper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    pub id: String,
    pub latex: String,
    pub description: String,
}

/// A proof obligation that must be verified
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofObligation {
    pub property: String,
    pub formal: String,
    pub verification_level: VerificationLevel,
    pub lean_theorem: Option<String>,
}

/// Verification hierarchy (L0-L4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationLevel {
    /// L0: Untested
    Untested,
    /// L1: Unit tests
    UnitTested,
    /// L2: Property-based testing  
    PropertyTested,
    /// L3: Bounded model checking (Kani)
    BoundedVerified,
    /// L4: Theorem proved (Lean 4)
    TheoremProved,
}

/// Parse a contract from YAML
pub fn parse_contract(yaml: &str) -> Result<Contract, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Count obligations at each verification level
pub fn verification_summary(contract: &Contract) -> Vec<(VerificationLevel, usize)> {
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<u8, (VerificationLevel, usize)> = BTreeMap::new();
    for ob in &contract.obligations {
        let key = match ob.verification_level {
            VerificationLevel::Untested => 0,
            VerificationLevel::UnitTested => 1,
            VerificationLevel::PropertyTested => 2,
            VerificationLevel::BoundedVerified => 3,
            VerificationLevel::TheoremProved => 4,
        };
        counts.entry(key).or_insert((ob.verification_level, 0)).1 += 1;
    }
    counts.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CONTRACT: &str = r#"
name: softmax
equations:
  - id: eq1
    latex: "\\sigma(x)_i = \\frac{e^{x_i}}{\\sum_j e^{x_j}}"
    description: "Softmax function definition"
obligations:
  - property: "Output sums to 1"
    formal: "|Σ σ(x)_i - 1.0| < ε"
    verification_level: TheoremProved
    lean_theorem: "Softmax.partition_of_unity"
  - property: "All outputs non-negative"
    formal: "∀i. σ(x)_i ≥ 0"
    verification_level: PropertyTested
    lean_theorem: null
  - property: "Preserves ordering"
    formal: "x_i > x_j → σ(x)_i > σ(x)_j"
    verification_level: Untested
    lean_theorem: null
"#;

    #[test]
    fn test_parse_contract() {
        let contract = parse_contract(SAMPLE_CONTRACT).unwrap();
        assert_eq!(contract.name, "softmax");
        assert_eq!(contract.equations.len(), 1);
        assert_eq!(contract.obligations.len(), 3);
    }

    #[test]
    fn test_verification_summary() {
        let contract = parse_contract(SAMPLE_CONTRACT).unwrap();
        let summary = verification_summary(&contract);
        assert_eq!(summary.len(), 3); // 3 distinct levels
    }

    #[test]
    fn test_obligation_levels() {
        let contract = parse_contract(SAMPLE_CONTRACT).unwrap();
        assert_eq!(contract.obligations[0].verification_level, VerificationLevel::TheoremProved);
        assert_eq!(contract.obligations[1].verification_level, VerificationLevel::PropertyTested);
        assert_eq!(contract.obligations[2].verification_level, VerificationLevel::Untested);
    }
}
