use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::statement::block::BlockStatement;
use crate::tree::statement::control_flow::IfStatement;
use crate::tree::statement::control_flow::UsingStatement;
use crate::tree::statement::expression::ExpressionStatement;
use crate::tree::statement::r#loop::BreakStatement;
use crate::tree::statement::r#loop::ContinueStatement;
use crate::tree::statement::r#loop::DoWhileStatement;
use crate::tree::statement::r#loop::ForStatement;
use crate::tree::statement::r#loop::ForeachStatement;
use crate::tree::statement::r#loop::WhileStatement;
use crate::tree::statement::r#return::ReturnStatement;
use crate::tree::statement::r#try::TryStatement;
use crate::tree::Node;

pub mod block;
pub mod control_flow;
pub mod expression;
pub mod r#loop;
pub mod r#return;
pub mod r#try;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum Statement {
    DoWhile(Box<DoWhileStatement>),
    While(Box<WhileStatement>),
    For(Box<ForStatement>),
    Foreach(Box<ForeachStatement>),
    Break(Box<BreakStatement>),
    Continue(Box<ContinueStatement>),
    If(Box<IfStatement>),
    Using(Box<UsingStatement>),
    Try(Box<TryStatement>),
    Expression(Box<ExpressionStatement>),
    Return(Box<ReturnStatement>),
    Block(Box<BlockStatement>),
    Empty(usize),
}

impl Node for Statement {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match &self {
            Self::DoWhile(statement) => statement.initial_position(),
            Self::While(statement) => statement.initial_position(),
            Self::For(statement) => statement.initial_position(),
            Self::Foreach(statement) => statement.initial_position(),
            Self::Break(statement) => statement.initial_position(),
            Self::Continue(statement) => statement.initial_position(),
            Self::If(statement) => statement.initial_position(),
            Self::Using(statement) => statement.initial_position(),
            Self::Try(statement) => statement.initial_position(),
            Self::Expression(statement) => statement.initial_position(),
            Self::Return(statement) => statement.initial_position(),
            Self::Block(statement) => statement.initial_position(),
            Self::Empty(position) => *position,
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Self::DoWhile(statement) => statement.final_position(),
            Self::While(statement) => statement.final_position(),
            Self::For(statement) => statement.final_position(),
            Self::Foreach(statement) => statement.final_position(),
            Self::Break(statement) => statement.final_position(),
            Self::Continue(statement) => statement.final_position(),
            Self::If(statement) => statement.final_position(),
            Self::Using(statement) => statement.final_position(),
            Self::Try(statement) => statement.final_position(),
            Self::Expression(statement) => statement.final_position(),
            Self::Return(statement) => statement.final_position(),
            Self::Block(statement) => statement.final_position(),
            Self::Empty(position) => *position + 1,
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Self::DoWhile(statement) => vec![statement.as_ref()],
            Self::While(statement) => vec![statement.as_ref()],
            Self::For(statement) => vec![statement.as_ref()],
            Self::Foreach(statement) => vec![statement.as_ref()],
            Self::Break(statement) => vec![statement.as_ref()],
            Self::Continue(statement) => vec![statement.as_ref()],
            Self::If(statement) => vec![statement.as_ref()],
            Self::Using(statement) => vec![statement.as_ref()],
            Self::Try(statement) => vec![statement.as_ref()],
            Self::Expression(statement) => vec![statement.as_ref()],
            Self::Return(statement) => vec![statement.as_ref()],
            Self::Block(statement) => vec![statement.as_ref()],
            Self::Empty(_) => vec![],
        }
    }

    fn get_description(&self) -> String {
        match &self {
            Self::DoWhile(statement) => statement.get_description(),
            Self::While(statement) => statement.get_description(),
            Self::For(statement) => statement.get_description(),
            Self::Foreach(statement) => statement.get_description(),
            Self::Break(statement) => statement.get_description(),
            Self::Continue(statement) => statement.get_description(),
            Self::If(statement) => statement.get_description(),
            Self::Using(statement) => statement.get_description(),
            Self::Try(statement) => statement.get_description(),
            Self::Expression(statement) => statement.get_description(),
            Self::Return(statement) => statement.get_description(),
            Self::Block(statement) => statement.get_description(),
            Self::Empty(_) => "empty statement".to_string(),
        }
    }
}
