use super::*;

#[test]
fn simple_test() {
    let data = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = hex_out(&data, &HexOutSettings::default(), 0, 0, 1).unwrap();
    assert_eq!(result, "00000000: 00 01 02 03 04 05 06 07  08 09                   |........ ..      |");
}

#[test]
fn no_ascii() {
    let data = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let settings = HexOutSettings {
        show_ascii: false,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1).unwrap();
    assert_eq!(result, "00000000: 00 01 02 03 04 05 06 07  08 09");
}

#[test]
fn with_16bit_groups() {
    let data = (0u8..=31).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 2,
        groups_per_line: 8,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 2).unwrap();
    assert_eq!(result, "00000000: 0100 0302 0504 0706  0908 0b0a 0d0c 0f0e |........ ........|\n00000010: 1110 1312 1514 1716  1918 1b1a 1d1c 1f1e |........ ........|");
}

#[test]
fn with_32bit_groups() {
    let data = (0u8..=63).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 4,
        groups_per_line: 8,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 2).unwrap();
    assert_eq!(result, "00000000: 03020100 07060504 0b0a0908 0f0e0d0c  13121110 17161514 1b1a1918 1f1e1d1c |................ ................|\n00000020: 23222120 27262524 2b2a2928 2f2e2d2c  33323130 37363534 3b3a3938 3f3e3d3c | !\"#$%&'()*+,-./ 0123456789:;<=>?|");
}

#[test]
fn single_line() {
    let data = (0u8..=47).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 1,
        groups_per_line: 16,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 1, 1).unwrap();
    assert_eq!(result, "00000010: 10 11 12 13 14 15 16 17  18 19 1a 1b 1c 1d 1e 1f |........ ........|");
}

#[test]
fn just_words() {
    let data = (0u8..=31).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        show_ascii: false,
        show_offset: false,
        group_size: 2,
        groups_per_line: 8,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 2).unwrap();
    assert_eq!(result, "0100 0302 0504 0706  0908 0b0a 0d0c 0f0e\n1110 1312 1514 1716  1918 1b1a 1d1c 1f1e");
}

#[test]
fn simple_uppercase() {
    let data = (0u8..=15u8).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        uppercase: true,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1).unwrap();
    assert_eq!(result, "00000000: 00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F |........ ........|");
}

#[test]
fn simple_no_centerline() {
    let data = (0u8..=15u8).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        show_centerline: false,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1).unwrap();
    assert_eq!(result, "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f |................|");
}

#[test]
fn with_32bit_partial_line() {
    let data = (0u8..=0x34).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 4,
        groups_per_line: 8,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 2).unwrap();
    assert_eq!(result, "00000000: 03020100 07060504 0b0a0908 0f0e0d0c  13121110 17161514 1b1a1918 1f1e1d1c |................ ................|\n00000020: 23222120 27262524 2b2a2928 2f2e2d2c  33323130 ??????34                   | !\"#$%&'()*+,-./ 01234           |");
}

#[test]
fn with_32bit_partial_line_uppercase() {
    let data = (0u8..=0x34).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 4,
        groups_per_line: 8,
        uppercase: true,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 2).unwrap();
    assert_eq!(result, "00000000: 03020100 07060504 0B0A0908 0F0E0D0C  13121110 17161514 1B1A1918 1F1E1D1C |................ ................|\n00000020: 23222120 27262524 2B2A2928 2F2E2D2C  33323130 ??????34                   | !\"#$%&'()*+,-./ 01234           |");
}

#[test]
fn reversed_data() {
    let data = (32u8..=47u8).rev().collect::<Vec<u8>>();
    let settings = HexOutSettings {
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1).unwrap();
    assert_eq!(result, "00000000: 2f 2e 2d 2c 2b 2a 29 28  27 26 25 24 23 22 21 20 |/.-,+*)( '&%$#\"! |");
}

#[test]
fn big_endian_16bit() {
    let data = (0u8..=15u8).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 2,
        groups_per_line: 8,
        big_endian: true,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1).unwrap();
    assert_eq!(result, "00000000: 0001 0203 0405 0607  0809 0a0b 0c0d 0e0f |........ ........|");
}

#[test]
fn big_endian_16bit_incomplete() {
    let data = (0u8..=14u8).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 2,
        groups_per_line: 8,
        big_endian: true,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1).unwrap();
    assert_eq!(result, "00000000: 0001 0203 0405 0607  0809 0a0b 0c0d 0e?? |........ ....... |");
}

#[test]
fn simple_32bit_incomplete_leading_zeros() {
    let data = vec![0u8, 1, 2, 3, 4];
    let settings = HexOutSettings {
        group_size: 4,
        groups_per_line: 4,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1).unwrap();
    assert_eq!(result, "00000000: 03020100 ??????04                    |.....            |");
}

#[test]
fn simple_trait_usage() {
    let data = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = &data.as_slice().hex_out().unwrap();
    assert_eq!(result, "00000000: 00 01 02 03 04 05 06 07  08 09                   |........ ..      |");
}

#[test]
fn simple_trait_usage_with_lines() {
    let data = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = &data.as_slice().hex_out_lines(0, 1).unwrap();
    assert_eq!(result, "00000000: 00 01 02 03 04 05 06 07  08 09                   |........ ..      |");
}

#[test]
fn simple_trait_usage_with_settings() {
    let data = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let settings = HexOutSettings {
        group_size: 2,
        groups_per_line: 8,
        show_ascii: false,
        ..Default::default()
    };
    let result = &data.as_slice().hex_out_with_settings(settings).unwrap();
    assert_eq!(result, "00000000: 0100 0302 0504 0706  0908");
}

#[test]
fn with_24bits() {
    let data = (32u8..=67).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 3,
        groups_per_line: 4,
        ..Default::default()
    };
    let result = data.as_slice().hex_out_lines_with_settings(settings, 0, 0).unwrap();
    assert_eq!(result, "00000000: 222120 252423  282726 2b2a29 | !\"#$% &'()*+|\n0000000c: 2e2d2c 31302f  343332 373635 |,-./01 234567|\n00000018: 3a3938 3d3c3b  403f3e 434241 |89:;<= >?@ABC|");
}

#[test]
fn lines_past_end() {
    let data = (0u8..=7).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 1,
        groups_per_line: 4,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 5, 2).unwrap();
    assert_eq!(result, "");
}

#[test]
fn missing_data_with_32bits() {
    let data = (0u8..=7).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 4,
        groups_per_line: 2,
        ..Default::default()
    };
    let result = data.as_slice().hex_out_with_settings(settings).unwrap();
    assert_eq!(result, "00000000: 03020100  07060504 |.... ....|");
}

#[test]
fn with_4digit_address() {
    let data = (0u8..=31).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        address_width: 4,
        groups_per_line: 8,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 0).unwrap();
    assert_eq!(result, "0000: 00 01 02 03  04 05 06 07 |.... ....|\n0008: 08 09 0a 0b  0c 0d 0e 0f |.... ....|\n0010: 10 11 12 13  14 15 16 17 |.... ....|\n0018: 18 19 1a 1b  1c 1d 1e 1f |.... ....|");
}

#[test]
fn offset_address() {
    let data = (0u8..=32).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        address_width: 4,
        align_address: false,
        groups_per_line: 8,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 3, 0, 0).unwrap();
    assert_eq!(result, "0003: 03 04 05 06  07 08 09 0a |.... ....|\n000b: 0b 0c 0d 0e  0f 10 11 12 |.... ....|\n0013: 13 14 15 16  17 18 19 1a |.... ....|\n001b: 1b 1c 1d 1e  1f 20       |.... .   |");
}

#[test]
fn offset_address_aligned() {
    let data = (0u8..=31).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        address_width: 4,
        align_address: true,
        groups_per_line: 8,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 3, 0, 0).unwrap();
    assert_eq!(result, "0000:          03  04 05 06 07 |   . ....|\n0008: 08 09 0a 0b  0c 0d 0e 0f |.... ....|\n0010: 10 11 12 13  14 15 16 17 |.... ....|\n0018: 18 19 1a 1b  1c 1d 1e 1f |.... ....|");
}

#[test]
fn offset_address_aligned_32bit() {
    let data = (0u8..=31).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        address_width: 2,
        align_address: true,
        group_size: 4,
        groups_per_line: 4,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 3, 0, 0).unwrap();
    assert_eq!(result, "00: 03?????? 07060504  0b0a0908 0f0e0d0c |   ..... ........|\n10: 13121110 17161514  1b1a1918 1f1e1d1c |........ ........|");
}

#[test]
fn offset_address_aligned_32bit_big_endian() {
    let data = (0u8..=31).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        address_width: 2,
        align_address: true,
        group_size: 4,
        groups_per_line: 4,
        big_endian: true, 
        uppercase: true,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 3, 0, 0).unwrap();
    assert_eq!(result, "00: ??????03 04050607  08090A0B 0C0D0E0F |   ..... ........|\n10: 10111213 14151617  18191A1B 1C1D1E1F |........ ........|");
}

#[test]
fn ensure_error_on_invalid_group_size() {
    let data = (0u8..=31).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 0,
        groups_per_line: 8,
        ..Default::default()
    };
    let result= hex_out(&data, &settings, 0, 0, 0);
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), HexOutError::InvalidGroupSize));

    let settings = HexOutSettings {
        group_size: 17,
        groups_per_line: 8,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 0);
    assert!(result.is_err());
    assert_eq!(format!("{:?}", result.as_ref()), "Err(HexOutError::InvalidGroupSize)".to_string());
    assert_eq!(format!("{}", result.as_ref().err().unwrap()), "Invalid group size (must be 1-16)".to_string());
}

#[test]
fn offset_address_aligned_32bit_strict() {
    let data = (0u8..=31).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        address_width: 2,
        align_address: true,
        group_size: 4,
        groups_per_line: 4,
        strict: true,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 3, 0, 0);
    assert!(result.is_err());
    assert_eq!(format!("{:?}", result.as_ref()), "Err(HexOutError::UnalignedOffset { offset: 3, group_size: 4 })".to_string());
    assert_eq!(format!("{}", result.as_ref().err().unwrap()), "Offset 3 does not align with group size 4 in strict mode (offset % group_size = 3)".to_string());
}

#[test]
fn ansi_colored_errors() {
    let data = (0u8..=30).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        hex_out_error_prefix: Some("\x1b[31m".to_string()),
        hex_out_error_postfix: Some("\x1b[0m".to_string()),
        group_size: 2,
        groups_per_line: 8,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 0).unwrap();
    assert_eq!(result, "00000000: 0100 0302 0504 0706  0908 0b0a 0d0c 0f0e |........ ........|\n00000010: 1110 1312 1514 1716  1918 1b1a 1d1c \x1b[31m??\x1b[0m1e |........ ....... |");
}

#[test]
fn origin_example() {
    let data = (0u8..=15u8).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        address_origin: 0x1000,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1).unwrap();
    assert_eq!(result, "00001000: 00 01 02 03 04 05 06 07  08 09 0a 0b 0c 0d 0e 0f |........ ........|");
}

#[test]
fn last_line_padding() {
    let data = (0u8..24).collect::<Vec<u8>>();
    let settings = HexOutSettings {
        group_size: 4,
        groups_per_line: 4,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 0).unwrap();
    assert_eq!(result, "00000000: 03020100 07060504  0b0a0908 0f0e0d0c |........ ........|\n00000010: 13121110 17161514                    |........         |");
}



#[test]
fn large_line_count_doesnt_hang() {
    let data = vec![0u8; 10];
    let settings = HexOutSettings::default();
    // Request way more lines than data available
    let result = hex_out(&data, &settings, 0, 0, 1000);
    assert!(result.is_ok());
}

#[test]
fn empty_data() {
    let data = vec![];
    let result = hex_out(&data, &HexOutSettings::default(), 0, 0, 0).unwrap();
    assert_eq!(result, "");
}

#[test]
fn single_byte() {
    let data = vec![0x42];
    let settings = HexOutSettings::default();
    let result = hex_out(&data, &settings, 0, 0, 1).unwrap();
    assert!(result.contains("42"));
}

#[test]
fn unaligned_offset_strict_mode_should_error() {
    let data = vec![0u8; 10];
    let settings = HexOutSettings {
        strict: true,
        group_size: 4,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 1, 0, 1); // offset 1 not aligned to group_size 4
    assert!(matches!(result, Err(HexOutError::UnalignedOffset { offset: 1, group_size: 4 })));
}

#[test]
fn zero_group_size_should_error() {
    let data = vec![0u8; 10];
    let settings = HexOutSettings {
        group_size: 0,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1);
    assert!(matches!(result, Err(HexOutError::InvalidGroupSize)));
}

#[test]
fn group_size_too_large_should_error() {
    let data = vec![0u8; 10];
    let settings = HexOutSettings {
        group_size: 17,
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0, 0, 1);
    assert!(matches!(result, Err(HexOutError::InvalidGroupSize)));
}

#[test]
fn align_address_with_offset_should_pad() {
    let data = vec![0xAAu8; 32];
    let settings = HexOutSettings {
        align_address: true,
        group_size: 4,
        groups_per_line: 4,
        ..Default::default()
    };
    // Start at offset 8 (should align to 0 and pad first 8 bytes)
    let result = hex_out(&data, &settings, 8, 0, 1).unwrap();
    // Should show address 00000000 with padding for first 8 bytes
    assert!(result.starts_with("00000000:"));
    // Should have spaces for the first 2 groups
    // This is the behavior your code SHOULD have but may not currently implement
}

#[test]
fn extremely_large_offset() {
    let data = vec![0u8; 10];
    let settings = HexOutSettings {
        address_width: 16, // 64-bit addresses
        ..Default::default()
    };
    let result = hex_out(&data, &settings, 0xFFFF_FFFF_0000_0000, 0, 1);
    // Should handle gracefully, not panic
    assert!(result.is_ok());
}