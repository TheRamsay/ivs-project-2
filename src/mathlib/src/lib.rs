use rust_decimal::prelude::*;

/// Enum representing any mathematical expression.
///
/// Some of the operands must be wrapped in `Box<T>`,
/// because you cannot store self-containing structures
/// on the stack.
///
pub enum MathExpr {
    /// AddExpr also represents subtraction, just
    /// add negative number
    AddExpr(Vec<MathExpr>),
    MulExpr(Vec<MathExpr>),
    DivExpr(Box<MathExpr>, Box<MathExpr>),
    PowExpr(Box<MathExpr>, Box<MathExpr>),
    Const(Decimal),
}

impl MathExpr {
    /// Evaluates given expression and returns `Some` of value
    /// or `None`, if expression is invalid (such as division by zero)
    pub fn eval(&self) -> Option<Decimal> {
        match self {
            MathExpr::AddExpr(terms) => terms
                .iter()
                .map(|term| term.eval())
                .reduce(|a, b| {
                    if let (Some(a), Some(b)) = (a, b) {
                        a.checked_add(b)
                    } else {
                        None
                    }
                })
                .flatten(),

            MathExpr::MulExpr(terms) => terms
                .iter()
                .map(|term| term.eval())
                .reduce(|a, b| {
                    if let (Some(a), Some(b)) = (a, b) {
                        a.checked_mul(b)
                    } else {
                        None
                    }
                })
                .flatten(),

            MathExpr::DivExpr(a, b) => {
                let (a, b) = (a.eval(), b.eval());
                if let (Some(a), Some(b)) = (a, b) {
                    a.checked_div(b)
                } else {
                    None
                }
            }

            MathExpr::PowExpr(a, b) => {
                let (a, b) = (a.eval(), b.eval());
                if let (Some(a), Some(b)) = (a, b) {
                    // We need to make sure the operation make sense
                    // (no fractional exponents on negatice numbers).
                    if a.is_sign_negative() && b != b.round() {
                        None
                    } else {
                        a.checked_powd(b)
                    }
                } else {
                    None
                }
            }

            MathExpr::Const(a) => Some(*a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use MathExpr::*;

    fn const_val(value: &str) -> MathExpr {
        Const(Decimal::from_str(value).unwrap())
    }

    fn box_const(value: &str) -> Box<MathExpr> {
        Box::new(Const(Decimal::from_str(value).unwrap()))
    }

    #[test]
    fn one_plus_one() {
        let a = const_val("1");
        let b = const_val("1");
        let c = AddExpr(vec![a, b]);
        assert_eq!(c.eval(), Some(dec!(2)));
    }

    #[test]
    fn addition_subtraction() {
        // (14 + (-77004 - 42000 + 78.58) - 0.666) == -118912.086
        let inner = AddExpr(vec![
            const_val("-77_004"),
            const_val("-42_000"),
            const_val("78.58"),
        ]);
        let expr = AddExpr(vec![const_val("14"), inner, const_val("-0.666")]);
        assert_eq!(expr.eval(), Some(dec!(-118912.086)));
    }

    #[test]
    fn multiplication() {
        // (-4.35 * (0.44 * 80) * -78.5 * -0.5) == 6009.96
        let inner = MulExpr(vec![const_val("0.44"), const_val("80")]);
        let expr = MulExpr(vec![
            const_val("4.35"),
            inner,
            const_val("-78.5"),
            const_val("-0.5"),
        ]);
        assert_eq!(expr.eval(), Some(dec!(6009.96)));
    }

    #[test]
    fn division() {
        // -4466.7 / ( 97 / -7.3) == 336.1537 (4 decimal places)
        let divisor = DivExpr(box_const("97"), box_const("-7.3"));
        let expr = DivExpr(box_const("-4466.7"), Box::new(divisor));
        let value = expr.eval().unwrap();
        assert_eq!(format!("{value:.4}"), "336.1537");
    }

    #[test]
    fn division_by_zero() {
        // -4466.7 / ( 0 / -7.3) == None
        let divisor = DivExpr(box_const("0"), box_const("-7.3"));
        assert_eq!(divisor.eval(), Some(dec!(0)));
        let expr = DivExpr(box_const("-4466.7"), Box::new(divisor));
        assert_eq!(expr.eval(), None);
    }

    #[test]
    fn exponentiation_integers() {
        // 4 ** (3 ** 2) == 262144
        let exp = PowExpr(box_const("3"), box_const("2"));
        let expr = PowExpr(box_const("4"), Box::new(exp));
        assert_eq!(expr.eval(), Some(dec!(262144)));
    }

    #[test]
    fn exponentiation_decimals() {
        // 3.5 ** (7.8 ** -1.4) == 1.07317 (5 decimal places)
        let exp = PowExpr(box_const("7.8"), box_const("-1.4"));
        let expr = PowExpr(box_const("3.5"), Box::new(exp));
        let value = expr.eval().unwrap();
        assert_eq!(format!("{value:.5}"), "1.07317");
    }

    #[test]
    fn exponentiation_invalid() {
        // -1 ** 0.5 == None
        let expr = PowExpr(box_const("-1"), box_const("0.5"));
        assert_eq!(expr.eval(), None);
    }

    #[test]
    fn large_expression() {
        // -1 * (-998 + 45 ** (44.7 - 44.125)) /
        // (3.2 * 63.004 * (78.44 ** -8 - 34.996)) == 0.142712 (6 decimal digits)
        let first_subexpr = PowExpr(
            box_const("45"),
            Box::new(AddExpr(vec![const_val("44.7"), const_val("-44.125")])),
        );
        let first_expr = MulExpr(vec![
            const_val("-1"),
            AddExpr(vec![const_val("998"), first_subexpr]),
        ]);

        let second_subexpr = AddExpr(vec![
            const_val("-34.996"),
            PowExpr(box_const("78.44"), box_const("-8")),
        ]);
        let second_expr = MulExpr(vec![const_val("3.2"), const_val("63.004"), second_subexpr]);

        let value = DivExpr(Box::new(first_expr), Box::new(second_expr))
            .eval()
            .unwrap();
        assert_eq!(format!("{value:.6}"), "0.142712");
    }
}
