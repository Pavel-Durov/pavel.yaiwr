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

Function -> Result<AstNode, ()>:
    "FUNCTION" "ID" "(" ParamList ")" "{" "}" { 
        let id = $2.map_err(|_| ())?;
        Ok(AstNode::Function{ 
            id: $lexer.span_str(id.span()).to_string(),
            params: $4.map_err(|_| ())?
        }) 
     }
    ;


ParamList -> Result<Vec<AstNode>, ()>:
    ParamList ',' Expr { combine($1.map_err(|_| ())?, $3.map_err(|_| ())?) }
    | Expr { 
        Ok(vec![$1.map_err(|_| ())?])
     }
    ;

Term -> Result<AstNode, ()>:
      Term 'MUL' Factor { Ok(AstNode::Mul{ lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Factor { $1 }
    ;

Factor -> Result<AstNode, ()>:
    "(" Expr ")" { $2 }
    | "INTEGER" { 
        let v = $1.map_err(|_| ())?;
        parse_int($lexer.span_str(v.span()))
      }
    | "ID" { 
       let v = $1.map_err(|_| ())?;
       Ok(AstNode::ID{ value: $lexer.span_str(v.span()).to_string() })
    }
    ;



Unmatched -> ():
      "UNMATCHED" { };
%%

use crate::ast::AstNode;
use lrlex::DefaultLexeme;

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
