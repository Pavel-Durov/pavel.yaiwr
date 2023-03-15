%start Expr
%avoid_insert "INTEGER"
%expect-unused Unmatched "UNMATCHED"
%%

Expr -> Result<AstNode, ()>:
    Expr "ADD" Term { Ok(AstNode::Add{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Term { $1 } 
    | "PRINT_LN" "(" Expr ")" ";" { Ok(AstNode::PrintLn{ rhs: Box::new($3?) }) }
    | "ASSIGN" "ID" "=" Expr ";" { 
       let v = $2.map_err(|_| ())?;
       Ok(AstNode::Assign{ id: $lexer.span_str(v.span()).to_string(), rhs: Box::new($4?) }) 
     }
    | Function { $1 }
    ;


ParamList -> Result<Vec<AstNode>, ()>:
    ParamList ',' Id { combine($1.map_err(|_| ())?, $3.map_err(|_| ())?) }
    | Id {  Ok(vec![$1.map_err(|_| ())?]) }
    ;

Function -> Result<AstNode, ()>:
    "FUNCTION" "ID" "(" ")" "{" Expr "}" { 
        let id = $2.map_err(|_| ())?;
        Ok(AstNode::Function{ 
            id: $lexer.span_str(id.span()).to_string(),
            params: vec![],
            block: Box::new($6?)
        }) 
     }
    | "FUNCTION" "ID" "(" ParamList ")" "{" Expr "}" { 
        let id = $2.map_err(|_| ())?;
        Ok(AstNode::Function{ 
            id: $lexer.span_str(id.span()).to_string(),
            params: $4.map_err(|_| ())?,
            block: Box::new($7?)
        }) 
     }
    ;

ExprList -> Result<Vec<AstNode>, ()>:
    ExprList ',' Expr { combine($1.map_err(|_| ())?, $3.map_err(|_| ())?) }
    | Expr {  Ok(vec![$1.map_err(|_| ())?]) }
    ;

Term -> Result<AstNode, ()>:
      Term 'MUL' CallableExpr { Ok(AstNode::Mul{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | CallableExpr { $1 }
    ;

CallableExpr -> Result<AstNode, ()>:
    "(" Expr ")" { $2 }
    | "RETURN" Expr ";" { Ok(AstNode::Return{ block: Box::new($2?) }) }
    | Integer { $1 }
    | Id { $1 }
    | FunctionCall { $1 }
    ;

Integer -> Result<AstNode, ()>:
    "INTEGER" { parse_int($lexer.span_str(($1.map_err(|_| ())?).span())) }
    ;

Id -> Result<AstNode, ()>:
    "ID" { 
       let v = $1.map_err(|_| ())?;
       Ok(AstNode::ID{ value: $lexer.span_str(v.span()).to_string() })
    };

FunctionCall -> Result<AstNode, ()>:
     "ID" "(" ")" ";" { 
        let id = $1.map_err(|_| ())?;
        Ok(AstNode::FunctionCall{ 
            id: $lexer.span_str(id.span()).to_string(),
            args: vec![]
        })
    }
    |
      "ID" "(" ExprList ")" ";" { 
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

fn combine(mut lhs: Vec<AstNode>, rhs: AstNode ) -> Result<Vec<AstNode>, ()>{
    lhs.push(rhs);
    Ok(lhs)
}

fn parse_int(s: &str) -> Result<AstNode, ()> {
    match s.parse::<u64>() {
        Ok(n_val) => Ok(AstNode::Number{ value: n_val }),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}
