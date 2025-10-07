use powerletters::*;
use std::path::{Path, PathBuf};
use std::ffi::{OsStr, OsString};
use std::collections::HashMap;

#[test]
fn test_s_string_function() {
    let s: String = S("foo");
    assert_eq!(s, "foo");
}

#[test]
fn test_s_string_method() {
    let s: String = "foo".S();
    assert_eq!(s, "foo");
}

#[test]
fn test_s_primitives_function() {
    assert_eq!(S(&42i32), "42");
    assert_eq!(S(&42u32), "42");
    assert_eq!(S(&3.14f64), "3.14");
    assert_eq!(S(&true), "true");
}

#[test]
fn test_s_primitives_method() {
    assert_eq!(42i32.S(), "42");
    assert_eq!(42u32.S(), "42");
    assert_eq!(3.14f64.S(), "3.14");
    assert_eq!(true.S(), "true");
}

// Note: Path, PathBuf, OsStr don't implement Display/ToString,
// so they cannot use S(). Use to_string_lossy() or display() instead.

#[test]
fn test_c_string_function() {
    let original = String::from("hello");
    let cloned = C(&original);
    assert_eq!(cloned, "hello");
    assert_eq!(original, "hello"); // original still exists
}

#[test]
fn test_c_string_method() {
    let original = String::from("hello");
    let cloned = original.C();
    assert_eq!(cloned, "hello");
    assert_eq!(original, "hello"); // original still exists
}

#[test]
fn test_c_vec_function() {
    let bagostuff = vec!["a", "b", "c"];
    let newbag = C(&bagostuff);
    assert_eq!(newbag, vec!["a", "b", "c"]);
}

#[test]
fn test_c_vec_method() {
    let bagostuff = vec!["a", "b", "c"];
    let newbag = bagostuff.C();
    assert_eq!(newbag, vec!["a", "b", "c"]);
}

#[test]
fn test_c_pathbuf_function() {
    let p = PathBuf::from("foo");
    let p2 = C(&p);
    assert_eq!(p2, PathBuf::from("foo"));
}

#[test]
fn test_c_pathbuf_method() {
    let p = PathBuf::from("foo");
    let p2 = p.C();
    assert_eq!(p2, PathBuf::from("foo"));
}

#[test]
fn test_c_hashmap_function() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    let map2 = C(&map);
    assert_eq!(map2.get("key"), Some(&"value"));
}

#[test]
fn test_c_hashmap_method() {
    let mut map = HashMap::new();
    map.insert("key", "value");
    let map2 = map.C();
    assert_eq!(map2.get("key"), Some(&"value"));
}

#[test]
fn test_c_primitives_function() {
    assert_eq!(C(&42i32), 42);
    assert_eq!(C(&3.14f64), 3.14);
}

#[test]
fn test_c_primitives_method() {
    assert_eq!(42i32.C(), 42);
    assert_eq!(3.14f64.C(), 3.14);
}

#[test]
fn test_o_str_function() {
    let s: &str = "hello";
    let owned: String = O(s);
    assert_eq!(owned, "hello");
}

#[test]
fn test_o_str_method() {
    let owned: String = "hello".O();
    assert_eq!(owned, "hello");
}

#[test]
fn test_o_path_function() {
    let p = Path::new("chill");
    let owned: PathBuf = O(p);
    assert_eq!(owned, PathBuf::from("chill"));
}

#[test]
fn test_o_path_method() {
    let owned: PathBuf = Path::new("chill").O();
    assert_eq!(owned, PathBuf::from("chill"));
}

#[test]
fn test_o_osstr_function() {
    let os = OsStr::new("foo");
    let owned: OsString = O(os);
    assert_eq!(owned, OsString::from("foo"));
}

#[test]
fn test_o_osstr_method() {
    let owned: OsString = OsStr::new("foo").O();
    assert_eq!(owned, OsString::from("foo"));
}

#[test]
fn test_o_slice_function() {
    let slice: &[i32] = &[1, 2, 3];
    let owned: Vec<i32> = O(slice);
    assert_eq!(owned, vec![1, 2, 3]);
}

#[test]
fn test_o_slice_method() {
    let slice_ref: &[i32] = &[1, 2, 3];
    let owned: Vec<i32> = (*slice_ref).O();
    assert_eq!(owned, vec![1, 2, 3]);
}

#[test]
fn test_x_option_some_function() {
    let maybe_thing = Some("thing");
    let thing = X(maybe_thing);
    assert_eq!(thing, "thing");
}

#[test]
fn test_x_option_some_method() {
    let maybe_thing = Some("thing");
    let thing = maybe_thing.X();
    assert_eq!(thing, "thing");
}

#[test]
#[should_panic(expected = "impossible `None` option")]
fn test_x_option_none_function() {
    let maybe_thing: Option<&str> = None;
    let _thing = X(maybe_thing);
}

#[test]
#[should_panic(expected = "impossible `None` option")]
fn test_x_option_none_method() {
    let maybe_thing: Option<&str> = None;
    let _thing = maybe_thing.X();
}

#[test]
fn test_x_result_ok_function() {
    let good_thing: Result<&str, std::io::Error> = Ok("thing");
    let thing = X(good_thing);
    assert_eq!(thing, "thing");
}

#[test]
fn test_x_result_ok_method() {
    let good_thing: Result<&str, std::io::Error> = Ok("thing");
    let thing = good_thing.X();
    assert_eq!(thing, "thing");
}

#[test]
#[should_panic(expected = "impossible `Err` result")]
fn test_x_result_err_function() {
    let bad_thing: Result<&str, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "oops"));
    let _thing = X(bad_thing);
}

#[test]
#[should_panic(expected = "impossible `Err` result")]
fn test_x_result_err_method() {
    let bad_thing: Result<&str, std::io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "oops"));
    let _thing = bad_thing.X();
}

#[test]
fn test_x_result_parse_int() {
    let parsed: Result<i32, _> = "42".parse();
    let value = X(parsed);
    assert_eq!(value, 42);
}

#[test]
fn test_x_result_display_only() {
    #[derive(Debug)]
    struct DisplayOnlyError(String);

    impl std::fmt::Display for DisplayOnlyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    let result: Result<i32, DisplayOnlyError> = Ok(42);
    let value = X(result);
    assert_eq!(value, 42);
}

#[test]
fn test_i_result_function() {
    let result: Result<(), std::io::Error> = Ok(());
    I(result);
}

#[test]
fn test_i_result_method() {
    let result: Result<(), std::io::Error> = Ok(());
    result.I();
}

#[test]
fn test_i_result_with_value_function() {
    let result: Result<String, std::io::Error> = Ok(String::from("ignored"));
    I(result);
}

#[test]
fn test_i_result_with_value_method() {
    let result: Result<String, std::io::Error> = Ok(String::from("ignored"));
    result.I();
}
