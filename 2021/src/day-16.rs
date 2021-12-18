use nom::{self, bits::complete::take};
use std::io::{self, BufRead};
use std::str;

type BitBuffer<'a> = (&'a [u8], usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum PacketType {
    Literal(u64),
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
struct Packet {
    version: u8,
    ptype: PacketType,
    subpackets: Vec<Packet>,
}

fn get_buffer_size(buffer: BitBuffer) -> usize {
    buffer.0.len() * 8 - buffer.1
}

fn take_bits(input: BitBuffer, count: usize) -> nom::IResult<BitBuffer, usize> {
    take::<_, usize, usize, _>(count)(input)
}

fn parse_value(mut input: BitBuffer) -> nom::IResult<BitBuffer, u64> {
    let mut value = 0;
    loop {
        let (input_, group) = take_bits(input, 5)?;
        input = input_;
        value = (value << 4) | (group & !(1 << 4)) as u64;
        if (group & (1 << 4)) == 0 {
            return Ok((input, value));
        }
    }
}

const PACKET_TYPES: &'static [PacketType] = &[
    PacketType::Sum,
    PacketType::Product,
    PacketType::Minimum,
    PacketType::Maximum,
    PacketType::Literal(0),
    PacketType::GreaterThan,
    PacketType::LessThan,
    PacketType::EqualTo,
];

impl Packet {
    fn parse_subpackets(input: BitBuffer) -> nom::IResult<BitBuffer, Vec<Packet>> {
        let (input, length_type) = take_bits(input, 1)?;
        Ok(if length_type == 0 {
            let (mut input, subpacket_bits) = take_bits(input, 15)?;

            let mut subpackets = vec![];
            let buffer_size_end = get_buffer_size(input) - subpacket_bits;
            while get_buffer_size(input) > buffer_size_end {
                Packet::parse(input).map(|(input_, packet)| {
                    input = input_;
                    subpackets.push(packet);
                })?;
            }
            (input, subpackets)
        } else {
            let (mut input, subpacket_count) = take_bits(input, 11)?;
            let subpackets = (0..subpacket_count)
                .flat_map(|_| {
                    Packet::parse(input).map(|(input_, packet)| {
                        input = input_;
                        packet
                    })
                })
                .collect();
            (input, subpackets)
        })
    }

    fn parse(input: BitBuffer) -> nom::IResult<BitBuffer, Packet> {
        let (input, version) = take_bits(input, 3)?;
        let (input, mut ptype) =
            take_bits(input, 3).map(|(input, ptype)| (input, PACKET_TYPES[ptype]))?;
        let (input, subpackets) = if let PacketType::Literal(_) = ptype {
            let (input, value) = parse_value(input)?;
            ptype = PacketType::Literal(value);
            (input, vec![])
        } else {
            Packet::parse_subpackets(input)?
        };
        Ok((
            input,
            Packet {
                version: version as u8,
                ptype,
                subpackets,
            },
        ))
    }

    fn sum_versions(&self) -> usize {
        (self.version as usize)
            + self
                .subpackets
                .iter()
                .map(|p| p.sum_versions())
                .sum::<usize>()
    }

    fn evaluate_expression(&self) -> u64 {
        match self.ptype {
            PacketType::Sum => self
                .subpackets
                .iter()
                .map(Packet::evaluate_expression)
                .sum(),
            PacketType::Product => self
                .subpackets
                .iter()
                .map(Packet::evaluate_expression)
                .product(),
            PacketType::Minimum => self
                .subpackets
                .iter()
                .map(Packet::evaluate_expression)
                .min()
                .unwrap(),
            PacketType::Maximum => self
                .subpackets
                .iter()
                .map(Packet::evaluate_expression)
                .max()
                .unwrap(),
            PacketType::Literal(value) => value,
            PacketType::GreaterThan => {
                (self.subpackets[0].evaluate_expression()
                    > self.subpackets[1].evaluate_expression()) as u64
            }
            PacketType::LessThan => {
                (self.subpackets[0].evaluate_expression()
                    < self.subpackets[1].evaluate_expression()) as u64
            }
            PacketType::EqualTo => {
                (self.subpackets[0].evaluate_expression()
                    == self.subpackets[1].evaluate_expression()) as u64
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut line: String = String::new();
    io::stdin().lock().read_line(&mut line)?;

    let input: Vec<u8> = line
        .as_bytes()
        .chunks(2)
        .map(|bytes| u8::from_str_radix(str::from_utf8(&bytes).unwrap(), 16).unwrap())
        .collect();
    if let Ok((_, packet)) = Packet::parse((&input[..], 0)) {
        println!(
            "(1) The sum of all version fields is {}",
            packet.sum_versions()
        );
        println!(
            "(2) Evaluating the expression gives {}",
            packet.evaluate_expression()
        );
    }

    Ok(())
}
