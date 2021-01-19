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
