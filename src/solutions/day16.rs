use itertools::join;

use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(16);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let x = decode_hex(&input)
        .iter()
        .map(|b| vec![b >> 3 & 1, b >> 2 & 1, b >> 1 & 1, b >> 0 & 1])
        .flatten()
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut version_sum: i64 = 0;

    loop {
        // Nothing more to read, break to now overflow.
        // Version (3) + Type (3) + At least 1 byte (4)
        let min_bites_required = 3 + 3 + 4;
        if i + min_bites_required >= x.len() {
            break;
        }

        let (version, packet_type) = version_and_type(&x[i..=i + 5]);

        version_sum += version as i64;

        // We start reading the actual packet at the 6th byte.
        i += 6;

        match packet_type {
            4u8 => {
                let (next_packet_begins, literals) = read_literal_values(&x[i..]);
                let (_b, _v) = literal_values_to_binary(&literals);

                i += next_packet_begins;
            }
            _ => {
                let (next_packet_begins, _sub_packets_total_len, _is_package_count) =
                    read_operator_values(&x[i..]);

                i += next_packet_begins;
            }
        }
    }

    version_sum
}

fn part_two(_input: String) -> i64 {
    1
}

fn version_and_type(bytes: &[u8]) -> (u8, u8) {
    let version_bytes = join(&bytes[0..=2], "");
    let version = u8::from_str_radix(&version_bytes, 2).unwrap();

    let packet_type_bytes = join(&bytes[3..=5], "");
    let packet_type = u8::from_str_radix(&packet_type_bytes, 2).unwrap();

    (version, packet_type)
}

fn read_operator_values(bytes: &[u8]) -> (usize, u32, bool) {
    let is_11_bit_len = &bytes[0] == &1u8;
    let sub_packets_stop = if is_11_bit_len { 11 } else { 15 };

    let sub_packets_len_bytes = join(&bytes[1..=sub_packets_stop], "");
    let sub_packets_len = u32::from_str_radix(&sub_packets_len_bytes, 2).unwrap();

    // +1 for the next byte and +1 for the first length bit
    (sub_packets_stop + 1, sub_packets_len, is_11_bit_len)
}

fn read_literal_values(bytes: &[u8]) -> (usize, Vec<u8>) {
    let mut values: Vec<u8> = vec![];
    let mut is_last_group: bool = false;

    let mut i = 0usize;
    while !is_last_group {
        is_last_group = &bytes[i] != &1u8;

        let packet_bytes = join(&bytes[i + 1..=i + 4], "");
        let packet = u8::from_str_radix(&packet_bytes, 2).unwrap();
        values.push(packet);

        i += 5;
    }

    (i, values)
}

fn literal_values_to_binary(literals: &[u8]) -> (String, u64) {
    let binary_strings = literals
        .iter()
        .map(|d| format!("{:04b}", d))
        .collect::<Vec<_>>();

    let binary_string = join(binary_strings, "");

    (
        binary_string.clone(),
        u64::from_str_radix(&binary_string, 2).unwrap(),
    )
}

fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.trim_end().len())
        .map(|i| u8::from_str_radix(&s[i..i + 1], 16).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        for (input, expected) in vec![
            ("D2FE28", 6),
            ("38006F45291200", 9),
            ("EE00D40C823060", 14),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ] {
            assert_eq!(super::part_one(input.to_string()), expected);
        }
    }

    #[test]
    fn part_two() {
        for (input, expected) in vec![("9C0141080250320F1802104A08", 1)] {
            assert_eq!(super::part_two(input.to_string()), expected);
        }
    }
}
