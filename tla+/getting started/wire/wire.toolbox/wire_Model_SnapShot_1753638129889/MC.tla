---- MODULE MC ----
EXTENDS wire, TLC

\* Constant expression definition @modelExpressionEval
const_expr_1753638126718134000 == 
[type: {"trash", "recycle"}, size: 1..6] \X [type: {"trash", "recycle"}, size: 1..6] \X [type: {"trash", "recycle"}, size: 1..6] \X [type: {"trash", "recycle"}, size: 1..6]
----

\* Constant expression ASSUME statement @modelExpressionEval
ASSUME PrintT(<<"$!@$!@$!@$!@$!",const_expr_1753638126718134000>>)
----

\* INIT definition @modelBehaviorNoSpec:0
init_1753638126718135000 ==
FALSE/\sender = 0
----
\* NEXT definition @modelBehaviorNoSpec:0
next_1753638126718136000 ==
FALSE/\sender' = sender
----
=============================================================================
\* Modification History
\* Created Sun Jul 27 23:12:06 IST 2025 by ngavali
