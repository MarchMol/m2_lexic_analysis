%{
delim         [ \n\s]
int           [0-9]+
%}

%%
[0-9]+                          { INT }
[0-9]+"."[0-9]*                  {FLOAT}
[a-z]+                          { ID }
"while"                         { WHILE }
">"                             { GT }
">="                            { GTE }
"<"                             { LT }
"<="                            { LTE }
(" "|"\n"|"\t"|"\s")+           { DELIM }
%%