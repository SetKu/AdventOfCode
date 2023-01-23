#![allow(non_snake_case)]

use nom::{
    bytes::complete::tag,
    bytes::complete::take,
    character::complete::{newline, u32 as u32_parser},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
    Parser,
};

#[derive(PartialEq, Debug)]
struct Valve<'a> {
    id: &'a str,
    flow_rate: u32,
    connection_ids: Vec<&'a str>,
}

fn parse_line(input: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    tuple((
        preceded(tag("Valve "), take(2u8)),
        preceded(tag(" has flow rate="), u32_parser),
        preceded(
            tag("; tunnels lead to valves ").or(tag("; tunnel leads to valve ")),
            separated_list0(tag(", "), take(2u8)),
        ),
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list0(
        newline,
        parse_line,
    )(input)
    .map(|(left, list)| {
        (
            left,
            list.into_iter()
                .map(|info| Valve {
                    id: info.0,
                    flow_rate: info.1,
                    connection_ids: info.2,
                })
                .collect(),
        )
    })
}

fn main() {
    let input = include_str!("input.txt");
    let data = parse(input).expect("Failed to parse input.");
}

#[cfg(test)]
mod tests {
    use crate::{parse, Valve};

    #[test]
    fn parsing_works() {
        let input = "Valve DS has flow rate=21; tunnel leads to valve PB\nValve QQ has flow rate=0; tunnels lead to valves FS, ID";
        let parsed = parse(input);
        
        assert_eq!(parsed, Ok(
            (
                "",
                vec![
                    Valve { 
                        id: "DS", 
                        flow_rate: 21, 
                        connection_ids: vec!["PB"], 
                    },
                    Valve { 
                        id: "QQ", 
                        flow_rate: 0, 
                        connection_ids: vec!["FS", "ID"], 
                    },
                ],
            )
        ));
    }
}