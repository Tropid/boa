//! Update expression parsing.
//!
//! More information:
//!  - [ECMAScript specification][spec]
//!
//! [spec]: https://tc39.es/ecma262/#sec-update-expressions

use super::left_hand_side::LeftHandSideExpression;
use crate::syntax::{
    ast::{node::Node, op::UnaryOp, punc::Punctuator, token::TokenKind},
    parser::{AllowAwait, AllowYield, Cursor, ParseError, ParseResult, TokenParser},
};

/// Parses an update expression.
///
/// More information:
///  - [ECMAScript specification][spec]
///
/// [spec]: https://tc39.es/ecma262/#prod-UpdateExpression
#[derive(Debug, Clone, Copy)]
pub(super) struct UpdateExpression {
    allow_yield: AllowYield,
    allow_await: AllowAwait,
}

impl UpdateExpression {
    /// Creates a new `UpdateExpression` parser.
    pub(super) fn new<Y, A>(allow_yield: Y, allow_await: A) -> Self
    where
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
    {
        Self {
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
        }
    }
}

impl TokenParser for UpdateExpression {
    type Output = Node;

    fn parse(self, cursor: &mut Cursor<'_>, interner: &mut Interner) -> ParseResult {
        let tok = cursor.peek(0).ok_or(ParseError::AbruptEnd)?;
        match tok.kind {
            TokenKind::Punctuator(Punctuator::Inc) => {
                cursor.next().expect("token disappeared");
                return Ok(Node::unary_op(
                    UnaryOp::IncrementPre,
                    LeftHandSideExpression::new(self.allow_yield, self.allow_await)
                        .parse(cursor, interner)?,
                ));
            }
            TokenKind::Punctuator(Punctuator::Dec) => {
                cursor.next().expect("token disappeared");
                return Ok(Node::unary_op(
                    UnaryOp::DecrementPre,
                    LeftHandSideExpression::new(self.allow_yield, self.allow_await)
                        .parse(cursor, interner)?,
                ));
            }
            _ => {}
        }

        let lhs = LeftHandSideExpression::new(self.allow_yield, self.allow_await)
            .parse(cursor, interner)?;
        if let Some(tok) = cursor.peek(0) {
            match tok.kind {
                TokenKind::Punctuator(Punctuator::Inc) => {
                    cursor.next().expect("token disappeared");
                    return Ok(Node::unary_op(UnaryOp::IncrementPost, lhs));
                }
                TokenKind::Punctuator(Punctuator::Dec) => {
                    cursor.next().expect("token disappeared");
                    return Ok(Node::unary_op(UnaryOp::DecrementPost, lhs));
                }
                _ => {}
            }
        }

        Ok(lhs)
    }
}