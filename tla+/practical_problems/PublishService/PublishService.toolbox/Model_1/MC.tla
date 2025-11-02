---- MODULE MC ----
EXTENDS PublishService, TLC

\* CONSTANT definitions @modelParameterConstants:0OutputCount
const_176211155252763000 == 
20
----

\* CONSTANT definitions @modelParameterConstants:1NodeIpAddr
const_176211155252764000 == 
[ 
	N1 |-> 1, 
	N2 |-> 2 
]
----

\* CONSTANT definitions @modelParameterConstants:2Nodes
const_176211155252765000 == 
{ "N1", "N2" }
----

\* CONSTANT definitions @modelParameterConstants:3BeginNodeHealth
const_176211155252766000 == 
1
----

\* CONSTANT definitions @modelParameterConstants:4LeadershipBonus
const_176211155252767000 == 
5
----

\* CONSTANT definitions @modelParameterConstants:6MaxNodeIterations
const_176211155252768000 == 
2
----

=============================================================================
\* Modification History
\* Created Mon Nov 03 00:55:52 IST 2025 by ngavali
