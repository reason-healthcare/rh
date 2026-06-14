//! Literal expression parsers.
//!
//! All parsers return a `nom::IResult` over `Span`.

use crate::parser::ast::*;
use crate::parser::lexer::{
    date_literal, datetime_literal, decimal_literal, integer_literal, keyword, long_literal,
    quantity_literal, string_literal, time_literal,
};
use crate::parser::span::Span;
use nom::{
    branch::alt,
    character::complete::multispace1,
    combinator::{map, value},
    IResult,
};

// ============================================================================
// Literal Parsers
// ============================================================================

pub(crate) fn parse_null_literal(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(keyword("null"), |_| Expression::Literal(Literal::Null))(input)
}

pub(crate) fn parse_boolean_literal(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    alt((
        map(keyword("true"), |_| {
            Expression::Literal(Literal::Boolean(true))
        }),
        map(keyword("false"), |_| {
            Expression::Literal(Literal::Boolean(false))
        }),
    ))(input)
}

pub(crate) fn parse_string_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(string_literal, |s| Expression::Literal(Literal::String(s)))(input)
}

pub(crate) fn parse_integer_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(integer_literal, |n| {
        Expression::Literal(Literal::Integer(n))
    })(input)
}

pub(crate) fn parse_long_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(long_literal, |n| Expression::Literal(Literal::Long(n)))(input)
}

pub(crate) fn parse_decimal_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(decimal_literal, |n| {
        Expression::Literal(Literal::Decimal(n))
    })(input)
}

pub(crate) fn parse_quantity_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(quantity_literal, |(value, unit)| {
        Expression::Literal(Literal::Quantity { value, unit })
    })(input)
}

/// Parse a ratio literal: `<quantity> : <quantity>` where each side is a
/// quantity (`1 'mg'`) or a plain number (`1:128`).
pub(crate) fn parse_ratio_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    fn ratio_part(input: Span<'_>) -> IResult<Span<'_>, Literal> {
        alt((
            map(quantity_literal, |(value, unit)| Literal::Quantity {
                value,
                unit,
            }),
            map(decimal_literal, Literal::Decimal),
            map(long_literal, Literal::Long),
            map(integer_literal, Literal::Integer),
        ))(input)
    }

    let (input, numerator) = ratio_part(input)?;
    let (input, _) = nom::character::complete::multispace0(input)?;
    let (input, _) = nom::character::complete::char(':')(input)?;
    let (input, _) = nom::character::complete::multispace0(input)?;
    let (input, denominator) = ratio_part(input)?;
    Ok((
        input,
        Expression::Literal(Literal::Ratio {
            numerator: Box::new(numerator),
            denominator: Box::new(denominator),
        }),
    ))
}

pub(crate) fn parse_duration_quantity_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    let (input, qty_value) = alt((decimal_literal, map(integer_literal, |i| i as f64)))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, unit) = alt((
        value("year", keyword("year")),
        value("year", keyword("years")),
        value("month", keyword("month")),
        value("month", keyword("months")),
        value("week", keyword("week")),
        value("week", keyword("weeks")),
        value("day", keyword("day")),
        value("day", keyword("days")),
        value("hour", keyword("hour")),
        value("hour", keyword("hours")),
        value("minute", keyword("minute")),
        value("minute", keyword("minutes")),
        value("second", keyword("second")),
        value("second", keyword("seconds")),
        value("millisecond", keyword("millisecond")),
        value("millisecond", keyword("milliseconds")),
    ))(input)?;

    Ok((
        input,
        Expression::Literal(Literal::Quantity {
            value: qty_value,
            unit: unit.to_string(),
        }),
    ))
}

pub(crate) fn parse_date_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(date_literal, |s| Expression::Literal(Literal::Date(s)))(input)
}

pub(crate) fn parse_time_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(time_literal, |s| Expression::Literal(Literal::Time(s)))(input)
}

pub(crate) fn parse_datetime_literal_expr(input: Span<'_>) -> IResult<Span<'_>, Expression> {
    map(datetime_literal, |s| {
        Expression::Literal(Literal::DateTime(s))
    })(input)
}
