use std::ffi::CString;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::many_till;
use nom::number::complete::le_u32;
use nom::sequence::terminated;

#[derive(Clone, Debug)]
pub enum VdfNode {
    Nested {
        key: VdfString,
        nodes: Vec<VdfNode>,
    },
    String {
        key: VdfString,
        value: VdfString,
    },
    Int {
        key: VdfString,
        value: u32,
    },
}

#[derive(Clone, Debug)]
pub struct VdfStringRef(u32);

#[derive(Clone, Debug)]
pub enum VdfString {
    StringRef(u32),
    // String(&'a str),
    String(CString),
}

pub fn parse_vdf_nodes(input: &[u8]) -> IResult<&[u8], Vec<VdfNode>> {
    let mut parser = many_till(parse_vdf_node, tag(b"\x08"));
    let (input, (nodes, _)) = parser(input)?;

    Ok((input, nodes))
}

fn parse_vdf_node(input: &[u8]) -> IResult<&[u8], VdfNode> {
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

    let (input, key) = parse_vdf_key(input)?;
    let (input, nodes) = parse_vdf_nodes(input)?;

    Ok((input, VdfNode::Nested { key, nodes }))
}

/// Parse a VDF node with an encoded string value.
fn parse_vdf_node_string(input: &[u8]) -> IResult<&[u8], VdfNode> {
    let (input, _) = tag(b"\x01")(input)?;

    let (input, key) = parse_vdf_key(input)?;
    let (input, value) = parse_vdf_string(input)?;

    Ok((input, VdfNode::String { key, value: VdfString::String(CString::from(value)) }))
}


/// Parse a VDF node with an encoded integer value.
fn parse_vdf_node_integer(input: &[u8]) -> IResult<&[u8], VdfNode> {
    let (input, _) = tag(b"\x02")(input)?;

    let (input, key) = parse_vdf_key(input)?;
    let (input, value) = le_u32(input)?;

    Ok((input, VdfNode::Int { key, value }))
}

/// Parse a VDF encoded string.
fn parse_vdf_string(input: &[u8]) -> IResult<&[u8], CString> {
    let null_str = terminated(take_until("\0"), tag("\0"));
    map_res(null_str, CString::new)(input)
}

fn parse_vdf_key(input: &[u8]) -> IResult<&[u8], VdfString> {
    let (input, keyref) = le_u32(input)?;

    Ok((input, VdfString::StringRef(keyref)))
}