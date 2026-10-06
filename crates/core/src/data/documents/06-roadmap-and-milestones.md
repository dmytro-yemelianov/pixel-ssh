# The VPA Book: Roadmap & Engineering Stages

## Chapter 6: Ecosystem Roadmap & Progression

### 6.1 Roadmap Overview

```
  Stage 1: Seed Engine & Proof Profiles 0-5 [DELIVERED v0.2.0]
    │
  Stage 2: Self-Host Compiler & v1 Language Contract [PASSED / IN PROGRESS]
    │
  Stage 3: Mutable Memory Slices & Lean 4 Proof Expansion
    │
  Stage 4: Safe Peripheral Ecosystem (Gfx, Audio, Net, I/O)
    │
  Stage 5: Autonomous Subagent & Self-Verifying Proof Loop
```

---

### 6.2 Stage 1 & Stage 2 Bootstrap Gate
- **Self-Hosted Compiler (`selfhost/compiler.vpa`)**: A 121-function VPA compiler written in VPA syntax that parses, verifies, and emits WebAssembly Text (`.wat`).
- **Bootstrap Verification**: Tests in `tests/test_selfhost_compiler.py` execute `compile` and `emit_wat` via the reference interpreter, compiling modules to WebAssembly text bit-exactly matching the Python seed reference `py_emit_wat`.

---

### 6.3 Next Sequence: Peripheral System & Lean 4 Expansion
- **Lean 4 Proof Profiles**: Extending formal theorem proving to Language 1 (`option`/`match` construct proof profile 6 and mutable memory slices).
- **Peripheral Subsystem Contracts**: Standardizing `sys.gfx`, `sys.audio`, `sys.net`, `sys.gpio`, `sys.spi`, `sys.uart` capability-gated slice handles and intrinsic traps.
- **Autonomous AI Subagent Harness**: Continuous automated subagent execution (`scripts/nvidia_subagent.py`) for self-verifying proof issuance.
