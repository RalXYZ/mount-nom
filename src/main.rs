use nom::{
    IResult,
    sequence::delimited,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::char,
    bytes::complete::is_not
};

extern crate mount_nom;

fn parens(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}

fn main() {
    let a = parens("(asd))")
        .unwrap().0;

    println!("{}", a);
}
