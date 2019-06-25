
/// make a blank string with indented padd
pub(in crate) fn indent(n: i32) -> String {
    let mut buffer = String::new();
    for _ in 0..n {
        buffer += "    ";
    }
    buffer
}
