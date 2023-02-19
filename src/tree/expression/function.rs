use bincode::Decode;
use bincode::Encode;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::tree::comment::CommentGroup;
use crate::tree::definition::attribute::AttributeGroupDefinition;
use crate::tree::definition::function::FunctionLikeParameterListDefinition;
use crate::tree::definition::function::FunctionLikeReturnTypeDefinition;
use crate::tree::expression::Expression;
use crate::tree::statement::block::BlockStatement;
use crate::tree::token::Keyword;
use crate::tree::utils::CommaSeparated;
use crate::tree::variable::Variable;
use crate::tree::Node;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArrowFunctionExpression {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub r#static: Option<Keyword>,
    pub r#fn: Keyword,
    pub parameters: FunctionLikeParameterListDefinition,
    pub return_type: FunctionLikeReturnTypeDefinition,
    pub double_arrow: usize,
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousFunctionExpression {
    pub comments: CommentGroup,
    pub attributes: Vec<AttributeGroupDefinition>,
    pub r#static: Option<Keyword>,
    pub function: Keyword,
    pub parameters: FunctionLikeParameterListDefinition,
    pub use_clause: Option<AnonymousFunctionUseClauseExpression>,
    pub return_type: FunctionLikeReturnTypeDefinition,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousFunctionUseClauseExpression {
    pub comments: CommentGroup,
    pub r#use: Keyword,
    pub left_parenthesis: usize,
    pub variables: CommaSeparated<AnonymousFunctionUseClauseVariableExpression>,
    pub right_parenthesis: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize, Encode, Decode, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AnonymousFunctionUseClauseVariableExpression {
    pub comments: CommentGroup,
    pub variable: Variable,
}

impl Node for ArrowFunctionExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attribute) = self.attributes.first() {
            return attribute.initial_position();
        }

        if let Some(r#static) = &self.r#static {
            return r#static.initial_position();
        }

        self.r#fn.initial_position()
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        if let Some(r#static) = &self.r#static {
            children.push(r#static);
        }

        children.push(&self.r#fn);

        children.push(&self.parameters);
        children.push(&self.return_type);
        children.push(self.body.as_ref());

        children
    }

    fn get_description(&self) -> String {
        "arrow function expression".to_string()
    }
}

impl Node for AnonymousFunctionExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        if let Some(attribute) = self.attributes.first() {
            return attribute.initial_position();
        }

        if let Some(r#static) = &self.r#static {
            return r#static.initial_position();
        }

        self.function.initial_position()
    }

    fn final_position(&self) -> usize {
        self.body.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![];

        for attribute in &self.attributes {
            children.push(attribute);
        }

        if let Some(r#static) = &self.r#static {
            children.push(r#static);
        }

        children.push(&self.function);
        children.push(&self.parameters);
        children.push(&self.return_type);
        children.push(&self.body);

        children
    }

    fn get_description(&self) -> String {
        "anonymous function expression".to_string()
    }
}

impl Node for AnonymousFunctionUseClauseExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.r#use.initial_position()
    }

    fn final_position(&self) -> usize {
        self.right_parenthesis
    }

    fn children(&self) -> Vec<&dyn Node> {
        let mut children: Vec<&dyn Node> = vec![&self.r#use];

        for variable in &self.variables.inner {
            children.push(variable);
        }

        children
    }

    fn get_description(&self) -> String {
        "anonymous function use clause expression".to_string()
    }
}

impl Node for AnonymousFunctionUseClauseVariableExpression {
    fn comments(&self) -> Option<&CommentGroup> {
        Some(&self.comments)
    }

    fn initial_position(&self) -> usize {
        self.variable.initial_position()
    }

    fn final_position(&self) -> usize {
        self.variable.final_position()
    }

    fn children(&self) -> Vec<&dyn Node> {
        vec![&self.variable]
    }

    fn get_description(&self) -> String {
        "anonymous function use clause variable expression".to_string()
    }
}

impl std::fmt::Display for ArrowFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(r#static) = &self.r#static {
            write!(f, "{} ", r#static)?;
        }
        write!(
            f,
            "{} {}{} => {};",
            self.r#fn, self.parameters, self.return_type, self.body
        )
    }
}

impl std::fmt::Display for AnonymousFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(r#static) = &self.r#static {
            write!(f, "{} ", r#static)?;
        }
        write!(f, "{} {}", self.function, self.parameters)?;

        if let Some(use_clause) = &self.use_clause {
            write!(f, " {}", use_clause)?;
        }

        write!(f, "{} {}", self.return_type, self.body)
    }
}

impl std::fmt::Display for AnonymousFunctionUseClauseExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.r#use, self.variables)
    }
}

impl std::fmt::Display for AnonymousFunctionUseClauseVariableExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.variable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::byte_string::ByteString;
    use crate::tree::definition::function::FunctionLikeParameterDefinition;
    use crate::tree::definition::r#type::SignedIntegerTypeDefinition;
    use crate::tree::definition::r#type::TypeDefinition;

    #[test]
    fn arrow_function_expression_display() {
        let arrow_function_expression = ArrowFunctionExpression {
            attributes: vec![],
            comments: CommentGroup { comments: vec![] },
            r#static: Some(Keyword::new(ByteString::from("static"), 0)),
            r#fn: Keyword::new(ByteString::from("fn"), 0),
            parameters: FunctionLikeParameterListDefinition {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                parameters: CommaSeparated {
                    inner: vec![FunctionLikeParameterDefinition {
                        attributes: vec![],
                        comments: CommentGroup { comments: vec![] },
                        type_definition: TypeDefinition::SignedInteger(
                            SignedIntegerTypeDefinition::I64(Keyword::new(
                                ByteString::from("i64"),
                                15,
                            )),
                        ),
                        ellipsis: None,
                        variable: Variable {
                            position: 0,
                            name: ByteString::from("foo"),
                        },
                        default: None,
                    }],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
            return_type: FunctionLikeReturnTypeDefinition {
                colon: 0,
                type_definition: TypeDefinition::SignedInteger(SignedIntegerTypeDefinition::I64(
                    Keyword::new(ByteString::from("i64"), 15),
                )),
            },
            double_arrow: 0,
            body: Box::new(Expression::Variable(Variable {
                position: 0,
                name: ByteString::from("foo"),
            })),
        };

        assert_eq!(
            arrow_function_expression.to_string(),
            "static fn (i64 $foo): i64 => $foo;"
        );
    }

    #[test]
    fn anonymous_function_expression_display() {
        let anonymous_function_expression = AnonymousFunctionExpression {
            attributes: vec![],
            comments: CommentGroup { comments: vec![] },
            r#static: Some(Keyword::new(ByteString::from("static"), 0)),
            function: Keyword::new(ByteString::from("function"), 0),
            parameters: FunctionLikeParameterListDefinition {
                comments: CommentGroup { comments: vec![] },
                left_parenthesis: 0,
                parameters: CommaSeparated {
                    inner: vec![FunctionLikeParameterDefinition {
                        attributes: vec![],
                        comments: CommentGroup { comments: vec![] },
                        type_definition: TypeDefinition::SignedInteger(
                            SignedIntegerTypeDefinition::I32(Keyword::new(
                                ByteString::from("i32"),
                                15,
                            )),
                        ),
                        ellipsis: None,
                        variable: Variable {
                            position: 0,
                            name: ByteString::from("foo"),
                        },
                        default: None,
                    }],
                    commas: vec![],
                },
                right_parenthesis: 0,
            },
            return_type: FunctionLikeReturnTypeDefinition {
                colon: 0,
                type_definition: TypeDefinition::SignedInteger(SignedIntegerTypeDefinition::I64(
                    Keyword::new(ByteString::from("i64"), 15),
                )),
            },
            use_clause: Some(AnonymousFunctionUseClauseExpression {
                comments: CommentGroup { comments: vec![] },
                r#use: Keyword::new(ByteString::from("use"), 0),
                variables: CommaSeparated {
                    inner: vec![AnonymousFunctionUseClauseVariableExpression {
                        comments: CommentGroup { comments: vec![] },
                        variable: Variable {
                            position: 0,
                            name: ByteString::from("bar"),
                        },
                    }],
                    commas: vec![],
                },
                left_parenthesis: 0,
                right_parenthesis: 0,
            }),
            body: BlockStatement {
                comments: CommentGroup { comments: vec![] },
                left_brace: 0,
                statements: vec![],
                right_brace: 0,
            },
        };

        assert_eq!(
            anonymous_function_expression.to_string(),
            "static function (i32 $foo) use ($bar): i64 { /* ... */ }"
        );
    }
}
