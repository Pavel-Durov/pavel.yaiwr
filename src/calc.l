%%
[0-9]+ "INTEGER"
"(?:\\\\|\\"|[^"])*" "STRING"
_[a-z] "ID"
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
