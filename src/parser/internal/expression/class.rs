use crate::lexer::token::TokenKind;
use crate::parser::internal::definition::attribute;
use crate::parser::internal::definition::class::class_definition_member;
use crate::parser::internal::expression::argument;
use crate::parser::internal::identifier;
use crate::parser::internal::utils;
use crate::parser::result::ParseResult;
use crate::parser::state::State;
use crate::tree::definition::class::ClassDefinitionBody;
use crate::tree::definition::class::ClassDefinitionExtends;
use crate::tree::definition::class::ClassDefinitionImplements;
use crate::tree::expression::class::AnonymousClassExpression;
use crate::tree::expression::operator::ClassOperationExpression;

pub fn anonymous_initialization_class_operation_expression(
    state: &mut State,
) -> ParseResult<ClassOperationExpression> {
    Ok(ClassOperationExpression::AnonymousInitialization {
        comments: state.iterator.comments(),
        new: utils::skip_keyword(state, TokenKind::New)?,
        class: anonymous_class_expression(state)?,
    })
}

pub fn anonymous_class_expression(state: &mut State) -> ParseResult<AnonymousClassExpression> {
    attribute::gather(state)?;

    let attributes = state.get_attributes();
    let class = utils::skip_keyword(state, TokenKind::Class)?;
    let comments = state.iterator.comments();
    let arguments = argument::argument_list_expression(state)?;
    let current = state.iterator.current();
    let extends = if current.kind == TokenKind::Extends {
        let extends = utils::skip_keyword(state, TokenKind::Extends)?;
        let parent = identifier::fully_qualified_templated_identifier(state)?;

        Some(ClassDefinitionExtends { extends, parent })
    } else {
        None
    };

    let current = state.iterator.current();
    let implements = if current.kind == TokenKind::Implements {
        let implements = utils::skip_keyword(state, TokenKind::Implements)?;
        let interfaces = utils::at_least_one_comma_separated(
            state,
            &identifier::fully_qualified_templated_identifier,
            TokenKind::LeftBrace,
        )?;

        Some(ClassDefinitionImplements {
            implements,
            interfaces,
        })
    } else {
        None
    };

    Ok(AnonymousClassExpression {
        comments,
        attributes,
        class,
        arguments,
        extends,
        implements,
        body: ClassDefinitionBody {
            left_brace: utils::skip_left_brace(state)?,
            members: {
                let mut members = Vec::new();
                while state.iterator.current().kind != TokenKind::RightBrace {
                    members.push(class_definition_member(state)?);
                }

                members
            },
            right_brace: utils::skip_right_brace(state)?,
        },
    })
}
