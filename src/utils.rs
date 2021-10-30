use regex::Regex;
use regex::RegexSet;
use std::collections::HashMap;

#[derive(Debug)]
pub enum QueryType {
    GET, 
    PUT,
    DELETE
}

#[derive(Debug)]
pub struct Query {
    query_type: QueryType,
    key: String,
    value: String
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn parse_query(query: &str) -> Option<Query>  {
    let regexes = RegexSet::new(&[
        r"^(?P<query_type>GET) <(?P<key>.{1,255})>$",
        r"^(?P<query_type>PUT) <(?P<key>.{1,255})> <(?P<value>.{1,255})>$",
        r"^(?P<query_type>DELETE) <(?P<key>.{1,255})> <(?P<value>.{1,255})>$"
    ]).unwrap();

    let matches: Vec<_> = regexes.matches(query).into_iter().collect();
    if matches.len() != 1 { return None; }

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
                value: String::from("")
            });
        }
        "PUT" => {
            println!("Got PUT");
            return Some(Query {
                query_type: QueryType::PUT,
                key: String::from(dict["key"]),
                value: String::from(dict["value"])
            });
        }
        "DELETE" => {
            println!("Got DELETE");
            return Some(Query {
                query_type: QueryType::DELETE,
                key: String::from(dict["key"]),
                value: String::from(dict["value"])
            });
        }

        _ => return None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        println!("Hello, world!");

        let query = parse_query("GET <test>");
        println!("{:?}", query);

        let query = parse_query("PUT <test_k> <test_v>");
        println!("{:?}", query);
        
        let query = parse_query("DELETE <test_k> <test_v>");
        println!("{:?}", query);
        
        let query = parse_query("GET <test >");
        println!("{:?}", query);

        let query = parse_query("GET <test > ");
        println!("{:?}", query);
    }
}
