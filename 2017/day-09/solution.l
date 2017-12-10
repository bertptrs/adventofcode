%{
#include <stdio.h>

int nest = 0;
int score = 0;
int garbage = 0;
%}
%x GARBAGE
%%
\{ ++nest;
\} score += nest--;
, ;
\< BEGIN GARBAGE;

<GARBAGE>!. ;
<GARBAGE>\> BEGIN 0;
<GARBAGE>. ++garbage;

%%

int yywrap()
{
	return 1;
}

int main()
{
	yylex();
	printf("Groups: %d\nGarbage: %d\n", score, garbage);
}
