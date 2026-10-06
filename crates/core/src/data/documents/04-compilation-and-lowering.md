# The VPA Book: Compilation & Lowering

## Chapter 4: Multi-Target Lowering (ESP32, WASM, RV64, ARM64)

### 4.1 Architecture: One Verified AST, Multiple Native Lowerings

VPA acts as a compiler mid-end. Verified VPA modules can be targeted to multiple backend code generators:

```
                      +-------------------+
                      | Verified VPA AST  |
                      +-------------------+
                                |
       +------------------------+------------------------+
       |                        |                        |
       v                        v                        v
+--------------+       +----------------+       +------------------+
| ESP32 Native |       | WebAssembly    |       | RISC-V 64 / ARM  |
|  (C Emitter) |       |  (WAT / WASM)  |       |   (Assembly)     |
+--------------+       +----------------+       +------------------+
```

---

### 4.2 WebAssembly Emitter (`codegen_wasm.py`)
- Emits WebAssembly Text format (`.wat`) and raw `.wasm` binaries.
- Exports functions directly into WebAssembly modules.
- Maps VPA slice handles to WebAssembly linear memory (`i32.load`, `i32.store`).

```bash
python3 -m vpa.cli compile my_module.vpa --target wat -o my_module.wat
```

---

### 4.3 ESP32 & Embedded C Emitter (`codegen_esp32.py`)
- Generates allocation-free C code suitable for ESP-IDF and microcontroller SDKs.
- Includes `vpa0_runtime_trap` trap handlers and explicit bounds checks.
- Compatible with GCC and Clang cross-compilers (`xtensa-esp32-elf-gcc`).

---

### 4.4 RISC-V 64 & ARM64 Emitters (`codegen_riscv.py`, `codegen_arm64.py`)
- Emits direct assembly text for RV64I and ARM64 targets.
- Preserves register allocation invariants and stack frame isolation.
