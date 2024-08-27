use darling::{error::Accumulator, Error, Result};
use syn::{Expr, Lit};

// Extend if necessary
pub enum ExprType {
    Array,
    Path,
}

impl ExprType {
    fn name(&self) -> &'static str {
        self.select().0
    }

    fn example(&self) -> &'static str {
        self.select().1
    }

    fn select(&self) -> (&'static str, &'static str) {
        match self {
            Self::Array => ("Array", "[foo, bar]"),
            Self::Path => ("Path", "std::string::String"),
        }
    }
}

pub trait DarlingErrorExt {
    /// unexpected, expected
    fn unexpected_expr_w(u: &Expr, e: ExprType) -> Self;
    /// name, current size, limit,
    fn length_limit(n: &str, c: usize, l: usize) -> Self;
    fn unsupported_lit_format(lit: &Lit) -> Self;
    fn not_empty() -> Self;
}

impl DarlingErrorExt for Error {
    fn unexpected_expr_w(u: &Expr, e: ExprType) -> Self {
        Self::custom(format_args!(
            "Unexpected expression: `{}`, expected `{}` (e.g. `{}`)",
            name_from_expr(u),
            e.name(),
            e.example()
        ))
    }

    fn length_limit(n: &str, c: usize, l: usize) -> Self {
        Self::custom(format_args!(
            "Too many elements: {n} constains {c}; Allowed limit is {l}"
        ))
    }

    fn unsupported_lit_format(lit: &Lit) -> Self {
        let lit_name = match lit {
            Lit::Str(_) => "Str",
            Lit::ByteStr(_) => "ByteStr",
            Lit::CStr(_) => "Cstr",
            Lit::Byte(_) => "Byte",
            Lit::Char(_) => "Char",
            Lit::Int(_) => "Int",
            Lit::Float(_) => "Float",
            Lit::Bool(_) => "Bool",
            Lit::Verbatim(_) => "Vervatim",
            _ => "unknown",
        };

        Self::unsupported_format(lit_name)
    }

    fn not_empty() -> Self {
        Self::custom("Expresion must contain at least one element")
    }
}

fn name_from_expr(expr: &Expr) -> &'static str {
    match expr {
        Expr::Array(_) => "array",
        Expr::Assign(_) => "assign",
        Expr::Async(_) => "async",
        Expr::Await(_) => "await",
        Expr::Binary(_) => "binary",
        Expr::Block(_) => "block",
        Expr::Break(_) => "break",
        Expr::Call(_) => "call",
        Expr::Cast(_) => "cast",
        Expr::Closure(_) => "closure",
        Expr::Const(_) => "const",
        Expr::Continue(_) => "continue",
        Expr::Field(_) => "field",
        Expr::ForLoop(_) => "for_loop",
        Expr::Group(_) => "group",
        Expr::If(_) => "if",
        Expr::Index(_) => "index",
        Expr::Infer(_) => "infer",
        Expr::Let(_) => "let",
        Expr::Lit(_) => "lit",
        Expr::Loop(_) => "loop",
        Expr::Macro(_) => "macro",
        Expr::Match(_) => "match",
        Expr::MethodCall(_) => "method_call",
        Expr::Paren(_) => "paren",
        Expr::Path(_) => "path",
        Expr::Range(_) => "range",
        Expr::Reference(_) => "reference",
        Expr::Repeat(_) => "repeat",
        Expr::Return(_) => "return",
        Expr::Struct(_) => "struct",
        Expr::Try(_) => "try",
        Expr::TryBlock(_) => "try_block",
        Expr::Tuple(_) => "tuple",
        Expr::Unary(_) => "unary",
        Expr::Unsafe(_) => "unsafe",
        Expr::Verbatim(_) => "verbatim",
        Expr::While(_) => "while",
        Expr::Yield(_) => "yield",
        _ => "unknown",
    }
}

pub trait AccumulatorExt<T> {
    fn finish_with_result(self, res: Result<T>) -> Result<T>;
    fn finish_with_err(self, err: Error) -> Result<T>;
}

impl<T> AccumulatorExt<T> for Accumulator {
    fn finish_with_result(mut self, res: Result<T>) -> Result<T> {
        let opt = self.handle(res);

        self.finish().map(|_| opt.unwrap())
    }

    fn finish_with_err(mut self, err: Error) -> Result<T> {
        self.push(err);
        Err(self.finish().unwrap_err())
    }
}
