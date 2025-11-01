--------------------------- MODULE DistributedLeader ---------------------------

(*
 * Try setting BeginHealth to 
 * 1. 1 
 * 2. MaxHealth without LeadershipBonus 
 * 3. Higher than (2)
 *)

EXTENDS Naturals, Sequences, TLC

CONSTANTS 
    LeadershipBonus,
    MaxIterations,
    BeginHealth,
    OutputCount,
    None
    
VARIABLES 
    health,          \* [node -> health value]
    iteration,       \* [node -> iteration count]
    leader,          \* current leader node or Null
    outputs,
    pc,
    publishLog       \* set of nodes that have published in this iteration
    
States == {"ELECTLEADER", "ITERATION_CYCLE_START", "COMPUTE", "PREPARE_PUBLISH", "EXEC_PUBLISH", "DONE", "ELECTLEADER", "SETHEALTH", "ITERATION_CYCLE_COMPLETE"}
Nodes == {"N1", "N2"}  \* Define explicitly for TLC to enumerate
NodeNumbers ==
  [ N1 |-> 1,
    N2 |-> 2
  ]

\* --- Utility function: max of a non-empty finite set ---
Max(S) == IF S = {} THEN 0
    ELSE CHOOSE x \in S : \A y \in S : x >= y

Min(S) == IF S = {} THEN 0
    ELSE CHOOSE x \in S : \A y \in S : x <= y

RECURSIVE SumSet(_)
SumSet(S) ==
    IF S = {} THEN 0
    ELSE LET x == CHOOSE v \in S : TRUE IN x + SumSet(S \ {x})
    
TotalPublished == 
    SumSet({ publishLog[n] : n \in Nodes })

MinIteration ==
    IF Nodes = {} THEN 0
    ELSE Min({ iteration[n] : n \in Nodes })

PublishSumOk ==
    TotalPublished = MaxIterations
        
\* --- Initial state ---
Init ==
    /\ pc = [n \in Nodes |-> "ELECTLEADER"]
    /\ health = [n \in Nodes |-> BeginHealth]
    /\ iteration = [n \in Nodes |-> 1]
    /\ outputs = [n \in Nodes |-> 0]
    /\ publishLog = [n \in Nodes |-> 0]
    /\ leader = None

ELECTLEADER(n) ==
    /\ pc[n] = "ELECTLEADER"
    /\ pc' = [pc EXCEPT ![n] = "ITERATION_CYCLE_START"]
    /\ LET maxH == Max({ health[n1] : n1 \in Nodes })
           maxNodes == { n1 \in Nodes : health[n1] = maxH }
       IN leader' = CHOOSE n1 \in maxNodes : NodeNumbers[n1] = Max({ NodeNumbers[nx] : nx \in maxNodes })
    /\ health' = [health EXCEPT ![n] = health[n] + IF leader' = n THEN LeadershipBonus ELSE 0]
    /\ UNCHANGED << iteration, outputs, publishLog>>

ITERATION_CYCLE_START(n) ==
    /\ pc[n] = "ITERATION_CYCLE_START"
    /\ pc' = [pc EXCEPT ![n] = IF iteration[n] > MaxIterations THEN "DONE" ELSE "COMPUTE"]
    /\ UNCHANGED << outputs, health, publishLog, leader, iteration>>

COMPUTE(n) ==
    /\ pc[n] = "COMPUTE"
    /\ pc' = [pc EXCEPT ![n] = "PREPARE_PUBLISH"]
    /\ outputs' = [outputs EXCEPT ![n] = OutputCount]
    /\ UNCHANGED << health, publishLog, iteration, leader >>

\* --- Leader publishes outputs (only once per cycle) ---
PREPARE_PUBLISH(n) ==
    /\ pc[n] = "PREPARE_PUBLISH"
    /\ IF n = leader THEN
              pc' = [pc EXCEPT ![n] = "EXEC_PUBLISH"]
            ELSE
              pc' = [pc EXCEPT ![n] = "SETHEALTH"]
    /\ UNCHANGED << health, leader, iteration, publishLog, outputs>>

EXEC_PUBLISH(n) ==
    /\ pc[n] = "EXEC_PUBLISH"
    /\ publishLog' = [publishLog EXCEPT ![n] = @ + 1]
    /\ pc' = [pc EXCEPT ![n] = "SETHEALTH"]
    /\ UNCHANGED << health, leader, iteration, outputs>>

SETHEALTH(n) == 
    /\ pc[n] = "SETHEALTH"
    /\ pc' = [pc EXCEPT ![n] = "ITERATION_CYCLE_COMPLETE"]
    /\ health' = [health EXCEPT ![n] = OutputCount + IF leader = n THEN LeadershipBonus ELSE 0]
    /\ LET maxH == Max({ health'[n1] : n1 \in Nodes })
           maxNodes == { n1 \in Nodes : health'[n1] = maxH }
       IN leader' = CHOOSE n1 \in maxNodes : NodeNumbers[n1] = Max({ NodeNumbers[nx] : nx \in maxNodes })
    /\ UNCHANGED << iteration, outputs, publishLog>>
    
ITERATION_CYCLE_COMPLETE(n) ==
    /\ pc[n] = "ITERATION_CYCLE_COMPLETE"
    /\ pc' = [pc EXCEPT ![n] = "ITERATION_CYCLE_START"] 
    /\ iteration' = [iteration EXCEPT ![n] = @ + 1]
    /\ UNCHANGED <<health, outputs, leader, publishLog>>

TypeOK ==
    /\ leader \in Nodes \cup {None}
    /\ iteration \in [Nodes -> Nat]
    /\ outputs \in [Nodes -> Nat]
    /\ publishLog \in [Nodes -> Nat]
    /\ pc \in [Nodes -> States]
    
AllStopped == \A n \in Nodes : pc[n] = "DONE"

Done ==
    AllStopped
    /\ UNCHANGED << health, iteration, outputs, leader, pc, publishLog >>

Next ==
        \/ \E n \in Nodes : ELECTLEADER(n)
        \/ \E n \in Nodes : SETHEALTH(n)
        \/ \E n \in Nodes : ITERATION_CYCLE_START(n)
        \/ \E n \in Nodes : COMPUTE(n)
        \/ \E n \in Nodes : PREPARE_PUBLISH(n)
        \/ \E n \in Nodes : EXEC_PUBLISH(n)
        \/ \E n \in Nodes : ITERATION_CYCLE_COMPLETE(n)
        \/ Done 

\* --- Termination condition (optional) ---
Termination ==
    Done

\*Invariants
\* We MUST have a leader after ELECTLEADER is done
LeaderElectedOK ==
    \/ leader # None
    \/ \E n \in Nodes : pc[n] \in {"ELECTLEADER"}

\* Publish must have happened after each node completes it's iteration
PublishConsistency ==
    LET totalPub == SumSet({ publishLog[m] : m \in Nodes })
    IN totalPub >= Min({ iteration[m] : m \in Nodes }) - 1

Spec == Init /\ [][Next]_<<health, iteration, outputs, leader, pc, publishLog >>

===============================================================================
