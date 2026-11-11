use resp::{RespValue, encode};

#[test]
fn simple_string() {
    let v = RespValue::Simple("OK".to_string());
    assert_eq!(encode(&v), b"+OK\r\n");
}

#[test]
fn error() {
    let v = RespValue::Error("ERR unknown command".to_string());
    assert_eq!(encode(&v), b"-ERR unknown command\r\n");
}

#[test]
fn integer() {
    assert_eq!(encode(&RespValue::Integer(0)), b":0\r\n");
    assert_eq!(encode(&RespValue::Integer(-42)), b":-42\r\n");
}

#[test]
fn bulk_string() {
    let v = RespValue::Bulk(b"hello".to_vec());
    assert_eq!(encode(&v), b"$5\r\nhello\r\n");
}

#[test]
fn empty_and_null_bulk() {
    assert_eq!(encode(&RespValue::Bulk(Vec::new())), b"$0\r\n\r\n");
    assert_eq!(encode(&RespValue::Null), b"$-1\r\n");
}

#[test]
fn array_is_length_prefixed_and_recursive() {
    let v = RespValue::Array(vec![
        RespValue::Bulk(b"GET".to_vec()),
        RespValue::Bulk(b"foo".to_vec()),
    ]);
    assert_eq!(encode(&v), b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n");
}

#[test]
fn nested_empty_array() {
    let v = RespValue::Array(vec![RespValue::Array(vec![])]);
    assert_eq!(encode(&v), b"*1\r\n*0\r\n");
}
