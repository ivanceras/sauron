//! utility functions that are not directly related to virtual node

/// make a blank string with indented padd
pub fn indent(n: usize) -> String {
    std::iter::repeat("    ").take(n).collect::<String>()
}
