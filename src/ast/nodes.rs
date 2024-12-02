use crate::{ast::statements::VarStatement, token::Token};

pub trait Node {
    fn token_literal(&self) -> String;
    fn print_string(&self) -> String;
}

pub enum StatementNode {
    Var(VarStatement),
}

pub enum ExpressionNode {
    Identifier(Identifier),
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        return match self {
            StatementNode::Var(statement) => statement.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        return match self {
            StatementNode::Var(statement) => statement.print_string(),
        }
    }
}

impl Node for ExpressionNode {
    fn token_literal(&self) -> String {
        return match self {
            ExpressionNode::Identifier(identifier) => identifier.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        return match self {
            ExpressionNode::Identifier(identifier) => identifier.print_string(),
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        self.value.clone()
    }
}
