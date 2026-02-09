------------------------- MODULE bridge_controller -------------------------

EXTENDS Integers, Naturals, Sequences, TLC
CONSTANT d, bound
AXIOM /\ d \in Nat 
      /\ d > 0
      
(*
--algorithm bridge_controller_model {
    variable n = 0, i = 0;
    
    procedure ML_out() {
        ML_out_action: n := n + 1;
        return;
    }
    
    procedure ML_in() {
        ML_in_action: n := n - 1;
        return;
    }
    
    \* Main program
    {
        loop: while(i < bound) {
            choice: either { 
                ML_out_gaurd: if (TRUE) { 
                    ML_out_happens: call ML_out(); 
                }; 
            }
            or { 
                ML_in_gaurd: if (TRUE) { 
                    ML_in_happens: call ML_in(); 
                }; 
            };
            progress: i := i + 1;
        }
    }
    
}
*)
\* BEGIN TRANSLATION (chksum(pcal) = "ea878b25" /\ chksum(tla) = "b136e7a")
VARIABLES pc, n, i, stack

vars == << pc, n, i, stack >>

Init == (* Global variables *)
        /\ n = 0
        /\ i = 0
        /\ stack = << >>
        /\ pc = "loop"

ML_out_action == /\ pc = "ML_out_action"
                 /\ n' = n + 1
                 /\ pc' = Head(stack).pc
                 /\ stack' = Tail(stack)
                 /\ i' = i

ML_out == ML_out_action

ML_in_action == /\ pc = "ML_in_action"
                /\ n' = n - 1
                /\ pc' = Head(stack).pc
                /\ stack' = Tail(stack)
                /\ i' = i

ML_in == ML_in_action

loop == /\ pc = "loop"
        /\ IF i < bound
              THEN /\ pc' = "choice"
              ELSE /\ pc' = "Done"
        /\ UNCHANGED << n, i, stack >>

choice == /\ pc = "choice"
          /\ \/ /\ pc' = "ML_out_gaurd"
             \/ /\ pc' = "ML_in_gaurd"
          /\ UNCHANGED << n, i, stack >>

ML_out_gaurd == /\ pc = "ML_out_gaurd"
                /\ IF TRUE
                      THEN /\ pc' = "ML_out_happens"
                      ELSE /\ pc' = "progress"
                /\ UNCHANGED << n, i, stack >>

ML_out_happens == /\ pc = "ML_out_happens"
                  /\ stack' = << [ procedure |->  "ML_out",
                                   pc        |->  "progress" ] >>
                               \o stack
                  /\ pc' = "ML_out_action"
                  /\ UNCHANGED << n, i >>

ML_in_gaurd == /\ pc = "ML_in_gaurd"
               /\ IF TRUE
                     THEN /\ pc' = "ML_in_happens"
                     ELSE /\ pc' = "progress"
               /\ UNCHANGED << n, i, stack >>

ML_in_happens == /\ pc = "ML_in_happens"
                 /\ stack' = << [ procedure |->  "ML_in",
                                  pc        |->  "progress" ] >>
                              \o stack
                 /\ pc' = "ML_in_action"
                 /\ UNCHANGED << n, i >>

progress == /\ pc = "progress"
            /\ i' = i + 1
            /\ pc' = "loop"
            /\ UNCHANGED << n, stack >>

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == ML_out \/ ML_in \/ loop \/ choice \/ ML_out_gaurd \/ ML_out_happens
           \/ ML_in_gaurd \/ ML_in_happens \/ progress
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION 

Inv0 == n \in Nat
Inv1 == n <= d

ML_out_event_gaurd == TRUE
ML_in_event_gaurd == TRUE
delock_free == ML_out_event_gaurd \/ ML_in_event_gaurd 
=============================================================================
\* Modification History
\* Last modified Mon Feb 09 23:45:56 IST 2026 by ngavali
\* Created Mon Feb 09 23:31:06 IST 2026 by ngavali
