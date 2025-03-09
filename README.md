# Remo-Windows

Remo-Windows is a high-performance remote desktop and cloud gaming solution written in Rust. It provides low-latency screen sharing, audio streaming, and input control capabilities using WebRTC technology.

## Features

- **Screen Capture**: Hardware-accelerated screen capture with support for cursor overlay and window highlighting
- **Audio Streaming**: Real-time audio capture and streaming
- **Input Control**: 
  - Keyboard and mouse input forwarding
  - Xbox 360 controller emulation
  - Multi-controller support
- **Video Encoding**:
  - Hardware-accelerated H.264 encoding
  - Configurable quality settings
  - Adaptive bitrate
- **Audio Encoding**:
  - Opus audio codec
  - Configurable quality settings
- **Network**:
  - WebRTC for low-latency streaming
  - Automatic NAT traversal using STUN/TURN
  - Secure WebSocket signaling

## Getting Started

### Prerequisites

1. Install Rust and Cargo (https://rustup.rs/)
2. Required system dependencies:
   - FFmpeg libraries for video/audio encoding
   - Screen capture capabilities (Windows: DXGI, macOS: ScreenCaptureKit, Linux: X11/Wayland)
   - ViGEmBus driver (Windows-only, for controller support)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/adidharmatoru/remo-core.git
cd remo-core
```

2. Build the project:
```bash
cargo build --release
```

### Running

```bash
cargo run --release
```

## Development

### Running Tests

```bash
cargo test
```

### Test Coverage

```bash
cargo install cargo-tarpaulin
cargo tarpaulin
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.