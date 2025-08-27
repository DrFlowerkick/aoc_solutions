//!day_16.rs

use anyhow::Result;

struct ChallengeInput {
    bits: String,
}

impl From<&str> for ChallengeInput {
    fn from(value: &str) -> Self {
        ChallengeInput {
            bits: value
                .chars()
                .flat_map(|hex| {
                    match hex {
                        '0' => "0000",
                        '1' => "0001",
                        '2' => "0010",
                        '3' => "0011",
                        '4' => "0100",
                        '5' => "0101",
                        '6' => "0110",
                        '7' => "0111",
                        '8' => "1000",
                        '9' => "1001",
                        'A' => "1010",
                        'B' => "1011",
                        'C' => "1100",
                        'D' => "1101",
                        'E' => "1110",
                        'F' => "1111",
                        _ => panic!("unexpected char"),
                    }
                    .chars()
                })
                .collect(),
        }
    }
}

impl ChallengeInput {
    fn solution_part_1_and_2(&self) -> (u64, u64) {
        let (version_sum, _, value) = Self::parse_packet(&self.bits);
        (version_sum, value)
    }
    fn parse_packet(packet_slice: &str) -> (u64, usize, u64) {
        let mut slice_position = 0;
        // 1. parse version
        let version = &packet_slice[slice_position..slice_position + 3];
        let mut version_sum = u64::from_str_radix(version, 2).unwrap();
        slice_position += 3;
        // 2. parse type ID
        let type_id = &packet_slice[slice_position..slice_position + 3];
        let type_id = u8::from_str_radix(type_id, 2).unwrap();
        slice_position += 3;
        // 3. check type ID
        let value = match type_id {
            4 => {
                // literal value -> parse it
                let mut literal_value = String::new();
                // use while to prevent endless loop
                let mut parse_bit_groups = true;
                while parse_bit_groups && slice_position < packet_slice.len() {
                    parse_bit_groups = &packet_slice[slice_position..slice_position + 1] == "1";
                    literal_value.push_str(&packet_slice[slice_position + 1..slice_position + 5]);
                    slice_position += 5;
                }
                // probably somehow use literal_value in part 2
                u64::from_str_radix(&literal_value, 2).unwrap()
            }
            _ => {
                // some operator, rework in part 2
                let length_type_id = &packet_slice[slice_position..slice_position + 1] == "1";
                slice_position += 1;
                let mut values: Vec<u64> = Vec::new();
                if length_type_id {
                    // parse number of sub packets
                    let num_sub_packets = &packet_slice[slice_position..slice_position + 11];
                    let mut num_sub_packets = usize::from_str_radix(num_sub_packets, 2).unwrap();
                    slice_position += 11;
                    while num_sub_packets > 0 {
                        let (sub_version_sum, sub_length, sub_value) =
                            Self::parse_packet(&packet_slice[slice_position..]);
                        version_sum += sub_version_sum;
                        slice_position += sub_length;
                        num_sub_packets -= 1;
                        values.push(sub_value);
                    }
                } else {
                    // parse length of bits of sub packets
                    let length = &packet_slice[slice_position..slice_position + 15];
                    let mut length = usize::from_str_radix(length, 2).unwrap();
                    slice_position += 15;
                    while length > 0 {
                        let (sub_version_sum, sub_length, sub_value) =
                            Self::parse_packet(&packet_slice[slice_position..]);
                        version_sum += sub_version_sum;
                        slice_position += sub_length;
                        assert!(length >= sub_length);
                        length -= sub_length;
                        values.push(sub_value);
                    }
                }
                match type_id {
                    0 => {
                        assert!(!values.is_empty());
                        values.iter().sum()
                    }
                    1 => {
                        assert!(!values.is_empty());
                        values.iter().product()
                    }
                    2 => {
                        assert!(!values.is_empty());
                        *values.iter().min().unwrap()
                    }
                    3 => {
                        assert!(!values.is_empty());
                        *values.iter().max().unwrap()
                    }
                    5 => {
                        assert!(values.len() == 2);
                        if values[0] > values[1] { 1 } else { 0 }
                    }
                    6 => {
                        assert!(values.len() == 2);
                        if values[0] < values[1] { 1 } else { 0 }
                    }
                    7 => {
                        assert!(values.len() == 2);
                        if values[0] == values[1] { 1 } else { 0 }
                    }
                    _ => panic!("unexpected operator ID"),
                }
            }
        };

        (version_sum, slice_position, value)
    }
}

pub fn solution() -> Result<()> {
    let input = include_str!("../../../../aoc_input/aoc-2021/day_16.txt");
    let challenge = ChallengeInput::from(input);

    let (result_part1, result_part2) = challenge.solution_part_1_and_2();
    println!("result day_16 part 1: {result_part1}");
    assert_eq!(result_part1, 938);

    println!("result day_16 part 2: {result_part2}");
    assert_eq!(result_part2, 1_495_959_086_337);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn byte_test() {
        let test_01 = "D2FE28";
        let bits = ChallengeInput::from(test_01);
        assert_eq!(bits.bits, "110100101111111000101000");
        let (version, length, value) = ChallengeInput::parse_packet(&bits.bits);
        assert_eq!(version, 6);
        assert_eq!(length, 21);
        assert_eq!(value, 2021);
        assert_eq!(&bits.bits[length..], "000");

        let test_02 = "38006F45291200";
        let bits = ChallengeInput::from(test_02);
        let (version, length, value) = ChallengeInput::parse_packet(&bits.bits);
        assert_eq!(version, 9);
        assert_eq!(length, 49);
        assert_eq!(value, 1);
        assert_eq!(&bits.bits[length..], "0000000");

        let test_03 = "EE00D40C823060";
        let bits = ChallengeInput::from(test_03);
        let (version, length, value) = ChallengeInput::parse_packet(&bits.bits);
        assert_eq!(version, 14);
        assert_eq!(length, 51);
        assert_eq!(value, 3);
        assert_eq!(&bits.bits[length..], "00000");
    }

    #[test]
    fn test_example_part() -> Result<()> {
        let input = include_str!("../../../../aoc_input/aoc-2021/day_16_example.txt");
        let expected_results_part_1 = [16, 12, 23, 31];
        let expected_results_part_2 = [15, 46, 46, 54];
        for (index, line) in input.lines().enumerate() {
            let example = ChallengeInput::from(line);

            let (result_part1, result_part2) = example.solution_part_1_and_2();
            println!("result day_16 part 1: {result_part1}");
            assert_eq!(result_part1, expected_results_part_1[index]);

            println!("result day_16 part 2: {result_part2}");
            assert_eq!(result_part2, expected_results_part_2[index]);
        }

        Ok(())
    }
}
