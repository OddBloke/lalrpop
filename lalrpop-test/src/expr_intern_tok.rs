#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Expr<
        'input,
    >(
        scale: i32,
        input: &'input str,
    ) -> Result<i32, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(scale, input, &mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Expr((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        Expr((usize, i32, usize)),
        Factor((usize, i32, usize)),
        Num((usize, i32, usize)),
        Term((usize, i32, usize)),
        ____Expr((usize, i32, usize)),
    }

    // State 0
    //     Kind = None
    //     AllInputs = []
    //     OptionalInputs = []
    //     FixedInputs = []
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = (*) Expr "+" Factor [EOF]
    //     Expr = (*) Expr "+" Factor ["+"]
    //     Expr = (*) Expr "+" Factor ["-"]
    //     Expr = (*) Expr "-" Factor [EOF]
    //     Expr = (*) Expr "-" Factor ["+"]
    //     Expr = (*) Expr "-" Factor ["-"]
    //     Expr = (*) Factor [EOF]
    //     Expr = (*) Factor ["+"]
    //     Expr = (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [EOF]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [EOF]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [EOF]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [EOF]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     __Expr = (*) Expr [EOF]
    //
    //     "(" -> Shift(S5)
    //     r#"[0-9]+"# -> Shift(S6)
    //
    //     Expr -> S1
    //     Factor -> S2
    //     Num -> S3
    //     Term -> S4
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym0 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, input, __tokens, __sym0));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym0 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__sym0) => {
                    __result = try!(__state1(scale, input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Factor(__sym0) => {
                    __result = try!(__state2(scale, input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Num(__sym0) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__sym0) => {
                    __result = try!(__custom1(scale, input, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //     Kind = None
    //     AllInputs = [Expr]
    //     OptionalInputs = []
    //     FixedInputs = [Expr]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr (*) "+" Factor [EOF]
    //     Expr = Expr (*) "+" Factor ["+"]
    //     Expr = Expr (*) "+" Factor ["-"]
    //     Expr = Expr (*) "-" Factor [EOF]
    //     Expr = Expr (*) "-" Factor ["+"]
    //     Expr = Expr (*) "-" Factor ["-"]
    //     __Expr = Expr (*) [EOF]
    //
    //     EOF -> Reduce(__Expr = Expr => ActionFn(0);)
    //     "+" -> Shift(S7)
    //     "-" -> Shift(S8)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (3, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state7(scale, input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state8(scale, input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            None => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(scale, input, __sym0);
                let __nt = __Nonterminal::____Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //     Kind = None
    //     AllInputs = [Factor]
    //     OptionalInputs = []
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Factor (*) [EOF]
    //     Expr = Factor (*) ["+"]
    //     Expr = Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [EOF]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [EOF]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     EOF -> Reduce(Expr = Factor => ActionFn(3);)
    //     "*" -> Shift(S9)
    //     "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "/" -> Shift(S10)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state9(scale, input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state10(scale, input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(scale, input, __sym0);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //     Kind = None
    //     AllInputs = ["("]
    //     OptionalInputs = []
    //     FixedInputs = ["("]
    //     WillPushLen = 2
    //     WillPush = [Expr, ")"]
    //     WillProduce = Some(Term)
    //
    //     Expr = (*) Expr "+" Factor [")"]
    //     Expr = (*) Expr "+" Factor ["+"]
    //     Expr = (*) Expr "+" Factor ["-"]
    //     Expr = (*) Expr "-" Factor [")"]
    //     Expr = (*) Expr "-" Factor ["+"]
    //     Expr = (*) Expr "-" Factor ["-"]
    //     Expr = (*) Factor [")"]
    //     Expr = (*) Factor ["+"]
    //     Expr = (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [")"]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [")"]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [")"]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [")"]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = "(" (*) Expr ")" [EOF]
    //     Term = "(" (*) Expr ")" ["*"]
    //     Term = "(" (*) Expr ")" ["+"]
    //     Term = "(" (*) Expr ")" ["-"]
    //     Term = "(" (*) Expr ")" ["/"]
    //
    //     "(" -> Shift(S15)
    //     r#"[0-9]+"# -> Shift(S16)
    //
    //     Expr -> S11
    //     Factor -> S12
    //     Num -> S13
    //     Term -> S14
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state15(scale, input, __tokens, __sym1));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym0.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__sym1) => {
                    __result = try!(__state11(scale, input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__sym1) => {
                    __result = try!(__state12(scale, input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__sym1) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__sym1) => {
                    __result = try!(__custom1(scale, input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 7
    //     Kind = None
    //     AllInputs = [Expr, "+"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "+"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "+" (*) Factor [EOF]
    //     Expr = Expr "+" (*) Factor ["+"]
    //     Expr = Expr "+" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [EOF]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [EOF]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [EOF]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [EOF]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //
    //     "(" -> Shift(S5)
    //     r#"[0-9]+"# -> Shift(S6)
    //
    //     Factor -> S17
    //     Num -> S3
    //     Term -> S4
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state17(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__sym2) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom1(scale, input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 8
    //     Kind = None
    //     AllInputs = [Expr, "-"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "-"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "-" (*) Factor [EOF]
    //     Expr = Expr "-" (*) Factor ["+"]
    //     Expr = Expr "-" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [EOF]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [EOF]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [EOF]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [EOF]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //
    //     "(" -> Shift(S5)
    //     r#"[0-9]+"# -> Shift(S6)
    //
    //     Factor -> S18
    //     Num -> S3
    //     Term -> S4
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state18(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__sym2) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom1(scale, input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 9
    //     Kind = None
    //     AllInputs = [Factor, "*"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "*"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "*" (*) Term [EOF]
    //     Factor = Factor "*" (*) Term ["*"]
    //     Factor = Factor "*" (*) Term ["+"]
    //     Factor = Factor "*" (*) Term ["-"]
    //     Factor = Factor "*" (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [EOF]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //
    //     "(" -> Shift(S5)
    //     r#"[0-9]+"# -> Shift(S6)
    //
    //     Num -> S3
    //     Term -> S19
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__sym2) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom3(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 10
    //     Kind = None
    //     AllInputs = [Factor, "/"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "/"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "/" (*) Term [EOF]
    //     Factor = Factor "/" (*) Term ["*"]
    //     Factor = Factor "/" (*) Term ["+"]
    //     Factor = Factor "/" (*) Term ["-"]
    //     Factor = Factor "/" (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [EOF]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [EOF]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [EOF]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //
    //     "(" -> Shift(S5)
    //     r#"[0-9]+"# -> Shift(S6)
    //
    //     Num -> S3
    //     Term -> S20
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state5(scale, input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__sym2) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom4(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 11
    //     Kind = None
    //     AllInputs = ["(", Expr]
    //     OptionalInputs = ["("]
    //     FixedInputs = [Expr]
    //     WillPushLen = 1
    //     WillPush = [")"]
    //     WillProduce = None
    //
    //     Expr = Expr (*) "+" Factor [")"]
    //     Expr = Expr (*) "+" Factor ["+"]
    //     Expr = Expr (*) "+" Factor ["-"]
    //     Expr = Expr (*) "-" Factor [")"]
    //     Expr = Expr (*) "-" Factor ["+"]
    //     Expr = Expr (*) "-" Factor ["-"]
    //     Term = "(" Expr (*) ")" [EOF]
    //     Term = "(" Expr (*) ")" ["*"]
    //     Term = "(" Expr (*) ")" ["+"]
    //     Term = "(" Expr (*) ")" ["-"]
    //     Term = "(" Expr (*) ")" ["/"]
    //
    //     ")" -> Shift(S21)
    //     "+" -> Shift(S22)
    //     "-" -> Shift(S23)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                let __sym0 = __sym0.take().unwrap();
                __result = try!(__custom5(scale, input, __tokens, __sym0, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state22(scale, input, __tokens, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state23(scale, input, __tokens, __sym1, __sym2));
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //     Kind = None
    //     AllInputs = [Factor]
    //     OptionalInputs = []
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Factor (*) [")"]
    //     Expr = Factor (*) ["+"]
    //     Expr = Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [")"]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [")"]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     ")" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "*" -> Shift(S24)
    //     "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //     "/" -> Shift(S25)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state24(scale, input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state25(scale, input, __tokens, __sym0, __sym1));
                return Ok(__result);
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(scale, input, __sym0);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //     Kind = None
    //     AllInputs = ["("]
    //     OptionalInputs = []
    //     FixedInputs = ["("]
    //     WillPushLen = 2
    //     WillPush = [Expr, ")"]
    //     WillProduce = Some(Term)
    //
    //     Expr = (*) Expr "+" Factor [")"]
    //     Expr = (*) Expr "+" Factor ["+"]
    //     Expr = (*) Expr "+" Factor ["-"]
    //     Expr = (*) Expr "-" Factor [")"]
    //     Expr = (*) Expr "-" Factor ["+"]
    //     Expr = (*) Expr "-" Factor ["-"]
    //     Expr = (*) Factor [")"]
    //     Expr = (*) Factor ["+"]
    //     Expr = (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [")"]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [")"]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [")"]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [")"]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //     Term = "(" (*) Expr ")" [")"]
    //     Term = "(" (*) Expr ")" ["*"]
    //     Term = "(" (*) Expr ")" ["+"]
    //     Term = "(" (*) Expr ")" ["-"]
    //     Term = "(" (*) Expr ")" ["/"]
    //
    //     "(" -> Shift(S15)
    //     r#"[0-9]+"# -> Shift(S16)
    //
    //     Expr -> S26
    //     Factor -> S12
    //     Num -> S13
    //     Term -> S14
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__state15(scale, input, __tokens, __sym1));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym1 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym0.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__sym1) => {
                    __result = try!(__state26(scale, input, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__sym1) => {
                    __result = try!(__state12(scale, input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Num(__sym1) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__sym1) => {
                    __result = try!(__custom1(scale, input, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 17
    //     Kind = None
    //     AllInputs = [Expr, "+", Factor]
    //     OptionalInputs = [Expr, "+"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "+" Factor (*) [EOF]
    //     Expr = Expr "+" Factor (*) ["+"]
    //     Expr = Expr "+" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [EOF]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [EOF]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     EOF -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "*" -> Shift(S9)
    //     "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "/" -> Shift(S10)
    //
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, i32, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state9(scale, input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state10(scale, input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(scale, input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 18
    //     Kind = None
    //     AllInputs = [Expr, "-", Factor]
    //     OptionalInputs = [Expr, "-"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "-" Factor (*) [EOF]
    //     Expr = Expr "-" Factor (*) ["+"]
    //     Expr = Expr "-" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [EOF]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [EOF]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     EOF -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "*" -> Shift(S9)
    //     "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "/" -> Shift(S10)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, i32, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state9(scale, input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state10(scale, input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(scale, input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //     Kind = None
    //     AllInputs = [Expr, "+"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "+"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "+" (*) Factor [")"]
    //     Expr = Expr "+" (*) Factor ["+"]
    //     Expr = Expr "+" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [")"]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [")"]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [")"]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [")"]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //
    //     "(" -> Shift(S15)
    //     r#"[0-9]+"# -> Shift(S16)
    //
    //     Factor -> S27
    //     Num -> S13
    //     Term -> S14
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state15(scale, input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state27(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__sym2) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom1(scale, input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 23
    //     Kind = None
    //     AllInputs = [Expr, "-"]
    //     OptionalInputs = []
    //     FixedInputs = [Expr, "-"]
    //     WillPushLen = 1
    //     WillPush = [Factor]
    //     WillProduce = Some(Expr)
    //
    //     Expr = Expr "-" (*) Factor [")"]
    //     Expr = Expr "-" (*) Factor ["+"]
    //     Expr = Expr "-" (*) Factor ["-"]
    //     Factor = (*) Factor "*" Term [")"]
    //     Factor = (*) Factor "*" Term ["*"]
    //     Factor = (*) Factor "*" Term ["+"]
    //     Factor = (*) Factor "*" Term ["-"]
    //     Factor = (*) Factor "*" Term ["/"]
    //     Factor = (*) Factor "/" Term [")"]
    //     Factor = (*) Factor "/" Term ["*"]
    //     Factor = (*) Factor "/" Term ["+"]
    //     Factor = (*) Factor "/" Term ["-"]
    //     Factor = (*) Factor "/" Term ["/"]
    //     Factor = (*) Term [")"]
    //     Factor = (*) Term ["*"]
    //     Factor = (*) Term ["+"]
    //     Factor = (*) Term ["-"]
    //     Factor = (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [")"]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //
    //     "(" -> Shift(S15)
    //     r#"[0-9]+"# -> Shift(S16)
    //
    //     Factor -> S28
    //     Num -> S13
    //     Term -> S14
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __sym0 = &mut Some(__sym0);
        let __sym1 = &mut Some(__sym1);
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state15(scale, input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            if __sym1.is_none() {
                return Ok(__result);
            }
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__sym2) => {
                    __result = try!(__state28(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Num(__sym2) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom1(scale, input, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 24
    //     Kind = None
    //     AllInputs = [Factor, "*"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "*"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "*" (*) Term [")"]
    //     Factor = Factor "*" (*) Term ["*"]
    //     Factor = Factor "*" (*) Term ["+"]
    //     Factor = Factor "*" (*) Term ["-"]
    //     Factor = Factor "*" (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [")"]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //
    //     "(" -> Shift(S15)
    //     r#"[0-9]+"# -> Shift(S16)
    //
    //     Num -> S13
    //     Term -> S29
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state15(scale, input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__sym2) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom3(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 25
    //     Kind = None
    //     AllInputs = [Factor, "/"]
    //     OptionalInputs = []
    //     FixedInputs = [Factor, "/"]
    //     WillPushLen = 1
    //     WillPush = [Term]
    //     WillProduce = Some(Factor)
    //
    //     Factor = Factor "/" (*) Term [")"]
    //     Factor = Factor "/" (*) Term ["*"]
    //     Factor = Factor "/" (*) Term ["+"]
    //     Factor = Factor "/" (*) Term ["-"]
    //     Factor = Factor "/" (*) Term ["/"]
    //     Num = (*) r#"[0-9]+"# [")"]
    //     Num = (*) r#"[0-9]+"# ["*"]
    //     Num = (*) r#"[0-9]+"# ["+"]
    //     Num = (*) r#"[0-9]+"# ["-"]
    //     Num = (*) r#"[0-9]+"# ["/"]
    //     Term = (*) Num [")"]
    //     Term = (*) Num ["*"]
    //     Term = (*) Num ["+"]
    //     Term = (*) Num ["-"]
    //     Term = (*) Num ["/"]
    //     Term = (*) "(" Expr ")" [")"]
    //     Term = (*) "(" Expr ")" ["*"]
    //     Term = (*) "(" Expr ")" ["+"]
    //     Term = (*) "(" Expr ")" ["-"]
    //     Term = (*) "(" Expr ")" ["/"]
    //
    //     "(" -> Shift(S15)
    //     r#"[0-9]+"# -> Shift(S16)
    //
    //     Num -> S13
    //     Term -> S30
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((__loc1, (0, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state15(scale, input, __tokens, __sym2));
            }
            Some((__loc1, (6, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__custom2(scale, input, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Num(__sym2) => {
                    __result = try!(__custom0(scale, input, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__sym2) => {
                    __result = try!(__custom4(scale, input, __tokens, __lookahead, __sym0, __sym1, __sym2));
                    return Ok(__result);
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    // State 26
    //     Kind = None
    //     AllInputs = ["(", Expr]
    //     OptionalInputs = ["("]
    //     FixedInputs = [Expr]
    //     WillPushLen = 1
    //     WillPush = [")"]
    //     WillProduce = None
    //
    //     Expr = Expr (*) "+" Factor [")"]
    //     Expr = Expr (*) "+" Factor ["+"]
    //     Expr = Expr (*) "+" Factor ["-"]
    //     Expr = Expr (*) "-" Factor [")"]
    //     Expr = Expr (*) "-" Factor ["+"]
    //     Expr = Expr (*) "-" Factor ["-"]
    //     Term = "(" Expr (*) ")" [")"]
    //     Term = "(" Expr (*) ")" ["*"]
    //     Term = "(" Expr (*) ")" ["+"]
    //     Term = "(" Expr (*) ")" ["-"]
    //     Term = "(" Expr (*) ")" ["/"]
    //
    //     ")" -> Shift(S31)
    //     "+" -> Shift(S22)
    //     "-" -> Shift(S23)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
        __sym1: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (1, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                let __sym0 = __sym0.take().unwrap();
                __result = try!(__custom5(scale, input, __tokens, __sym0, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, (3, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state22(scale, input, __tokens, __sym1, __sym2));
                return Ok(__result);
            }
            Some((__loc1, (4, __tok0), __loc2)) => {
                let __sym2 = (__loc1, (__tok0), __loc2);
                __result = try!(__state23(scale, input, __tokens, __sym1, __sym2));
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 27
    //     Kind = None
    //     AllInputs = [Expr, "+", Factor]
    //     OptionalInputs = [Expr, "+"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "+" Factor (*) [")"]
    //     Expr = Expr "+" Factor (*) ["+"]
    //     Expr = Expr "+" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [")"]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [")"]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     ")" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "*" -> Shift(S24)
    //     "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(2);)
    //     "/" -> Shift(S25)
    //
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, i32, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state24(scale, input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state25(scale, input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2(scale, input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 28
    //     Kind = None
    //     AllInputs = [Expr, "-", Factor]
    //     OptionalInputs = [Expr, "-"]
    //     FixedInputs = [Factor]
    //     WillPushLen = 0
    //     WillPush = []
    //     WillProduce = None
    //
    //     Expr = Expr "-" Factor (*) [")"]
    //     Expr = Expr "-" Factor (*) ["+"]
    //     Expr = Expr "-" Factor (*) ["-"]
    //     Factor = Factor (*) "*" Term [")"]
    //     Factor = Factor (*) "*" Term ["*"]
    //     Factor = Factor (*) "*" Term ["+"]
    //     Factor = Factor (*) "*" Term ["-"]
    //     Factor = Factor (*) "*" Term ["/"]
    //     Factor = Factor (*) "/" Term [")"]
    //     Factor = Factor (*) "/" Term ["*"]
    //     Factor = Factor (*) "/" Term ["+"]
    //     Factor = Factor (*) "/" Term ["-"]
    //     Factor = Factor (*) "/" Term ["/"]
    //
    //     ")" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "*" -> Shift(S24)
    //     "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(1);)
    //     "/" -> Shift(S25)
    //
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<(usize, i32, usize)>,
        __sym1: &mut Option<(usize, &'input str, usize)>,
        __sym2: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, (2, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state24(scale, input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((__loc1, (5, __tok0), __loc2)) => {
                let __sym3 = (__loc1, (__tok0), __loc2);
                __result = try!(__state25(scale, input, __tokens, __sym2, __sym3));
                return Ok(__result);
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1(scale, input, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Expr((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
                return Ok(__result);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // Custom 0
    //    Reduce Term = Num => ActionFn(7);
    pub fn __custom0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7(scale, input, __sym0);
        let __nt = __Nonterminal::Term((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 1
    //    Reduce Factor = Term => ActionFn(6);
    pub fn __custom1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6(scale, input, __sym0);
        let __nt = __Nonterminal::Factor((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 2
    //    Reduce Num = r#"[0-9]+"# => ActionFn(9);
    pub fn __custom2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9(scale, input, __sym0);
        let __nt = __Nonterminal::Num((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 3
    //    Reduce Factor = Factor, "*", Term => ActionFn(4);
    pub fn __custom3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
        __sym2: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4(scale, input, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::Factor((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 4
    //    Reduce Factor = Factor, "/", Term => ActionFn(5);
    pub fn __custom4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: (usize, i32, usize),
        __sym1: (usize, &'input str, usize),
        __sym2: (usize, i32, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5(scale, input, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::Factor((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }

    // Custom 5
    //    Reduce Term = "(", Expr, ")" => ActionFn(8);
    pub fn __custom5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        scale: i32,
        input: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: (usize, &'input str, usize),
        __sym1: (usize, i32, usize),
        __sym2: (usize, &'input str, usize),
    ) -> Result<(Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8(scale, input, __sym0, __sym1, __sym2);
        let __nt = __Nonterminal::Term((
            __start,
            __nt,
            __end,
        ));
        __result = (__lookahead, __nt);
        return Ok(__result);
    }
}
pub use self::__parse__Expr::parse_Expr;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        42 => /* '*' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        43 => /* '+' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        47 => /* '/' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((6, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    (__0)
}

pub fn __action1<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l - r
}

pub fn __action2<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l + r
}

pub fn __action3<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    (__0)
}

pub fn __action4<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l * r
}

pub fn __action5<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l / r
}

pub fn __action6<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    (__0)
}

pub fn __action7<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, n, _): (usize, i32, usize),
) -> i32
{
    n * scale
}

pub fn __action8<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
) -> i32
{
    (__0)
}

pub fn __action9<
    'input,
>(
    scale: i32,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
