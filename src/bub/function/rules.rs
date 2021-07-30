use crate::bub::function::variable::{FunctionVariable, FunctionVariable::*};

use mpl::choices::{First, Second};
use mpl::rules::{RightRule, Rules};
use mpl::symbols::{Metasymbol::*, TerminalSymbol, U8SliceTerminal, U8SliceTerminal::*, E};

pub struct FunctionRules;

type Rule<'a> = RightRule<U8SliceTerminal<'a>, FunctionVariable>;

impl<'a> FunctionRules {
    // BubbleFunctions
    /// BubbleFunctions = BubbleFunction ZeroOrMoreBubbleFunctions / f
    const BUBBLE_FUNCTIONS_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(BubbleFunction),
            rhs: E::V(ZeroOrMoreBubbleFunctions),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// ZeroOrMoreBubbleFunctions = SpaceAndBubbleFunction ZeroOrMoreBubbleFunctions / ()
    const ZERO_OR_MORE_BUBBLE_FUNCTIONS_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(SpaceAndBubbleFunction),
            rhs: E::V(ZeroOrMoreBubbleFunctions),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Empty))),
    };
    /// SpaceAndBubbleFunction = Space BubbleFunction / f
    const SPACE_AND_BUBBLE_FUNCTION_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Space),
            rhs: E::V(BubbleFunction),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // BubbleFunction
    /// BubbleFunction = SumAndSpace BubbleFunction1 / f
    const BUBBLE_FUNCTION_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(SumAndSpace),
            rhs: E::V(BubbleFunction1),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// BubbleFunction1 = SumAndSpace BubbleFunction2 / f
    const BUBBLE_FUNCTION1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(SumAndSpace),
            rhs: E::V(BubbleFunction2),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// BubbleFunction2 = SumAndSpace BubbleFunction3 / f
    const BUBBLE_FUNCTION2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(SumAndSpace),
            rhs: E::V(BubbleFunction3),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// BubbleFunction3 = OrOrExpressionAndSpace BubbleFunction4 / f
    const BUBBLE_FUNCTION3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(OrOrExpressionAndSpace),
            rhs: E::V(BubbleFunction4),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// BubbleFunction4 = Sum () / f
    const BUBBLE_FUNCTION4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Sum),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    /// SumAndSpace = Sum Space / f
    const SUM_AND_SPACE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Sum),
            rhs: E::V(Space),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// OrOrExpressionAndSpace = OrOrExpression Space / f
    const OR_OR_EXPRESSION_AND_SPACE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(OrOrExpression),
            rhs: E::V(Space),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // OrOr Expression
    /// OrOrExpression = AndAndExpression OrOrExpression1 / AndAndExpression
    const OR_OR_EXPRESSION_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(AndAndExpression),
            rhs: E::V(OrOrExpression1),
        },
        second: Second(E::V(AndAndExpression)),
    };
    /// OrOrExpression1 = OrOr OrOrExpression / f
    const OR_OR_EXPRESSION1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(OrOr),
            rhs: E::V(OrOrExpression),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// OrOr = "||" () / f
    const OR_OR_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("||"))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // AndAnd Expression
    /// AndAndExpression = ComparisonExpression AndAndExpression1 / ComparisonExpression
    const AND_AND_EXPRESSION_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(ComparisonExpression),
            rhs: E::V(AndAndExpression1),
        },
        second: Second(E::V(ComparisonExpression)),
    };
    /// AndAndExpression1 = AndAnd AndAndExpression / f
    const AND_AND_EXPRESSION1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(AndAnd),
            rhs: E::V(AndAndExpression),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// AndAnd = "&&" () / f
    const AND_AND_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("&&"))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Comparison Expression
    /// ComparisonExpression = Sum ComparisonExpression1 / f
    const COMPARISON_EXPRESSION_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Sum),
            rhs: E::V(ComparisonExpression1),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// ComparisonExpression1 = Comparison Sum / f
    const COMPARISON_EXPRESSION1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Comparison),
            rhs: E::V(Sum),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Comparison
    /// Comparison = EqEq () / Comparison1
    const COMPARISON_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(EqEq),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Comparison1)),
    };
    /// Comparison1 = Ne () / Comparison2
    const COMPARISON1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Ne),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Comparison2)),
    };
    /// Comparison2 = Ge () / Comparison3
    const COMPARISON2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Ge),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Comparison3)),
    };
    /// Comparison3 = Le () / Comparison4
    const COMPARISON3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Le),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Comparison4)),
    };
    /// Comparison4 = Gt () / Comparison5
    const COMPARISON4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Gt),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Comparison5)),
    };
    /// Comparison5 = Lt () / f
    const COMPARISON5_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Lt),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    /// EqEq = "==" () / f
    const EQ_EQ_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("=="))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Ne = "!=" () / f
    const NE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("!="))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Ge = ">=" () / f
    const GE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str(">="))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Le = "<=" () / f
    const LE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("<="))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Gt = '>' () / f
    const GT_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('>'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Lt = '<' () / f
    const LT_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('<'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Sum
    /// Sum = Term ZeroOrMorePlusOrMinusAndTerms / f
    const SUM_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Term),
            rhs: E::V(ZeroOrMorePlusOrMinusAndTerms),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// ZeroOrMorePlusOrMinusAndTerms = PlusOrMinusAndTerm ZeroOrMorePlusOrMinusAndTerms / ()
    const ZERO_OR_MORE_PLUS_OR_MINUS_AND_TERM_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(PlusOrMinusAndTerm),
            rhs: E::V(ZeroOrMorePlusOrMinusAndTerms),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Empty))),
    };
    /// PlusOrMinusAndTerm = PlusOrMinus Term / f
    const PLUS_OR_MINUS_AND_TERM_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(PlusOrMinus),
            rhs: E::V(Term),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Term
    /// Term = Factor ZeroOrMoreStarOrSlashAndFactors / f
    const TERM_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Factor),
            rhs: E::V(ZeroOrMoreStarOrSlashAndFactors),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// ZeroOrMoreStarOrSlashAndFactors = StarOrSlashAndFactor ZeroOrMoreStarOrSlashAndFactors / ()
    const ZERO_OR_MORE_STAR_OR_SLASH_AND_FACTOR_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(StarOrSlashAndFactor),
            rhs: E::V(ZeroOrMoreStarOrSlashAndFactors),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Empty))),
    };
    /// StarOrSlashAndFactor = StarOrSlash Factor / f
    const STAR_OR_SLASH_AND_FACTOR_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(StarOrSlash),
            rhs: E::V(Factor),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Factor
    /// Factor = PlusOrMinus Factor / Power
    const FACTOR_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(PlusOrMinus),
            rhs: E::V(Factor),
        },
        second: Second(E::V(Power)),
    };

    // Power
    /// Power = Atom PowerAndFactor / Atom
    const POWER_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Atom),
            rhs: E::V(PowerAndFactor),
        },
        second: Second(E::V(Atom)),
    };
    /// PowerAndFactor = '^' Factor / f
    const POWER_AND_FACTOR_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('^'))),
            rhs: E::V(Factor),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Atom
    /// Atom = ExpressionInParentheses () / Atom1
    const ATOM_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(ExpressionInParentheses),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Atom1)),
    };
    /// Atom1 = FloatLiteral () / Atom2
    const ATOM1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(FloatLiteral),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Atom2)),
    };
    /// Atom2 = IntegerLiteral () / Atom3
    const ATOM2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(IntegerLiteral),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Atom3)),
    };
    /// Atom3 = Function () / Atom4
    const ATOM3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Function),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Atom4)),
    };
    /// Atom4 = Variable () / Atom5
    const ATOM4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Variable),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Atom5)),
    };
    /// Atom5 = Constant () / f
    const ATOM5_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Constant),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Variable
    /// Variable = UppercaseX () / Variable1
    const VARIABLE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(UppercaseX),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable1)),
    };
    /// Variable1 = UppercaseY () / Variable2
    const VARIABLE1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(UppercaseY),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable2)),
    };
    /// Variable2 = UppercaseZ () / Variable3
    const VARIABLE2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(UppercaseZ),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable3)),
    };
    /// Variable3 = LowercaseX () / Variable4
    const VARIABLE3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(LowercaseX),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable4)),
    };
    /// Variable4 = LowercaseY () / Variable5
    const VARIABLE4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(LowercaseY),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable5)),
    };
    /// Variable5 = LowercaseZ () / Variable6
    const VARIABLE5_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(LowercaseZ),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable6)),
    };
    /// Variable6 = UppercaseN () / Variable7
    const VARIABLE6_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(UppercaseN),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable7)),
    };
    /// Variable7 = LowercaseN () / Variable8
    const VARIABLE7_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(LowercaseN),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable8)),
    };
    /// Variable8 = LowercaseN () / Variable9
    const VARIABLE8_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(UppercaseF),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Variable9)),
    };
    /// Variable9 = UppercaseS () / f
    const VARIABLE9_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(UppercaseS),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    /// UppercaseX = 'X' () / f
    const UPPERCASE_X_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('X'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// UppercaseY = 'Y' () / f
    const UPPERCASE_Y_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('Y'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// UppercaseZ = 'Z' () / f
    const UPPERCASE_Z_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('Z'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// LowercaseX = 'x' () / f
    const LOWERCASE_X_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('x'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// LowercaseY = 'y' () / f
    const LOWERCASE_Y_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('y'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// LowercaseZ = 'z' () / f
    const LOWERCASE_Z_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('z'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// UppercaseN = 'N' () / f
    const UPPERCASE_N_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('N'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// LowercaseN = 'n' () / f
    const LOWERCASE_N_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('n'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// UppercaseF = 'F' () / f
    const UPPERCASE_F_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('F'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// UppercaseS = 'S' () / f
    const UPPERCASE_S_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('S'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Constant
    /// Constant = E () / Constant1
    const CONSTANT_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(E),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Constant1)),
    };
    /// Constant1 = Pi () / f
    const CONSTANT1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Pi),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// E = 'E' () / f
    const E_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('E'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Pi = "PI" () / f
    const PI_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("PI"))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Function
    const FUNCTION_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Sine),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Function1)),
    };
    const FUNCTION1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Cosine),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Function2)),
    };
    const FUNCTION2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Tangent),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Function3)),
    };
    const FUNCTION3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Ln),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(Function4)),
    };
    const FUNCTION4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Lg),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// `Sine = "sin" Factor / f`
    const SINE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("sin"))),
            rhs: E::V(Factor),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Cosine = "cos" Factor / f
    const COSINE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("cos"))),
            rhs: E::V(Factor),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Tangent = "tan" Factor / f
    const TANGENT_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("tan"))),
            rhs: E::V(Factor),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Ln = "ln" Factor / f
    const LN_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("ln"))),
            rhs: E::V(Factor),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Lg = "lg" Factor / f
    const LG_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Str("lg"))),
            rhs: E::V(Factor),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Delimiters
    /// ExpressionInParentheses = '(' ExpressionAndClose / f
    const EXPRESSION_IN_PARENTHESES_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('('))),
            rhs: E::V(ExpressionAndClose),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// ExpressionAndClose = Expression ')' / f
    const EXPRESSION_AND_CLOSE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Sum),
            rhs: E::T(TerminalSymbol::Original(Char(')'))),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Integer
    /// IntegerLiteral = DecLiteral () / f
    const INTEGER_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(DecLiteral),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Float
    /// FloatLiteral = DecLiteral PointAndDecLiteral / BytesF64Literal
    const FLOAT_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(DecLiteral),
            rhs: E::V(PointAndDecLiteral),
        },
        second: Second(E::V(BytesF64Literal)),
    };
    /// PointAndDecLiteral = '.' DecLiteral / f
    const POINT_AND_DEC_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('.'))),
            rhs: E::V(DecLiteral),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// BytesF64Literal = 'b' ???????? / f
    const BYTES_F64_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('b'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Any(8))),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    // Dec
    /// DecLiteral = DecDigit ZeroOrMoreDecDigits / f
    const DEC_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(DecDigit),
            rhs: E::V(ZeroOrMoreDecDigits),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// ZeroOrMoreDecDigits = DecDigit ZeroOrMoreDecDigits / ()
    const ZERO_OR_DEC_LITERAL_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(DecDigit),
            rhs: E::V(ZeroOrMoreDecDigits),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Empty))),
    };

    /// DecDigit = '0' () / DecDigit1
    const DEC_DIGIT_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('0'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit1)),
    };
    /// DecDigit1 = '1' () / DecDigit2
    const DEC_DIGIT1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('1'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit2)),
    };
    /// DecDigit2 = '2' () / DecDigit3
    const DEC_DIGIT2_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('2'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit3)),
    };
    /// DecDigit3 = '3' () / DecDigit4
    const DEC_DIGIT3_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('3'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit4)),
    };
    /// DecDigit4 = '4' () / DecDigit5
    const DEC_DIGIT4_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('4'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit5)),
    };
    /// DecDigit5 = '5' () / DecDigit6
    const DEC_DIGIT5_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('5'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit6)),
    };
    /// DecDigit6 = '6' () / DecDigit7
    const DEC_DIGIT6_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('6'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit7)),
    };
    /// DecDigit7 = '7' () / DecDigit8
    const DEC_DIGIT7_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('7'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(DecDigit8)),
    };
    /// DecDigit8 = '8' () / '9'
    const DEC_DIGIT8_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('8'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Original(Char('9')))),
    };

    // Others
    /// PlusOrMinus = Plus () / PlusOrMinus1
    const PLUS_OR_MINUS_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Plus),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(PlusOrMinus1)),
    };
    /// PlusOrMinus1 = Minus () / f
    const PLUS_OR_MINUS1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Minus),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Plus = '+' () / f
    const PLUS_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('+'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Minus = '-' () / f
    const MINUS_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('-'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    /// StarOrSlash = Star () / StarOrSlash1
    const STAR_OR_SLASH_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Star),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::V(StarOrSlash1)),
    };
    /// StarOrSlash1 = Slash () / f
    const STAR_OR_SLASH1_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::V(Slash),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Star = '*' () / f
    const STAR_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('*'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
    /// Slash = '/' () / f
    const SLASH_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char('/'))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };

    /// Space = ' ' () / f
    const SPACE_RULE: Rule<'a> = RightRule {
        first: First {
            lhs: E::T(TerminalSymbol::Original(Char(' '))),
            rhs: E::T(TerminalSymbol::Metasymbol(Empty)),
        },
        second: Second(E::T(TerminalSymbol::Metasymbol(Failure))),
    };
}

impl<'a> Rules<U8SliceTerminal<'a>, FunctionVariable> for FunctionRules {
    fn get(&self, variable: &FunctionVariable) -> Option<&Rule<'a>> {
        Some(match variable {
            // BubbleFunctions
            BubbleFunctions => &Self::BUBBLE_FUNCTIONS_RULE,
            ZeroOrMoreBubbleFunctions => &Self::ZERO_OR_MORE_BUBBLE_FUNCTIONS_RULE,

            SpaceAndBubbleFunction => &Self::SPACE_AND_BUBBLE_FUNCTION_RULE,

            // BubbleFunction
            BubbleFunction => &Self::BUBBLE_FUNCTION_RULE,
            BubbleFunction1 => &Self::BUBBLE_FUNCTION1_RULE,
            BubbleFunction2 => &Self::BUBBLE_FUNCTION2_RULE,
            BubbleFunction3 => &Self::BUBBLE_FUNCTION3_RULE,
            BubbleFunction4 => &Self::BUBBLE_FUNCTION4_RULE,

            SumAndSpace => &Self::SUM_AND_SPACE_RULE,
            OrOrExpressionAndSpace => &Self::OR_OR_EXPRESSION_AND_SPACE_RULE,

            // OrOr Expression
            OrOrExpression => &Self::OR_OR_EXPRESSION_RULE,
            OrOrExpression1 => &Self::OR_OR_EXPRESSION1_RULE,

            OrOr => &Self::OR_OR_RULE,

            // AndAnd Expression
            AndAndExpression => &Self::AND_AND_EXPRESSION_RULE,
            AndAndExpression1 => &Self::AND_AND_EXPRESSION1_RULE,

            AndAnd => &Self::AND_AND_RULE,

            // Comparison Expression
            ComparisonExpression => &Self::COMPARISON_EXPRESSION_RULE,
            ComparisonExpression1 => &Self::COMPARISON_EXPRESSION1_RULE,

            Comparison => &Self::COMPARISON_RULE,
            Comparison1 => &Self::COMPARISON1_RULE,
            Comparison2 => &Self::COMPARISON2_RULE,
            Comparison3 => &Self::COMPARISON3_RULE,
            Comparison4 => &Self::COMPARISON4_RULE,
            Comparison5 => &Self::COMPARISON5_RULE,

            EqEq => &Self::EQ_EQ_RULE,
            Ne => &Self::NE_RULE,
            Ge => &Self::GE_RULE,
            Le => &Self::LE_RULE,
            Gt => &Self::GT_RULE,
            Lt => &Self::LT_RULE,

            // Sum
            Sum => &Self::SUM_RULE,
            ZeroOrMorePlusOrMinusAndTerms => &Self::ZERO_OR_MORE_PLUS_OR_MINUS_AND_TERM_RULE,
            PlusOrMinusAndTerm => &Self::PLUS_OR_MINUS_AND_TERM_RULE,

            // Term
            Term => &Self::TERM_RULE,
            ZeroOrMoreStarOrSlashAndFactors => &Self::ZERO_OR_MORE_STAR_OR_SLASH_AND_FACTOR_RULE,
            StarOrSlashAndFactor => &Self::STAR_OR_SLASH_AND_FACTOR_RULE,

            // Factor
            Factor => &Self::FACTOR_RULE,

            // Power
            Power => &Self::POWER_RULE,
            PowerAndFactor => &Self::POWER_AND_FACTOR_RULE,

            // Atom
            Atom => &Self::ATOM_RULE,
            Atom1 => &Self::ATOM1_RULE,
            Atom2 => &Self::ATOM2_RULE,
            Atom3 => &Self::ATOM3_RULE,
            Atom4 => &Self::ATOM4_RULE,
            Atom5 => &Self::ATOM5_RULE,

            // Variable
            Variable => &Self::VARIABLE_RULE,
            Variable1 => &Self::VARIABLE1_RULE,
            Variable2 => &Self::VARIABLE2_RULE,
            Variable3 => &Self::VARIABLE3_RULE,
            Variable4 => &Self::VARIABLE4_RULE,
            Variable5 => &Self::VARIABLE5_RULE,
            Variable6 => &Self::VARIABLE6_RULE,
            Variable7 => &Self::VARIABLE7_RULE,
            Variable8 => &Self::VARIABLE8_RULE,
            Variable9 => &Self::VARIABLE9_RULE,

            UppercaseX => &Self::UPPERCASE_X_RULE,
            UppercaseY => &Self::UPPERCASE_Y_RULE,
            UppercaseZ => &Self::UPPERCASE_Z_RULE,
            LowercaseX => &Self::LOWERCASE_X_RULE,
            LowercaseY => &Self::LOWERCASE_Y_RULE,
            LowercaseZ => &Self::LOWERCASE_Z_RULE,
            UppercaseN => &Self::UPPERCASE_N_RULE,
            LowercaseN => &Self::LOWERCASE_N_RULE,
            UppercaseF => &Self::UPPERCASE_F_RULE,
            UppercaseS => &Self::UPPERCASE_S_RULE,

            // Constant
            Constant => &Self::CONSTANT_RULE,
            Constant1 => &Self::CONSTANT1_RULE,

            E => &Self::E_RULE,
            Pi => &Self::PI_RULE,

            // Function
            Function => &Self::FUNCTION_RULE,
            Function1 => &Self::FUNCTION1_RULE,
            Function2 => &Self::FUNCTION2_RULE,
            Function3 => &Self::FUNCTION3_RULE,
            Function4 => &Self::FUNCTION4_RULE,

            Sine => &Self::SINE_RULE,
            Cosine => &Self::COSINE_RULE,
            Tangent => &Self::TANGENT_RULE,
            Ln => &Self::LN_RULE,
            Lg => &Self::LG_RULE,

            // Delimiters
            ExpressionInParentheses => &Self::EXPRESSION_IN_PARENTHESES_RULE,
            ExpressionAndClose => &Self::EXPRESSION_AND_CLOSE_RULE,

            // Integer
            IntegerLiteral => &Self::INTEGER_LITERAL_RULE,

            // Float
            FloatLiteral => &Self::FLOAT_LITERAL_RULE,
            PointAndDecLiteral => &Self::POINT_AND_DEC_LITERAL_RULE,

            BytesF64Literal => &Self::BYTES_F64_LITERAL_RULE,

            DecLiteral => &Self::DEC_LITERAL_RULE,
            ZeroOrMoreDecDigits => &Self::ZERO_OR_DEC_LITERAL_RULE,

            DecDigit => &Self::DEC_DIGIT_RULE,
            DecDigit1 => &Self::DEC_DIGIT1_RULE,
            DecDigit2 => &Self::DEC_DIGIT2_RULE,
            DecDigit3 => &Self::DEC_DIGIT3_RULE,
            DecDigit4 => &Self::DEC_DIGIT4_RULE,
            DecDigit5 => &Self::DEC_DIGIT5_RULE,
            DecDigit6 => &Self::DEC_DIGIT6_RULE,
            DecDigit7 => &Self::DEC_DIGIT7_RULE,
            DecDigit8 => &Self::DEC_DIGIT8_RULE,

            // Other
            PlusOrMinus => &Self::PLUS_OR_MINUS_RULE,
            PlusOrMinus1 => &Self::PLUS_OR_MINUS1_RULE,
            Plus => &Self::PLUS_RULE,
            Minus => &Self::MINUS_RULE,

            StarOrSlash => &Self::STAR_OR_SLASH_RULE,
            StarOrSlash1 => &Self::STAR_OR_SLASH1_RULE,
            Star => &Self::STAR_RULE,
            Slash => &Self::SLASH_RULE,

            Space => &Self::SPACE_RULE,
        })
    }
}
