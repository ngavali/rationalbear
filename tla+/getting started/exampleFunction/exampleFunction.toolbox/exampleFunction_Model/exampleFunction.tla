-------------------------- MODULE exampleFunction --------------------------

Flags == {"f1", "f2"}

(*--algorithm flags
variables
    \*flags \in { config \in [Flags -> BOOLEAN] : \E f \in Flags: config[f]} 
    flags \in [Flags -> BOOLEAN] 
    \*flags = [f \in Flags |-> FALSE];
begin
    with f \in Flags do
        flags[f] := TRUE;
    end with;
end algorithm; *)
\* BEGIN TRANSLATION (chksum(pcal) = "c0088da0" /\ chksum(tla) = "642834bf")
VARIABLES flags, pc

vars == << flags, pc >>

Init == (* Global variables *)
        /\ flags \in [Flags -> BOOLEAN]
        /\ pc = "Lbl_1"

Lbl_1 == /\ pc = "Lbl_1"
         /\ \E f \in Flags:
              flags' = [flags EXCEPT ![f] = TRUE]
         /\ pc' = "Done"

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Lbl_1
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION 

=============================================================================
\* Modification History
\* Last modified Mon Jul 28 22:50:07 IST 2025 by ngavali
\* Created Mon Jul 28 21:44:52 IST 2025 by ngavali
