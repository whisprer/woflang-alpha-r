# woflang-analog

**Bounded continuum arithmetic for WofLang** 🐺📊

This crate implements WofLang's **analog computing mode** - a fundamentally different computational paradigm where values saturate at boundaries instead of overflowing.

## Philosophy

Traditional computing is digital: values overflow, wrap around, or produce unexpected results at boundaries. Analog mode embraces bounded computation where values saturate at limits, mimicking physical analog systems like:

- 🎛️ **Control voltages** in synthesizers (±5V, ±10V)
- 🧠 **Activation functions** in neural networks (tanh, sigmoid)
- 📊 **Percentage values** in games (health bars, stats)
- 🎮 **Joystick input** (clamped to [-1, 1])

## Features

### Modes

| Mode | Range | Use Case |
|------|-------|----------|
| `Int201` | [-100, +100] | General purpose, percentage-like |
| `Int2001` | [-1000, +1000] | Extended precision |
| `FloatUnit` | [-1.0, +1.0] | Normalized signals, neural nets |
| `FloatCustom` | [min, max] | User-defined domains |

### Operations

- **Basic Math**: add, sub, mul, div, pow, sqrt, abs, neg
- **Interpolation**: lerp, smoothstep, remap, deadzone
- **Trigonometry**: sin, cos, tan, asin, acos, atan, atan2
- **Activation**: tanh, sigmoid, relu, leaky_relu, softplus
- **Linear Algebra**: dot_2d/3d, magnitude, distance, normalize
- **Coordinates**: cartesian↔polar, cartesian↔spherical

## Quick Start

```rust
use woflang_analog::{AnalogMode, AnalogConfig};

// Classic -100 to +100 mode
let config = AnalogConfig::new(AnalogMode::Int201);

// Normal math works as expected
assert_eq!(config.add(50.0, 30.0), 80.0);

// But results SATURATE at boundaries!
assert_eq!(config.add(80.0, 50.0), 100.0);  // Would be 130, saturates to 100
assert_eq!(config.mul(50.0, 50.0), 100.0);  // Would be 2500, saturates to 100
```

## Origin

This crate resurrects WofLang's original analog computing paradigm that was lost during the v2→v3 migration. It's based on extensive C++ implementations from the WofLang archaeological archives.

> "WofLang represents a radical departure from conventional programming languages by embracing several key philosophical approaches: Analog Computing Paradigm… operates on an analog-like continuum from -100 to +100…"
> 
> — *WofLang Design Philosophy*

## License

MIT OR Apache-2.0 (same as the rest of WofLang)
