use crate::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement {
    fn statement_node(&self);
}

trait Expression {
    fn expression_node(&self);
}

struct Program<S>
where
    S: Statement + Node,
{
    statements: Vec<S>,
}

impl<S> Node for Program<S>
where
    S: Node + Statement,
{
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

struct LetStatement<E>
where
    E: Node + Expression,
{
    token: Token, // Token::Let
    name: Identifier,
    value: E,
}

impl<E> Node for LetStatement<E>
where
    E: Node + Expression,
{
    fn token_literal(&self) -> String {
        match self.token {
            Token::Let => "let".to_string(),
            _ => unreachable!(),
        }
    }
}

impl<E> Statement for LetStatement<E>
where
    E: Node + Expression,
{
    fn statement_node(&self) {}
}

struct Identifier {
    token: Token, // Token::Ident
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        match &self.token {
            Token::Ident(ident) => ident.clone(),
            _ => unreachable!(),
        }
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
