use nom::IResult;

struct AppInfo {
    // universe: u32,
    // apps: Vec<AppSection>,
}

fn parse_app_info(input: &[u8]) -> IResult<(), AppInfo> {
    Ok(((), AppInfo {}))
}

