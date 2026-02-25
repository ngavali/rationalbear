---- MODULE MC ----
EXTENDS AwsDynamoDbProblem, TLC

\* MV CONSTANT declarations@modelParameterConstants
CONSTANTS
e1, e2, e3
----

\* MV CONSTANT definitions Enactors
const_1772041577860352000 == 
{e1, e2, e3}
----

\* SYMMETRY definition
symm_1772041577860353000 == 
Permutations(const_1772041577860352000)
----

\* CONSTANT definitions @modelParameterConstants:0PLAN_THRESHOLD
const_1772041577860354000 == 
2
----

\* CONSTANT definitions @modelParameterConstants:1MAX_PLANS
const_1772041577860355000 == 
5
----

=============================================================================
\* Modification History
\* Created Wed Feb 25 23:16:17 IST 2026 by ngavali
