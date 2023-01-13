use crate::{Position, Sensor};
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn position_parser(input: &str) -> IResult<&str, (i64, i64)> {
    // parse a single i64 with:
    // let test = "-14";
    // let (input, res) = complete::i64(test)?;
    separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64),
    )(input)
}

pub fn line_parser(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor_pair) = position_parser(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, beacon_pair) = position_parser(input)?;

    let sensor = Sensor {
        position: Position {
            x: sensor_pair.0,
            y: sensor_pair.1,
        },
        beacon_position: Position {
            x: beacon_pair.0,
            y: beacon_pair.1,
        },
    };
    Ok((input, sensor))
}

pub fn parse_to_sensors(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(line_ending, line_parser)(input)
}
