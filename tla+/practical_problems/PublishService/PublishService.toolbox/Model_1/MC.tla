---- MODULE MC ----
EXTENDS PublishService, TLC

\* CONSTANT definitions @modelParameterConstants:0OutputCount
const_176219676255038000 == 
300
----

\* CONSTANT definitions @modelParameterConstants:1NodeIpAddr
const_176219676255039000 == 
[ 
	N1 |-> 1, 
	N2 |-> 2 
]
----

\* CONSTANT definitions @modelParameterConstants:2Nodes
const_176219676255040000 == 
{ "N1", "N2" }
----

\* CONSTANT definitions @modelParameterConstants:3BeginNodeHealth
const_176219676255041000 == 
1
----

\* CONSTANT definitions @modelParameterConstants:4LeadershipBonus
const_176219676255042000 == 
5
----

\* CONSTANT definitions @modelParameterConstants:6MaxNodeIterations
const_176219676255043000 == 
2
----

\* CONSTANT definitions @modelParameterConstants:7BeginNodeOutputCount
const_176219676255044000 == 
1
----

\* CONSTANT definitions @modelParameterConstants:8BeginNodeIteration
const_176219676255045000 == 
1
----

=============================================================================
\* Modification History
\* Created Tue Nov 04 00:36:02 IST 2025 by ngavali
