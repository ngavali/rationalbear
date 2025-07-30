--------------------------- MODULE simpleMessage ---------------------------
EXTENDS Sequences, TLC

(* --algorithm simpleMessage

variables 
    to_send = <<1, 2, 3>>,
    receive = <<>>,
    in_transit = {};
    can_send = TRUE;
    
begin
    while Len(receive) /= 3 do
    \* Send
        if can_send /\ to_send /= <<>> then
            \*in_transit := in_transit \cup {Head(to_send)};
            can_send := FALSE;
            \*to_send := Tail(to_send);
        end if;
    \* Receive
        either
            with msg \in in_transit do
                receive := Append(receive, msg);
                in_transit := in_transit \ {msg};
                either
                    can_send := TRUE;
                or
                    skip;
                end either;
            end with;
        or
            skip;
        end either;
    end while;
    \* assert receive = <<1, 2, 3>>;
end algorithm; *)
\* BEGIN TRANSLATION (chksum(pcal) = "a8e41445" /\ chksum(tla) = "7d1c2f85")
VARIABLES to_send, receive, in_transit, can_send, pc

vars == << to_send, receive, in_transit, can_send, pc >>

Init == (* Global variables *)
        /\ to_send = <<1, 2, 3>>
        /\ receive = <<>>
        /\ in_transit = {}
        /\ can_send = TRUE
        /\ pc = "Lbl_1"

Lbl_1 == /\ pc = "Lbl_1"
         /\ IF Len(receive) /= 3
               THEN /\ IF can_send /\ to_send /= <<>>
                          THEN /\ can_send' = FALSE
                          ELSE /\ TRUE
                               /\ UNCHANGED can_send
                    /\ \/ /\ pc' = "Lbl_2"
                       \/ /\ TRUE
                          /\ pc' = "Lbl_1"
               ELSE /\ pc' = "Done"
                    /\ UNCHANGED can_send
         /\ UNCHANGED << to_send, receive, in_transit >>

Lbl_2 == /\ pc = "Lbl_2"
         /\ \E msg \in in_transit:
              /\ receive' = Append(receive, msg)
              /\ in_transit' = in_transit \ {msg}
              /\ \/ /\ can_send' = TRUE
                 \/ /\ TRUE
                    /\ UNCHANGED can_send
         /\ pc' = "Lbl_1"
         /\ UNCHANGED to_send

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Lbl_1 \/ Lbl_2
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION 
=============================================================================
\* Modification History
\* Last modified Mon Jul 28 00:21:27 IST 2025 by ngavali
\* Created Mon Jul 28 00:04:48 IST 2025 by ngavali
