#[derive(Clone, Copy)]
struct Position(isize, isize);

struct Direction {
    x: isize,
    y: isize,
}

const WIDTH: isize = 17;
const HEIGHT: isize = 9;
const SYMBOLS: &str = " .o+=*BOX@%&#/^SE";

const DIRECTIONS: [Direction; 4] = [
    Direction { x: -1, y: -1 }, // ↖
    Direction { x: 1, y: -1 }, // ↗
    Direction { x: -1, y: 1 }, // ↙
    Direction { x: 1, y: 1 }, // ↘
];

fn clamp(min: isize, max: isize, x: isize) -> isize {
    std::cmp::max(min, std::cmp::min(max, x))
}

fn next_position(position: &Position, move_id: u8) -> Position {
    let delta = &DIRECTIONS[move_id as usize];
    Position(
        clamp(0, WIDTH - 1, position.0 + delta.x),
        clamp(0, HEIGHT - 1, position.1 + delta.y),
    )
}

fn split_byte_into_command(value: u8) -> [u8; 4] {
    [(value & 3), (value >> 2 & 3), (value >> 4 & 3), (value >> 6 & 3)]
}

fn parse_commands(s: &str) -> Vec<u8> {
    let mut commands = Vec::new();
    for i in (0..s.len()).step_by(2) {
        let value = u8::from_str_radix(&s[i..=i + 1], 16).unwrap();
        let command = split_byte_into_command(value);
        commands.extend(command);
    }
    commands
}

fn step(world: &mut Vec<u8>, position: &Position, command: u8) -> Position {
    world[(position.1 * WIDTH + position.0) as usize] += 1;
    next_position(&position, command)
}

fn simulate(commands: &[u8]) -> Vec<u8> {
    let start = Position(8, 4);
    let mut position = start.clone();
    let mut world = vec![0u8; WIDTH as usize * HEIGHT as usize];

    for &command in commands {
        position = step(&mut world, &position, command);
    }

    let y = start.1.clone();
    let x = start.0.clone();

    let end = position;
    world[(y * WIDTH + x) as usize] = 15;  // Set S
    world[(end.1 * WIDTH + end.0) as usize] = 16;      // Set E

    world
}

fn draw(world: &[u8]) -> String {
    let result: Vec<String> = world
        .iter()
        .map(|&cell| SYMBOLS.chars().nth((cell % SYMBOLS.len() as u8) as usize).unwrap().to_string())
        .collect();

    let mut drawing = String::new();
    drawing.push_str("+");
    drawing.push_str(&"-".repeat(WIDTH as usize));
    drawing.push_str("+");
    drawing.push('\n');

    for i in 0..HEIGHT {
        let row: String = result[(i * WIDTH) as usize..((i + 1) * WIDTH) as usize].concat();
        drawing.push_str(&format!("|{}|\n", row));
    }

    drawing.push_str("+");
    drawing.push_str(&"-".repeat(WIDTH as usize));
    drawing.push_str("+");
    drawing
}


pub fn main() {
    let mut steps = 0;
    let commands = parse_commands("fc94b0c1e5b0987c5843997697ee9fb7");
    let mut world = Vec::new();

    while steps < commands.len() + 1 {
        world = simulate(&commands[0..steps]);
        steps += 1;
    }
    println!("{}", draw(&world));
}

#[cfg(test)]
mod test {
    use crate::{draw, parse_commands, simulate};

    #[test]
    fn test1() {
        let mut steps = 0;
        let commands = parse_commands("fc94b0c1e5b0987c5843997697ee9fb7");
        let mut world = Vec::new();

        while steps < commands.len() + 1 {
            world = simulate(&commands[0..steps]);
            steps += 1;
        }
        assert_eq!(draw(&world), r#"+-----------------+
|       .=o.  .   |
|     . *+*. o    |
|      =.*..o     |
|       o + ..    |
|        S o.     |
|         o  .    |
|          .  . . |
|              o .|
|               E.|
+-----------------+"#);
    }
    #[test]
    fn test2() {
        let mut steps = 0;
        let commands = parse_commands("37e46a2d48381a0af3726dd9176bbd5e");
        let mut world = Vec::new();

        while steps < commands.len() + 1 {
            world = simulate(&commands[0..steps]);
            steps += 1;
        }
        assert_eq!(draw(&world), r#"+-----------------+
|                 |
|                 |
|          .      |
|     .   o       |
|o . o . S +      |
|.+ + = . B .     |
|o + + o B o E    |
| o .   + . o     |
|         .o      |
+-----------------+"#);
    }
}