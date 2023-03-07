%start AstNode
%avoid_insert "INT"
%expect-unused Unmatched "UNMATCHED"
%%

AstNode -> Result<AstNode, ()>:
      AstNode '+' Term {
        Ok(AstNode::Add{ 
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Term { $1 }
    | "PRINT" Term { Ok(AstNode::Print{ rhs: Box::new($2?) }) }
    ;

Term -> Result<AstNode, ()>:
      Term '*' Factor {
        Ok(AstNode::Mul{  
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Factor { $1 }
    ;

Factor -> Result<AstNode, ()>:
      '(' AstNode ')' { $2 }
    | 'INT' { 
        match $1.map_err(|err| format!("Parsing Error: {}", err)) {
            Ok(s) => {
              let s = $lexer.span_str(s.span());
              match s.parse::<u64>() {
                  Ok(n_val) => Ok(AstNode::Number{ value: n_val }),
                  Err(_) => Err(())
              }
            }
            Err(_) => Err(())
        }
      }
    ;


// Print -> Result<AstNode, ():    "PRINT" { Ok(AstNode::Print{  rhs: Box::new($2?) }) };
Unmatched -> ():
      "UNMATCHED" { };
%%
use crate::ast::AstNode;
