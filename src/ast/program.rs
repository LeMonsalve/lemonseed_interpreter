use crate::ast::{nodes::{Node, StatementNode}, statements::VarStatement};

struct Program {
    statements: Vec<StatementNode>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        return if self.statements.len() > 0 {
            match &self.statements[0] {
                StatementNode::Var(statement) => statement.token_literal(),
            }
        } else {
            String::from("")
        };
    }

    fn print_string(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            out.push_str(&statement.print_string());
        }
        out
    }
}