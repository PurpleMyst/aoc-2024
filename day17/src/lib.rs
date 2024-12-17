use std::fmt::Display;

type Number = u64;

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

#[derive(Clone, Copy)]
struct Computer {
    registers: [Number; 3],
    pc: usize,
    output: Option<Number>,
}

impl Computer {
    fn step(&mut self, program: &[u8]) -> bool {
        let Some(opcode) = program.get(self.pc).copied().map(Opcode::from) else {
            return false;
        };
        let operand = program[self.pc + 1];
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
                self.output = Some(self.parse_combo_operand(operand) & 0b111);
            }

            Opcode::Bdv => {
                self.registers[1] =
                    self.registers[0] / (2 as Number).pow(u32::try_from(self.parse_combo_operand(operand)).unwrap());
            }

            Opcode::Cdv => {
                self.registers[2] =
                    self.registers[0] / (2 as Number).pow(u32::try_from(self.parse_combo_operand(operand)).unwrap());
            }
        }

        true
    }

    fn step_until_output(&mut self, program: &[u8]) -> Option<Number> {
        while self.output.is_none() && self.step(program) {}
        self.output.take()
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
    let program = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let computer = Computer {
        registers: [a_value, b_value, c_value],
        pc: 0,
        output: None,
    };

    let p1 = solve_part1(&program, computer);
    let p2 = solve_part2(&program, computer, program.len() - 1, 0).unwrap();

    (p1, p2)
}

fn solve_part1(program: &[u8], mut computer: Computer) -> String {
    std::iter::from_fn(|| computer.step_until_output(program))
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

// adapted from u/mental-chaos
fn solve_part2(program: &[u8], computer: Computer, cursor: usize, a: Number) -> Option<u64> {
    (0..8)
        .filter_map(|c| {
            let new_a = (a << 3) | c;
            let mut computer = computer;
            computer.registers[0] = new_a;
            if computer.step_until_output(program)? == program[cursor].into() {
                Some(if cursor == 0 {
                    new_a
                } else {
                    solve_part2(program, computer, cursor - 1, new_a)?
                })
            } else {
                None
            }
        })
        .min()
}
