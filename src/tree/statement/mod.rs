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

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, JsonSchema)]
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
}

impl Node for Statement {
    fn comments(&self) -> Option<&CommentGroup> {
        None
    }

    fn initial_position(&self) -> usize {
        match &self {
            Statement::DoWhile(statement) => statement.initial_position(),
            Statement::While(statement) => statement.initial_position(),
            Statement::For(statement) => statement.initial_position(),
            Statement::Foreach(statement) => statement.initial_position(),
            Statement::Break(statement) => statement.initial_position(),
            Statement::Continue(statement) => statement.initial_position(),
            Statement::If(statement) => statement.initial_position(),
            Statement::Using(statement) => statement.initial_position(),
            Statement::Try(statement) => statement.initial_position(),
            Statement::Expression(statement) => statement.initial_position(),
            Statement::Return(statement) => statement.initial_position(),
            Statement::Block(statement) => statement.initial_position(),
        }
    }

    fn final_position(&self) -> usize {
        match &self {
            Statement::DoWhile(statement) => statement.final_position(),
            Statement::While(statement) => statement.final_position(),
            Statement::For(statement) => statement.final_position(),
            Statement::Foreach(statement) => statement.final_position(),
            Statement::Break(statement) => statement.final_position(),
            Statement::Continue(statement) => statement.final_position(),
            Statement::If(statement) => statement.final_position(),
            Statement::Using(statement) => statement.final_position(),
            Statement::Try(statement) => statement.final_position(),
            Statement::Expression(statement) => statement.final_position(),
            Statement::Return(statement) => statement.final_position(),
            Statement::Block(statement) => statement.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self {
            Statement::DoWhile(statement) => vec![statement.as_ref()],
            Statement::While(statement) => vec![statement.as_ref()],
            Statement::For(statement) => vec![statement.as_ref()],
            Statement::Foreach(statement) => vec![statement.as_ref()],
            Statement::Break(statement) => vec![statement.as_ref()],
            Statement::Continue(statement) => vec![statement.as_ref()],
            Statement::If(statement) => vec![statement.as_ref()],
            Statement::Using(statement) => vec![statement.as_ref()],
            Statement::Try(statement) => vec![statement.as_ref()],
            Statement::Expression(statement) => vec![statement.as_ref()],
            Statement::Return(statement) => vec![statement.as_ref()],
            Statement::Block(statement) => vec![statement.as_ref()],
        }
    }

    fn get_description(&self) -> String {
        "statement".to_string()
    }
}
