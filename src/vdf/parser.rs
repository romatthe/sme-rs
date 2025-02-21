use std::ffi::CString;
use indexmap::IndexMap;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::none_of;
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::many_till;
use nom::number::complete::le_u32;
use nom::sequence::terminated;

#[derive(Clone, Debug)]
pub enum VdfNode {
    Nested { nodes: IndexMap<VdfStringRef, VdfNode> },
    String { value: String, },
    Int { value: u32, },
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct VdfStringRef(pub u32);

pub fn parse_vdf_nodes(input: &[u8]) -> IResult<&[u8], IndexMap<VdfStringRef, VdfNode>> {
    let mut parser = many_till(parse_vdf_node, tag(b"\x08"));
    let (input, (nodes, _)) = parser(input)?;

    // Turn the key/value pairs into a IndexMap (a hashmap retaining ordering)
    let nodes = nodes.into_iter().collect::<IndexMap<VdfStringRef,VdfNode>>();

    Ok((input, nodes))
}

fn parse_vdf_node(input: &[u8]) -> IResult<&[u8], (VdfStringRef, VdfNode)> {
    let (input, node) = alt((
        parse_vdf_node_nested,
        parse_vdf_node_string,
        parse_vdf_node_integer,
    ))(input)?;

    Ok((input, node))
}

/// Parse a VDF node with nested child nodes.
fn parse_vdf_node_nested(input: &[u8]) -> IResult<&[u8], (VdfStringRef, VdfNode)> {
    let (input, _) = tag(b"\x00")(input)?;

    let (input, key) = parse_vdf_key(input)?;
    let (input, nodes) = parse_vdf_nodes(input)?;

    Ok((input, (key, VdfNode::Nested { nodes })))
}

/// Parse a VDF node with an encoded string value.
fn parse_vdf_node_string(input: &[u8]) -> IResult<&[u8], (VdfStringRef, VdfNode)> {
    let (input, _) = tag(b"\x01")(input)?;

    let (input, key) = parse_vdf_key(input)?;
    let (input, value) = parse_vdf_string(input)?;

    Ok((input, (key, VdfNode::String {  value })))
}


/// Parse a VDF node with an encoded integer value.
fn parse_vdf_node_integer(input: &[u8]) -> IResult<&[u8], (VdfStringRef, VdfNode)> {
    let (input, _) = tag(b"\x02")(input)?;

    let (input, key) = parse_vdf_key(input)?;
    let (input, value) = le_u32(input)?;

    Ok((input, (key, VdfNode::Int { value })))
}

/// Parse a VDF encoded string.
fn parse_vdf_string(input: &[u8]) -> IResult<&[u8], String> {
    let null_str = terminated(take_until("\0"), tag("\0"));
    map_res(null_str, |s: &[u8]| String::from_utf8(s.to_vec()))(input)
}

fn parse_vdf_key(input: &[u8]) -> IResult<&[u8], VdfStringRef> {
    let (input, keyref) = le_u32(input)?;

    Ok((input, VdfStringRef(keyref)))
}