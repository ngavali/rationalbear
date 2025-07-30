---------------------------- MODULE simpleAdder ----------------------------
EXTENDS Integers

(* --algorithm addOne

variables x \in 1..5

begin
 Add:
    x := x + 1;

end algorithm *)
\* BEGIN TRANSLATION (chksum(pcal) = "663053a4" /\ chksum(tla) = "864bc52f")
VARIABLES x, pc

vars == << x, pc >>

Init == (* Global variables *)
        /\ x \in 1..5
        /\ pc = "Add"

Add == /\ pc = "Add"
       /\ x' = x + 1
       /\ pc' = "Done"

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Add
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION 

=============================================================================
\* Modification History
\* Last modified Wed Jul 23 17:30:09 IST 2025 by ngavali
\* Created Wed Jul 23 17:26:54 IST 2025 by ngavali
