/// make a blank string with indented padd
pub(in crate) fn indent(n: usize) -> String {
    std::iter::repeat("    ").take(n).collect::<String>()
}
