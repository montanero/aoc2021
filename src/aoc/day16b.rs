use std::cmp::{max, min};
use std::path::Path;
use std::str::Chars;

use crate::aoc::file;

pub(crate) fn solve() -> i64 {
    solve_file(&file::input("input16.txt"))
}

fn solve_file(f: &Path) -> i64 {
    let hex = read_file(f);
    let p = Packet::parse_string(&hex);
    p.evaluate()
}

struct Packet {
    version: u32,
    type_id: u32,
    content: PacketContent,
}

enum PacketContent {
    LiteralContent(u64),
    OperatorContent(Vec<Packet>),
}

impl Packet {
    fn parse(r: &mut BitReader) -> Self {
        let ver = r.read_bits(3);
        let ptype = r.read_bits(3);
        let content = if ptype == 4 {
            Packet::parse_literal(r)
        } else {
            Packet::parse_operator(r)
        };
        Packet {
            version: ver,
            type_id: ptype,
            content: content,
        }
    }

    fn parse_string(hex: &str) -> Packet {
        let mut r = BitReader::from(&hex);
        Packet::parse(&mut r)
    }

    fn parse_literal(b: &mut BitReader) -> PacketContent {
        let mut val = 0u64;
        loop {
            let word = b.read_bits(5);
            if (word & 0x10) != 0 {
                val = (val << 4) + (word & 0xf) as u64;
            } else {
                val = (val << 4) + word as u64;
                break;
            }
        }
        PacketContent::LiteralContent(val)
    }
    fn parse_operator(b: &mut BitReader) -> PacketContent {
        let length_type_id = b.read_bit().unwrap();
        let packets: Vec<Packet> = if length_type_id == 0 {
            let bit_len = b.read_bits(15) as usize;
            Packet::read_packets_by_len(b, bit_len)
        } else {
            let packet_no = b.read_bits(11) as usize;
            Packet::read_packets_by_number(b, packet_no)
        };
        PacketContent::OperatorContent(packets)
    }

    fn read_packets_by_len(b: &mut BitReader, bit_len: usize) -> Vec<Packet> {
        let mut packets = Vec::new();
        let start = b.bits_read;
        while b.bits_read - start < bit_len {
            let packet = Packet::parse(b);
            packets.push(packet);
        }
        packets
    }

    fn read_packets_by_number(b: &mut BitReader, packet_no: usize) -> Vec<Packet> {
        let mut packets = Vec::new();
        for _ in 0..packet_no {
            let packet = Packet::parse(b);
            packets.push(packet);
        }
        packets
    }

    fn evaluate(&self) -> i64 {
        match &self.content {
            PacketContent::LiteralContent(x) => *x as i64,
            PacketContent::OperatorContent(vec) => match self.type_id {
                0 => Packet::calc(&vec, |a, b| a + b),
                1 => Packet::calc(&vec, |a, b| a * b),
                2 => Packet::calc(&vec, |a, b| min(a, b)),
                3 => Packet::calc(&vec, |a, b| max(a, b)),
                5 => Packet::calc(&vec, |a, b| if a > b { 1 } else { 0 }),
                6 => Packet::calc(&vec, |a, b| if a < b { 1 } else { 0 }),
                7 => Packet::calc(&vec, |a, b| if a == b { 1 } else { 0 }),
                _ => panic!("??"),
            },
        }
    }

    fn calc<T>(vec: &Vec<Packet>, f: T) -> i64
    where
        T: Fn(i64, i64) -> i64,
    {
        vec.iter().map(|p| p.evaluate()).reduce(f).unwrap()
    }
}

fn read_file(p0: &Path) -> String {
    let input = file::read_lines(p0).unwrap();
    let l: Vec<String> = input.map(|x| x.unwrap()).collect();
    l.get(0).unwrap().to_string()
}

struct BitReader<'a> {
    chars: Chars<'a>,
    current: Option<u8>,
    bitno: u8,
    bits_read: usize,
}

impl<'a> BitReader<'a> {
    fn from(line: &'a str) -> Self {
        //let l: &'a str = &line;
        let x: Chars<'a> = line.chars();
        BitReader {
            chars: x,
            current: None,
            bitno: 0,
            bits_read: 0,
        }
    }

    fn read_bit(&mut self) -> Option<u32> {
        match self.current {
            None => {
                let c_o = self.chars.next();
                match c_o {
                    None => {}
                    Some(c) => {
                        self.current = Some(c.to_digit(16).unwrap() as u8);
                        self.bitno = 8;
                    }
                }
            }
            Some(_) => {}
        }
        match self.current {
            None => None,
            Some(bits) => {
                let val = bits & self.bitno;
                self.bitno = self.bitno >> 1;
                if self.bitno == 0 {
                    self.current = None;
                }
                self.bits_read += 1;
                if val == 0 {
                    Some(0)
                } else {
                    Some(1)
                }
            }
        }
    }

    fn read_bits(&mut self, n: usize) -> u32 {
        let mut result = 0;
        for _ in 0..n {
            result = (result << 1) | self.read_bit().unwrap();
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 6802496672062);
    }

    #[test]
    fn result_sample2() {
        let p = Packet::parse_string("04005AC33890");
        assert_eq!(p.evaluate(), 54)
    }

    #[test]
    fn result_sample3() {
        let p = Packet::parse_string("880086C3E88112");
        assert_eq!(p.evaluate(), 7)
    }
    #[test]
    fn result_sample4() {
        let p = Packet::parse_string("CE00C43D881120");
        assert_eq!(p.evaluate(), 9)
    }

    #[test]
    fn result_sample5() {
        let p = Packet::parse_string("D8005AC2A8F0");
        assert_eq!(p.evaluate(), 1)
    }

    #[test]
    fn result_sample6() {
        let p = Packet::parse_string("F600BC2D8F");
        assert_eq!(p.evaluate(), 0)
    }

    #[test]
    fn result_sample7() {
        let p = Packet::parse_string("9C005AC2F8F0");
        assert_eq!(p.evaluate(), 0)
    }

    #[test]
    fn result_sample8() {
        let p = Packet::parse_string("9C0141080250320F1802104A08");
        assert_eq!(p.evaluate(), 1)
    }
}
