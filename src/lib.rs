#[derive(Debug, Clone, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}
fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input)
    }
}
fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str>{
    {
        move |input| match input.get(0..expected.len()) {
            Some(next) if next == expected => {
                Ok((&input[expected.len()..], ()))
            },
            _ => Err(input),
        }
    }
}
fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    // This step is to ensure the first character is alphabetic
    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }
    // This step handles alphabetic characters and the - symbol
    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }
    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}
fn pair<P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
where
    P1: Fn(&str) -> Result<(&str, R1), &str>,
    P2: Fn(&str) -> Result<(&str, R2), &str>
{
    move |input| match parser1(input) {
        Ok((next_input, result1)) => match parser2(next_input) {
            Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
            Err(err) => Err(err)
        },
        Err(err) => Err(err),
    }
}
fn map<P, F, A, B> (parser: P, map_fn: F) -> impl Fn(&str) -> Result<(&str, B), &str>
where
    P: Fn(&str) -> Result<(&str>, A), &str>,
    F: Fn(A) -> B,
{

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let test_value = "abcdefg";
        let out = the_letter_a(test_value).unwrap();
    }
    #[test]
    #[should_panic]
    fn parse_fail() {
        let test_value = "bcdefg";
        let out = the_letter_a(test_value).unwrap();
    }
    #[test]
    fn literal_parser() {
        let parse_joe = match_literal("Hello, Joe!");
        assert_eq!(
            Ok(("", ())),
            parse_joe("Hello, Joe!")
        );
        assert_eq!(
            Ok((" Hello, Robert!", ())),
            parse_joe("Hello, Joe! Hello, Robert!")
        );
        assert_eq!(
            Err("Hello, Mike"),
            parse_joe("Hello, Mike")
        );
    }
    #[test]
    fn identifier_parser() {
        assert_eq!(
            Ok(("", "i-am-an-identifier".to_string())),
            identifier("i-am-an-identifier")
        );
        assert_eq!(
            Ok((" entirely an identifier", "not".to_string())),
            identifier("not entirely an identifier")
        );
        assert_eq!(
            Err("!not at all an identifier"),
            identifier("!not at all an identifier")
        );
    }
    #[test]
    fn pair_combinator() {
        let tag_opener = pair(match_literal("<"), identifier);
        assert_eq!(
            Ok(("/>", ((), "my-first-element".to_string()))),
            tag_opener("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener("oops"));
        assert_eq!(Err("!oops"), tag_opener("<!oops"));
    }
}
