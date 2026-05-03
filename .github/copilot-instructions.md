# AION-Math — Copilot instructions

Purpose: Give AI coding agents the essential, actionable knowledge to be productive in this repo.

Big picture
- Lightweight, high-performance mathematical and tensor computation library for AI-native infrastructure.
- Provides efficient numeric operations for vectors, matrices, and basic tensor operations.
- Core utility component for AION ecosystem, supporting RL training, technical indicators, and real-time data analysis.

Key files & where to look
- src/lib.rs: Module declarations and public API
- src/math/: Stateless mathematical functions (average, variance)
- src/continuous_math/: Stateful streaming operations (SMA, EMA, MACD)
- src/continuous_math/frame/: Frame data structure for state management

Build & run (reproducible commands)
- Build: `cargo build --release`
- Test: `cargo test`

Project-specific conventions (do not change silently)
- Label-based multi-instance tracking: u32 labels for independent data streams
- Option returns for warmup windows: SMA/EMA/MACD return None until sufficient data
- Stateful mutation: ContinuousMath methods take &mut self for state maintenance
- Zero dependencies: Only Rust std lib for minimal overhead

Integration points & expectations
- Used by AION-RLT for neural network computations
- Supports streaming data processing for real-time applications
- Frame-based state management for efficient rolling calculations

Safe edits checklist for feature changes
- When adding new indicators: Follow Option pattern for warmup periods
- When modifying state: Ensure &mut self for mutable operations
- When adding math functions: Consider both stateless (Math) and stateful (ContinuousMath) variants

Quick debugging tips
- Check label usage for multi-stream scenarios
- Verify warmup periods for indicator calculations
- Test with small datasets to validate incremental algorithms</content>
<parameter name="filePath">/media/elpixeler/Develop/Projects/AION/AION-Math/.github/copilot-instructions.md