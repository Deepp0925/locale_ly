use super::localize_string::LocalizeString;

/// All parsers should implement this trait
/// This will allow the program to place the tokens back after translation
pub trait ParseString {
    /// return the a string with the interpolation
    /// items replaced with {number}  
    /// For example
    /// ```
    /// let s = "Hello {name}, there are {count} items in your cart";
    /// ```
    /// should return
    /// ```
    /// "Hello {0}, there are {1} items in your cart"
    /// ```
    /// and the vector should contain
    /// ```
    /// vec!["name", "count"]
    /// ```
    /// # Arguments
    /// * `s` - The string to parse
    /// # Returns
    /// a struct that contains the parsed string and the items that were replaced
    /// - txt: The parsed string
    /// - items: The items that were replaced
    fn parse_string(s: &mut str) -> LocalizeString;
}
