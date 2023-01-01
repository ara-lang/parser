use crate::lexer::result::SyntaxResult;
use crate::lexer::state::State;
use crate::lexer::token::TokenKind;

pub fn tokenize(state: &mut State) -> SyntaxResult<(bool, Vec<u8>)> {
    let mut qualified = false;
    let mut last_was_slash = false;

    let mut buffer = vec![];

    if let Some(next @ crate::ident_start!()) = state.bytes.current() {
        buffer.push(*next);
        state.bytes.next();
    } else {
        crate::lexer_bail!(state, unrecognizable_token);
    }

    while let Some(next @ crate::ident!() | next @ b'\\') = state.bytes.current() {
        if matches!(next, crate::ident!()) {
            buffer.push(*next);
            state.bytes.next();
            last_was_slash = false;
            continue;
        }

        if *next == b'\\' && !last_was_slash {
            qualified = true;
            last_was_slash = true;
            buffer.push(*next);
            state.bytes.next();
            continue;
        }

        break;
    }

    Ok((qualified, buffer))
}

pub fn peek<'a>(state: &'a State) -> Option<&'a [u8]> {
    let mut size = 0;

    if let [crate::ident_start!()] = state.bytes.read(1) {
        size += 1;
        while let [crate::ident!()] = state.bytes.peek(size, 1) {
            size += 1;
        }

        Some(state.bytes.read(size))
    } else {
        None
    }
}

pub fn consume(state: &mut State) -> Vec<u8> {
    let ident = peek(state).unwrap().to_vec();
    state.bytes.skip(ident.len());

    ident
}

#[inline(always)]
pub fn to_keyword(ident: &[u8]) -> Option<TokenKind> {
    Some(match ident.to_ascii_lowercase().as_slice() {
        b"exit" => TokenKind::Exit,
        b"enddeclare" => TokenKind::EndDeclare,
        b"endswitch" => TokenKind::EndSwitch,
        b"endfor" => TokenKind::EndFor,
        b"endwhile" => TokenKind::EndWhile,
        b"endforeach" => TokenKind::EndForeach,
        b"endif" => TokenKind::EndIf,
        b"from" => TokenKind::From,
        b"and" => TokenKind::LogicalAnd,
        b"or" => TokenKind::LogicalOr,
        b"xor" => TokenKind::LogicalXor,
        b"print" => TokenKind::Print,
        b"readonly" => TokenKind::Readonly,
        b"match" => TokenKind::Match,
        b"abstract" => TokenKind::Abstract,
        b"array" => TokenKind::Array,
        b"vec" => TokenKind::Vec,
        b"dict" => TokenKind::Dict,
        b"as" => TokenKind::As,
        b"break" => TokenKind::Break,
        b"case" => TokenKind::Case,
        b"catch" => TokenKind::Catch,
        b"class" => TokenKind::Class,
        b"clone" => TokenKind::Clone,
        b"continue" => TokenKind::Continue,
        b"const" => TokenKind::Const,
        b"declare" => TokenKind::Declare,
        b"default" => TokenKind::Default,
        b"do" => TokenKind::Do,
        b"echo" => TokenKind::Echo,
        b"else" => TokenKind::Else,
        b"elseif" => TokenKind::ElseIf,
        b"enum" => TokenKind::Enum,
        b"extends" => TokenKind::Extends,
        b"false" => TokenKind::False,
        b"final" => TokenKind::Final,
        b"finally" => TokenKind::Finally,
        b"fn" => TokenKind::Fn,
        b"for" => TokenKind::For,
        b"foreach" => TokenKind::Foreach,
        b"function" => TokenKind::Function,
        b"goto" => TokenKind::Goto,
        b"if" => TokenKind::If,
        b"implements" => TokenKind::Implements,
        b"interface" => TokenKind::Interface,
        b"instanceof" => TokenKind::Instanceof,
        b"namespace" => TokenKind::Namespace,
        b"new" => TokenKind::New,
        b"null" => TokenKind::Null,
        b"private" => TokenKind::Private,
        b"protected" => TokenKind::Protected,
        b"public" => TokenKind::Public,
        b"return" => TokenKind::Return,
        b"static" => TokenKind::Static,
        b"switch" => TokenKind::Switch,
        b"type" => TokenKind::Type,
        b"throw" => TokenKind::Throw,
        b"async" => TokenKind::Async,
        b"await" => TokenKind::Await,
        b"concurrently" => TokenKind::Concurrently,
        b"trait" => TokenKind::Trait,
        b"true" => TokenKind::True,
        b"try" => TokenKind::Try,
        b"use" => TokenKind::Use,
        b"var" => TokenKind::Var,
        b"yield" => TokenKind::Yield,
        b"__dir__" => TokenKind::DirConstant,
        b"__file__" => TokenKind::FileConstant,
        b"__line__" => TokenKind::LineConstant,
        b"__function__" => TokenKind::FunctionConstant,
        b"__class__" => TokenKind::ClassConstant,
        b"__method__" => TokenKind::MethodConstant,
        b"__trait__" => TokenKind::TraitConstant,
        b"__namespace__" => TokenKind::NamespaceConstant,
        b"__halt_compiler_offset__" => TokenKind::HaltCompilerOffsetConstant,
        b"while" => TokenKind::While,
        b"where" => TokenKind::Where,
        b"insteadof" => TokenKind::Insteadof,
        b"list" => TokenKind::List,
        b"empty" => TokenKind::Empty,
        b"isset" => TokenKind::Isset,
        b"unset" => TokenKind::Unset,
        b"is" => TokenKind::Is,
        b"self" => TokenKind::Self_,
        b"parent" => TokenKind::Parent,
        b"die" => TokenKind::Die,
        b"eval" => TokenKind::Eval,
        b"include" => TokenKind::Include,
        b"include_once" => TokenKind::IncludeOnce,
        b"require" => TokenKind::Require,
        b"require_once" => TokenKind::RequireOnce,
        b"__halt_compiler" => TokenKind::HaltCompiler,
        b"global" => TokenKind::Global,
        _ => return None,
    })
}
