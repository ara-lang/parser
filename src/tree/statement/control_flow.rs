use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::expression::Expression;
use crate::tree::statement::block::BlockStatement;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfStatement {
    pub comments: CommentGroup,
    pub r#if: Keyword,
    pub conditions: CommaSeparated<Expression>,
    pub block: BlockStatement,
    pub elseifs: Vec<IfElseIfStatement>,
    pub r#else: Option<IfElseStatement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfElseIfStatement {
    pub comments: CommentGroup,
    pub elseif: Keyword,
    pub condition: Expression,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IfElseStatement {
    pub comments: CommentGroup,
    pub r#else: Keyword,
    pub block: IfElseBlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IfElseBlockStatement {
    If(Box<IfStatement>),
    Block(BlockStatement),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsingStatement {
    pub comments: CommentGroup,
    pub r#using: Keyword,
    pub assignments: CommaSeparated<UsingAssignmentStatement>,
    pub if_clause: Option<UsingIfClauseStatement>,
    pub block: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsingAssignmentStatement {
    pub comments: CommentGroup,
    pub variable: Variable,
    pub equals: usize,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsingIfClauseStatement {
    pub comments: CommentGroup,
    pub r#if: Keyword,
    pub condition: Expression,
}

impl Node for IfStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#if.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#if, &self.block];

        for condition in &self.conditions.inner {
            children.push(condition);
        }

        for elseif in &self.elseifs {
            children.push(elseif);
        }

        if let Some(r#else) = &self.r#else {
            children.push(r#else);
        }

        children
    }

    fn get_description(&self) -> String {
        "if statement".to_string()
    }
}

impl Node for IfElseIfStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.elseif.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.elseif, &self.condition, &self.block]
    }

    fn get_description(&self) -> String {
        "elseif statement".to_string()
    }
}

impl Node for IfElseStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#else.initial_position()
    }

    fn final_position(&self) -> usize {
        match &self.block {
            IfElseBlockStatement::If(r#if) => r#if.final_position(),
            IfElseBlockStatement::Block(block) => block.final_position(),
        }
    }

    fn children(&self) -> Vec<&dyn Node> {
        match &self.block {
            IfElseBlockStatement::If(r#if) => vec![&self.r#else, r#if.as_ref()],
            IfElseBlockStatement::Block(block) => vec![&self.r#else, block],
        }
    }

    fn get_description(&self) -> String {
        "else statement".to_string()
    }
}

impl Node for UsingStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#using.initial_position()
    }

    fn final_position(&self) -> usize {
        self.block.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#using];
        for assignment in &self.assignments.inner {
            children.push(assignment);
        }

        if let Some(if_clause) = &self.if_clause {
            children.push(if_clause);
        }

        children.push(&self.block);

        children
    }

    fn get_description(&self) -> String {
        "using statement".to_string()
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

    fn get_description(&self) -> String {
        "using assignment statement".to_string()
    }
}

impl Node for UsingIfClauseStatement {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#if.initial_position()
    }

    fn final_position(&self) -> usize {
        self.condition.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.r#if, &self.condition]
    }

    fn get_description(&self) -> String {
        "using if clause statement".to_string()
    }
}
