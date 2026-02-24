------------------------- MODULE AwsDynamoDbProblem -------------------------
EXTENDS Integers, Sequences, TLC

CONSTANTS 
    NullPtr, 
    PLAN_THRESHOLD, 
    MAX_PLANS,
    Enactors

(*--algorithm AwsDynamoDbProblem

variables
    rollbackPlan = NullPtr,
    activePlan = NullPtr,
    generatedPlans = <<>>,
    enactorPlans = [ e \in Enactors |-> 0 ],
    enactedPlans = <<>>,
    deletedPlans = <<>>,
    mutex = {},

    process Planner = "p1"
    variable planId = 0
    begin
    GeneratePlan:
        while(TRUE) do
           planId := planId + 1;
           generatedPlans := Append(generatedPlans, planId);
        end while;
    end process;
    
    process Enactor \in Enactors 
    variables 
        workingOnPlan = NullPtr
    begin
    EnactorLoop:
        while (TRUE) do
            PullPlan:
                await generatedPlans # <<>>; 
                workingOnPlan := Head(generatedPlans);
                generatedPlans := Tail(generatedPlans);
            WaitForLock:
                await mutex = {};
                mutex := {self};
            LockAcquired:
                skip;
            UpdatePlan:
                rollbackPlan := activePlan;
                activePlan := workingOnPlan;
                enactorPlans[self] := workingOnPlan;
                enactedPlans := Append(enactedPlans, workingOnPlan);
            LockRelease:
                if mutex = {self} then
                    LockReleased:
                        mutex := {};
                end if;
            DeleteOldPlans:
                if enactedPlans # <<>> /\ workingOnPlan # NullPtr then
                    deletedPlans := deletedPlans \o SelectSeq(enactedPlans, LAMBDA x: x <= (workingOnPlan - PLAN_THRESHOLD) );
                    enactedPlans := SelectSeq(enactedPlans, LAMBDA x: x > (workingOnPlan - PLAN_THRESHOLD) );
                end if;
        end while;
    end process;

end algorithm; *)
\* BEGIN TRANSLATION (chksum(pcal) = "acda1da9" /\ chksum(tla) = "7438021e")
VARIABLES rollbackPlan, activePlan, generatedPlans, enactorPlans, 
          enactedPlans, deletedPlans, mutex, pc, planId, workingOnPlan

vars == << rollbackPlan, activePlan, generatedPlans, enactorPlans, 
           enactedPlans, deletedPlans, mutex, pc, planId, workingOnPlan >>

ProcSet == {"p1"} \cup (Enactors)

Init == (* Global variables *)
        /\ rollbackPlan = NullPtr
        /\ activePlan = NullPtr
        /\ generatedPlans = <<>>
        /\ enactorPlans = [ e \in Enactors |-> 0 ]
        /\ enactedPlans = <<>>
        /\ deletedPlans = <<>>
        /\ mutex = {}
        (* Process Planner *)
        /\ planId = 0
        (* Process Enactor *)
        /\ workingOnPlan = [self \in Enactors |-> NullPtr]
        /\ pc = [self \in ProcSet |-> CASE self = "p1" -> "GeneratePlan"
                                        [] self \in Enactors -> "EnactorLoop"]

GeneratePlan == /\ pc["p1"] = "GeneratePlan"
                /\ planId' = planId + 1
                /\ generatedPlans' = Append(generatedPlans, planId')
                /\ pc' = [pc EXCEPT !["p1"] = "GeneratePlan"]
                /\ UNCHANGED << rollbackPlan, activePlan, enactorPlans, 
                                enactedPlans, deletedPlans, mutex, 
                                workingOnPlan >>

Planner == GeneratePlan

EnactorLoop(self) == /\ pc[self] = "EnactorLoop"
                     /\ pc' = [pc EXCEPT ![self] = "PullPlan"]
                     /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     mutex, planId, workingOnPlan >>

PullPlan(self) == /\ pc[self] = "PullPlan"
                  /\ generatedPlans # <<>>
                  /\ workingOnPlan' = [workingOnPlan EXCEPT ![self] = Head(generatedPlans)]
                  /\ generatedPlans' = Tail(generatedPlans)
                  /\ pc' = [pc EXCEPT ![self] = "WaitForLock"]
                  /\ UNCHANGED << rollbackPlan, activePlan, enactorPlans, 
                                  enactedPlans, deletedPlans, mutex, planId >>

WaitForLock(self) == /\ pc[self] = "WaitForLock"
                     /\ mutex = {}
                     /\ mutex' = {self}
                     /\ pc' = [pc EXCEPT ![self] = "LockAcquired"]
                     /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     planId, workingOnPlan >>

LockAcquired(self) == /\ pc[self] = "LockAcquired"
                      /\ TRUE
                      /\ pc' = [pc EXCEPT ![self] = "UpdatePlan"]
                      /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                      enactorPlans, enactedPlans, deletedPlans, 
                                      mutex, planId, workingOnPlan >>

UpdatePlan(self) == /\ pc[self] = "UpdatePlan"
                    /\ rollbackPlan' = activePlan
                    /\ activePlan' = workingOnPlan[self]
                    /\ enactorPlans' = [enactorPlans EXCEPT ![self] = workingOnPlan[self]]
                    /\ enactedPlans' = Append(enactedPlans, workingOnPlan[self])
                    /\ pc' = [pc EXCEPT ![self] = "LockRelease"]
                    /\ UNCHANGED << generatedPlans, deletedPlans, mutex, 
                                    planId, workingOnPlan >>

LockRelease(self) == /\ pc[self] = "LockRelease"
                     /\ IF mutex = {self}
                           THEN /\ pc' = [pc EXCEPT ![self] = "LockReleased"]
                           ELSE /\ pc' = [pc EXCEPT ![self] = "DeleteOldPlans"]
                     /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     mutex, planId, workingOnPlan >>

LockReleased(self) == /\ pc[self] = "LockReleased"
                      /\ mutex' = {}
                      /\ pc' = [pc EXCEPT ![self] = "DeleteOldPlans"]
                      /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                      enactorPlans, enactedPlans, deletedPlans, 
                                      planId, workingOnPlan >>

DeleteOldPlans(self) == /\ pc[self] = "DeleteOldPlans"
                        /\ IF enactedPlans # <<>> /\ workingOnPlan[self] # NullPtr
                              THEN /\ deletedPlans' = deletedPlans \o SelectSeq(enactedPlans, LAMBDA x: x <= (workingOnPlan[self] - PLAN_THRESHOLD) )
                                   /\ enactedPlans' = SelectSeq(enactedPlans, LAMBDA x: x > (workingOnPlan[self] - PLAN_THRESHOLD) )
                              ELSE /\ TRUE
                                   /\ UNCHANGED << enactedPlans, deletedPlans >>
                        /\ pc' = [pc EXCEPT ![self] = "EnactorLoop"]
                        /\ UNCHANGED << rollbackPlan, activePlan, 
                                        generatedPlans, enactorPlans, mutex, 
                                        planId, workingOnPlan >>

Enactor(self) == EnactorLoop(self) \/ PullPlan(self) \/ WaitForLock(self)
                    \/ LockAcquired(self) \/ UpdatePlan(self)
                    \/ LockRelease(self) \/ LockReleased(self)
                    \/ DeleteOldPlans(self)

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == /\ \A self \in ProcSet: pc[self] = "Done"
               /\ UNCHANGED vars

Next == Planner
           \/ (\E self \in Enactors: Enactor(self))
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(\A self \in ProcSet: pc[self] = "Done")

\* END TRANSLATION 

\*Ensure we dont have inconsistent behavior in the process
\*Active Plan is not deleted!!!
Inv_DoNotDeleteActivePlan == activePlan \notin { deletedPlans[dp] : dp \in 1..Len(deletedPlans) }
\*Rollback plan is not deleted!!!
DoNotDeleteRollbackPlan == rollbackPlan \notin { deletedPlans[dp] : dp \in 1..Len(deletedPlans) }
\*Both Active and BackUp plans are not deleted!!!
Inv_ActiveAndRollbackNotDeleted == Inv_DoNotDeleteActivePlan \/ DoNotDeleteRollbackPlan

=============================================================================
