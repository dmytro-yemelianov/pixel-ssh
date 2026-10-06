# The VPA Book: Device Interaction & Peripherals

## Chapter 3: Graphics, Audio, Network & Peripheral Hardware

### 3.1 Overview: The Safe Hardware Boundary
Conventional assembly interacts with peripherals via raw Memory-Mapped I/O (MMIO) addresses or unstructured interrupts. VPA abstracts peripheral interaction into **Capability-Gated Slices** and **Typed Intrinsic Traps**, maintaining 100% formal safety.

```
 +-------------------------------------------------------+
 |                  VPA Application Code                 |
 |  (Draws RGBA pixels, synthesizes PCM, builds packets) |
 +-------------------------------------------------------+
        |                      |                     |
   mut.buf.u8             mut.buf.i16           slice.u8
  (Framebuffer)          (Audio Ring)         (Net Packet)
        |                      |                     |
 +-------------------------------------------------------+
 |                 VPA Hardware Runtime                  |
 |  (ESP-IDF DMA / WebGL Canvas / WebAudio / Socket OS)  |
 +-------------------------------------------------------+
```

---

### 3.2 Graphics Subsystem (`sys.gfx`)

Graphics in VPA are modeled over 2D and 3D linear memory buffers (`mut.buf.u8` or `mut.buf.u32`) representing RGBA framebuffers.

#### 1. Framebuffer Allocation & Mapping
The runtime passes a `mut.buf.u8` slice to an internal VPA function representing a screen of $W \times H \times 4$ bytes.

```vpa
module gfx_renderer
language 1
profile safe

export fn render_frame frame:mut.buf.u8 width:u32 height:u32 color:u32 -> bool
    local len_bytes u32 0
    len len_bytes frame
    
    local idx u32 0
    loop
        ge break u32 idx len_bytes
        store32 frame idx color
        add idx u32 idx 4
    end
    
    call ok sys.gfx.present frame width height
    ret ok
end
```

#### 2. Graphics Intrinsics
- `sys.gfx.present frame width height`: Flips or commits the current framebuffer to physical display / WebGL texture via DMA.
- `sys.gfx.blit dest src dx dy dw dh sx sy`: Hardware-accelerated block transfer between two slice regions with bounds traps.

---

### 3.3 Audio Subsystem (`sys.audio`)

Audio rendering in VPA utilizes PCM audio buffers (`mut.buf.i16` or `mut.buf.f32`) for synthesis, DSP filters, and playback.

#### 1. Audio Ring Buffer Model
Hardware (I2S DMA, WebAudio AudioBuffer, ALSA/CoreAudio) provides double-buffered PCM channels:

```vpa
module audio_synth
language 1
profile safe

fn generate_sine buffer:mut.buf.i16 frequency:u32 sample_rate:u32 -> u32
    local count u32 0
    len count buffer
    
    local i u32 0
    loop
        ge break u32 i count
        # Compute PCM sample and store into audio slice
        local sample u32 0
        call sample DSP_sine_table i frequency
        store16 buffer i sample
        add i u32 i 2
    end
    
    call samples_written sys.audio.commit buffer sample_rate
    ret samples_written
end
```

#### 2. Audio Intrinsics
- `sys.audio.commit buffer sample_rate`: Hands off a filled PCM slice to the hardware audio queue.
- `sys.audio.input_slot slot`: Populates a target slice with incoming microphone/line-in PCM data.

---

### 3.4 Network Subsystem (`sys.net`)

Network I/O in VPA is packet-oriented and asynchronous, using `slice.u8` for read-only received packets and `mut.buf.u8` for transmit buffers.

#### 1. Transmit & Receive Flow
- **RX (Receive)**: The OS/driver places incoming Ethernet/IP/UDP frames into a read-only `slice.u8`. VPA inspects headers safely without buffer overflow risks.
- **TX (Transmit)**: VPA serializes packet data into a `mut.buf.u8` and issues a `sys.net.send` call.

```vpa
module net_handler
language 1
profile safe

export fn process_packet packet:slice.u8 tx_buf:mut.buf.u8 -> u32
    local pkt_len u32 0
    len pkt_len packet
    
    # Read UDP port header at byte offset 20
    local dest_port u32 0
    load16 dest_port packet 20
    
    # Construct response in tx_buf
    store8 tx_buf 0 0x45 # IPv4 version header
    call sent sys.net.send tx_buf pkt_len
    ret sent
end
```

#### 2. Network Intrinsics
- `sys.net.send buffer len`: Queues network packet buffer for NIC transmission.
- `sys.net.poll_rx rx_slot -> bytes_read`: Fills `rx_slot` with next frame if available.

---

### 3.5 General Peripheral I/O (GPIO, SPI, I2C, UART)

For microcontrollers (ESP32, STM32, RISC-V), hardware peripherals are mapped into safe intrinsic calls:

- **GPIO**: `call val sys.gpio.read pin_id` / `call ok sys.gpio.write pin_id val`
- **SPI/I2C**: `call bytes_transferred sys.spi.transfer tx_slice rx_buf`
- **UART/Serial**: `call bytes_read sys.uart.read_bytes slot timeout_ms`

All peripheral interactions are bounds-checked, non-blocking, and statically verified by VPA's Lean 4 safety theorem framework.
