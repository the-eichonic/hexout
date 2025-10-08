use std::{fmt::{Display, Debug}};

/// A library for generating customizable hex dumps of binary data in Rust.
/// # Features
/// - Configurable group sizes to any arbitrary byte count (up to 16 bytes)
/// - Adjustable number of groups per line
/// - Options for big-endian or little-endian byte order
/// - Toggleable ASCII representation alongside hex output
/// - Customizable display options (offsets, centerline, uppercase hex)
/// # Usage
/// Add `hexout` to your `Cargo.toml`:
/// ```toml
/// [dependencies]
/// hexout = "0.1"
/// ```
/// Then use it in your Rust code:
/// ```rust
/// use hexout::{hex_dump, HexDumpSettings};
/// let data = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let settings = HexDumpSettings {
///     group_size: 2,
///     groups_per_line: 8,
///     big_endian: false,
///     show_ascii: true,
///     ..Default::default()
/// };
/// let dump = hex_dump(&data, &settings, 0, 0, 1).unwrap();
/// println!("{}", dump);
/// ```
/// # License
/// This project is licensed under the MIT License.
/// # Author

#[derive(Debug, Clone)]
/// Settings to customize the hex dump output.
pub struct HexDumpSettings {
    /// Width of the address field in characters (default is 8 for 32-bit addresses).
    pub address_width: usize,
    /// Whether to align the address to the nearest group boundary.
    pub align_address: bool,
    /// Whether to use big-endian byte order.
    pub big_endian: bool,
    /// Number of bytes in each group.  This can be arbitrary number up to 16, doesn't have to be 1, 2, 4, or 8.
    pub group_size: usize,
    /// Number of groups to display per line.
    pub groups_per_line: usize,
    /// Whether to show the ASCII representation alongside the hex output.
    pub show_ascii: bool,
    /// Whether to show a centerline between groups.
    pub show_centerline: bool,
    /// Whether to show the offset at the start of each line.
    pub show_offset: bool,
    /// Whether to use strict mode (HexOutError on incomplete groups).
    pub strict: bool,
    /// Whether to use uppercase letters for hex digits.
    pub uppercase: bool,
    /// Optional prefix to add to HexOutError indicators.  This is useful for things like ANSI color codes.
    pub hex_out_error_prefix: Option<String>,
    /// Optional postfix to add to HexOutError indicators.  This is useful for things like ANSI color codes.
    pub hex_out_error_postfix: Option<String>,
}

impl Default for HexDumpSettings {
    fn default() -> Self {
        Self {
            address_width: 8,
            align_address: true,
            big_endian: false,
            group_size: 1,
            groups_per_line: 16,
            show_ascii: true,
            show_centerline: true,
            show_offset: true,
            strict: false,
            uppercase: false,
            hex_out_error_prefix: None,
            hex_out_error_postfix: None,
        }
    }
}

pub enum HexOutError {
    /// The specified group size is invalid (must be between 1 and 16).
    InvalidGroupSize,
    /// The specified offset does not align with the group size in strict mode.
    UnalignedOffset { offset: usize, group_size: usize },
}

impl Display for HexOutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HexOutError::InvalidGroupSize => write!(f, "Invalid group size (must be 1-16)"),
            HexOutError::UnalignedOffset { offset, group_size } => {
                write!(
                    f, 
                    "Offset {} does not align with group size {} in strict mode (offset % group_size = {})",
                    offset, group_size, offset % group_size
                )
            }
        }
    }
}

impl Debug for HexOutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HexOutError::InvalidGroupSize => write!(f, "HexOutError::InvalidGroupSize"),
            HexOutError::UnalignedOffset { offset, group_size } => {
                write!(f, "HexOutError::UnalignedOffset {{ offset: {offset}, group_size: {group_size} }}")
            }
        }
    }
}

impl std::error::Error for HexOutError {}

/// A trait to provide hex dump functionality for byte slices.
pub trait HexOut {
    fn hex_dump(&self) -> Result<String, HexOutError>;
    fn hex_dump_lines(&self, start_line: usize, line_count: usize) -> Result<String, HexOutError>;
    fn hex_dump_with_settings(&self, settings: HexDumpSettings) -> Result<String, HexOutError>;
    fn hex_dump_lines_with_settings(
        &self,
        settings: HexDumpSettings,
        start_line: usize,
        line_count: usize,
    ) -> Result<String, HexOutError>;
}

impl HexOut for &[u8] {
    fn hex_dump(&self) -> Result<String, HexOutError> {
        hex_dump(self, &HexDumpSettings::default(), 0, 0, 0)
    }

    fn hex_dump_lines(&self, start_line: usize, line_count: usize) -> Result<String, HexOutError> {
        hex_dump(self, &HexDumpSettings::default(), 0, start_line, line_count)
    }

    fn hex_dump_with_settings(&self, settings: HexDumpSettings) -> Result<String, HexOutError> {
        hex_dump(self, &settings, 0, 0, 0)
    }

    fn hex_dump_lines_with_settings(
        &self,
        settings: HexDumpSettings,
        start_line: usize,
        line_count: usize,
    ) -> Result<String, HexOutError> {
        hex_dump(self, &settings, 0, start_line, line_count)
    }
}

/// Generate a hex dump of the given data with the specified settings.
/// `data`: The byte slice to be dumped.
/// `settings`: The settings to customize the output.
/// `offset`: The starting offset for the dump.
/// `start_line`: The line number to start the dump from (0-based).
/// `line_count`: The number of lines to include in the dump. (If 0, dumps all lines from start_line to the end of data.)
/// Returns a formatted string representing the hex dump.
pub fn hex_dump(
    data: &[u8],
    settings: &HexDumpSettings,
    offset: usize,
    start_line: usize,
    line_count: usize,
) -> Result<String, HexOutError> {
    // Validate group_size
    if settings.group_size == 0 || settings.group_size > 16 {
        return Err(HexOutError::InvalidGroupSize);
    }
    // If strict mode is enabled, ensure we don't start in the middle of a group
    if settings.strict && (offset % settings.group_size != 0) {
        return Err(HexOutError::UnalignedOffset { 
            offset, 
            group_size: settings.group_size 
        });
    }
    let total_bytes_per_line = settings.group_size * settings.groups_per_line;
    // Setup buffers
    let mut line = String::with_capacity(total_bytes_per_line * 3);
    let mut ascii = String::with_capacity(total_bytes_per_line);
    // Calculate total lines
    let last_line_offset = if line_count == 0 {
        data.len()
    } else {
        ((start_line + line_count) * total_bytes_per_line + offset).min(data.len())
    };
    // Align last_line_offset to group boundary
    let last_line_offset = last_line_offset
        + (settings.group_size - (last_line_offset % settings.group_size)) % settings.group_size;

    // Calculate starting index
    // Allocate result string with estimated capacity
    let mut result = String::with_capacity(total_bytes_per_line * line_count * 5);
    let mut cursor = if settings.align_address { 0 } else { offset };
    let mut group_index = 0;
    let mut group_byte_index = 0;
    let mut group_value: u128 = 0;
    let mut out_of_bounds_count = 0;
    // Move cursor to the start line
    cursor += start_line * total_bytes_per_line;
    // Setup the starting address
    let mut addr = if settings.align_address {
        cursor - (cursor % (settings.group_size * settings.groups_per_line))
    } else {
        cursor
    };
    // Main loop to process each byte
    while cursor < last_line_offset {
        let byte = if let Some(b) = data.get(cursor) {
            *b
        } else {
            out_of_bounds_count += 1;
            0
        };
        // If enabled, store ASCII representation of each byte
        if settings.show_ascii {
            if out_of_bounds_count > 0 || cursor < offset {
                ascii.push(' ');
            } else if (0x20..0x80).contains(&byte) {
                ascii.push(byte as char);
            } else {
                ascii.push('.');
            }
        }
        if settings.big_endian {
            // Big-endian: fill the group from the end
            group_value = (group_value << 8) | byte as u128;
        } else {
            // Little-endian: fill the group from the start
            group_value |= (byte as u128) << (8 * group_byte_index);
        }
        group_byte_index += 1;
        // Check if the group is full
        if group_byte_index >= settings.group_size {
            // Add space before group if not the first group
            if !line.is_empty() {
                line.push(' ');
            }
            if cursor < offset {
                // If before the offset, just add spaces
                line.push_str("  ".repeat(settings.group_size).as_str());
            // Group is full, output it
            } else {
                let mut value = if settings.uppercase {
                    format!("{group_value:0width$X}", width = settings.group_size * 2)
                } else {
                    format!("{group_value:0width$x}", width = settings.group_size * 2)
                };
                // If cursor - group_byte_index + 1 < offset, we are still before the offset
                // replace the leading digits (or trailing if big-endian) with question marks
                if cursor.saturating_sub(group_byte_index) < offset || out_of_bounds_count > 0 {
                    let missing_bytes = if out_of_bounds_count > 0 {
                        out_of_bounds_count
                    } else {
                        settings.group_size - (group_byte_index - cursor % settings.group_size)
                    };
                    let replace_chars = (missing_bytes * 2).min(value.len());
                    // The decision to replace leading or trailing characters is based on endiannes and whether we are missing bytes
                    // The following condition represents an XOR operation for the following table:
                    // Big-endian  | Out of bounds bytes | Replace leading chars
                    //    false    |        false        |       false
                    //    false    |         true        |        true
                    //     true    |        false        |        true
                    //     true    |         true        |       false
                    if settings.big_endian != (out_of_bounds_count > 0) {
                        // Big-endian: replace leading characters
                        value.replace_range(0..replace_chars, "?".repeat(replace_chars).as_str());
                    } else {
                        // Little-endian: replace trailing characters
                        let start = value.len() - replace_chars;
                        value.replace_range(start..value.len(), "?".repeat(replace_chars).as_str());
                    }
                }
                line.push_str(&value);
            }
            group_index += 1;
            let is_last_line = cursor + 1 == last_line_offset;
            // Check if we need to add a centerline
            if group_index == settings.groups_per_line / 2 && settings.show_centerline {
                if settings.show_ascii {
                    ascii.push(' ');
                }
                line.push(' ');
            }
            if group_index == settings.groups_per_line || is_last_line || out_of_bounds_count > 0 {
                // End of line or last line
                // Add the address offset if enabled
                if settings.show_offset {
                    result.push_str(&format!(
                        "{:0width$x}: ",
                        addr,
                        width = settings.address_width
                    ));
                }
                // If this is the last line, we may need to pad the line
                if (is_last_line || out_of_bounds_count > 0) && settings.show_ascii {
                    // Calculate padding needed
                    let pad_length = total_bytes_per_line - group_index * settings.group_size;
                    //if group_index > settings.groups_per_line / 2 && settings.show_centerline {
                    //    pad_length -= 1;
                    //}
                    // Pad both hex and ASCII parts
                    let centerline_size = if settings.show_centerline
                        && group_index >= settings.groups_per_line / 2
                    {
                        1
                    } else {
                        0
                    };
                    line.push_str(
                        &" ".repeat(
                            (pad_length * 3 + centerline_size)
                                .saturating_sub(out_of_bounds_count * 2 + 1),
                        ),
                    );
                    if settings.show_ascii {
                        ascii.push_str(
                            &" ".repeat((pad_length + centerline_size).saturating_sub(1)),
                        );
                    }
                }

                // Append the line and ASCII representation to the result
                result.push_str(&line);
                if settings.show_ascii {
                    result.push(' ');
                    result.push('|');
                    result.push_str(&ascii);
                    result.push('|');
                }
                // Add newline if not the last line
                if !is_last_line {
                    result.push('\n');
                }
                // Stop processing if we're past the data length
                if out_of_bounds_count > 0 {
                    break;
                }
                // Rinse and repeat
                line.clear();
                ascii.clear();
                addr += total_bytes_per_line;
                group_index = 0;
            }
            group_value = 0;
            group_byte_index = 0;
        }
        cursor += 1;
    }
    Ok(result)
}

#[cfg(test)]
mod tests;
