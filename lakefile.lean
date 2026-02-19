import Lake
open Lake DSL

package mixedGroundTruth where
  leanOptions := #[⟨`autoImplicit, false⟩]

@[default_target]
lean_lib GroundTruth where
  srcDir := "lean"
