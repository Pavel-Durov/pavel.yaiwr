%start Statements
%expect-unused Unmatched "UNMATCHED"
%%

Statements -> Result<Vec<AstNode>, ()>:
    Statements Statement { append($1.map_err(|_| ())?, $2.map_err(|_| ())?)  }
  | { Ok(vec![]) }
  ;

Statement -> Result<AstNode, ()>:
    Statement "ADD" Term { Ok(AstNode::Add{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Term { $1 }
    | FunctionDeclaration { $1 }
    | Statement "LESS_THAN" Term { Ok(AstNode::LessThan{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Statement "GREATER_THAN" Term { Ok(AstNode::GreaterThan{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    ;

Term -> Result<AstNode, ()>:
      Term 'MUL' Factor { Ok(AstNode::Mul{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Factor { $1 }
    ;


Builtins -> Result<AstNode, ()>:
    "PRINT_LN" Factor { Ok(AstNode::PrintLn{ rhs: Box::new($2?) }) };

VarAssign -> Result<AstNode, ()>:
    "ASSIGN" "IDENTIFIER" "=" Statement  { 
        Ok(AstNode::Assign { 
            id: $lexer.span_str(($2.map_err(|_| ())?).span()).to_string(), rhs: Box::new($4?) 
        })
     };

Literal -> Result<AstNode, ()>:
    "INTEGER_LITERAL" { parse_number($lexer.span_str(($1.map_err(|_| ())?).span())) }
    | "BOOLEAN_LITERAL" { parse_boolean($lexer.span_str(($1.map_err(|_| ())?).span())) }
    ;

Factor -> Result<AstNode, ()>:
     "(" Statement ")" { $2 }
    | Id  { $1 } ";"
    | Literal { $1 }
    | Return { $1 } ";" 
    | VarAssign { $1 } ";"
    ;

ArgList -> Result<Vec<AstNode>, ()>:
    ArgList ',' Statement { append($1.map_err(|_| ())?, $3.map_err(|_| ())?) }
    | Statement {  Ok(vec![$1.map_err(|_| ())?]) }
    ;

Id -> Result<AstNode, ()>:
    Builtins { $1 }
    | "IDENTIFIER"  { 
        Ok(AstNode::ID { 
            value: $lexer.span_str(($1.map_err(|_| ())?).span()).to_string() 
        })
    }
    | "IDENTIFIER" "(" ")" { 
        let id = $1.map_err(|_| ())?;
        Ok(AstNode::FunctionCall{ 
            id: $lexer.span_str(id.span()).to_string(),
            args: vec![]
        })
    }
    | "IDENTIFIER" "(" ArgList ")" { 
        let id = $1.map_err(|_| ())?;
        Ok(AstNode::FunctionCall{ 
            id: $lexer.span_str(id.span()).to_string(),
            args: $3.map_err(|_| ())?
        })
      }
    ;
// Function Declaration

ParamList -> Result<Vec<AstNode>, ()>:
    ParamList ',' Id { append($1.map_err(|_| ())?, $3.map_err(|_| ())?) }
    | Id {  Ok(vec![$1.map_err(|_| ())?]) }
    ;

FunctionDeclaration -> Result<AstNode, ()>:
    "FUNCTION" "IDENTIFIER" "(" ")" "{" Statements "}" { 
        let id = $2.map_err(|_| ())?;
        Ok(AstNode::Function{ 
            id: $lexer.span_str(id.span()).to_string(),
            params: vec![],
            block: $6?
        }) 
     }
    | 
    "FUNCTION" "IDENTIFIER" "(" ParamList ")" "{" Statements "}" { 
        let id = $2.map_err(|_| ())?;
        Ok(AstNode::Function{ 
            id: $lexer.span_str(id.span()).to_string(),
            params: $4.map_err(|_| ())?,
            block: $7?
        }) 
     }
    ;


Return -> Result<AstNode, ()>:
    "RETURN" Statement { Ok(AstNode::Return{ block: Box::new($2?) }) };

Unmatched -> ():
      "UNMATCHED" { };
%%

use crate::ast::AstNode;

fn append(mut lhs: Vec<AstNode>, rhs: AstNode ) -> Result<Vec<AstNode>, ()>{
    lhs.push(rhs);
    Ok(lhs)
}

fn parse_number(s: &str) -> Result<AstNode, ()> {
    match s.parse::<u64>() {
        Ok(n_val) => Ok(AstNode::Number{ value: n_val }),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}

fn parse_boolean(s: &str) -> Result<AstNode, ()> {
    match s.parse::<bool>() {
        Ok(n_val) => Ok(AstNode::Boolean{ value: n_val }),
        Err(_) => {
            eprintln!("{} cannot be represented as a boolean", s);
            Err(())
        }
    }
}
