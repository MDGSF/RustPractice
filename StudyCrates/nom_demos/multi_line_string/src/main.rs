use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::none_of;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::IResult;

pub fn is_nonescaped_string_char(c: char) -> bool {
    let cv = c as u32;
    (cv >= 0x20) && (cv != 0x22) && (cv != 0x5C)
}

// One or more unescaped text characters
pub fn nonescaped_string(input: &str) -> IResult<&str, &str> {
    take_while1(is_nonescaped_string_char)(input)
}

pub fn escape_code(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        tag("\\"),
        alt((
            tag("\""),
            tag("\\"),
            tag("/"),
            tag("b"),
            tag("f"),
            tag("n"),
            tag("r"),
            tag("t"),
            tag("u"),
        )),
    ))(input)
}

pub fn string_body(input: &str) -> IResult<&str, &str> {
    recognize(many0(alt((nonescaped_string, escape_code))))(input)
}

pub fn string_literal(input: &str) -> IResult<&str, String> {
    let res = delimited(tag("\""), string_body, tag("\""))(input);

    match res {
        Ok((input, res)) => Ok((input, res.to_string())),
        Err(e) => Err(e),
    }
}

////////////////////////////////////////////////////////////////////////

pub fn multi_line_escape_code(input: &str) -> IResult<&str, String> {
    log::info!("multi_line_escape_code input: {:?}", input);

    let res = recognize(pair(
        tag("\\"),
        alt((
            tag("\""),
            tag("\\"),
            tag("/"),
            tag("b"),
            tag("f"),
            tag("n"),
            tag("r"),
            tag("t"),
            tag("u"),
        )),
    ))(input);

    match res {
        Ok((input, res)) => Ok((input, res.to_string())),
        Err(e) => {
            log::info!("multi_line_escape_code err: {:?}", e);
            Err(e)
        }
    }
}

pub fn multi_line_nonescaped_string(input: &str) -> IResult<&str, String> {
    log::info!("multi_line_nonescaped_string input: {:?}", input);

    let res = none_of("\"\\")(input);

    match res {
        Ok((remain, res)) => {
            log::info!(
                "multi_line_nonescaped_string remain: {:?}, res: {:?}",
                remain,
                res
            );
            Ok((remain, res.to_string()))
        }
        Err(e) => {
            log::info!("multi_line_nonescaped_string err: {:?}", e);
            Err(e)
        }
    }
}

pub fn multi_line_char_string_body(input: &str) -> IResult<&str, String> {
    log::info!("multi_line_char_string_body input: {:?}", input);

    let res = many0(alt((multi_line_nonescaped_string, multi_line_escape_code)))(input);

    match res {
        Ok((input, res)) => Ok((input, res.join(""))),
        Err(e) => {
            log::info!("multi_line_char_string_body err: {:?}", e);
            Err(e)
        }
    }
}

pub fn multi_line_char_string(input: &str) -> IResult<&str, String> {
    log::info!("multi_line_char_string input: {:?}", input);

    let res = delimited(tag("\""), multi_line_char_string_body, tag("\""))(input);

    match res {
        Ok((remain, res)) => {
            log::info!(
                "multi_line_char_string remain: {:?}, res: {:?}",
                remain,
                res,
            );
            Ok((remain, res.to_string()))
        }
        Err(e) => {
            log::info!("multi_line_char_string err: {:?}", e);
            Err(e)
        }
    }
}

fn main() {
    env_logger::init();

    let input = r#""hello  world \\t bbbb""#;
    log::info!("{:?}", multi_line_char_string(input));
    // log::info!("{:?}", string_literal(input));
}
