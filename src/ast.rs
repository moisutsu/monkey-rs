#[derive(Debug, Clone)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

impl Node {
    pub fn token_literal(&self) -> String {
        match self {
            Self::Program(program) => {
                if program.statements.len() > 0 {
                    program.statements[0].token_literal()
                } else {
                    "".to_string()
                }
            }
            Self::Statement(statement) => statement.token_literal(),
            Self::Expression(expression) => expression.token_literal(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(LetStatement),
}

impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Self::LetStatement(_) => "let".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
}

impl Expression {
    fn token_literal(&self) -> String {
        match self {
            Self::Identifier(expression) => expression.token_literal(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: Identifier,
    value: Expression,
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

impl Identifier {
    fn token_literal(&self) -> String {
        let Identifier(ident) = self;
        ident.clone()
    }
}
