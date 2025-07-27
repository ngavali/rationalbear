---------------------------- MODULE towerOfHanoi ----------------------------
EXTENDS TLC, Sequences, Integers
                
(* --algorithm Hanoi
variables tower = <<<<1, 2, 3>>, <<>>, <<>>>>, 

define 
  D == DOMAIN tower
end define;

begin
while TRUE do
  assert tower[3] /= <<1, 2, 3>>;
  with from \in {x \in D : tower[x] /= <<>>},
       to \in {
                y \in D : 
                  \/ tower[y] = <<>>
                  \/ Head(tower[from]) < Head(tower[y])
              } 
  do
    tower[from] := Tail(tower[from]) ||
    tower[to] := <<Head(tower[from])>> \o tower[to];
  end with;
end while;
end algorithm; *)
\* BEGIN TRANSLATION (chksum(pcal) = "751c85ff" /\ chksum(tla) = "19c39e85")
VARIABLE tower

(* define statement *)
D == DOMAIN tower


vars == << tower >>

Init == (* Global variables *)
        /\ tower = <<<<1, 2, 3>>, <<>>, <<>>>>

Next == /\ Assert(tower[3] /= <<1, 2, 3>>, 
                  "Failure of assertion at line 13, column 3.")
        /\ \E from \in {x \in D : tower[x] /= <<>>}:
             \E to \in {
                         y \in D :
                           \/ tower[y] = <<>>
                           \/ Head(tower[from]) < Head(tower[y])
                       }:
               tower' = [tower EXCEPT ![from] = Tail(tower[from]),
                                      ![to] = <<Head(tower[from])>> \o tower[to]]

Spec == Init /\ [][Next]_vars

\* END TRANSLATION 
=============================================================================
\* Modification History
\* Last modified Sun Jul 27 13:15:49 IST 2025 by ngavali
\* Created Sun Jul 27 13:15:25 IST 2025 by ngavali
