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


fn parse_int(s: &str) -> Result<AstNode, ()> {
    match s.parse::<u64>() {
        Ok(n_val) => Ok(AstNode::Number{ value: n_val }),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}
