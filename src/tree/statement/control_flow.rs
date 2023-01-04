use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::statement::block::BlockStatement;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfStatement {
    pub comments: CommentGroup,
    pub r#if: usize,
    pub condition: Expression,
    pub block: BlockStatement,
    pub elseifs: Vec<IfElseIfStatement>,
    pub r#else: Option<IfElseStatement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfElseIfStatement {
    pub comments: CommentGroup,
    pub elseif: usize,
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfElseStatement {
    pub comments: CommentGroup,
    pub r#else: usize,
    pub block: IfElseBlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IfElseBlockStatement {
    If(Box<IfStatement>),
    Block(BlockStatement),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsingStatement {
    pub comments: CommentGroup,
    pub r#using: usize,
    pub assignments: CommaSeparated<UsingAssignmentStatement>,
    pub if_clause: Option<UsingIfClauseStatement>,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsingAssignmentStatement {
    pub comments: CommentGroup,
    pub variable: Variable,
    pub equals: usize,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsingIfClauseStatement {
    pub comments: CommentGroup,
    pub r#if: usize,
    pub condition: Expression,
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

impl Node for IfElseIfStatement {
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

impl Node for IfElseStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#else
    }

    fn final_position(&self) -> usize {
        match &self.block {
            IfElseBlockStatement::If(r#if) => r#if.final_position(),
            IfElseBlockStatement::Block(block) => block.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self.block {
            IfElseBlockStatement::If(r#if) => vec![r#if.as_ref()],
            IfElseBlockStatement::Block(block) => vec![block],
        }
    }
}

impl Node for UsingStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#using
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];
        for assignment in &self.assignments.inner {
            children.push(assignment);
        }

        if let Some(if_clause) = &self.if_clause {
            children.push(if_clause);
        }

        children.push(&self.block);

        children
    }
}

impl Node for UsingAssignmentStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.variable.initial_position()
    }

    fn final_position(&self) -> usize {
        self.expression.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.variable, &self.expression]
    }
}

impl Node for UsingIfClauseStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#if
    }

    fn final_position(&self) -> usize {
        self.condition.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.condition]
    }
}
