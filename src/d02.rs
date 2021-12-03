use anyhow::{anyhow, bail, Context};

enum Command {
    Forward(u64),
    Up(u64),
    Down(u64),
}

fn inputs() -> anyhow::Result<Vec<Command>> {
    let input_string =
        std::fs::read_to_string("inputs/02_input.txt").context("Error while reading input")?;

    input_string
        .split('\n')
        .map(|s| parse_line(s))
        .collect::<anyhow::Result<Vec<_>>>()
}

fn parse_line(line: &str) -> anyhow::Result<Command> {
    let mut split = line.split_ascii_whitespace();

    let cmd_str = split
        .next()
        .ok_or(anyhow!("Missing command in line: `{}`", line))?;
    let val = split
        .next()
        .ok_or(anyhow!("Missing value in line: `{}`", line))?
        .parse::<u64>()
        .context("Parsed value is not a number")?;

    Ok(match cmd_str {
        "forward" => Command::Forward(val),
        "up" => Command::Up(val),
        "down" => Command::Down(val),
        _ => bail!("Unknown string for command `{}`", cmd_str),
    })
}

pub fn part_01() -> anyhow::Result<u64> {
    let cmds = inputs()?;

    let mut x = 0;
    let mut y = 0;

    for cmd in cmds {
        match cmd {
            Command::Forward(val) => x += val,
            Command::Up(val) => y -= val,
            Command::Down(val) => y += val,
        }
    }

    Ok(x * y)
}

pub fn part_02() -> anyhow::Result<u64> {
    let cmds = inputs()?;

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for cmd in cmds {
        match cmd {
            Command::Forward(val) => {
                x += val;
                y += aim * val;
            }
            Command::Up(val) => aim -= val,
            Command::Down(val) => aim += val,
        }
    }

    Ok(x * y)
}
