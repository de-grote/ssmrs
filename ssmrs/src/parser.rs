use chumsky::{
    prelude::Simple,
    primitive::{choice, just},
    text::{self, ident, whitespace, TextParser},
    Error, Parser,
};

use crate::register::Reg;
use crate::Instr;

pub fn parse() -> impl Parser<char, Vec<Instr>, Error = Simple<char>> {
    parse_instr().padded().repeated()
}

fn parse_instr() -> impl Parser<char, Instr, Error = Simple<char>> {
    let number = just('-')
        .or_not()
        .chain::<char, _, _>(text::int(10))
        .collect::<String>()
        .from_str()
        .unwrapped()
        .labelled("number");

    choice((
        i("STR", Instr::STR, parse_register()),
        i("STR", Instr::STR, parse_register()),
        i("STL", Instr::STL, number),
        i("STL", Instr::STL, number),
        i("STS", Instr::STS, number),
        i("STA", Instr::STA, number),
        i("LDR", Instr::LDR, parse_register()),
        i("LDL", Instr::LDL, number),
        i("LDS", Instr::LDS, number),
        i("LDA", Instr::LDA, number),
        i("LDC", Instr::LDC, number),
        i("LDLA", Instr::LDLA, number),
        i("LDSA", Instr::LDSA, number),
        i("LDAA", Instr::LDAA, number),
        i("BRA", Instr::Bra, text::ident()),
        i("BRF", Instr::Brf, text::ident()),
        i("BRT", Instr::Brt, text::ident()),
        i("BSR", Instr::Bsr, text::ident()),
        i("LINK", Instr::LINK, number),
        i("AJS", Instr::AJS, number),
        i("SWPR", Instr::SWPR, parse_register()),
        i("TRAP", Instr::TRAP, number),
    ))
    .or(choice((
        s("ADD", Instr::ADD),
        s("SUB", Instr::SUB),
        s("MUL", Instr::MUL),
        s("DIV", Instr::DIV),
        s("MOD", Instr::MOD),
        s("EQ", Instr::EQ),
        s("NE", Instr::NE),
        s("LT", Instr::LT),
        s("LE", Instr::LE),
        s("GT", Instr::GT),
        s("GE", Instr::GE),
        s("NEG", Instr::NEG),
        s("NOT", Instr::NOT),
        s("RET", Instr::RET),
        s("UNLINK", Instr::UNLINK),
        s("SWP", Instr::SWP),
        s("JSR", Instr::JSR),
        s("NOP", Instr::NOP),
        s("HALT", Instr::HALT),
        s("AND", Instr::AND),
        s("OR", Instr::OR),
        s("XOR", Instr::XOR),
    )))
    .or(choice((
        w("SWPRR", Instr::SWPRR, parse_register(), parse_register()),
        w("LDRR", Instr::LDRR, parse_register(), parse_register()),
        i("BRA", Instr::BRA, number),
        i("BRF", Instr::BRF, number),
        i("BRT", Instr::BRT, number),
        i("BSR", Instr::BSR, number),
    )))
    .or(text::ident().then_ignore(just(":")).map(Instr::LABEL))
}

fn i<A>(
    s: &'static str,
    f: impl Fn(A) -> Instr,
    p: impl Parser<char, A, Error = Simple<char>>,
) -> impl Parser<char, Instr, Error = Simple<char>> {
    ident()
        .try_map(move |st: String, span| {
            st.eq_ignore_ascii_case(s)
                .then_some(())
                .ok_or_else(|| Simple::expected_input_found(span, None, None))
        })
        .ignore_then(whitespace())
        .ignore_then(p)
        .map(f)
}

fn s(s: &'static str, i: Instr) -> impl Parser<char, Instr, Error = Simple<char>> {
    just(s).to(i)
}

fn w<A, B>(
    s: &'static str,
    f: impl Fn(A, B) -> Instr + 'static,
    p: impl Parser<char, A, Error = Simple<char>>,
    q: impl Parser<char, B, Error = Simple<char>>,
) -> impl Parser<char, Instr, Error = Simple<char>> {
    just(s)
        .ignore_then(whitespace())
        .ignore_then(p)
        .then_ignore(whitespace())
        .then(q)
        .map(move |(a, b)| f(a, b))
}

fn parse_register() -> impl Parser<char, Reg, Error = Simple<char>> {
    choice((
        just("PC").to(Reg::PC),
        just("SP").to(Reg::SP),
        just("MP").to(Reg::MP),
        just("R3").to(Reg::R3),
        just("R4").to(Reg::R4),
        just("R5").to(Reg::R5),
        just("R6").to(Reg::R6),
        just("R7").to(Reg::R7),
    ))
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    #[test]
    fn test_single_instr() {
        let input = "NOP";
        let result = super::parse().parse(input);
        assert_eq!(result, Ok(vec![super::Instr::NOP]));
    }

    #[test]
    fn test_multiple_instr() {
        let code = r#"
main:
        LDC 41
        LDC 1
        ADD
        TRAP 0
        HALT
        "#;
        let result = super::parse().parse(code);
        assert_eq!(
            result,
            Ok(vec![
                super::Instr::LABEL("main".to_string()),
                super::Instr::LDC(41),
                super::Instr::LDC(1),
                super::Instr::ADD,
                super::Instr::TRAP(0),
                super::Instr::HALT,
            ])
        );
    }
}
