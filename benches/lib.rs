#![feature(test)]

extern crate test;
use test::Bencher;

static DATE: &str = "2014-11-28";
static T: &str = "T";
static TIME: &str = "12:00:09Z";
static DATETIME: &str = "2014-11-28T12:00:09Z";


////
#[bench]
fn array_concat(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &[DATE, T, TIME].concat();
        test::black_box(datetime);
    });
}

#[test]
fn array_concat_test() {
    let datetime: &str = &[DATE, T, TIME].concat();
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn array_join(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &[DATE, TIME].join(T);
        test::black_box(datetime);
    });
}

#[test]
fn array_join_test() {
    let datetime: &str = &[DATE, TIME].join(T);
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn array_join_long(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &[DATE, T, TIME].join("");
        test::black_box(datetime);
    });
}

#[test]
fn array_join_long_test() {
    let datetime: &str = &[DATE, T, TIME].join("");
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn collect_to_string(b: &mut Bencher) {
    let list = vec![DATE, T, TIME];
    b.iter(|| {
        let datetime: String = list.iter().map(|x| *x).collect();
        test::black_box(datetime);
    });
}

#[test]
fn collect_to_string_test() {
    let list = vec![DATE, T, TIME];
    let datetime: String = list.iter().map(|x| *x).collect();
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn format_macro(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &format!("{}T{}", DATE, TIME);
        test::black_box(datetime);
    });
}

#[test]
fn format_macro_test() {
    let datetime: &str = &format!("{}{}{}", DATE, T, TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
#[cfg(unix)]
fn from_bytes(b: &mut Bencher) {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::slice;

    b.iter(|| {
        let bytes = unsafe { slice::from_raw_parts(DATE.as_ptr(), 20) };

        let datetime = OsStr::from_bytes(bytes);
        test::black_box(datetime);
    });
}

#[test]
fn from_bytes_test() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::slice;

    let bytes = unsafe { slice::from_raw_parts(DATE.as_ptr(), 20) };

    let datetime = OsStr::from_bytes(bytes);

    assert_eq!(String::from(DATETIME), datetime.to_owned().into_string().unwrap());
}

////
#[bench]
fn mut_string_push_str(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::new();
        datetime.push_str(DATE);
        datetime.push_str(T);
        datetime.push_str(TIME);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_push_str_test() {
    let mut datetime = String::new();
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn mut_string_push_string(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = Vec::<String>::new();
        datetime.push(String::from(DATE));
        datetime.push(String::from(T));
        datetime.push(String::from(TIME));
        let datetime = datetime.join("");
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_push_string_test() {
    let mut datetime = Vec::<String>::new();
    datetime.push(String::from(DATE));
    datetime.push(String::from(T));
    datetime.push(String::from(TIME));
    let datetime = datetime.join("");
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn mut_string_with_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::with_capacity(20);
        datetime.push_str(DATE);
        datetime.push_str(T);
        datetime.push_str(TIME);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_with_capacity_push_str_test() {
    let mut datetime = String::with_capacity(20);
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn mut_string_with_capacity_push_str_char(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::with_capacity(20);
        datetime.push_str(DATE);
        datetime.push('T');
        datetime.push_str(TIME);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_with_capacity_push_str_char_test() {
    let mut datetime = String::with_capacity(20);
    datetime.push_str(DATE);
    datetime.push('T');
    datetime.push_str(TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn mut_string_with_too_little_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::with_capacity(2);
        datetime.push_str(DATE);
        datetime.push_str(T);
        datetime.push_str(TIME);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_with_too_little_capacity_push_str_test() {
    let mut datetime = String::with_capacity(2);
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn mut_string_with_too_much_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
        let mut datetime = String::with_capacity(200);
        datetime.push_str(DATE);
        datetime.push_str(T);
        datetime.push_str(TIME);
        test::black_box(datetime);
    });
}

#[test]
fn mut_string_with_too_much_capacity_push_str_test() {
    let mut datetime = String::with_capacity(200);
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn string_from_all(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &(String::from(DATE) + &String::from(T) + &String::from(TIME));
        test::black_box(datetime);
    });
}

#[test]
fn string_from_all_test() {
    let datetime: &str = &(String::from(DATE) + &String::from(T) + &String::from(TIME));
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn string_from_plus_op(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &(String::from(DATE) + T + TIME);
        test::black_box(datetime);
    });
}

#[test]
fn string_from_plus_op_test() {
    let datetime: &str = &(String::from(DATE) + T + TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn to_owned_plus_op(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &(DATE.to_owned() + T + TIME);
        test::black_box(datetime);
    });
}

#[test]
fn to_owned_plus_op_test() {
    let datetime: &str = &(DATE.to_owned() + T + TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

////
#[bench]
fn to_string_plus_op(b: &mut Bencher) {
    b.iter(|| {
        let datetime: &str = &(DATE.to_string() + T + TIME);
        test::black_box(datetime);
    });
}

#[test]
fn to_string_plus_op_test() {
    let datetime: &str = &(DATE.to_string() + T + TIME);
    assert_eq!(String::from(DATETIME), datetime);
}
