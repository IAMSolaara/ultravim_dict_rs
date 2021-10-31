use regex::Regex;
use regex::RegexSet;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum QueryType {
    GET,
    PUT,
    DELETE,
}

#[derive(Debug)]
pub struct Query {
    query_type: QueryType,
    key: String,
    value: String,
}

impl PartialEq for Query {
    fn eq(&self, other: &Self) -> bool {
        self.query_type == other.query_type && self.key == other.key && self.value == other.value
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn parse_query(query: &str) -> Option<Query> {
    let regexes = RegexSet::new(&[
        r"^(?P<query_type>GET) <(?P<key>.{1,255})>$",
        r"^(?P<query_type>PUT) <(?P<key>.{1,255})> <(?P<value>.{1,255})>$",
        r"^(?P<query_type>DELETE) <(?P<key>.{1,255})> <(?P<value>.{1,255})>$",
    ])
    .unwrap();

    let matches: Vec<_> = regexes.matches(query).into_iter().collect();
    if matches.len() != 1 {
        return None;
    }

    let matched = matches[0];
    let the_right_regex = regexes.patterns()[matched].clone();

    let re = Regex::new(&the_right_regex).unwrap();
    let caps = re.captures(query).unwrap();
    let dict: HashMap<&str, &str> = re
        .capture_names()
        .flatten()
        .filter_map(|n| Some((n, caps.name(n)?.as_str())))
        .collect();
    let query_type = dict["query_type"];
    match query_type {
        "GET" => {
            println!("Got GET");
            return Some(Query {
                query_type: QueryType::GET,
                key: String::from(dict["key"]),
                value: String::from(""),
            });
        }
        "PUT" => {
            println!("Got PUT");
            return Some(Query {
                query_type: QueryType::PUT,
                key: String::from(dict["key"]),
                value: String::from(dict["value"]),
            });
        }
        "DELETE" => {
            println!("Got DELETE");
            return Some(Query {
                query_type: QueryType::DELETE,
                key: String::from(dict["key"]),
                value: String::from(dict["value"]),
            });
        }

        _ => return None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_get_test() {
        let query = parse_query("GET <test>").unwrap();
        assert_eq!(
            query,
            Query {
                query_type: QueryType::GET,
                key: String::from("test"),
                value: String::from("")
            }
        );
    }
    #[test]
    fn parser_put_test() {
        let query = parse_query("PUT <test_k> <test_v>").unwrap();
        assert_eq!(
            query,
            Query {
                query_type: QueryType::PUT,
                key: String::from("test_k"),
                value: String::from("test_v")
            }
        );
    }
    #[test]
    fn parser_delete_test() {
        let query = parse_query("DELETE <test_k> <test_v>").unwrap();
        assert_eq!(
            query,
            Query {
                query_type: QueryType::DELETE,
                key: String::from("test_k"),
                value: String::from("test_v")
            }
        );
    }
    #[test]
    fn parser_get_test_with_spaces() {
        let query = parse_query("GET <test >").unwrap();
        assert_eq!(
            query,
            Query {
                query_type: QueryType::GET,
                key: String::from("test "),
                value: String::from("")
            }
        );
    }
    #[test]
    fn parser_get_test_invalid_key() {
        let query = parse_query("GET <test > ");
        assert_eq!(query, None);
    }
}
