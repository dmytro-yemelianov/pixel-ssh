# The VPA Book: Paradigms & Architecture

## Chapter 1: Vision, Philosophy & Ontology

### 1.1 What is VPA?
**Verified Portable Assembly (VPA)** is a safe, low-level virtual assembly language, formal verification engine, and multi-target compilation framework.

VPA bridges the gap between high-level verification formalisms (such as Lean 4) and physical low-level execution (such as ESP32 microcontrollers, WebAssembly runtime containers, and RISC-V hardware).

### 1.2 Core Paradigms
1. **Safety by Construction**: VPA eliminates raw pointer arithmetic, arbitrary memory casts, and unverified global state mutations.
2. **Explicit Bounded Memory Slices**: Memory is modeled through immutable (`slice.u32`, `slice.u8`) and mutable (`mut.buf.u8`, `mut.buf.u32`) linear slice handles.
3. **Structured Control Flow**: No arbitrary goto instructions. Control flow uses structured loops and conditionals with provable termination or trapped bounds invariants.
4. **Deterministic Target Lowering**: VPA lowers deterministically to WebAssembly Text (WAT), ESP-IDF C, and RISC-V assembly.

### 1.3 Language Ontology
- **Module**: Named container of typed internal and exported functions.
- **Function**: Pure or side-effecting typed mapping from arguments to return values.
- **Local**: Immutable typed cell within a function body.
- **Outcome**: `return(value)`, `trap(code)`, or divergence.
