%%
[0-9]+ "INTEGER"
"(?:\\\\|\\"|[^"])*" "STRING"
_[a-zA-Z0-9]+ "ID"
\+ "ADD"
\* "MUL"
\( "("
\) ")"
; ";"
= "="
println "PRINT_LN" 
let "ASSIGN"
[\t\n ]+ ;
. "UNMATCHED"
