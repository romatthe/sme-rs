use std::ffi::CString;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::many_till;
use nom::number::complete::le_u32;
use nom::sequence::terminated;
use crate::vdf::{VdfNode, VdfNodeKind, VdfStringRef};

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

    Ok((input, VdfNode { key, value: VdfNodeKind::Nested { nodes }}))
}

/// Parse a VDF node with an encoded string value.
fn parse_vdf_node_string(input: &[u8]) -> IResult<&[u8], VdfNode> {
    let (input, _) = tag(b"\x01")(input)?;

    let (input, key) = parse_vdf_key(input)?;
    let (input, value) = parse_vdf_string(input)?;

    Ok((input, VdfNode { key, value: VdfNodeKind::String { value: String::from(value) }}))
}


/// Parse a VDF node with an encoded integer value.
fn parse_vdf_node_integer(input: &[u8]) -> IResult<&[u8], VdfNode> {
    let (input, _) = tag(b"\x02")(input)?;

    let (input, key) = parse_vdf_key(input)?;
    let (input, value) = le_u32(input)?;

    Ok((input, VdfNode { key, value: VdfNodeKind::Int { value }}))
}

/// Parse a VDF encoded string.
fn parse_vdf_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, null_str) = terminated(take_until("\0"), tag("\0"))(input)?;

    Ok((input, String::from_utf8(null_str.to_vec()).unwrap()))
}

fn parse_vdf_key(input: &[u8]) -> IResult<&[u8], VdfStringRef> {
    let (input, keyref) = le_u32(input)?;

    Ok((input, VdfStringRef(keyref)))
}