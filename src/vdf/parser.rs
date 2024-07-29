use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::many_till;
use nom::number::complete::le_u32;
use nom::sequence::terminated;

#[derive(Debug)]
pub enum VdfNode<'a> {
    Nested {
        key: VdfString<'a>,
        nodes: Vec<VdfNode<'a>>,
    },
    String {
        key: VdfString<'a>,
        value: VdfString<'a>,
    },
    Int {
        key: VdfString<'a>,
        value: u32,
    },
}

#[derive(Debug)]
pub struct VdfStringRef(u32);

#[derive(Debug)]
pub enum VdfString<'a>{
    StringRef(u32),
    String(&'a str),
}

pub fn parse_vdf_nodes(input: &[u8]) -> IResult<&[u8], Vec<VdfNode>> {
    // println!("Upcoming byte: {}", input[0]);

    let mut parser = many_till(parse_vdf_node, tag(b"\x08"));
    let (input, (nodes, _)) = parser(input)?;

    Ok((input, nodes))
}

fn parse_vdf_node(input: &[u8]) -> IResult<&[u8], VdfNode> {
    // println!("Choosing between parsers");

    let (input, node) = alt((
        parse_vdf_node_nested,
        parse_vdf_node_string,
        parse_vdf_node_integer,
    ))(input)?;

    Ok((input, node))
}

/// Parse a VDF node with nested child nodes.
fn parse_vdf_node_nested(input: &[u8]) -> IResult<&[u8], VdfNode> {
    let (input, _) = tag(b"\x00")(input)?;
    // println!("Parsing nested node");

    let (input, key) = parse_vdf_key(input)?;
    let (input, nodes) = parse_vdf_nodes(input)?;

    // println!("Nested node: Key {:?}, Value: {:?}", key, nodes);

    Ok((input, VdfNode::Nested { key, nodes }))
}

/// Parse a VDF node with an encoded string value.
fn parse_vdf_node_string(input: &[u8]) -> IResult<&[u8], VdfNode> {
    let (input, _) = tag(b"\x01")(input)?;
    // println!("Parsing string node");

    let (input, key) = parse_vdf_key(input)?;
    let (input, value) = parse_vdf_string(input)?;

    // println!("String node: Key {:?}, Value: {:?}", key, value);

    Ok((input, VdfNode::String { key, value: VdfString::String(value) }))
}


/// Parse a VDF node with an encoded integer value.
fn parse_vdf_node_integer(input: &[u8]) -> IResult<&[u8], VdfNode> {
    let (input, _) = tag(b"\x02")(input)?;
    // println!("Parsing integer node");

    let (input, key) = parse_vdf_key(input)?;
    let (input, value) = le_u32(input)?;

    // println!("Integer node: Key {:?}, Value {}", key, value);

    Ok((input, VdfNode::Int { key, value }))
}

/// Parse a VDF encoded string.
fn parse_vdf_string(input: &[u8]) -> IResult<&[u8], &str> {
    let null_str = terminated(take_until("\0"), tag("\0"));
    map_res(null_str,  |s|std::str::from_utf8(s))(input)
}

fn parse_vdf_key(input: &[u8]) -> IResult<&[u8], VdfString> {
    // println!("Key input: {:?}", input);

    let (input, keyref) = le_u32(input)?;

    // println!("Key parsed: {:?}", keyref);

    Ok((input, VdfString::StringRef(keyref)))
}