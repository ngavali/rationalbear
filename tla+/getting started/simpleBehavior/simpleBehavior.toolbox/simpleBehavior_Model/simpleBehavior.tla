--------------------------- MODULE simpleBehavior ---------------------------
EXTENDS TLC, Integers
(* --algorithm flags
variables flags \in [1..3 -> BOOLEAN], next = TRUE;
begin
  while next do
    with f \in DOMAIN flags, n \in BOOLEAN do
      flags[f] := ~flags[f];
      next := n;
    end with;
  end while;
end algorithm; *)
\* BEGIN TRANSLATION (chksum(pcal) = "8f3feb40" /\ chksum(tla) = "871ebbdc")
VARIABLES flags, next, pc

vars == << flags, next, pc >>

Init == (* Global variables *)
        /\ flags \in [1..3 -> BOOLEAN]
        /\ next = TRUE
        /\ pc = "Lbl_1"

Lbl_1 == /\ pc = "Lbl_1"
         /\ IF next
               THEN /\ \E f \in DOMAIN flags:
                         \E n \in BOOLEAN:
                           /\ flags' = [flags EXCEPT ![f] = ~flags[f]]
                           /\ next' = n
                    /\ pc' = "Lbl_1"
               ELSE /\ pc' = "Done"
                    /\ UNCHANGED << flags, next >>

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Lbl_1
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION 
=============================================================================
\* Modification History
\* Last modified Fri Jul 25 23:14:26 IST 2025 by ngavali
\* Created Wed Jul 23 17:40:46 IST 2025 by ngavali
