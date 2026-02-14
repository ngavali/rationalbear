------------------------- MODULE AwsDynamoDbProblem -------------------------
EXTENDS Integers, Sequences, TLC

CONSTANTS 
    NullPtr, 
    PLAN_THRESHOLD, 
    MAX_PLANS,
    Enactors

(*--algorithm AwsDynamoDbProblem

variables
    backupPlan = NullPtr,
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
            while(planId<MAX_PLANS) do
               planId := planId + 1;
               generatedPlans := Append(generatedPlans, planId);
            end while;
    end process;
    
    process Enactor \in Enactors 
    variables 
        workingOnPlan = NullPtr,
        crashed = FALSE,
    begin
        EnactorLoop:
            while (TRUE) do
                PullPlan:
                    await generatedPlans # <<>>; 
                    workingOnPlan := Head(generatedPlans);
                    generatedPlans := Tail(generatedPlans);
                ApplyPlan:
                    if workingOnPlan \notin { deletedPlans[dp] : dp \in 1..Len(deletedPlans) } then
                       WaitForLock:
                            await mutex = {};
                                mutex := {self};
                            LockAcquiried:
                                skip;
                            GetActivePlan:
                                if SelectSeq(deletedPlans, LAMBDA x: x = activePlan ) = <<activePlan>> then
                                    crashed:= TRUE; 
                                else 
                                UpdatePlan:
                                    backupPlan := activePlan;
                                    activePlan := workingOnPlan;
                                    enactorPlans[self] := workingOnPlan;
                                    enactedPlans := Append(enactedPlans, workingOnPlan);
                                end if;
                            LockRelease:
                                if mutex = {self} then
                                    LockReleased:
                                        mutex := {};
                                end if;
                            CheckException:
                                if crashed = TRUE then
                                    goto Crashed;
                            end if;
                            DeleteOldPlans:
                                if enactedPlans # <<>> /\ workingOnPlan # NullPtr then
                                    deletedPlans := deletedPlans \o SelectSeq(enactedPlans, LAMBDA x: x <= (workingOnPlan - PLAN_THRESHOLD) );
                                    enactedPlans := SelectSeq(enactedPlans, LAMBDA x: x > (workingOnPlan - PLAN_THRESHOLD) );
                                end if;
                    end if;
            end while;
        Crashed:
            skip;
    end process;

end algorithm; *)
\* BEGIN TRANSLATION (chksum(pcal) = "71404ebb" /\ chksum(tla) = "507021a1")
VARIABLES pc, backupPlan, activePlan, generatedPlans, enactorPlans, 
          enactedPlans, deletedPlans, mutex, planId, workingOnPlan, crashed

vars == << pc, backupPlan, activePlan, generatedPlans, enactorPlans, 
           enactedPlans, deletedPlans, mutex, planId, workingOnPlan, crashed
        >>

ProcSet == {"p1"} \cup (Enactors)

Init == (* Global variables *)
        /\ backupPlan = NullPtr
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
        /\ crashed = [self \in Enactors |-> FALSE]
        /\ pc = [self \in ProcSet |-> CASE self = "p1" -> "GeneratePlan"
                                        [] self \in Enactors -> "EnactorLoop"]

GeneratePlan == /\ pc["p1"] = "GeneratePlan"
                /\ IF (planId<MAX_PLANS)
                      THEN /\ planId' = planId + 1
                           /\ generatedPlans' = Append(generatedPlans, planId')
                           /\ pc' = [pc EXCEPT !["p1"] = "GeneratePlan"]
                      ELSE /\ pc' = [pc EXCEPT !["p1"] = "Done"]
                           /\ UNCHANGED << generatedPlans, planId >>
                /\ UNCHANGED << backupPlan, activePlan, enactorPlans, 
                                enactedPlans, deletedPlans, mutex, 
                                workingOnPlan, crashed >>

Planner == GeneratePlan

EnactorLoop(self) == /\ pc[self] = "EnactorLoop"
                     /\ pc' = [pc EXCEPT ![self] = "PullPlan"]
                     /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     mutex, planId, workingOnPlan, crashed >>

PullPlan(self) == /\ pc[self] = "PullPlan"
                  /\ generatedPlans # <<>>
                  /\ workingOnPlan' = [workingOnPlan EXCEPT ![self] = Head(generatedPlans)]
                  /\ generatedPlans' = Tail(generatedPlans)
                  /\ pc' = [pc EXCEPT ![self] = "ApplyPlan"]
                  /\ UNCHANGED << backupPlan, activePlan, enactorPlans, 
                                  enactedPlans, deletedPlans, mutex, planId, 
                                  crashed >>

ApplyPlan(self) == /\ pc[self] = "ApplyPlan"
                   /\ IF workingOnPlan[self] \notin { deletedPlans[dp] : dp \in 1..Len(deletedPlans) }
                         THEN /\ pc' = [pc EXCEPT ![self] = "WaitForLock"]
                         ELSE /\ pc' = [pc EXCEPT ![self] = "EnactorLoop"]
                   /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                   enactorPlans, enactedPlans, deletedPlans, 
                                   mutex, planId, workingOnPlan, crashed >>

WaitForLock(self) == /\ pc[self] = "WaitForLock"
                     /\ mutex = {}
                     /\ mutex' = {self}
                     /\ pc' = [pc EXCEPT ![self] = "LockAcquiried"]
                     /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     planId, workingOnPlan, crashed >>

LockAcquiried(self) == /\ pc[self] = "LockAcquiried"
                       /\ TRUE
                       /\ pc' = [pc EXCEPT ![self] = "GetActivePlan"]
                       /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                       enactorPlans, enactedPlans, 
                                       deletedPlans, mutex, planId, 
                                       workingOnPlan, crashed >>

GetActivePlan(self) == /\ pc[self] = "GetActivePlan"
                       /\ IF SelectSeq(deletedPlans, LAMBDA x: x = activePlan ) = <<activePlan>>
                             THEN /\ crashed' = [crashed EXCEPT ![self] = TRUE]
                                  /\ pc' = [pc EXCEPT ![self] = "LockRelease"]
                             ELSE /\ pc' = [pc EXCEPT ![self] = "UpdatePlan"]
                                  /\ UNCHANGED crashed
                       /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                       enactorPlans, enactedPlans, 
                                       deletedPlans, mutex, planId, 
                                       workingOnPlan >>

UpdatePlan(self) == /\ pc[self] = "UpdatePlan"
                    /\ backupPlan' = activePlan
                    /\ activePlan' = workingOnPlan[self]
                    /\ enactorPlans' = [enactorPlans EXCEPT ![self] = workingOnPlan[self]]
                    /\ enactedPlans' = Append(enactedPlans, workingOnPlan[self])
                    /\ pc' = [pc EXCEPT ![self] = "LockRelease"]
                    /\ UNCHANGED << generatedPlans, deletedPlans, mutex, 
                                    planId, workingOnPlan, crashed >>

LockRelease(self) == /\ pc[self] = "LockRelease"
                     /\ IF mutex = {self}
                           THEN /\ pc' = [pc EXCEPT ![self] = "LockReleased"]
                           ELSE /\ pc' = [pc EXCEPT ![self] = "CheckException"]
                     /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                     enactorPlans, enactedPlans, deletedPlans, 
                                     mutex, planId, workingOnPlan, crashed >>

LockReleased(self) == /\ pc[self] = "LockReleased"
                      /\ mutex' = {}
                      /\ pc' = [pc EXCEPT ![self] = "CheckException"]
                      /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                      enactorPlans, enactedPlans, deletedPlans, 
                                      planId, workingOnPlan, crashed >>

CheckException(self) == /\ pc[self] = "CheckException"
                        /\ IF crashed[self] = TRUE
                              THEN /\ pc' = [pc EXCEPT ![self] = "Crashed"]
                              ELSE /\ pc' = [pc EXCEPT ![self] = "DeleteOldPlans"]
                        /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                        enactorPlans, enactedPlans, 
                                        deletedPlans, mutex, planId, 
                                        workingOnPlan, crashed >>

DeleteOldPlans(self) == /\ pc[self] = "DeleteOldPlans"
                        /\ IF enactedPlans # <<>> /\ workingOnPlan[self] # NullPtr
                              THEN /\ deletedPlans' = deletedPlans \o SelectSeq(enactedPlans, LAMBDA x: x <= (workingOnPlan[self] - PLAN_THRESHOLD) )
                                   /\ enactedPlans' = SelectSeq(enactedPlans, LAMBDA x: x > (workingOnPlan[self] - PLAN_THRESHOLD) )
                              ELSE /\ TRUE
                                   /\ UNCHANGED << enactedPlans, deletedPlans >>
                        /\ pc' = [pc EXCEPT ![self] = "EnactorLoop"]
                        /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                        enactorPlans, mutex, planId, 
                                        workingOnPlan, crashed >>

Crashed(self) == /\ pc[self] = "Crashed"
                 /\ TRUE
                 /\ pc' = [pc EXCEPT ![self] = "Done"]
                 /\ UNCHANGED << backupPlan, activePlan, generatedPlans, 
                                 enactorPlans, enactedPlans, deletedPlans, 
                                 mutex, planId, workingOnPlan, crashed >>

Enactor(self) == EnactorLoop(self) \/ PullPlan(self) \/ ApplyPlan(self)
                    \/ WaitForLock(self) \/ LockAcquiried(self)
                    \/ GetActivePlan(self) \/ UpdatePlan(self)
                    \/ LockRelease(self) \/ LockReleased(self)
                    \/ CheckException(self) \/ DeleteOldPlans(self)
                    \/ Crashed(self)

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
\*Backup plan is not deleted!!!
DoNotDeleteBackupPlan == backupPlan \notin { deletedPlans[dp] : dp \in 1..Len(deletedPlans) }
\*Both Active and BackUp plans are not deleted!!!
Inv_ActiveAndBackupNotDeleted == Inv_DoNotDeleteActivePlan \/ DoNotDeleteBackupPlan


=============================================================================
