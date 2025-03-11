/*
$ mount

cat /proc/mounts

parser file /proc/mounts, output like `mount` command.
*/

use super::Mount;

fn not_whitespace(i: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::is_not(" \t")(i)
}

fn escaped_space(i: &str) -> nom::IResult<&str, &str> {
    nom::combinator::value(" ", nom::bytes::complete::tag("040"))(i)
}

fn escaped_backslash(i: &str) -> nom::IResult<&str, &str> {
    nom::combinator::recognize(nom::character::complete::char('\\'))(i)
}

fn transform_escaped(i: &str) -> nom::IResult<&str, std::string::String> {
    nom::bytes::complete::escaped_transform(
        nom::bytes::complete::is_not("\\"),
        '\\',
        nom::branch::alt((escaped_backslash, escaped_space)),
    )(i)
}

fn mount_opts(i: &str) -> nom::IResult<&str, std::vec::Vec<std::string::String>> {
    nom::multi::separated_list0(
        nom::character::complete::char(','),
        nom::combinator::map_parser(nom::bytes::complete::is_not(", \t"), transform_escaped),
    )(i)
}

/*
nsfs /run/docker/netns/default nsfs rw 0 0
nsfs /run/docker/netns/ff67e81ebabf nsfs rw 0 0
*/
pub fn parse_line(i: &str) -> nom::IResult<&str, Mount> {
    match nom::combinator::all_consuming(nom::sequence::tuple((
        /* part 1 */
        nom::combinator::map_parser(not_whitespace, transform_escaped), // device
        nom::character::complete::space1,
        nom::combinator::map_parser(not_whitespace, transform_escaped), // mount_point
        nom::character::complete::space1,
        not_whitespace, // file_system_type
        nom::character::complete::space1,
        mount_opts, // options
        nom::character::complete::space1,
        nom::character::complete::char('0'),
        nom::character::complete::space1,
        nom::character::complete::char('0'),
        nom::character::complete::space0,
    )))(i)
    {
        /* part 2 */
        Ok((
            remaining_input,
            (
                device,
                _, // whitespace
                mount_point,
                _, // whitespace
                file_system_type,
                _, // whitespace
                options,
                _, // whitespace
                _, // 0
                _, // whitespace
                _, // 0
                _, // optional whitespace
            ),
        )) => {
            /* part 3 */
            Ok((
                remaining_input,
                Mount {
                    device: device,
                    mount_point: mount_point,
                    file_system_type: file_system_type.to_string(),
                    options: options,
                },
            ))
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_whitespace() {
        assert_eq!(not_whitespace("abcd efg"), Ok((" efg", "abcd")));
        assert_eq!(not_whitespace("abcd\tefg"), Ok(("\tefg", "abcd")));
        assert_eq!(
            not_whitespace(" abcdefg"),
            Err(nom::Err::Error(nom::error::make_error(
                " abcdefg",
                nom::error::ErrorKind::IsNot
            )))
        );
    }

    #[test]
    fn test_escaped_space() {
        assert_eq!(escaped_space("040"), Ok(("", " ")));
        assert_eq!(
            escaped_space(" "),
            Err(nom::Err::Error(nom::error::make_error(
                " ",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_escaped_backslash() {
        assert_eq!(escaped_backslash("\\"), Ok(("", "\\")));
        assert_eq!(
            escaped_backslash("not a backslash"),
            Err(nom::Err::Error(nom::error::make_error(
                "not a backslash",
                nom::error::ErrorKind::Char
            )))
        );
    }

    #[test]
    fn test_transform_escaped() {
        assert_eq!(
            transform_escaped("abc\\040def\\\\g\\040h"),
            Ok(("", std::string::String::from("abc def\\g h")))
        );
        assert_eq!(
            transform_escaped("\\bad"),
            Err(nom::Err::Error(nom::error::make_error(
                "bad",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_mount_opts() {
        assert_eq!(
            mount_opts("a,bc,d\\040e"),
            Ok((
                "",
                vec!["a".to_string(), "bc".to_string(), "d e".to_string()]
            ))
        );
    }

    #[test]
    fn test_parse_line() {
        let mount1 = Mount {
            device: "device".to_string(),
            mount_point: "mount_point".to_string(),
            file_system_type: "file_system_type".to_string(),
            options: vec![
                "options".to_string(),
                "a".to_string(),
                "b=c".to_string(),
                "d e".to_string(),
            ],
        };
        let (_, mount2) =
            parse_line("device mount_point file_system_type options,a,b=c,d\\040e 0 0").unwrap();
        assert_eq!(mount1.device, mount2.device);
        assert_eq!(mount1.mount_point, mount2.mount_point);
        assert_eq!(mount1.file_system_type, mount2.file_system_type);
        assert_eq!(mount1.options, mount2.options);
    }
}
