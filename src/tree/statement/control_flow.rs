use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::statement::block::BlockStatement;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfStatement {
    pub comments: CommentGroup,
    pub r#if: usize,
    pub condition: Expression,
    pub block: BlockStatement,
    pub elseifs: Vec<IfStatementElseIf>,
    pub r#else: Option<IfStatementElse>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfStatementElseIf {
    pub comments: CommentGroup,
    pub elseif: usize,
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfStatementElse {
    pub comments: CommentGroup,
    pub r#else: usize,
    pub block: IfStatementElseBlock,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IfStatementElseBlock {
    If(Box<IfStatement>),
    Block(BlockStatement),
}

impl Node for IfStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#if
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.condition, &self.block];
        for elseif in &self.elseifs {
            children.push(elseif);
        }

        if let Some(r#else) = &self.r#else {
            children.push(r#else);
        }

        children
    }
}

impl Node for IfStatementElseIf {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.elseif
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.condition, &self.block]
    }
}

impl Node for IfStatementElse {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#else
    }

    fn final_position(&self) -> usize {
        match &self.block {
            IfStatementElseBlock::If(r#if) => r#if.final_position(),
            IfStatementElseBlock::Block(block) => block.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self.block {
            IfStatementElseBlock::If(r#if) => vec![r#if.as_ref()],
            IfStatementElseBlock::Block(block) => vec![block],
        }
    }
}
