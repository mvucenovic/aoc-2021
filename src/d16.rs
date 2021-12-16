use anyhow::Context;
use nom::IResult;

pub fn part_01() -> anyhow::Result<u32> {
    let bits = inputs()?;

    let (_, packet) = Packet::parse((&bits, 0)).unwrap();

    Ok(packet.version_sum())
}

pub fn part_02() -> anyhow::Result<u64> {
    let bits = inputs()?;

    let (_, packet) = Packet::parse((&bits, 0)).unwrap();

    Ok(packet.value())
}

fn inputs() -> anyhow::Result<Vec<u8>> {
    let input_string =
        std::fs::read_to_string("inputs/16_input.txt").context("Error while reading input")?;

    str_to_bit_vec(&input_string)
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    content: Content,
}

impl Packet {
    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
        let (input, version) = nom::bits::complete::take(3usize)(input)?;
        let (input, type_id) = nom::bits::complete::take(3usize)(input)?;

        let (input, content) = Content::parse(type_id, input)?;
        let packet = Packet {
            version,
            type_id,
            content,
        };
        Ok((input, packet))
    }

    fn version_sum(&self) -> u32 {
        match &self.content {
            Content::LiteralPacket { .. } => self.version as u32,
            Content::Operator(Operator { packets, .. }) => {
                let sub_packets_version_sum: u32 = packets
                    .iter()
                    .map(|sub_packet| sub_packet.version_sum())
                    .sum();
                self.version as u32 + sub_packets_version_sum
            }
        }
    }

    fn value(&self) -> u64 {
        match &self.content {
            Content::LiteralPacket { value, .. } => *value,
            Content::Operator(operator) => {
                let packets = &operator.packets;

                match self.type_id {
                    SUM_TYPE_ID => packets.iter().map(|p| p.value()).sum(),
                    PRODUCT_TYPE_ID => packets.iter().map(|p| p.value()).product(),
                    MINIMUM_TYPE_ID => packets.iter().map(|p| p.value()).min().unwrap(),
                    MAXIMUM_TYPE_ID => packets.iter().map(|p| p.value()).max().unwrap(),
                    GREATER_THAN_TYPE_ID => {
                        if packets[0].value() > packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    LESS_THAN_TYPE_ID => {
                        if packets[0].value() < packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    EQUAL_TYPE_ID => {
                        if packets[0].value() == packets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("uknown type id"),
                }
            }
        }
    }

    fn bit_length(&self) -> usize {
        self.content.bit_length() + 3 + 3
    }
}

#[derive(Debug)]
struct Operator {
    length_type_id: u8,
    bit_length: usize,
    packets: Vec<Packet>,
}

impl Operator {
    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Operator> {
        let mut bit_length = 1;
        let (input, length_type_id) = nom::bits::complete::take(1usize)(input)?;
        let mut packets = vec![];
        if length_type_id == 0 {
            let (mut input, total_bit_length): (_, usize) =
                nom::bits::complete::take(15usize)(input)?;
            bit_length += 15 + total_bit_length;

            let mut subpackets_bits_read = 0;
            while subpackets_bits_read < total_bit_length {
                let (new_input, sub_packet) = Packet::parse(input)?;
                input = new_input;
                subpackets_bits_read += sub_packet.bit_length();
                packets.push(sub_packet);
            }

            return Ok((
                input,
                Operator {
                    length_type_id,
                    bit_length,
                    packets,
                },
            ));
        } else {
            let (mut input, number_of_sub_packets): (_, usize) =
                nom::bits::complete::take(11usize)(input)?;
            bit_length += 11;

            for _ in 0..number_of_sub_packets {
                let (new_input, sub_packet) = Packet::parse(input)?;
                input = new_input;
                bit_length += sub_packet.bit_length();
                packets.push(sub_packet);
            }

            return Ok((
                input,
                Operator {
                    length_type_id,
                    bit_length,
                    packets,
                },
            ));
        }
    }
}

#[derive(Debug)]
pub enum Content {
    LiteralPacket { value: u64, bit_length: usize },
    Operator(Operator),
}

const SUM_TYPE_ID: u8 = 0;
const PRODUCT_TYPE_ID: u8 = 1;
const MINIMUM_TYPE_ID: u8 = 2;
const MAXIMUM_TYPE_ID: u8 = 3;
const LITERAL_TYPE_ID: u8 = 4;
const GREATER_THAN_TYPE_ID: u8 = 5;
const LESS_THAN_TYPE_ID: u8 = 6;
const EQUAL_TYPE_ID: u8 = 7;

impl Content {
    fn parse(type_id: u8, input: (&[u8], usize)) -> IResult<(&[u8], usize), Content> {
        if type_id == LITERAL_TYPE_ID {
            let mut value = 0;
            let mut bit_length = 0;
            let mut cur_input = input;
            loop {
                let (input, group): (_, u64) = nom::bits::complete::take(5usize)(cur_input)?;
                cur_input = input;
                bit_length += 5;
                value <<= 4;
                value |= group & 0b1111;
                if group & 0b1_0000 == 0 {
                    return Ok((cur_input, Content::LiteralPacket { value, bit_length }));
                }
            }
        } else {
            let (input, operator) = Operator::parse(input)?;
            return Ok((input, Content::Operator(operator)));
        }
    }

    fn bit_length(&self) -> usize {
        match &self {
            Self::LiteralPacket { bit_length, .. } => *bit_length,
            Self::Operator(op) => op.bit_length,
        }
    }
}

fn str_to_bit_vec(hex_str: &str) -> anyhow::Result<Vec<u8>> {
    hex::decode(hex_str).context("Bad input hex_string")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inputs() {
        let bits = str_to_bit_vec("D2FE28").unwrap();
        let (_, packet) = Packet::parse((&bits, 0)).unwrap();
        assert_eq!(packet.type_id, 4);

        let bits = str_to_bit_vec("8A004A801A8002F478").unwrap();
        let (_, packet) = Packet::parse((&bits, 0)).unwrap();
        assert_eq!(packet.version_sum(), 16);

        let bits = str_to_bit_vec("620080001611562C8802118E34").unwrap();
        let (_, packet) = Packet::parse((&bits, 0)).unwrap();
        assert_eq!(packet.version_sum(), 12);

        let bits = str_to_bit_vec("C0015000016115A2E0802F182340").unwrap();
        let (_, packet) = Packet::parse((&bits, 0)).unwrap();
        assert_eq!(packet.version_sum(), 23);

        let bits = str_to_bit_vec("A0016C880162017C3686B18A3D4780").unwrap();
        let (_, packet) = Packet::parse((&bits, 0)).unwrap();
        assert_eq!(packet.version_sum(), 31);
    }
}
