use chomp::ascii::{is_horizontal_space, is_whitespace};
use chomp::prelude::*;
use std::io::Read;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::str;


#[derive(Debug, PartialEq)]
pub struct Header {
    name: String,
    value: String,
}

struct EOLMatcher {
    found_carriage_return: bool,
}

impl EOLMatcher {
    fn new() -> Self {
        EOLMatcher {
            found_carriage_return: false
        }
    }

    fn at_eol<I: U8Input<Token=u8>>(&mut self, token: I::Token) -> bool {
        if token == b'\r' {
            self.found_carriage_return = true;
            false
        } else if token == b'\n' && self.found_carriage_return {
            self.found_carriage_return = false;
            true
        } else {
            self.found_carriage_return = false;
            false
        }
    }
}

fn header<'a, I: U8Input<Buffer=&'a [u8]>>(i: I) -> SimpleResult<I, Option<HeaderType>> {
    let mut eol_matcher = EOLMatcher::new();
    parse!{i;
                   skip_while(is_whitespace);
        let name = take_while(|c| c != b':');
                   token(b':');
                   skip_while(is_horizontal_space);
        let value = take_while(|token| token != b'\r');
                    string(b"\r\n");


        ret {
            if let (Ok(name_str), Ok(value_str)) = (str::from_utf8(name), str::from_utf8(value)) {
                HeaderType::from_raw(name_str, value_str)
            } else {
                None
            }
        }
    }
}

fn headers<'a, I: U8Input<Buffer=&'a [u8]>>(i: I) -> SimpleResult<I, Headers> {
    many1(i, header)
        .bind::<_, _, Error<u8>>(|i, headers: Vec<Option<HeaderType>>| {
            i.ret(headers.iter().filter_map(|v| {
                match v {
                    &Some(ref header_type) => Some((header_type.as_key(), Box::new(header_type))),
                    &None => None
                }
            }).collect::<Headers>())
        })
}

fn read_content<R: Read>(reader: &mut R, size: usize) -> String {
    let mut buf = Vec::with_capacity(size);
    reader.read_exact(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

fn content<I: U8Input<Token=u8>>(i: I, size: usize) -> SimpleResult<I, String> {
    take(i, size).bind(|i, bytes| i.ret(String::from_utf8(bytes.to_vec()).unwrap()))
}

struct IMessage {
    headers: Vec<Header>,
    content: String,
}

// fn message<I: U8Input>(i: I) -> SimpleResult<I, IMessage> {
//     parse!{i;
//         let my_headers = headers;

//         ret Message {
//             headers: headers,
//             content: "".to_string()
//         }
//     }
// }

type ContentLength = usize;
type ContentType = String;

#[derive(Clone, Debug, PartialEq)]
enum HeaderType {
    ContentLengthHeader(ContentLength),
    ContentTypeHeader(ContentType),
}

impl HeaderType {
    fn from_raw(name: &str, value: &str) -> Option<HeaderType> {
        match name {
            "Content-Type" => Some(HeaderType::ContentTypeHeader(value.to_string())),
            "Content-Length" => Some(HeaderType::ContentLengthHeader(value.parse::<usize>().unwrap())),
            _ => None
        }
    }

    fn as_key(&self) -> &'static str {
        match *self {
            HeaderType::ContentLengthHeader(_) => "Content-Length",
            HeaderType::ContentTypeHeader(_) => "Content-Type",
        }
    }
}

type Headers = HashMap<&'static str, HeaderType>;

struct Content;
/// Per the spec, calls can be batched and sent in one message as an array
pub struct Message;


#[cfg(test)]
mod test {
    use chomp::prelude::*;
    use super::{Message, Headers, Header, HeaderType, header, headers, content};
    use std::io::Read;
use std::str;

    #[test]
    fn header_parser_works() {
        let expected = HeaderType::ContentLengthHeader(9000);
        let actual = parse_only(header, b"Content-Length: 9000\r\n").unwrap();
        assert_eq!(expected, actual.unwrap());
    }

    fn valid_headers() -> &'static [u8] {
        "
Content-Length: 1\r
Content-Type: application/vscode-jsonrpc; charset=utf8\r
\r\n
".as_bytes()
    }

    #[test]
    fn headers_parse_works() {
        let headers = parse_only(headers, valid_headers()).unwrap();
        if let Some(&HeaderType::ContentLengthHeader(length)) = headers.get("Content-Length") {
            assert_eq!(length, 1);
        } else {
            panic!();
        }
    }

    fn valid_message() -> &'static [u8] {
        "
Content-Length: 4\r
Content-Type: application/vscode-jsonrpc; charset=utf8\r
\r
{\"foo\": true}
".as_bytes()
    }

    #[test]
    fn content_parser_works() {
        let data = b"It's over 9000!!!";
        let expected = "It's over".to_string();
        let result = parse_only(|i| content(i, 9), data).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn content_can_read_to_exhaustion() {
        let data = b"It's over 9000!!!";
        let result = parse_only(|i| content(i, 17), data).unwrap();
        assert_eq!(result, str::from_utf8(data).unwrap().to_string());
    }

    #[test]
    fn content_works_with_multibyte_characters() {
        let data = "ワンパタン";
        let result = parse_only(|i| content(i, 15), data.as_bytes()).unwrap();
        assert_eq!(result, data.to_string());
    }

    #[test]
    fn message_can_parse_a_whole_message() {
        panic!();
    }

    #[test]
    fn messages_can_parse_an_arbitrary_number_of_messages() {
        panic!();
    }
}
