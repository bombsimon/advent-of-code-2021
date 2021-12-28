use itertools::join;

use crate::input;

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: u8,
    literal: i64,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn version_sum(&self) -> i64 {
        self.sub_packets
            .iter()
            .fold(self.version as i64, |acc, pkt| acc + pkt.version_sum())
    }

    fn evaluate(&self) -> i64 {
        match self.packet_type {
            4 => self.literal,
            0 => self.sub_packets.iter().map(Packet::evaluate).sum(),
            1 => self.sub_packets.iter().map(Packet::evaluate).product(),
            2 => self.sub_packets.iter().map(Packet::evaluate).min().unwrap(),
            3 => self.sub_packets.iter().map(Packet::evaluate).max().unwrap(),
            x => {
                let fst = self.sub_packets[0].evaluate();
                let snd = self.sub_packets[1].evaluate();

                match x {
                    5 => (fst > snd).into(),
                    6 => (fst < snd).into(),
                    7 => (fst == snd).into(),
                    _ => unreachable!(),
                }
            }
        }
    }
}

pub fn solve() {
    let x = input::raw_file_for_day(16);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let bin = hex_string_as_binary(input);
    let (packet, _) = read_packets(&bin);

    packet.version_sum() as i64
}

fn part_two(input: String) -> i64 {
    let bin = hex_string_as_binary(input);
    let (packet, _) = read_packets(&bin);

    packet.evaluate() as i64
}

fn read_packets(bytes: &[u8]) -> (Packet, usize) {
    let (version, packet_type) = version_and_type(&bytes[0..=5]);
    let mut packet = Packet {
        version,
        packet_type,
        literal: 0,
        sub_packets: vec![],
    };

    let mut i = 6usize;
    match packet_type {
        4u8 => {
            let (bits_read, literals) = read_literal_values(&bytes[i..]);
            let (_b, v) = literal_values_to_binary(&literals);

            packet.literal = v;

            i += bits_read;
        }
        _ => {
            let (next_packet_begins, packet_len_or_count, is_package_count) =
                read_operator_values(&bytes[i..]);

            i += next_packet_begins;

            if is_package_count {
                for _ in 0..packet_len_or_count {
                    let (sub_packet, consumed) = read_packets(&bytes[i..]);
                    packet.sub_packets.push(sub_packet);

                    i += consumed;
                }
            } else {
                let stop_at = i + packet_len_or_count as usize;
                while i < stop_at {
                    let (sub_packet, consumed) = read_packets(&bytes[i..]);
                    packet.sub_packets.push(sub_packet);

                    i += consumed;
                }
            }
        }
    }

    (packet, i)
}

fn hex_string_as_binary(input: String) -> Vec<u8> {
    decode_hex(&input)
        .iter()
        .map(|b| vec![b >> 3 & 1, b >> 2 & 1, b >> 1 & 1, b & 1])
        .flatten()
        .collect()
}

fn version_and_type(bytes: &[u8]) -> (u8, u8) {
    let version_bytes = join(&bytes[0..=2], "");
    let version = u8::from_str_radix(&version_bytes, 2).unwrap();

    let packet_type_bytes = join(&bytes[3..=5], "");
    let packet_type = u8::from_str_radix(&packet_type_bytes, 2).unwrap();

    (version, packet_type)
}

fn read_operator_values(bytes: &[u8]) -> (usize, u32, bool) {
    let is_11_bit_len = bytes[0] == 1u8;
    let sub_packets_stop = if is_11_bit_len { 11 } else { 15 };

    let sub_packets_len_bytes = join(&bytes[1..=sub_packets_stop], "");
    let sub_packets_len = u32::from_str_radix(&sub_packets_len_bytes, 2).unwrap();

    // +1 for the next bit
    (sub_packets_stop + 1, sub_packets_len, is_11_bit_len)
}

fn read_literal_values(bytes: &[u8]) -> (usize, Vec<u8>) {
    let mut values: Vec<u8> = vec![];
    let mut is_last_group: bool = false;
    let mut i = 0usize;

    while !is_last_group {
        is_last_group = bytes[i] != 1u8;

        let packet_bytes = join(&bytes[i + 1..=i + 4], "");
        let packet = u8::from_str_radix(&packet_bytes, 2).unwrap();
        values.push(packet);

        i += 5;
    }

    (i, values)
}

fn literal_values_to_binary(literals: &[u8]) -> (String, i64) {
    let binary_strings: Vec<String> = literals.iter().map(|d| format!("{:04b}", d)).collect();

    let binary_string = join(binary_strings, "");
    let decimal = i64::from_str_radix(&binary_string, 2).unwrap();

    (binary_string, decimal)
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
        for (input, expected) in &[
            ("D2FE28", 6),
            ("38006F45291200", 9),
            ("EE00D40C823060", 14),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ] {
            assert_eq!(super::part_one(input.to_string()), *expected);
        }
    }

    #[test]
    fn part_two() {
        for (input, expected) in &[
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ] {
            assert_eq!(super::part_two(input.to_string()), *expected);
        }
    }
}
