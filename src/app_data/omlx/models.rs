// One model for every task. The machine can only hold one model in RAM,
// so every constant MUST point at the same model id (no swaps during a run).
// Each task keeps its own constant so changing one task later is a one-line edit.
pub const DEFAULT_MODEL: &str = "Ternary-Bonsai-27B-mlx-2bit";
pub const NLU_MODEL: &str = "Ternary-Bonsai-27B-mlx-2bit";
pub const NEWS_WRITER_MODEL: &str = "Ternary-Bonsai-27B-mlx-2bit";
pub const X_POSTS_MODEL: &str = "Ternary-Bonsai-27B-mlx-2bit";
pub const STORY_CHECK_MODEL: &str = "Ternary-Bonsai-27B-mlx-2bit";
