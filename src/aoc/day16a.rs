use std::path::Path;
use std::str::Chars;

use crate::aoc::file;

pub(crate) fn solve() -> u32 {
    solve_file(&file::input("input16.txt"))
}

fn solve_file(f: &Path) -> u32 {
    let hex = read_file(f);
    let p = Packet::parse_string(&hex);
    sum_packet_versions(&p)
}

fn sum_packet_versions(p: &Packet) -> u32 {
    p.version
        + match &p.content {
            PacketContent::LiteralContent(_) => 0,
            PacketContent::OperatorContent(v) => v.iter().map(|p0| sum_packet_versions(p0)).sum(),
        }
}

struct Packet {
    version: u32,
    type_id: u32,
    content: PacketContent,
}

enum PacketContent {
    LiteralContent(u32),
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
        let mut val = 0u32;
        loop {
            let word = b.read_bits(5);
            if (word & 0x10) != 0 {
                val = (val << 4) | (word & 0xf);
            } else {
                val = (val << 4) | word;
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
        assert_eq!(result, 936);
    }

    #[test]
    fn result_sample() {
        let p = Packet::parse_string("38006F45291200");
        assert_eq!(p.version, 1);
        assert_eq!(p.type_id, 6);
        match p.content {
            PacketContent::LiteralContent(_) => {
                assert!(false);
            }
            PacketContent::OperatorContent(p) => {
                assert_eq!(p.len(), 2);
            }
        };
    }

    #[test]
    fn result_sample1() {
        let p = Packet::parse_string("D2FE28");
        assert_eq!(p.version, 6);
        assert_eq!(p.type_id, 4);
        match p.content {
            PacketContent::LiteralContent(x) => {
                assert_eq!(x, 2021)
            }
            _ => assert!(false),
        };
    }

    #[test]
    fn result_sample3() {
        let p = Packet::parse_string("EE00D40C823060");
        assert_eq!(p.version, 7);
        assert_eq!(p.type_id, 3);
        match p.content {
            PacketContent::LiteralContent(_) => {
                assert!(false);
            }
            PacketContent::OperatorContent(vec) => {
                assert_eq!(vec.len(), 3);
                match vec[0].content {
                    PacketContent::LiteralContent(x) => assert_eq!(1, x),
                    _ => assert!(false),
                };
                match vec[1].content {
                    PacketContent::LiteralContent(x) => assert_eq!(2, x),
                    _ => assert!(false),
                };
                match vec[2].content {
                    PacketContent::LiteralContent(x) => assert_eq!(3, x),
                    _ => assert!(false),
                };
            }
        }
    }
    #[test]
    fn result_sample4() {
        let p = Packet::parse_string("8A004A801A8002F478");
        assert_eq!(sum_packet_versions(&p), 16);
    }
    #[test]
    fn result_sample5() {
        let p = Packet::parse_string("620080001611562C8802118E34");
        assert_eq!(sum_packet_versions(&p), 12);
    }
    #[test]
    fn result_sample6() {
        let p = Packet::parse_string("C0015000016115A2E0802F182340");
        assert_eq!(sum_packet_versions(&p), 23);
    }

    #[test]
    fn result_sample7() {
        let p = Packet::parse_string("A0016C880162017C3686B18A3D4780");
        assert_eq!(sum_packet_versions(&p), 31);
    }
}
