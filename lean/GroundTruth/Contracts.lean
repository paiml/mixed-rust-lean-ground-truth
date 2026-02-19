/-!
# Contract Verification Properties

General-purpose contract verification theorems.
-/

namespace GroundTruth.Contracts

/-- A verification level in the proof hierarchy -/
inductive VerificationLevel where
  | untested
  | unitTested
  | propertyTested
  | boundedVerified
  | theoremProved
  deriving Repr, DecidableEq

/-- Higher verification levels dominate lower ones -/
def VerificationLevel.dominates : VerificationLevel → VerificationLevel → Bool
  | .theoremProved, _ => true
  | .boundedVerified, .untested | .boundedVerified, .unitTested
  | .boundedVerified, .propertyTested | .boundedVerified, .boundedVerified => true
  | .propertyTested, .untested | .propertyTested, .unitTested
  | .propertyTested, .propertyTested => true
  | .unitTested, .untested | .unitTested, .unitTested => true
  | .untested, .untested => true
  | _, _ => false

/-- Dominance is reflexive -/
theorem dominates_refl (v : VerificationLevel) :
    v.dominates v = true := by
  cases v <;> simp [VerificationLevel.dominates]

/-- TheoremProved dominates all levels -/
theorem theorem_proved_dominates_all (v : VerificationLevel) :
    VerificationLevel.theoremProved.dominates v = true := by
  cases v <;> simp [VerificationLevel.dominates]

end GroundTruth.Contracts
