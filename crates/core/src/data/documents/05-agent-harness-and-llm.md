# The VPA Book: Autonomous Subagents & LLM Harness

## Chapter 5: AI Subagents & Automated Verification Loops

### 5.1 Overview
VPA features a built-in integration harness (`scripts/vpa_llm_runner.py`, `nvidia_subagent.py`) designed to allow autonomous AI agents (OpenAI, Claude, Gemini, Llama, DeepSeek) to generate, verify, compile, and execute VPA code safely.

---

### 5.2 NVIDIA Build API Integration
Using `scripts/nvidia_client.py` and `nvidia_subagent.py`, subagents can query models such as Llama 3.3 70B, DeepSeek R1, or Mixtral 8x22B to perform code generation, static auditing, or proof generation tasks.

```bash
# Execute subagent task via NVIDIA Build API
python3 scripts/nvidia_subagent.py "Write a VPA function that calculates dot product of two slice.u32 vectors" --model deepseek
```

---

### 5.3 Execution & Diagnostic Loop (`vpa_llm_runner.py`)
The LLM runner executes VPA code across 3 execution environments in parallel:
1. **Reference Interpreter**: Dynamic VPA AST evaluation.
2. **Native C Execution**: Compiles via `clang` and runs binary in temporary container.
3. **WebAssembly Output**: Assembles WAT to binary WASM.

If parsing or verification fails, structured JSON error diagnostics are returned to the agent for self-correction.
