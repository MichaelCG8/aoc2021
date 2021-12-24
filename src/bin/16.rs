use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/16");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn hex_to_bin(hex: &str) -> String {
    hex
        .trim()
        .chars()
        .map(|c| u8::from_str_radix(c.to_string().as_str(), 16).unwrap())
        .map(|num| format!("{:04b}", num))
        .collect::<Vec<String>>()
        .join("")
        .to_string()
}


fn parse_binary(binary: &str) -> usize {
    usize::from_str_radix(binary, 2).unwrap()
}


fn parse_group(binary: &str) -> (bool, usize) {
    let last = &binary[..1] == "0";
    let value = parse_binary(&binary[1..]);
    (last, value)
}


struct Packet {
    version: usize,
    packet_type: usize,
    sub_packets: Option<Vec<Packet>>,
    value: Option<usize>,
}

impl Packet {
    fn sum_versions(&self) -> usize {
        let mut sum = self.version;
        if let Some(sub_packets) = &self.sub_packets {
            sum += sub_packets.iter().map(|sp| sp.sum_versions()).sum::<usize>();
        }
        sum
    }

    fn value(&self) -> usize {
        match self.packet_type {
            0 => self.sub_packets.as_ref().unwrap().iter().map(|p| p.value()).sum(),
            1 => self.sub_packets.as_ref().unwrap().iter().map(|p| p.value()).product(),
            2 => self.sub_packets.as_ref().unwrap().iter().map(|p| p.value()).min().unwrap(),
            3 => self.sub_packets.as_ref().unwrap().iter().map(|p| p.value()).max().unwrap(),
            4 => self.value.unwrap(),
            5 => if self.sub_packets.as_ref().unwrap()[0].value() > self.sub_packets.as_ref().unwrap()[1].value() { 1 } else { 0 },
            6 => if self.sub_packets.as_ref().unwrap()[0].value() < self.sub_packets.as_ref().unwrap()[1].value() { 1 } else { 0 },
            7 => if self.sub_packets.as_ref().unwrap()[0].value() == self.sub_packets.as_ref().unwrap()[1].value() { 1 } else { 0 },
            _ => panic!("packet type out of range, got {}", self.packet_type),
        }
    }
}


fn parse_packet(mut binary: String) -> (Packet, String) {
    let version = parse_binary(&binary[..3]);
    binary = binary[3..].to_string();
    let packet_type = parse_binary(&binary[..3]);
    binary = binary[3..].to_string();
    let packet = match packet_type {
        4 => {
            // let mut n_packet_bits = 6;
            let mut value = 0;
            loop {
                value <<= 4;
                let (last, val) = parse_group(&binary[..5]);
                value += val;
                binary = binary[5..].to_string();
                // n_packet_bits += 5;
                if last { break; }
            }
            // let n_nibbles = n_packet_bits / 4;
            // let n_nibble_bits = n_nibbles * 4;
            // let padding_bits = match n_nibble_bits == n_packet_bits {
            //     true => 0,
            //     false => n_nibble_bits + 4 - n_packet_bits,
            // };
            // binary = binary[padding_bits..].to_string();
            Packet{ version, packet_type, sub_packets: None, value: Some(value) }
        },
        _ => {
            let length_type = &binary[..1];
            let sub_packets = match length_type {
                "0" => {
                    let length = parse_binary(&binary[1..16]);
                    binary = binary[16..].to_string();
                    let packets = parse_packets(binary[..length].to_string());
                    binary = binary[length..].to_string();
                    packets
                },
                _ => {
                    let length = parse_binary(&binary[1..12]);
                    binary = binary[12..].to_string();
                    let mut packets = Vec::new();
                    for _ in 0..length {
                        let (packet, trimmed_binary) = parse_packet(binary);
                        packets.push(packet);
                        binary = trimmed_binary;
                    }
                    packets
                },
            };
            Packet{ version, packet_type, sub_packets: Some(sub_packets), value: None }
        },
    };

    (packet, binary)
}


fn parse_packets(mut binary: String) -> Vec<Packet>
{
    let mut result = Vec::new();
    while !binary.is_empty() {
        if binary.chars().all(|c| c == '0') { break; }
        let (packet, trimmed_binary) = parse_packet(binary);
        binary = trimmed_binary;
        result.push(packet);
    }
    result
}

fn part1(data: &str) -> usize {
    let binary_string = hex_to_bin(data);
    let (packet, _trimmed_binary) = parse_packet(binary_string);
    packet.sum_versions()
}


fn part2(data: &str) -> usize {
    let binary_string = hex_to_bin(data);
    let (packet, _trimmed_binary) = parse_packet(binary_string);
    packet.value()
}


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "8A004A801A8002F478";
    // static DATA : &str = "38006F45291200";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 16);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }
}
