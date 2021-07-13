use std::io::BufRead;
use std::string::ParseError;
use nom::IResult;

#[derive(Clone, Default, Debug)]
pub struct Mount {
    pub device: String,
    pub mount_point: String,
    pub file_system_type: String,
    pub options: Vec<String>,
}

pub fn p(i: &str) -> IResult<&str, Mount> {
    parsers::parse_line(i)
}

pub(self) mod parsers {
    use super::Mount;
    use nom::IResult;
    use nom::bytes::complete::{escaped_transform, is_not, tag};
    use nom::branch::alt;
    use nom::character::complete::{char, space1, space0};
    use nom::combinator::{value, recognize, map_parser, all_consuming};
    use nom::multi::separated_list1;
    use nom::sequence::tuple;

    fn not_whitespace(i: &str) -> IResult<&str, &str> {
        is_not(" \t")(i)
    }

    fn escaped_space(i: &str) -> IResult<&str, &str> {
        value(" ", tag("040"))(i)
    }

    fn escaped_backslash(i : &str) -> IResult<&str, &str> {
        recognize(char('\\'))(i)
    }

    fn transform_escaped(i: &str) -> IResult<&str, String> {
        escaped_transform(
            is_not("\\"),
            '\\',
            alt((escaped_backslash, escaped_space))
        )(i)
    }

    fn mount_opts(i: &str) -> IResult<&str, Vec<String>> {
        separated_list1(
            char(','),
            map_parser(
                is_not(", \t"),
                transform_escaped
            )
        )(i)
    }

    pub fn parse_line(i: &str) -> IResult<&str, Mount> {
        match all_consuming(tuple((
            map_parser(not_whitespace, transform_escaped),
            space1,
            map_parser(not_whitespace, transform_escaped),
            space1,
            not_whitespace,
            space1,
            mount_opts,
            space1,
            char('0'),
            space1,
            char('0'),
            space0,
        )))(i) {
            Ok((remaining_input, (
                device,
                _,
                mount_point,
                _,
                file_system_type,
                _,
                options,
                _,
                _,
                _,
                _,
                _,
            ))) => {
                Ok((remaining_input, Mount {
                    device,
                    mount_point,
                    file_system_type: file_system_type.to_string(),
                    options,
                }))
            }
            Err(e) => Err(e)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use nom::error::ErrorKind;

        #[test]
        fn test_not_whitespace() {
            assert_eq!(not_whitespace("abcd efg"),
                       Ok((" efg", "abcd")),
            );
            assert_eq!(not_whitespace("abcd\tefg"),
                       Ok(("\tefg", "abcd"))
            );
            assert_eq!(not_whitespace(" abcdefg"),
                       Err(nom::Err::Error(nom::error::Error {
                           input: " abcdefg",
                           code: ErrorKind::IsNot
                       }))
            );
        }

        #[test]
        fn test_escaped_space() {
            assert_eq!(escaped_space("040"), Ok(("", " ")));
            assert_eq!(escaped_space(" "),
                       Err(nom::Err::Error(nom::error::Error {
                           input: " ",
                           code: ErrorKind::Tag
                       }))
            );
        }

        #[test]
        fn test_escaped_backslash() {
            assert_eq!(escaped_backslash("\\"), Ok(("", "\\")));
            assert_eq!(escaped_backslash("not a backslash"),
                       Err(nom::Err::Error(nom::error::Error {
                           input: "not a backslash",
                           code: ErrorKind::Char
                       }))
            );
        }

        #[test]
        fn test_transform_escaped() {
            assert_eq!(transform_escaped("abc\\040def\\\\g\\040h"),
                       Ok(("", String::from("abc def\\g h")))
            );
            assert_eq!(transform_escaped("\\bad"),
                       Err(nom::Err::Error(nom::error::Error {
                           input: "bad",
                           code: ErrorKind::Tag
                       }))
            );
        }

        #[test]
        fn test_mount_opts() {
            assert_eq!(
                mount_opts("a,bc,d\\040e"),
                Ok(("", vec!["a".to_string(), "bc".to_string(), "d e".to_string()]))
            );
        }

        #[test]
        fn test_parse_line() {
            let mount1 = Mount{
                device: "device".to_string(),
                mount_point: "mount_point".to_string(),
                file_system_type: "file_system_type".to_string(),
                options: vec!["options".to_string(), "a".to_string(), "b=c".to_string(), "d e".to_string()]
            };
            let (_, mount2) = parse_line("device mount_point file_system_type options,a,b=c,d\\040e 0 0").unwrap();
            assert_eq!(mount1.device, mount2.device);
            assert_eq!(mount1.mount_point, mount2.mount_point);
            assert_eq!(mount1.file_system_type, mount2.file_system_type);
            assert_eq!(mount1.options, mount2.options);
        }
    }
}
