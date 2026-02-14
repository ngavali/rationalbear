---- MODULE MC ----
EXTENDS AwsDynamoDbProblem, TLC

\* MV CONSTANT declarations@modelParameterConstants
CONSTANTS
e1, e2, e3
----

\* MV CONSTANT definitions Enactors
const_1771099508595385000 == 
{e1, e2, e3}
----

\* CONSTANT definitions @modelParameterConstants:0PLAN_THRESHOLD
const_1771099508595386000 == 
2
----

\* CONSTANT definitions @modelParameterConstants:1MAX_PLANS
const_1771099508595387000 == 
20
----

=============================================================================
\* Modification History
\* Created Sun Feb 15 01:35:08 IST 2026 by ngavali
