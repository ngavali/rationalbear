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
    enactorPlans = [ e \in Enactors |-> [
                        PrevEnacted |-> NullPtr, 
                        WorkingOn |-> NullPtr 
                   ]],
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
    begin
    EnactorLoop:
        while (TRUE) do
            PullPlan:
                await generatedPlans # <<>>; 
                enactorPlans[self]["WorkingOn"] := Head(generatedPlans);
                generatedPlans := Tail(generatedPlans);
            WaitForLock:
                await mutex = {};
                mutex := {self};
            LockAcquired:
                skip;
            UpdatePlan:
                rollbackPlan := activePlan;
                activePlan := enactorPlans[self]["WorkingOn"];
                enactorPlans[self]["PrevEnacted"] := enactorPlans[self]["WorkingOn"];
                enactedPlans := Append(enactedPlans, enactorPlans[self]["WorkingOn"]);
            LockRelease:
                if mutex = {self} then
                    LockReleased:
                        mutex := {};
                end if;
            DeleteOldPlans:
                if enactedPlans # <<>> /\ enactorPlans[self]["PrevEnacted"] # NullPtr then
                    deletedPlans := deletedPlans \o SelectSeq(enactedPlans, LAMBDA x: x <= (enactorPlans[self]["PrevEnacted"] - PLAN_THRESHOLD) );
                    enactedPlans := SelectSeq(enactedPlans, LAMBDA x: x > (enactorPlans[self]["PrevEnacted"] - PLAN_THRESHOLD) );
                end if;
                enactorPlans[self]["WorkingOn"] := NullPtr
        end while;
    end process;

end algorithm; *)
\* BEGIN TRANSLATION (chksum(pcal) = "6fc410d7" /\ chksum(tla) = "8967eea2")
VARIABLES rollbackPlan, activePlan, generatedPlans, enactorPlans, 
          enactedPlans, deletedPlans, mutex, pc, planId

vars == << rollbackPlan, activePlan, generatedPlans, enactorPlans, 
           enactedPlans, deletedPlans, mutex, pc, planId >>

ProcSet == {"p1"} \cup (Enactors)

Init == (* Global variables *)
        /\ rollbackPlan = NullPtr
        /\ activePlan = NullPtr
        /\ generatedPlans = <<>>
        /\ enactorPlans = [ e \in Enactors |-> [
                               PrevEnacted |-> NullPtr,
                               WorkingOn |-> NullPtr
                          ]]
        /\ enactedPlans = <<>>
        /\ deletedPlans = <<>>
        /\ mutex = {}
        (* Process Planner *)
        /\ planId = 0
        /\ pc = [self \in ProcSet |-> CASE self = "p1" -> "GeneratePlan"
                                        [] self \in Enactors -> "EnactorLoop"]

GeneratePlan == /\ pc["p1"] = "GeneratePlan"
                /\ planId' = planId + 1
                /\ generatedPlans' = Append(generatedPlans, planId')
                /\ pc' = [pc EXCEPT !["p1"] = "GeneratePlan"]
                /\ UNCHANGED << rollbackPlan, activePlan, enactorPlans, 
                                enactedPlans, deletedPlans, mutex >>

Planner == GeneratePlan

EnactorLoop(self) == /\ pc[self] = "EnactorLoop"
                     /\ pc' = [pc EXCEPT ![self] = "PullPlan"]
                     /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     mutex, planId >>

PullPlan(self) == /\ pc[self] = "PullPlan"
                  /\ generatedPlans # <<>>
                  /\ enactorPlans' = [enactorPlans EXCEPT ![self]["WorkingOn"] = Head(generatedPlans)]
                  /\ generatedPlans' = Tail(generatedPlans)
                  /\ pc' = [pc EXCEPT ![self] = "WaitForLock"]
                  /\ UNCHANGED << rollbackPlan, activePlan, enactedPlans, 
                                  deletedPlans, mutex, planId >>

WaitForLock(self) == /\ pc[self] = "WaitForLock"
                     /\ mutex = {}
                     /\ mutex' = {self}
                     /\ pc' = [pc EXCEPT ![self] = "LockAcquired"]
                     /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     planId >>

LockAcquired(self) == /\ pc[self] = "LockAcquired"
                      /\ TRUE
                      /\ pc' = [pc EXCEPT ![self] = "UpdatePlan"]
                      /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                      enactorPlans, enactedPlans, deletedPlans, 
                                      mutex, planId >>

UpdatePlan(self) == /\ pc[self] = "UpdatePlan"
                    /\ rollbackPlan' = activePlan
                    /\ activePlan' = enactorPlans[self]["WorkingOn"]
                    /\ enactorPlans' = [enactorPlans EXCEPT ![self]["PrevEnacted"] = enactorPlans[self]["WorkingOn"]]
                    /\ enactedPlans' = Append(enactedPlans, enactorPlans'[self]["WorkingOn"])
                    /\ pc' = [pc EXCEPT ![self] = "LockRelease"]
                    /\ UNCHANGED << generatedPlans, deletedPlans, mutex, 
                                    planId >>

LockRelease(self) == /\ pc[self] = "LockRelease"
                     /\ IF mutex = {self}
                           THEN /\ pc' = [pc EXCEPT ![self] = "LockReleased"]
                           ELSE /\ pc' = [pc EXCEPT ![self] = "DeleteOldPlans"]
                     /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     mutex, planId >>

LockReleased(self) == /\ pc[self] = "LockReleased"
                      /\ mutex' = {}
                      /\ pc' = [pc EXCEPT ![self] = "DeleteOldPlans"]
                      /\ UNCHANGED << rollbackPlan, activePlan, generatedPlans, 
                                      enactorPlans, enactedPlans, deletedPlans, 
                                      planId >>

DeleteOldPlans(self) == /\ pc[self] = "DeleteOldPlans"
                        /\ IF enactedPlans # <<>> /\ enactorPlans[self]["PrevEnacted"] # NullPtr
                              THEN /\ deletedPlans' = deletedPlans \o SelectSeq(enactedPlans, LAMBDA x: x <= (enactorPlans[self]["PrevEnacted"] - PLAN_THRESHOLD) )
                                   /\ enactedPlans' = SelectSeq(enactedPlans, LAMBDA x: x > (enactorPlans[self]["PrevEnacted"] - PLAN_THRESHOLD) )
                              ELSE /\ TRUE
                                   /\ UNCHANGED << enactedPlans, deletedPlans >>
                        /\ enactorPlans' = [enactorPlans EXCEPT ![self]["WorkingOn"] = NullPtr]
                        /\ pc' = [pc EXCEPT ![self] = "EnactorLoop"]
                        /\ UNCHANGED << rollbackPlan, activePlan, 
                                        generatedPlans, mutex, planId >>

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
DeletedSet ==
    { deletedPlans[i] : i \in 1..Len(deletedPlans) }
\*Active Plan is not deleted!!!
ActivePlanDeleted == activePlan \in DeletedSet
\*Rollback plan is not deleted!!!
RollbackPlanDeleted == rollbackPlan \in DeletedSet
\*Both Active and BackUp plans are not deleted!!! System is recoverable!!!
Inv_DontDeleteActivePlan == ~ActivePlanDeleted

Inv_Recoverable ==
    ~ ( ActivePlanDeleted /\ RollbackPlanDeleted )
=============================================================================
