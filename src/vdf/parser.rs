use std::ffi::CString;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::IResult;
use nom::multi::many_till;
use nom::number::complete::le_u32;

#[derive(Debug)]
pub enum VdfNode<'a> {
    Nested {
        key: VdfString<'a>,
        nodes: Vec<VdfNode<'a>>,
    },
    String {
        key: VdfString<'a>,
        value: CString,
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

    Ok((input, VdfNode::String { key, value }))
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
fn parse_vdf_string(input: &[u8]) -> IResult<&[u8], CString> {
    // println!("Parsing vdf encoded string");
    let pos = input
        .iter().position(|b| *b == b'\0').unwrap();
    let (input, bytes) = take(pos + 1)(input)?;
    // let string = unsafe { CString::from_vec_unchecked(bytes.to_vec()) };
    let string = CString::new("".to_string().as_str()).unwrap();

    // println!("Bytes: {:?}", bytes);
    // println!("String: {:?}", string);

    Ok((input, string))
}

fn parse_vdf_key(input: &[u8]) -> IResult<&[u8], VdfString> {
    // println!("Key input: {:?}", input);

    let (input, keyref) = le_u32(input)?;

    // println!("Key parsed: {:?}", keyref);

    Ok((input, VdfString::StringRef(keyref)))
}