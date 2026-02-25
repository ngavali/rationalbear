---- MODULE MC ----
EXTENDS AwsDynamoDbProblem, TLC

\* MV CONSTANT declarations@modelParameterConstants
CONSTANTS
e1, e2, e3
----

\* MV CONSTANT definitions Enactors
const_1772039939274341000 == 
{e1, e2, e3}
----

\* SYMMETRY definition
symm_1772039939275342000 == 
Permutations(const_1772039939274341000)
----

\* CONSTANT definitions @modelParameterConstants:0PLAN_THRESHOLD
const_1772039939275343000 == 
2
----

\* CONSTANT definitions @modelParameterConstants:1MAX_PLANS
const_1772039939275344000 == 
5
----

=============================================================================
\* Modification History
\* Created Wed Feb 25 22:48:59 IST 2026 by ngavali
