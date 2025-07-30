---- MODULE MC ----
EXTENDS wire, TLC

\* Constant expression definition @modelExpressionEval
const_expr_1753638920448183000 == 
[type: {"trash", "recycle"}, size: 1..6] \X [type: {"trash", "recycle"}, size: 1..6]
----

\* Constant expression ASSUME statement @modelExpressionEval
ASSUME PrintT(<<"$!@$!@$!@$!@$!",const_expr_1753638920448183000>>)
----

=============================================================================
\* Modification History
\* Created Sun Jul 27 23:25:20 IST 2025 by ngavali
