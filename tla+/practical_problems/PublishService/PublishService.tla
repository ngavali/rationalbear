--------------------------- MODULE PublishService ---------------------------

(*
 * Try setting BeginNodeHealth to 
 * 1. 1 
 * 2. MaxNodeHealth without LeadershipBonus 
 * 3. Higher than (2)
 *)

EXTENDS Naturals, Sequences, FiniteSets, TLC

CONSTANTS 
    LeadershipBonus,
    MaxNodeIterations,
    BeginNodeHealth,
    OutputCount,
    NodeIpAddr,
    None,
    Nodes

ASSUME IsFiniteSet(Nodes)
ASSUME IsFiniteSet(NodeIpAddr)

VARIABLES 
    NodeHealth,      \* [node -> NodeHealth value]
    NodeIteration,   \* [node -> NodeIteration count]
    Leader,          \* current Leader node or Null
    NodeOutput,
    pc,
    NodePublishLog   \* set of nodes that have published in this NodeIteration

States == {"ELECT_LEADER", "ITERATION_START", "GENERATE_OUTPUT", "PREPARE_PUBLISH", "EXEC_PUBLISH", "SET_HEALTH_BEFORE", "SET_HEALTH_AFTER", "ITERATION_COMPLETE", "DONE"}

MaxNodeHealth == OutputCount + LeadershipBonus

\* --- Helpers ---
\* Find Max in Set
Max(S) == IF S = {} THEN 0
    ELSE CHOOSE x \in S : \A y \in S : x >= y
\* Find Min in Set
Min(S) == IF S = {} THEN 0
    ELSE CHOOSE x \in S : \A y \in S : x <= y
\* Sum up all elements in Set
RECURSIVE SumSet(_)
SumSet(S) ==
    IF S = {} THEN 0
    ELSE LET x == CHOOSE v \in S : TRUE IN x + SumSet(S \ {x})

\* --- Initial state ---
Init ==
    /\ pc =             [n \in Nodes |-> "ELECT_LEADER"]
    /\ NodeHealth =     [n \in Nodes |-> BeginNodeHealth]
    /\ NodeIteration =  [n \in Nodes |-> 1]
    /\ NodeOutput =     [n \in Nodes |-> 1]
    /\ NodePublishLog = [n \in Nodes |-> 0]
    /\ Leader =         None

(* --- Elect Leader ---
 * Or Alteast pretend to do so
 * Bonus is not applied in this step
 * You just elect a leader
 *)

ELECT_LEADER(n) ==
    /\ pc[n] = "ELECT_LEADER"
    /\ pc' = [ n1 \in Nodes |-> "ITERATION_START" ]
    /\ LET maxH == Max({ NodeHealth[n1] : n1 \in Nodes })
           maxNodes == { n1 \in Nodes : NodeHealth[n1] = maxH }
       IN Leader' = CHOOSE n1 \in maxNodes : NodeIpAddr[n1] = Max({ NodeIpAddr[nx] : nx \in maxNodes })
    /\ UNCHANGED << NodeIteration, NodeHealth, NodeOutput, NodePublishLog >>

(* --- Start NodeIteration Cycle
 * NodeIteration starts from 1
 * If NodeIteration < MaxNodeIteration proceed to generate NodeOutput
 * otherwise end this cycle and mark as DONE
 *)

ITERATION_START(n) ==
    /\ pc[n] = "ITERATION_START"
    /\ pc' = [pc EXCEPT ![n] = IF NodeIteration[n] > MaxNodeIterations THEN "DONE" ELSE "SET_HEALTH_BEFORE"]
    /\ NodeHealth' = [NodeHealth EXCEPT ![n] = NodeOutput[n] + IF Leader = n THEN LeadershipBonus ELSE 0]
    /\ UNCHANGED << NodeOutput, NodePublishLog, Leader, NodeIteration >>

GENERATE_OUTPUT(n) ==
    /\ pc[n] = "GENERATE_OUTPUT"
    /\ pc' = [pc EXCEPT ![n] = "PREPARE_PUBLISH"]
    /\ NodeOutput' = [NodeOutput EXCEPT ![n] = OutputCount]
    /\ UNCHANGED << NodeHealth, NodePublishLog, NodeIteration, Leader >>

\* --- Leader publishes NodeOutput (only once per cycle) ---
PREPARE_PUBLISH(n) ==
    /\ pc[n] = "PREPARE_PUBLISH"
    /\ IF n = Leader THEN
              pc' = [pc EXCEPT ![n] = "EXEC_PUBLISH"]
            ELSE
              pc' = [pc EXCEPT ![n] = "SET_HEALTH_AFTER"]
    /\ UNCHANGED << NodeHealth, Leader, NodeIteration, NodePublishLog, NodeOutput >>

EXEC_PUBLISH(n) ==
    /\ pc[n] = "EXEC_PUBLISH"
    /\ NodePublishLog' = [NodePublishLog EXCEPT ![n] = @ + 1]
    /\ pc' = [pc EXCEPT ![n] = "SET_HEALTH_AFTER"]
    /\ UNCHANGED << NodeHealth, Leader, NodeIteration, NodeOutput >>

SET_HEALTH_BEFORE(n) == 
    /\ pc[n] = "SET_HEALTH_BEFORE"
    /\ pc' = [pc EXCEPT ![n] = "GENERATE_OUTPUT"]
    /\ NodeHealth' = [NodeHealth EXCEPT ![n] = NodeOutput[n] + IF Leader = n THEN LeadershipBonus ELSE 0]
    /\ LET maxH == Max({ NodeHealth[n1] : n1 \in Nodes })
           maxNodes == { n1 \in Nodes : NodeHealth[n1] = maxH }
        IN Leader' = CHOOSE n1 \in maxNodes : NodeIpAddr[n1] = Max({ NodeIpAddr[nx] : nx \in maxNodes })
    /\ UNCHANGED << NodeIteration, NodeOutput, NodePublishLog >>

SET_HEALTH_AFTER(n) == 
    /\ pc[n] = "SET_HEALTH_AFTER"
    /\ pc' = [pc EXCEPT ![n] = "ITERATION_COMPLETE"]
    /\ NodeHealth' = [NodeHealth EXCEPT ![n] = NodeOutput[n] + IF Leader = n THEN LeadershipBonus ELSE 0]
    /\ LET maxH == Max({ NodeHealth[n1] : n1 \in Nodes })
           maxNodes == { n1 \in Nodes : NodeHealth[n1] = maxH }
        IN Leader' = CHOOSE n1 \in maxNodes : NodeIpAddr[n1] = Max({ NodeIpAddr[nx] : nx \in maxNodes })
    /\ UNCHANGED << NodeIteration, NodeOutput, NodePublishLog >>

ITERATION_COMPLETE(n) ==
    /\ pc[n] = "ITERATION_COMPLETE"
    /\ pc' = [pc EXCEPT ![n] = "ITERATION_START"] 
    /\ NodeIteration' = [NodeIteration EXCEPT ![n] = @ + 1]
    /\ UNCHANGED <<NodeHealth, NodeOutput, Leader, NodePublishLog >>

TypeOK ==
    /\ Leader \in Nodes \cup {None}
    /\ NodeIteration \in [Nodes -> Nat]
    /\ NodeOutput \in [Nodes -> Nat]
    /\ NodePublishLog \in [Nodes -> Nat]
    /\ pc \in [Nodes -> States]
    
AllStopped == \A n \in Nodes : pc[n] = "DONE"

vars == <<NodeHealth, NodeIteration, NodeOutput, Leader, pc, NodePublishLog >>

Done ==
    AllStopped
    /\ UNCHANGED vars

Next ==
        \/ \E n \in Nodes : ELECT_LEADER(n)
        \/ \E n \in Nodes : SET_HEALTH_BEFORE(n)
        \/ \E n \in Nodes : SET_HEALTH_AFTER(n)
        \/ \E n \in Nodes : ITERATION_START(n)
        \/ \E n \in Nodes : GENERATE_OUTPUT(n)
        \/ \E n \in Nodes : PREPARE_PUBLISH(n)
        \/ \E n \in Nodes : EXEC_PUBLISH(n)
        \/ \E n \in Nodes : ITERATION_COMPLETE(n)
        \/ Done 

(* --- Termination condition (optional) ---
Termination ==
    Done
*)
\* Invariants
\* We MUST have a Leader after ELECTLeader is done
LeaderElectedOK ==
    \/ Leader # None
    \/ \E n \in Nodes : pc[n] \in {"ELECT_LEADER"}

HealthOverShoot == 
    /\ \A n \in Nodes : NodeHealth[n] <= MaxNodeHealth
(* Publish must have happened after each node completes it's NodeIteration
 * 
 *)

MustPublish ==
    LET totalPub == SumSet({ NodePublishLog[m] : m \in Nodes })
    IN  totalPub >= Min({ NodeIteration[m] : m \in Nodes }) - 1

(* Multiple Publishes
 * 
 *)

NoOverPublish ==
    LET totalPub == SumSet({ NodePublishLog[m] : m \in Nodes })
    IN  totalPub <= Max({ NodeIteration[m] : m \in Nodes })

Spec == Init /\ [][Next]_vars

===============================================================================
