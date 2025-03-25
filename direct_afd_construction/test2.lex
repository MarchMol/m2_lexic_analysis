%{
delim         [ \n\s]
int           [0-9]+
%}

%%
[0-9]+                          { INT }
[0-9]+"."[0-9]*                  {FLOAT}
"while"                         { WHILE }
">"                             { GT }
">="                            { GTE }
"<"                             { LT }
"<="                            { LTE }
[a-z]+                          { ID }
(" "|"\n"|"\t"|"\s")+           { DELIM }
%%