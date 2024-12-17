use std::{fmt::Display, iter::once};

type Number = i64;

enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Adv => f.pad("adv"),
            Opcode::Bxl => f.pad("bxl"),
            Opcode::Bst => f.pad("bst"),
            Opcode::Jnz => f.pad("jnz"),
            Opcode::Bxc => f.pad("bxc"),
            Opcode::Out => f.pad("out"),
            Opcode::Bdv => f.pad("bdv"),
            Opcode::Cdv => f.pad("cdv"),
        }
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => panic!("Unknown opcode: {}", value),
        }
    }
}

struct Computer {
    registers: [Number; 3],
    pc: usize,
    program: Vec<u8>,

    output: Vec<Number>,
}

impl Computer {
    fn step(&mut self) -> bool {
        let Some(opcode) = self.program.get(self.pc).copied().map(Opcode::from) else {
            return false;
        };
        let operand = self.program[self.pc + 1];
        eprintln!("[{:04x} A={:8} B={:8} C={:8}] {} {:x}", self.pc, self.registers[0], self.registers[1], self.registers[2], opcode, operand);
        self.pc += 2;

        match opcode {
            Opcode::Adv => {
                self.registers[0] /= (2 as Number).pow(u32::try_from(self.parse_combo_operand(operand)).unwrap());
            }

            Opcode::Bxl => {
                self.registers[1] ^= Number::from(operand);
            }

            Opcode::Bst => {
                self.registers[1] = self.parse_combo_operand(operand) & 0b111;
            }

            Opcode::Jnz => {
                if self.registers[0] != 0 {
                    self.pc = usize::from(operand);
                }
            }

            Opcode::Bxc => {
                self.registers[1] ^= self.registers[2];
            }

            Opcode::Out => {
                self.output.push(self.parse_combo_operand(operand) & 0b111);
            }

            Opcode::Bdv => {
                self.registers[1] = self.registers[0] / (2 as Number).pow(u32::try_from(self.parse_combo_operand(operand)).unwrap());
            }

            Opcode::Cdv => {
                self.registers[2] = self.registers[0] / (2 as Number).pow(u32::try_from(self.parse_combo_operand(operand)).unwrap());
            }
        }

        true
    }
    
    fn compile(&self) -> String {
        use std::fmt::Write;

        let mut output = String::new();

        writeln!(output, "fn main() {{").unwrap();
        writeln!(output, "    let mut a = {};", self.registers[0]).unwrap();
        writeln!(output, "    let mut b = {};", self.registers[1]).unwrap();
        writeln!(output, "    let mut c = {};", self.registers[2]).unwrap();
        writeln!(output, "").unwrap();
        writeln!(output, "    loop {{").unwrap();

        let parse_combo_operand = |operand: u8| -> &str {
            match operand {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "a",
                5 => "b",
                6 => "c",
                _ => unreachable!(),
            }
        };


        for chunk in self.program.chunks(2) {
            let opcode = chunk[0];
            let operand = chunk[1];


        match Opcode::from(opcode) {
            Opcode::Adv => {
                writeln!(output, "        a /= 2u32.pow({});", parse_combo_operand(operand)).unwrap();
            }

            Opcode::Bxl => {
                writeln!(output, "        b ^= {};", operand).unwrap();
            }

            Opcode::Bst => {
                writeln!(output, "        b = {} & 0b111;", parse_combo_operand(operand)).unwrap();
            }

            Opcode::Jnz => {
                debug_assert_eq!(operand, 0);
                writeln!(output, "        if a != 0 {{").unwrap();
                writeln!(output, "            continue;").unwrap();
                writeln!(output, "        }}").unwrap();
            }

            Opcode::Bxc => {
                writeln!(output, "        b ^= c;").unwrap();
            }

            Opcode::Out => {
                writeln!(output, "        println!(\"{{}}\", {} & 0b111);", parse_combo_operand(operand)).unwrap();
            }

            Opcode::Bdv => {
                writeln!(output, "        b = a / 2u32.pow({});", parse_combo_operand(operand)).unwrap();
            }

            Opcode::Cdv => {
                writeln!(output, "        c = a / 2u32.pow({});", parse_combo_operand(operand)).unwrap();
            }
        }
        }

        writeln!(output, "        break;").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output, "}}").unwrap();
        output
    }

    fn parse_combo_operand(&self, operand: u8) -> Number {
        match operand {
            0..=3 => operand.into(),
            4..=6 => self.registers[operand as usize - 4],
            7 => panic!("got 7... reserved, but mysterious"),
            _ => unreachable!(),
        }
    }

}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let mut lines = input.lines();

    let a_value = lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap();
    let b_value = lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap();
    let c_value = lines.next().unwrap().split_once(": ").unwrap().1.parse().unwrap();
    lines.next(); // skip empty line
    let program = lines.next().unwrap().split_once(": ").unwrap().1.split(',').map(|s| s.parse().unwrap()).collect();

    let mut computer = Computer {
        registers: [a_value, b_value, c_value],
        pc: 0,
        program,
        output: Vec::new(),
    };
    // std::fs::write("output.rs", computer.compile()).unwrap();

    while computer.step() {}

    let p1 = computer.output.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
    
use z3::{ast::BV, ast::Ast, ast::Bool, Config, Context, Solver};

    // This corresponds to the Python array 'e'.
    let e = [2,4,1,7,7,5,0,3,1,7,4,1,5,5,3,0];

    // Each element of 'e' is constrained via 3 bits of 'a'.
    let bitlen = e.len() * 3;

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Create the bitvector 'a' with length bitlen.
    let a = BV::new_const(&ctx, "a", bitlen as u32);

    let mut offset = 0;
    for &k in &e {
        // Construct a bitvector for the offset.
        let offset_bv = BV::from_u64(&ctx, offset as u64, bitlen as u32);

        // a_ = a >> offset  (logical right shift)
        let a_shifted = a.bvashr(&offset_bv);

        // n = a_ & 7
        let n = a_shifted.bvand(&BV::from_u64(&ctx, 7, bitlen as u32));

        // n_xor_7 = n ^ 7
        let n_xor_7 = n.bvxor(&BV::from_u64(&ctx, 7, bitlen as u32));

        // p = (a_ >> (n ^ 7)) & 7
        // We must shift by n_xor_7, which is also a bitvector.
        let p_shifted = a_shifted.bvashr(&n_xor_7);
        let p = p_shifted.bvand(&BV::from_u64(&ctx, 7, bitlen as u32));

        // Constraint: (n ^ p) == k
        let n_xor_p = n.bvxor(&p);
        let k_bv = BV::from_u64(&ctx, k as u64, bitlen as u32);
        let constraint: Bool = n_xor_p._eq(&k_bv);

        solver.assert(&constraint);
        offset += 3;
    }

    let p2;
    match solver.check() {
        z3::SatResult::Sat => {
            println!("SAT");
            let model = solver.get_model().unwrap();
            p2 = model.eval(&a, true).unwrap().as_u64().unwrap();
        },
        z3::SatResult::Unsat => unreachable!(),
        z3::SatResult::Unknown => unreachable!(),
    }

    (p1, p2)
}
