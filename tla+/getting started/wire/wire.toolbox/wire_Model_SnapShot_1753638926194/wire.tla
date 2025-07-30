-------------------------------- MODULE wire --------------------------------
EXTENDS Integers
(* --algorithm wire 

variables 
    person = { "alice", "bob" },
    account = [ p \in person |-> 5];
   
define
    NoOverDrafts == \A p \in person : account[p] >= 0
    EventualConsistent == <>[](account["alice"]+account["bob"] = 10)
end define; 

process Wire \in 1..2
    variables 
        sender = "alice",
        receiver = "bob",
        amount \in 1..account[sender];

begin
    CheckAndWithdraw:
        if amount <= account[sender] then
            account[sender] := account[sender] - amount;
    Deposit:
            account[receiver] := account[receiver] + amount;
        end if;
end process;
end algorithm;*)
\* BEGIN TRANSLATION (chksum(pcal) = "bcde53a7" /\ chksum(tla) = "31a80abf")
VARIABLES person, account, pc

(* define statement *)
NoOverDrafts == \A p \in person : account[p] >= 0
EventualConsistent == <>[](account["alice"]+account["bob"] = 10)

VARIABLES sender, receiver, amount

vars == << person, account, pc, sender, receiver, amount >>

ProcSet == (1..2)

Init == (* Global variables *)
        /\ person = { "alice", "bob" }
        /\ account = [ p \in person |-> 5]
        (* Process Wire *)
        /\ sender = [self \in 1..2 |-> "alice"]
        /\ receiver = [self \in 1..2 |-> "bob"]
        /\ amount \in [1..2 -> 1..account[sender[CHOOSE self \in  1..2 : TRUE]]]
        /\ pc = [self \in ProcSet |-> "CheckAndWithdraw"]

CheckAndWithdraw(self) == /\ pc[self] = "CheckAndWithdraw"
                          /\ IF amount[self] <= account[sender[self]]
                                THEN /\ account' = [account EXCEPT ![sender[self]] = account[sender[self]] - amount[self]]
                                     /\ pc' = [pc EXCEPT ![self] = "Deposit"]
                                ELSE /\ pc' = [pc EXCEPT ![self] = "Done"]
                                     /\ UNCHANGED account
                          /\ UNCHANGED << person, sender, receiver, amount >>

Deposit(self) == /\ pc[self] = "Deposit"
                 /\ account' = [account EXCEPT ![receiver[self]] = account[receiver[self]] + amount[self]]
                 /\ pc' = [pc EXCEPT ![self] = "Done"]
                 /\ UNCHANGED << person, sender, receiver, amount >>

Wire(self) == CheckAndWithdraw(self) \/ Deposit(self)

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == /\ \A self \in ProcSet: pc[self] = "Done"
               /\ UNCHANGED vars

Next == (\E self \in 1..2: Wire(self))
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(\A self \in ProcSet: pc[self] = "Done")

\* END TRANSLATION
=============================================================================
\* Modification History
\* Last modified Sun Jul 27 22:25:34 IST 2025 by ngavali
\* Created Sun Jul 27 21:38:35 IST 2025 by ngavali
