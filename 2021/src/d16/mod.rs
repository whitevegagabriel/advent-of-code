use crate::d16::Operation::{EqualTo, GreaterThan, LessThan, Maximum, Minimum, Product, Sum};
use bitvec::{
    bits,
    field::BitField,
    order::{Lsb0, Msb0},
    view::BitView,
};
use bitvec_nom2::BSlice;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    combinator::{map, map_parser},
    multi::{count, many0, many_till},
    sequence::tuple,
    IResult,
};

pub fn solve(problem: &[&str]) -> (u64, u64) {
    let bytes = hex::decode(problem[0]).unwrap();
    let bits = bytes.view_bits::<Msb0>();
    let top_level_packet = Packet::parse(BSlice(bits)).unwrap();
    (solve1(&top_level_packet.1), solve2(&top_level_packet.1))
}

fn solve1(packet: &Packet) -> u64 {
    sum_version_numbers_recursively(packet)
}

fn solve2(packet: &Packet) -> u64 {
    perform_operation(packet)
}

fn sum_version_numbers_recursively(packet: &Packet) -> u64 {
    let s = packet.version as u64;
    s + match &packet.contents {
        PacketContents::Literal(_) => 0,
        PacketContents::Operator { packets, .. } => packets
            .iter()
            .map(sum_version_numbers_recursively)
            .sum::<u64>(),
    }
}

fn perform_operation(packet: &Packet) -> u64 {
    match &packet.contents {
        PacketContents::Literal(num) => *num,
        PacketContents::Operator { operation, packets } => match operation {
            Sum => packets.iter().map(perform_operation).sum(),
            Product => packets.iter().map(perform_operation).product(),
            Minimum => packets.iter().map(perform_operation).min().unwrap(),
            Maximum => packets.iter().map(perform_operation).max().unwrap(),
            GreaterThan => {
                let (first, second) = packets
                    .iter()
                    .map(perform_operation)
                    .collect_tuple()
                    .unwrap();
                if first > second {
                    1
                } else {
                    0
                }
            }
            LessThan => {
                let (first, second) = packets
                    .iter()
                    .map(perform_operation)
                    .collect_tuple()
                    .unwrap();
                if first < second {
                    1
                } else {
                    0
                }
            }
            EqualTo => {
                let (first, second) = packets
                    .iter()
                    .map(perform_operation)
                    .collect_tuple()
                    .unwrap();
                if first == second {
                    1
                } else {
                    0
                }
            }
        },
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Packet {
    version: u8,
    type_id: u8,
    contents: PacketContents,
}

impl Packet {
    fn parse(input: BSlice<u8, Msb0>) -> IResult<BSlice<u8, Msb0>, Self> {
        let (input, (version, type_id)) = map(
            tuple((take::<_, BSlice<u8, Msb0>, _>(3_usize), take(3_usize))),
            |(version, type_id)| (version.load_be::<u8>(), type_id.load_be::<u8>()),
        )(input)?;

        let (input, contents) = if type_id == 4 {
            map(
                many_till(
                    tuple((tag(BSlice(bits![1])), take(4_usize))),
                    tuple((tag(BSlice(bits![0])), take(4_usize))),
                ),
                |(mut accum_nums, last_accum_num): (Vec<(_, BSlice<u8, Msb0>)>, _)| {
                    accum_nums.push(last_accum_num);
                    let literal_value = accum_nums
                        .iter()
                        .fold(0, |acc, (_, num)| (acc << 4) + num.load_be::<u64>());
                    PacketContents::Literal(literal_value)
                },
            )(input)?
        } else {
            let (input, length_type_id) = map(take(1_usize), |length_type_id: BSlice<_, _>| {
                length_type_id.load_be::<u8>()
            })(input)?;
            let (input, packets) = if length_type_id == 1 {
                // 11 bits representing quantity of sub-packets
                let (input, qty) =
                    map(take(11_usize), |qty: BSlice<_, _>| qty.load_be::<usize>())(input)?;
                count(Packet::parse, qty)(input)?
            } else {
                // 15 bits representing length of sub-packets
                let (input, num_bits) = map(take(15_usize), |num_bits: BSlice<_, _>| {
                    num_bits.load_be::<usize>()
                })(input)?;
                map_parser(take(num_bits), many0(Packet::parse))(input)?
            };

            let operation = match type_id {
                0 => Sum,
                1 => Product,
                2 => Minimum,
                3 => Maximum,
                5 => GreaterThan,
                6 => LessThan,
                7 => EqualTo,
                _ => panic!("how?"),
            };
            (input, PacketContents::Operator { operation, packets })
        };

        Ok((
            input,
            Packet {
                version,
                type_id,
                contents,
            },
        ))
    }
}

#[derive(Debug)]
enum PacketContents {
    Literal(u64),
    Operator {
        operation: Operation,
        packets: Vec<Packet>,
    },
}

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[test]
fn test() {
    use crate::utils::basic_test;
    let input = include_str!("example.txt");
    basic_test(input, solve);
}
