
%start StatementList
%%

StatementList -> Result<Vec<AstNode>, ()>:
    StatementList Statement { append($1.map_err(|_| ())?, $2.map_err(|_| ())?)  }
    | { Ok(vec![]) }
    ;


SelectionStatement -> Result<AstNode, ()>:
    'IF' '(' Expression ')' '{' StatementList '}' { 
        Ok(AstNode::Conditional{
            condition:  Box::new($3?),
            block: $6?,
            alternative: None
        }) 
    }
    | 'IF' '(' Expression ')' '{' StatementList '}' 'ELSE' '{' StatementList '}' {  
        Ok(AstNode::Conditional{
            condition:  Box::new($3?),
            block: $6?,
            alternative: Some($10?)
        }) 
    }
    ;

Statement -> Result<AstNode, ()>:
    ExpressionStatement { $1 }
    | FunctionDefinition { $1 }
    | SelectionStatement { $1 }
    | Builtins { $1 }
    | 'RETURN' Expression ';' { Ok(AstNode::Return{ block: Box::new($2?) }) }
    ;

ExpressionStatement -> Result<AstNode, ()>:
    ';' { Ok(AstNode::Empty{}) }
    | Expression ';' { $1 }
    ;

Expression -> Result<AstNode, ()>:
    AssignmentExpression { $1 }
    | Expression ',' AssignmentExpression { $1 }
    ;


RelationalExpression -> Result<AstNode, ()>: 
    AdditiveExpression { $1 }
    | RelationalExpression 'LESS_THAN' AdditiveExpression {
        Ok(AstNode::LessThan{ lhs: Box::new($1?), rhs: Box::new($3?) }) 
    }
    | RelationalExpression 'GREATER_THAN' AdditiveExpression {
        Ok(AstNode::GreaterThan{ lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;

EqualityExpression -> Result<AstNode, ()>: 
    RelationalExpression { $1 }
    | EqualityExpression 'EQEQ' RelationalExpression { 
        Ok(AstNode::Equal { lhs: Box::new($1?), rhs: Box::new($3?) })
    }
	| EqualityExpression 'NOTEQ' RelationalExpression { 
        Ok(AstNode::NotEqual { lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;

LogincalAndExpression -> Result<AstNode, ()>:
    EqualityExpression { $1 }
    | LogincalAndExpression 'AND' EqualityExpression { 
        Ok(AstNode::LogicalAnd{ lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;

LogincalOrExpression -> Result<AstNode, ()>:
    LogincalAndExpression { $1 }
    | LogincalOrExpression 'OR' LogincalAndExpression { 
        Ok(AstNode::LogicalOr{ lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;
    
ConditionalExpression -> Result<AstNode, ()>:
	LogincalOrExpression { $1 }
	;

AssignmentExpression -> Result<AstNode, ()>:
    ConditionalExpression { $1 }
    | UnaryExpression '=' AssignmentExpression {
        match $1.map_err(|_| ())? {
            AstNode::ID { value } => {
                Ok(AstNode::Assign { id: value, rhs: Box::new($3?) })
            },
            _ => Err(())
        }
    }
    | 'LET' UnaryExpression '=' AssignmentExpression {
        match $2.map_err(|_| ())? {
            AstNode::ID { value } => {
                Ok(AstNode::Declare { id: value, rhs: Some(Box::new($4?)) })
            },
            _ => Err(())
        }
    } 
    ;

AdditiveExpression -> Result<AstNode, ()>:
    MultiplicativeExpression { $1 }
    | AdditiveExpression 'PLUS' MultiplicativeExpression { 
        Ok(AstNode::Add{ lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;

MultiplicativeExpression -> Result<AstNode, ()>: 
    UnaryExpression { $1 }
    | MultiplicativeExpression 'MUL' UnaryExpression { 
      Ok(AstNode::Mul{ lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;

UnaryExpression -> Result<AstNode, ()>: 
    PostfixExpression { $1 }
    | UnaryExpression 'INC_OP' { 
        match $1.map_err(|_| ())? {
            AstNode::ID { value: id } => Ok(AstNode::IncrementId{ id }) ,
            AstNode::Number { value: _ } => Ok(AstNode::IncrementLiteral{ }) ,
            _ => Err(())
        }
    }
    ;

PostfixExpression -> Result<AstNode, ()>:
    PrimaryExpression { $1 }
  | PostfixExpression '(' ')' { 
        match $1.map_err(|_| ())? {
            AstNode::ID { value: id } => Ok(AstNode::FunctionCall{ id, args: vec![] }),
            _ => Err(())
        }
    }
  | PostfixExpression '(' ArgumentExpressionList ')' { 
        match $1.map_err(|_| ())? {
        AstNode::ID { value: id } => Ok(AstNode::FunctionCall{ id, args: $3.map_err(|_| ())? }),
        _ => Err(())
        }
   }
  ;
    
ArgumentExpressionList -> Result<Vec<AstNode>, ()>:
    AssignmentExpression {  Ok(vec![$1.map_err(|_| ())?]) }
    | ArgumentExpressionList ',' AssignmentExpression { append($1.map_err(|_| ())?, $3.map_err(|_| ())?)  }
    ;
  
Id -> Result<AstNode, ()>:
  'IDENTIFIER' { Ok(AstNode::ID { value: $lexer.span_str(($1.map_err(|_| ())?).span()).to_string() }) }
  ;

PrimaryExpression -> Result<AstNode, ()>:
    Id { $1 }
    |  '(' Expression ')' { $2 }
    | Literals { $1 }
    ;

Literals -> Result<AstNode, ()>:
    'INTEGER_LITERAL' { parse_int($lexer.span_str(($1.map_err(|_| ())?).span())) }
    | 'BOOLEAN_LITERAL' { parse_boolean($lexer.span_str(($1.map_err(|_| ())?).span())) }
    ;

ParamList -> Result<Vec<AstNode>, ()>:
    ParamList ',' Id { append($1.map_err(|_| ())?, $3.map_err(|_| ())?) }
    | Id { Ok(vec![$1.map_err(|_| ())?]) }
    ;

FunctionDefinition -> Result<AstNode, ()>:
    'FUNCTION' 'IDENTIFIER' '(' ')' '{' StatementList '}' { 
        let id = $2.map_err(|_| ())?;
        Ok(AstNode::Function{ 
            id: $lexer.span_str(id.span()).to_string(),
            params: vec![],
            block: $6?
        }) 
     }
    | 
    'FUNCTION' 'IDENTIFIER' '(' ParamList ')' '{' StatementList '}' { 
        let id = $2.map_err(|_| ())?;
        Ok(AstNode::Function{ 
            id: $lexer.span_str(id.span()).to_string(),
            params: $4.map_err(|_| ())?,
            block: $7?
        }) 
     }
    ;

Builtins -> Result<AstNode, ()>:
    'PRINT_LN' '(' Expression ')' { Ok(AstNode::PrintLn{ rhs: Box::new($3?) }) };

%%
use crate::ast::AstNode;

fn append(mut lhs: Vec<AstNode>, rhs: AstNode ) -> Result<Vec<AstNode>, ()>{
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

fn parse_boolean(s: &str) -> Result<AstNode, ()> {
    match s.parse::<bool>() {
        Ok(n_val) => Ok(AstNode::Boolean{ value: n_val }),
        Err(_) => {
            eprintln!("{} cannot be represented as a boolean", s);
            Err(())
        }
    }
}
