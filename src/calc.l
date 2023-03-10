%%
[0-9]+ "INTEGER"
"(?:\\\\|\\"|[^"])*" "STRING"
_[a-zA-Z0-9_]+ "ID"
\+ "ADD"
\* "MUL"
\( "("
\) ")"
; ";"
, ","
= "="

println "PRINT_LN" 
let "ASSIGN"
fun "T_FUNCTION"
list "T_LIST"
[\t\n ]+ ;
. "UNMATCHED"
