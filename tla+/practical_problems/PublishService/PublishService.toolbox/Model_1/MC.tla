---- MODULE MC ----
EXTENDS PublishService, TLC

\* CONSTANT definitions @modelParameterConstants:0OutputCount
const_1762197053134132000 == 
300
----

\* CONSTANT definitions @modelParameterConstants:1NodeIpAddr
const_1762197053134133000 == 
[ 
	N1 |-> 1, 
	N2 |-> 2 
]
----

\* CONSTANT definitions @modelParameterConstants:2Nodes
const_1762197053134134000 == 
{ "N1", "N2" }
----

\* CONSTANT definitions @modelParameterConstants:3BeginNodeHealth
const_1762197053134135000 == 
1
----

\* CONSTANT definitions @modelParameterConstants:4LeadershipBonus
const_1762197053134136000 == 
5
----

\* CONSTANT definitions @modelParameterConstants:6MaxNodeIterations
const_1762197053134137000 == 
2
----

\* CONSTANT definitions @modelParameterConstants:7BeginNodeOutputCount
const_1762197053134138000 == 
1
----

\* CONSTANT definitions @modelParameterConstants:8BeginNodeIteration
const_1762197053134139000 == 
1
----

=============================================================================
\* Modification History
\* Created Tue Nov 04 00:40:53 IST 2025 by ngavali
