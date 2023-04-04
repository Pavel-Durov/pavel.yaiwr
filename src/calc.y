%start Statements
%expect-unused Unmatched "UNMATCHED"
%%

Statements -> Result<Vec<AstNode>, ()>:
    Statements Expr { append($1.map_err(|_| ())?, $2.map_err(|_| ())?)  }
  | { Ok(vec![]) }
  ;

Expr -> Result<AstNode, ()>:
    Expr "ADD" Term { Ok(AstNode::Add{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Term { $1 } 
    | Builtins { $1 }
    | AssigVar { $1 }
    | FunctionDeclaration { $1 }
    | Expr "LESS_THAN" Term { Ok(AstNode::LessThan{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Expr "GREATER_THAN" Term { Ok(AstNode::GreaterThan{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    ;

Term -> Result<AstNode, ()>:
      Term 'MUL' CallableExpr { Ok(AstNode::Mul{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | CallableExpr { $1 }
    ;


Builtins -> Result<AstNode, ()>:
    "PRINT_LN" "(" Expr ")" ";" { Ok(AstNode::PrintLn{ rhs: Box::new($3?) }) };

AssigVar -> Result<AstNode, ()>:
    "ASSIGN" "T_VARIABLE" "=" Expr ";" { 
        Ok(AstNode::Assign { 
            id: $lexer.span_str(($2.map_err(|_| ())?).span()).to_string(), rhs: Box::new($4?) 
        })
     };


Id -> Result<AstNode, ()>:
    "T_VARIABLE" { 
        Ok(AstNode::ID { 
            value: $lexer.span_str(($1.map_err(|_| ())?).span()).to_string() 
        })
    };

Literal -> Result<AstNode, ()>:
    "INTEGER_LITERAL" { parse_number($lexer.span_str(($1.map_err(|_| ())?).span())) }
    | "BOOLEAN_LITERAL" { parse_boolean($lexer.span_str(($1.map_err(|_| ())?).span())) }
    ;

CallableExpr -> Result<AstNode, ()>:
     "(" Expr ")" { $2 }
    | Literal { $1 }
    | FunctionCall { $1 }
    | Id { $1 }
    | Return { $1 }
    ;

// Function Declaration

ParamList -> Result<Vec<AstNode>, ()>:
    ParamList ',' Id { append($1.map_err(|_| ())?, $3.map_err(|_| ())?) }
    | Id {  Ok(vec![$1.map_err(|_| ())?]) }
    ;

FunctionDeclaration -> Result<AstNode, ()>:
    "FUNCTION" "T_FUNCTION" "(" ")" "{" Statements "}" { 
        let id = $2.map_err(|_| ())?;
        Ok(AstNode::Function{ 
            id: $lexer.span_str(id.span()).to_string(),
            params: vec![],
            block: $6?
        }) 
     }
    | 
    "FUNCTION" "T_FUNCTION" "(" ParamList ")" "{" Statements "}" { 
        let id = $2.map_err(|_| ())?;
        Ok(AstNode::Function{ 
            id: $lexer.span_str(id.span()).to_string(),
            params: $4.map_err(|_| ())?,
            block: $7?
        }) 
     }
    ;

Return -> Result<AstNode, ()>:
    "RETURN" Expr ";" { Ok(AstNode::Return{ block: Box::new($2?) }) };

// Function Call

ArgList -> Result<Vec<AstNode>, ()>:
    ArgList ',' Expr { append($1.map_err(|_| ())?, $3.map_err(|_| ())?) }
    | Expr {  Ok(vec![$1.map_err(|_| ())?]) }
    ;


FunctionCall -> Result<AstNode, ()>:
    "T_FUNCTION" "(" ")" ";"{ 
        let id = $1.map_err(|_| ())?;
        Ok(AstNode::FunctionCall{ 
            id: $lexer.span_str(id.span()).to_string(),
            args: vec![]
        })
    }
    | "T_FUNCTION" "(" ArgList ")" ";" { 
        let id = $1.map_err(|_| ())?;
        Ok(AstNode::FunctionCall{ 
            id: $lexer.span_str(id.span()).to_string(),
            args: $3.map_err(|_| ())?
        })
      }
    ;

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
