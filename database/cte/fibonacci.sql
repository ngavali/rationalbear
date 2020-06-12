set @limit:=100;
WITH RECURSIVE fibonacci ( num, sum ) as (
	select 0 num, 1 sum                                                 #Initialize 
	UNION                                                               #Collects into a Resultset
	select sum num, (num+sum) sum from fibonacci where sum <= @limit    #Repeat
)
select num as series from fibonacci                                         #Final result
;
