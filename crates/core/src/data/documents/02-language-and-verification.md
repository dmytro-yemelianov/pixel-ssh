# The VPA Book: Language & Verification

## Chapter 2: Language Syntax, Types & Formal Verification

### 2.1 Language Syntax (v0 and v1)

A VPA program consists of module headers, typed local definitions, structured control blocks, and exported or internal functions.

```vpa
module math_kernel
language 1
profile safe

export fn compute_sum input:slice.u32 -> u64
    local total u64 0
    local len_val u32 0
    len len_val input
    
    local idx u32 0
    loop
        ge break u32 idx len_val
        local val u32 0
        load val u32 input idx
        
        local val_u64 u64 0
        zext val_u64 u64 val
        add total u64 total val_u64
        
        add idx u32 idx 1
    end
    
    ret total
end
```

---

### 2.2 Type System & Memory Slices

VPA distinguishes between scalar values and linear memory slices:

| Type | Kind | Description |
|---|---|---|
| `bool` | Scalar | Boolean truth value (`true` / `false`) |
| `u32` | Scalar | 32-bit unsigned integer |
| `u64` | Scalar | 64-bit unsigned integer |
| `slice.u32` | Memory | Read-only slice of 32-bit integers with bounds traps |
| `slice.u8` | Memory | Read-only byte slice |
| `mut.buf.u8` | Memory | Mutable byte buffer (allowed in internal functions) |
| `mut.buf.u32` | Memory | Mutable 32-bit integer buffer |

#### Memory Bounds Traps
Unlike C or raw assembly, loading or storing outside slice bounds produces an explicit, trapped `bounds` outcome without causing undefined behavior or memory corruption.

---

### 2.3 Formal Verification Engine & Lean 4 Mechanics

VPA includes a formal verifier (`vpa.verifier`) and proof issuance pipeline (`_proof_issuance.py`).

1. **Static AST Verification**: Verifies type safety, single-assignment invariants for local cells, valid instruction operands, and slice safety profiles.
2. **Lean 4 Proof Generation**: Translates VPA module definitions into Lean 4 theorems, asserting semantic equivalence between the reference interpreter and low-level lowering pipelines.
3. **Replay Certificates**: Issues verifiable binary certificates (`replay_certificate.py`) ensuring that generated low-level C / WASM code preserves module invariants.
