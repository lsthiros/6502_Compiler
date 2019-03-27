
use crate::lexer::LexerToken;
use crate::lexer::TokenType;

use crate::token_stream::TokenStream;
use crate::token_stream::UnexpectedTokenError;

use crate::graphviz::CreatesGraphviz;
use crate::graphviz::Graphviz;

use crate::parser::bin_op::AstExprNode;
use crate::parser::bin_op::expression;


type FuncArg = String;

impl CreatesGraphviz for FuncArg {
    fn get_name(&self) -> String {
        return self.clone();
    }

    fn get_connections(&self) -> Vec<&CreatesGraphviz> {
        return vec![];
    }
}


struct FuncDecl {
    name: String,
    args: Vec<FuncArg>
}


impl CreatesGraphviz for FuncDecl {

    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn get_connections(&self) -> Vec<&CreatesGraphviz> {
        let mut result: Vec<&CreatesGraphviz> = Vec::new();
        for arg in &self.args {
            result.push(arg)
        }
        return result;
    }
}


fn get_func_decl<I>(token_stream: &mut TokenStream<I>) -> Result<FuncDecl, UnexpectedTokenError> where I: Iterator<Item = LexerToken>{
    let name_token = token_stream.expect(TokenType::IDENTIFIER)?;
    let name: String = name_token.label.unwrap();
    let _: LexerToken = token_stream.expect(TokenType::L_PAREN)?;
    let mut args: Vec<String> = Vec::new();

    let mut continuing_list: bool = token_stream.accept(TokenType::R_PAREN).is_none();

    while continuing_list {
        let arg_token: LexerToken = token_stream.expect(TokenType::IDENTIFIER)?;
        let arg_name = arg_token.label.unwrap();
        args.push(arg_name);

        continuing_list = token_stream.accept(TokenType::COMMA).is_some();
    }

    if args.len() > 0 {
        let _: LexerToken = token_stream.expect(TokenType::R_PAREN)?;
    }

    let result = FuncDecl {
        name: name,
        args: args
    };
    return Ok(result);
}


enum Statement {
    Select {
        condition: Box<AstExprNode>,
        statement: Box<Statement>,
        else_clause: Option<Box<Statement>>
    },
    ReturnExpr(Box<AstExprNode>)
}


impl CreatesGraphviz for Statement {
    fn get_name(&self) -> String {
        match self {
            Statement::Select {
                condition: _, statement: _, else_clause: _
            } => {
                String::from("if")
            }
            Statement::ReturnExpr(_)=> {
                String::from("return")
            }
        }
    }

    fn get_connections(&self) -> Vec<&CreatesGraphviz> {
        match self {
            Statement::Select {
                condition, statement, else_clause
            } => {
                let mut result: Vec<&CreatesGraphviz> = vec![condition.as_ref(), statement.as_ref()];
                if let Some(clause) = else_clause {
                    result.push(clause.as_ref());
                }
                return result;
            }
            Statement::ReturnExpr(expr) => {
                return vec![expr.as_ref()];
            }
        }
    }
}


fn statement<I>(token_stream: &mut TokenStream<I>) -> Result<Box<Statement>, UnexpectedTokenError> where I: Iterator<Item = LexerToken>{
    let token = token_stream.expect_multi(&vec![TokenType::IF, TokenType::RETURN])?;
    match token.token_type {
        TokenType::IF => {
            let _ = token_stream.expect(TokenType::L_PAREN)?;
            let condition = expression(token_stream)?;
            let _ = token_stream.expect(TokenType::R_PAREN)?;

            let inner_statement = statement(token_stream)?;
            let else_clause: Option<Box<Statement>>;

            if token_stream.accept(TokenType::ELSE).is_some() {
                else_clause = Some(statement(token_stream)?);
            }
            else {
                else_clause = None
            }

            let result = Statement::Select {
                condition: condition,
                statement: inner_statement,
                else_clause: else_clause
            };

            return Ok(Box::new(result));
        }
        TokenType::RETURN => {
            return Ok(Box::new(Statement::ReturnExpr(expression(token_stream)?)));
        }
        _ => {
            unreachable!()
        }
    }
}


enum PrimaryStatement {
    Definition {
        decl: FuncDecl,
        inner_statement: Box<Statement>
    },
    Extern(FuncDecl)
}


impl CreatesGraphviz for PrimaryStatement {
    fn get_name(&self) -> String  {
        match self {
            PrimaryStatement::Extern(_) => {
                String::from("Extern")
            }
            PrimaryStatement::Definition {
                decl: _, inner_statement: _
            } => {
                String::from("Def")
            }
        }
    }

    fn get_connections(&self) -> Vec<&CreatesGraphviz> {
        match self {
            PrimaryStatement::Definition {
                decl, inner_statement
            } => {
                return vec![decl, inner_statement.as_ref()]
            }
            PrimaryStatement::Extern(decl) => {
                return vec![decl]
            }
        }
    }
}


fn primary<I>(token_stream: &mut TokenStream<I>) -> Result<PrimaryStatement, UnexpectedTokenError> where I: Iterator<Item = LexerToken> {
    let token = token_stream.expect_multi(&vec![TokenType::EXTERN, TokenType::DEF])?;
    let result: PrimaryStatement;

    match token.token_type {
        TokenType::EXTERN => {
            let decl = get_func_decl(token_stream)?;
            result = PrimaryStatement::Extern(decl);
        }
        TokenType::DEF => {
            let decl = get_func_decl(token_stream)?;
            let statement = statement(token_stream)?;
            result = PrimaryStatement::Definition {
                decl: decl,
                inner_statement: statement
            }
        }
        _ => {
            unreachable!()
        }
    }
    return Ok(result);
}


pub fn parse_stream(token_stream: &Vec<LexerToken>) -> Result<String, UnexpectedTokenError> {
    let mut stream = TokenStream(token_stream.iter().cloned().peekable());

    let state = primary(&mut stream)?;
    let result = Graphviz::from(&state as &CreatesGraphviz);

    result.write_file(String::from("./a.out"));
    return Ok(String::from("Ok"));
}