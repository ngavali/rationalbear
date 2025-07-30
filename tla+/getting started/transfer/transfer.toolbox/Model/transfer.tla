---- MODULE transfer ----
EXTENDS Naturals, TLC

(* --algorithm transfer
variables alice_account = 10, bob_account = 10,
          account_total = alice_account + bob_account;
(* 
money \in 1..5; syntactic sugar for set of number 1 ot 5, inclusive
money \in {1, 2, 3, 4, 5};
money \in Nat ( Nat is the set of natural numbers ) --> this is not feasible
*)

\*Add two transfer process
process Transfer \in 1..2
  variable money \in 1..20;
begin
Transfer:
  if alice_account >= money then
    \*A: alice_account := alice_account - money;
    \*B: bob_account := bob_account + money;
    A: alice_account := alice_account - money;
       bob_account := bob_account + money; \* Both now part of A
end if;
C: assert alice_account >= 0;
end process

end algorithm *)
\* BEGIN TRANSLATION (chksum(pcal) = "2739b956" /\ chksum(tla) = "8ebd6df7")
\* Label Transfer of process Transfer at line 18 col 3 changed to Transfer_
VARIABLES alice_account, bob_account, account_total, pc, money

vars == << alice_account, bob_account, account_total, pc, money >>

ProcSet == (1..2)

Init == (* Global variables *)
        /\ alice_account = 10
        /\ bob_account = 10
        /\ account_total = alice_account + bob_account
        (* Process Transfer *)
        /\ money \in [1..2 -> 1..20]
        /\ pc = [self \in ProcSet |-> "Transfer_"]

Transfer_(self) == /\ pc[self] = "Transfer_"
                   /\ IF alice_account >= money[self]
                         THEN /\ pc' = [pc EXCEPT ![self] = "A"]
                         ELSE /\ pc' = [pc EXCEPT ![self] = "C"]
                   /\ UNCHANGED << alice_account, bob_account, account_total, 
                                   money >>

A(self) == /\ pc[self] = "A"
           /\ alice_account' = alice_account - money[self]
           /\ bob_account' = bob_account + money[self]
           /\ pc' = [pc EXCEPT ![self] = "C"]
           /\ UNCHANGED << account_total, money >>

C(self) == /\ pc[self] = "C"
           /\ Assert(alice_account >= 0, 
                     "Failure of assertion at line 24, column 4.")
           /\ pc' = [pc EXCEPT ![self] = "Done"]
           /\ UNCHANGED << alice_account, bob_account, account_total, money >>

Transfer(self) == Transfer_(self) \/ A(self) \/ C(self)

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == /\ \A self \in ProcSet: pc[self] = "Done"
               /\ UNCHANGED vars

Next == (\E self \in 1..2: Transfer(self))
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(\A self \in ProcSet: pc[self] = "Done")

\* END TRANSLATION 

(*
An invariant is different that an assert, something that must be true in all possible system states
Use MoneyNotNegative as an Invariant of the system
*)
\*MoneyNotNegative == money >= 0
MoneyInvariant == alice_account + bob_account = account_total
====
