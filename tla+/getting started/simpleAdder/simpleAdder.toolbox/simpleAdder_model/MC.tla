---- MODULE MC ----
EXTENDS simpleAdder, TLC

\* Constant expression definition @modelExpressionEval
const_expr_17532724249519000 == 
CHOOSE y \in {1, 2, 3} : y*y = 4
----

\* Constant expression ASSUME statement @modelExpressionEval
ASSUME PrintT(<<"$!@$!@$!@$!@$!",const_expr_17532724249519000>>)
----

\* INIT definition @modelBehaviorNoSpec:0
init_175327242495110000 ==
FALSE/\x = 0
----
\* NEXT definition @modelBehaviorNoSpec:0
next_175327242495111000 ==
FALSE/\x' = x
----
=============================================================================
\* Modification History
\* Created Wed Jul 23 17:37:04 IST 2025 by ngavali
