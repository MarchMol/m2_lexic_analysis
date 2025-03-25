%{
delim         [ \n\s]
int           [0-9]+
%}

%%
"-"[0-9]+                    { SIGNED }
(-?)[0-9]+"."[0-9]*                    { FLOAT }
[0-9]+                          { INT }
"while"                         { WHILE }
" "+                             { DELIM }
%%