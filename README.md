# hexout

A compact and dependency-free, flexible and customizable hex dump library for Rust that provides beautiful, configurable binary data visualization.

## Features

- **Arbitrary group sizes**: Group bytes in any size from 1 to 16 bytes (not limited to 1, 2, 4, or 8)
- **Configurable layout**: Adjust groups per line, address width, and spacing
- **Byte order support**: Display data in big-endian or little-endian format
- **ASCII representation**: Optional side-by-side ASCII view with printable character display
- **Flexible addressing**: Show/hide offsets, align addresses to boundaries
- **Customizable formatting**: Uppercase/lowercase hex, centerline separators
- **Partial data handling**: Gracefully handles incomplete groups and missing data
- **Trait-based API**: Convenient methods on byte slices via the `HexOut` trait

## Installation

Add `hexout` to your `Cargo.toml`:

```toml
[dependencies]
hexout = "0.1"
```

## Usage

### Basic Example

```rust
use hexout::hex_dump;

fn main() {
    let data = b"Hello, World!";
    let result = hex_dump(data, &Default::default(), 0, 0, 0).unwrap();
    println!("{}", result);
}
```

Output:
```
00000000: 48 65 6c 6c 6f 2c 20 57  6f 72 6c 64 21          |Hello, W orld!   |
```

### Using the Trait API

```rust
use hexout::HexOut;

fn main() {
    let data = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let dump = data.as_slice().hex_dump().unwrap();
    println!("{}", dump);
}
```

### Custom Settings

```rust
use hexout::{hex_dump, HexDumpSettings};

fn main() {
    let data: Vec<u8> = (0..32).collect();
    
    let settings = HexDumpSettings {
        group_size: 4,           // 4 bytes per group (32-bit words)
        groups_per_line: 4,      // 4 groups per line
        big_endian: false,       // Little-endian byte order
        show_ascii: true,        // Show ASCII representation
        show_centerline: true,   // Add space in the middle
        uppercase: true,         // Uppercase hex digits
        ..Default::default()
    };
    
    let dump = hex_dump(&data, &settings, 0, 0, 0).unwrap();
    println!("{}", dump);
}
```

Output:
```
00000000: 03020100 07060504  0B0A0908 0F0E0D0C |........ ........|
00000010: 13121110 17161514  1B1A1918 1F1E1D1C |........ ........|
```

### 16-bit Words (Big-Endian)

```rust
use hexout::{hex_dump, HexDumpSettings};

fn main() {
    let data: Vec<u8> = (0..32).collect();
    
    let settings = HexDumpSettings {
        group_size: 2,           // 2 bytes per group (16-bit words)
        groups_per_line: 8,      // 8 groups per line
        big_endian: true,        // Big-endian byte order
        show_ascii: true,
        ..Default::default()
    };
    
    let dump = hex_dump(&data, &settings, 0, 0, 0).unwrap();
    println!("{}", dump);
}
```

Output:
```
00000000: 0001 0203 0405 0607  0809 0a0b 0c0d 0e0f |........ ........|
00000010: 1011 1213 1415 1617  1819 1a1b 1c1d 1e1f |........ ........|
```

### Paginated Output

```rust
use hexout::{hex_dump, HexDumpSettings};

fn main() {
    let data: Vec<u8> = (0..128).collect();
    
    // Display lines 2-3 (0-based indexing)
    let dump = hex_dump(&data, &HexDumpSettings::default(), 0, 2, 2).unwrap();
    println!("{}", dump);
}
```

### Custom Address Offsets

```rust
use hexout::{hex_dump, HexDumpSettings};

fn main() {
    let data = vec![0xAA; 16];
    
    let settings = HexDumpSettings {
        address_width: 16,  // 64-bit addresses
        ..Default::default()
    };
    
    // Start addresses at 0x1000
    let dump = hex_dump(&data, &settings, 0x1000, 0, 0).unwrap();
    println!("{}", dump);
}
```

Output:
```
0000000000001000: aa aa aa aa aa aa aa aa  aa aa aa aa aa aa aa aa |........ ........|
```

### Minimal Output (No Offsets or ASCII)

```rust
use hexout::{hex_dump, HexDumpSettings};

fn main() {
    let data = vec![0x12, 0x34, 0x56, 0x78];
    
    let settings = HexDumpSettings {
        show_offset: false,
        show_ascii: false,
        show_centerline: false,
        ..Default::default()
    };
    
    let dump = hex_dump(&data, &settings, 0, 0, 0).unwrap();
    println!("{}", dump);
}
```

Output:
```
12 34 56 78
```

## Configuration Options

The `HexDumpSettings` struct provides the following options:

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `address_origin` | `usize` | `0` | The origin address to start from.  Does not change the actual offset of the data, just the displayed address. |
| `address_width` | `usize` | `8` | Width of the address field in hex characters |
| `align_address` | `bool` | `true` | Align addresses to group boundaries |
| `big_endian` | `bool` | `false` | Use big-endian byte order within groups |
| `group_size` | `usize` | `1` | Number of bytes per group (1-16) |
| `groups_per_line` | `usize` | `16` | Number of groups to display per line |
| `invalid_data_placeholder` | `char` | `?` | Character to use for invalid or out-of-bounds data |
| `show_ascii` | `bool` | `true` | Show ASCII representation |
| `show_centerline` | `bool` | `true` | Add extra space at line midpoint |
| `show_offset` | `bool` | `true` | Show address offset at line start |
| `strict` | `bool` | `false` | Return error on unaligned offsets |
| `uppercase` | `bool` | `false` | Use uppercase hex digits (A-F) |
| `hex_out_error_prefix` | `Option<String>` | `None` | Prefix for error indicators (e.g., ANSI codes) |
| `hex_out_error_postfix` | `Option<String>` | `None` | Postfix for error indicators |

## Error Handling

The library returns `Result<String, HexOutError>` with the following error types:

- `InvalidGroupSize`: Group size must be between 1 and 16
- `UnalignedOffset`: In strict mode, offset must align with group size

```rust
use hexout::{hex_dump, HexDumpSettings, HexOutError};

fn main() {
    let data = vec![0u8; 10];
    let settings = HexDumpSettings {
        group_size: 4,
        strict: true,
        ..Default::default()
    };
    
    // This will error because offset 1 doesn't align to group_size 4
    match hex_dump(&data, &settings, 1, 0, 0) {
        Ok(dump) => println!("{}", dump),
        Err(HexOutError::UnalignedOffset { offset, group_size }) => {
            eprintln!("Error: offset {} not aligned to group size {}", offset, group_size);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Roadmap

[ ] Selectable 'ascii section' border symbols

[ ] Highlighting (address, ascii, number ranges?)

[ ] Streaming mode
