------------------------- MODULE AwsDynamoDbProblem -------------------------
EXTENDS Integers, Sequences, TLC

CONSTANTS NULL, PLAN_THRESHOLD, MAX_PLANS
Enactors == {"e1","e2"}

(*--fair algorithm AwsDynamoDbProblem

variables
    backupPlan = NULL,
    activePlan = NULL,
    generatedPlans = <<>>,
    enactorPlans = [ e \in Enactors |-> 0 ],
    enactedPlans = <<>>,
    deletedPlans = <<>>,
    mutex = <<>>,

    process Planner = "p1"
    variable i = 0
    begin
        GeneratePlan:
        \* Keep generating plans indefinitely
            while(i<MAX_PLANS) do
                \*if Len(generatedPlans) < 2*PLAN_THRESHOLD then
                    i := i + 1;
                    generatedPlans := Append(generatedPlans, i);
                \*end if;
            end while;
    end process;
    
    process Enactor \in Enactors 
    variables 
        workingOnPlan= NULL,
        ep = 1
    begin
        EnactorProcess:
            while (TRUE) do
                    if generatedPlans # <<>> then 
                        workingOnPlan := Head(generatedPlans);
                        generatedPlans := Tail(generatedPlans);
                        WaitForLock:
                            if mutex = <<>> then
                                mutex := <<self>>;
                                backupPlan := activePlan;
                                activePlan := workingOnPlan;
                                enactorPlans[self] := workingOnPlan;
                                enactedPlans := Append(enactedPlans, workingOnPlan);
                            else
                                goto WaitForLock;
                            end if;
                        PlanApplied:
                            if mutex = <<self>> then
                                mutex := <<>>;
                            end if;
                        DeletePlan:
                        if enactedPlans # <<>> /\ workingOnPlan # NULL then
                            RemoveOldPlans: 
                                ep := 1;
                                UpdateDeletePlans:
                                    while ep <= Len(enactedPlans) do
                                        if workingOnPlan - PLAN_THRESHOLD >= enactedPlans[ep] then
                                            deletedPlans := Append(deletedPlans, enactedPlans[ep]);
                                        end if;
                                        ep := ep + 1;
                                    end while
                        end if;
                    end if;
            end while;
    end process;

end algorithm; *)
\* BEGIN TRANSLATION (chksum(pcal) = "7c28162a" /\ chksum(tla) = "c5ac8ce7")
VARIABLES pc, backupPlan, activePlan, generatedPlans, enactorPlans, 
          enactedPlans, deletedPlans, mutex, i, workingOnPlan, ep

vars == << pc, backupPlan, activePlan, generatedPlans, enactorPlans, 
           enactedPlans, deletedPlans, mutex, i, workingOnPlan, ep >>

ProcSet == {"p1"} \cup (Enactors)

Init == (* Global variables *)
        /\ backupPlan = NULL
        /\ activePlan = NULL
        /\ generatedPlans = <<>>
        /\ enactorPlans = [ e \in Enactors |-> 0 ]
        /\ enactedPlans = <<>>
        /\ deletedPlans = <<>>
        /\ mutex = <<>>
        (* Process Planner *)
        /\ i = 0
        (* Process Enactor *)
        /\ workingOnPlan = [self \in Enactors |-> NULL]
        /\ ep = [self \in Enactors |-> 1]
        /\ pc = [self \in ProcSet |-> CASE self = "p1" -> "GeneratePlan"
                                        [] self \in Enactors -> "EnactorProcess"]

GeneratePlan == /\ pc["p1"] = "GeneratePlan"
                /\ IF (i<MAX_PLANS)
                      THEN /\ i' = i + 1
                           /\ generatedPlans' = Append(generatedPlans, i')
                           /\ pc' = [pc EXCEPT !["p1"] = "GeneratePlan"]
                      ELSE /\ pc' = [pc EXCEPT !["p1"] = "Done"]
                           /\ UNCHANGED << generatedPlans, i >>
                /\ UNCHANGED << backupPlan, activePlan, enactorPlans, 
                                enactedPlans, deletedPlans, mutex, 
                                workingOnPlan, ep >>

Planner == GeneratePlan

EnactorProcess(self) == /\ pc[self] = "EnactorProcess"
                        /\ IF generatedPlans # <<>>
                              THEN /\ workingOnPlan' = [workingOnPlan EXCEPT ![self] = Head(generatedPlans)]
                                   /\ generatedPlans' = Tail(generatedPlans)
                                   /\ pc' = [pc EXCEPT ![self] = "WaitForLock"]
                              ELSE /\ pc' = [pc EXCEPT ![self] = "EnactorProcess"]
                                   /\ UNCHANGED << generatedPlans, 
                                                   workingOnPlan >>
                        /\ UNCHANGED << backupPlan, activePlan, enactorPlans, 
                                        enactedPlans, deletedPlans, mutex, i, 
                                        ep >>

WaitForLock(self) == /\ pc[self] = "WaitForLock"
                     /\ IF mutex = <<>>
                           THEN /\ mutex' = <<self>>
                                /\ backupPlan' = activePlan
                                /\ activePlan' = workingOnPlan[self]
                                /\ enactorPlans' = [enactorPlans EXCEPT ![self] = workingOnPlan[self]]
                                /\ enactedPlans' = Append(enactedPlans, workingOnPlan[self])
                                /\ pc' = [pc EXCEPT ![self] = "PlanApplied"]
                           ELSE /\ pc' = [pc EXCEPT ![self] = "WaitForLock"]
                                /\ UNCHANGED << backupPlan, activePlan, 
                                                enactorPlans, enactedPlans, 
                                                mutex >>
                     /\ UNCHANGED << generatedPlans, deletedPlans, i, 
                                     workingOnPlan, ep >>

PlanApplied(self) == /\ pc[self] = "PlanApplied"
                     /\ IF mutex = <<self>>
                           THEN /\ mutex' = <<>>
                           ELSE /\ TRUE
                                /\ mutex' = mutex
                     /\ pc' = [pc EXCEPT ![self] = "DeletePlan"]
                     /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     i, workingOnPlan, ep >>

DeletePlan(self) == /\ pc[self] = "DeletePlan"
                    /\ IF enactedPlans # <<>> /\ workingOnPlan[self] # NULL
                          THEN /\ pc' = [pc EXCEPT ![self] = "RemoveOldPlans"]
                          ELSE /\ pc' = [pc EXCEPT ![self] = "EnactorProcess"]
                    /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                    enactorPlans, enactedPlans, deletedPlans, 
                                    mutex, i, workingOnPlan, ep >>

RemoveOldPlans(self) == /\ pc[self] = "RemoveOldPlans"
                        /\ ep' = [ep EXCEPT ![self] = 1]
                        /\ pc' = [pc EXCEPT ![self] = "UpdateDeletePlans"]
                        /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                        enactorPlans, enactedPlans, 
                                        deletedPlans, mutex, i, workingOnPlan >>

UpdateDeletePlans(self) == /\ pc[self] = "UpdateDeletePlans"
                           /\ IF ep[self] <= Len(enactedPlans)
                                 THEN /\ IF workingOnPlan[self] - PLAN_THRESHOLD >= enactedPlans[ep[self]]
                                            THEN /\ deletedPlans' = Append(deletedPlans, enactedPlans[ep[self]])
                                            ELSE /\ TRUE
                                                 /\ UNCHANGED deletedPlans
                                      /\ ep' = [ep EXCEPT ![self] = ep[self] + 1]
                                      /\ pc' = [pc EXCEPT ![self] = "UpdateDeletePlans"]
                                 ELSE /\ pc' = [pc EXCEPT ![self] = "EnactorProcess"]
                                      /\ UNCHANGED << deletedPlans, ep >>
                           /\ UNCHANGED << backupPlan, activePlan, 
                                           generatedPlans, enactorPlans, 
                                           enactedPlans, mutex, i, 
                                           workingOnPlan >>

Enactor(self) == EnactorProcess(self) \/ WaitForLock(self)
                    \/ PlanApplied(self) \/ DeletePlan(self)
                    \/ RemoveOldPlans(self) \/ UpdateDeletePlans(self)

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == /\ \A self \in ProcSet: pc[self] = "Done"
               /\ UNCHANGED vars

Next == Planner
           \/ (\E self \in Enactors: Enactor(self))
           \/ Terminating

Spec == /\ Init /\ [][Next]_vars
        /\ WF_vars(Next)

Termination == <>(\A self \in ProcSet: pc[self] = "Done")

\* END TRANSLATION 

\* Active Plan is never deleted
\*ActivePlanExists == activePlan = NULL \/ activePlan < 5

ActivePlanExists == activePlan \notin { deletedPlans[dp] : dp \in 1..Len(deletedPlans) }

\*QueueEmpty == <>[]( Len(generatedPlans) < 3 )
\*AllPlansEnacted == <>[]( Len(enactedPlans) = planCount )
=============================================================================
