%{
delim         [ \n\s]
int           [0-9]+
%}

%%
(-?)[0-9]+"."[0-9]*             { FLOAT }
"-"[0-9]+                       { SIGNED }
[0-9]+                          { INT }
"while"                         { WHILE }
[a-z]+                          { ID }
">"                             { GT }
">="                            { GTE }
"<"                             { LT }
"<="                            { LTE }
(" "|"\n"|"\t"|"\s")+           { DELIM }
%%