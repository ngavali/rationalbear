---- MODULE MC ----
EXTENDS AwsDynamoDbProblem, TLC

\* MV CONSTANT declarations@modelParameterConstants
CONSTANTS
e1, e2, e3
----

\* MV CONSTANT definitions Enactors
const_17719644631527000 == 
{e1, e2, e3}
----

\* SYMMETRY definition
symm_17719644631528000 == 
Permutations(const_17719644631527000)
----

\* CONSTANT definitions @modelParameterConstants:0PLAN_THRESHOLD
const_17719644631529000 == 
2
----

\* CONSTANT definitions @modelParameterConstants:1MAX_PLANS
const_177196446315210000 == 
20
----

=============================================================================
\* Modification History
\* Created Wed Feb 25 01:51:03 IST 2026 by ngavali
