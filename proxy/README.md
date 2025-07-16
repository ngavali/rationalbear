# Wormhole 

TCP proxy written in Go language. [Work in progress]

Connect client(s) to the closest backend (one with the least connection time) among the set of muliple available backends.

Configration Example:

* Define backends

server.json =>

{
        "mysql-1" : {
        "host" : "mysql-backend-1",
        "port" : 3306,
        "proxy" : false
        },
        "mysql-2" : {
        "host" : "mysql-backend-2",
        "port" : 3306,
        "proxy" : false
        },
        "mysql-3" : {
        "host" : "mysql-backend-3",
        "port" : 3306,
        "proxy" : false
        }
}

* Define tunnels

wormhole.json =>

{
        "bg1" : {
                "name" : "Mysql Farm",
                "port" : 3306,
                "destination" : [
                        "mysql-1",
                        "mysql-2",
                        "mysql-3"
                ],
                "algo" : 0,
                "check" : 5
        }
}

* Start service

$ go run wormhole.go

Creating wormholes...\
Wormhole: Mysql Farm\
Listening on port: 3306\
Algo: Closest\
Num of backends:  3\
Check interval:   5 seconds\
Name: mysql-1, Index:  0, Host: mysql-backend-1, Port:  3306\
Name: mysql-2, Index:  1, Host: mysql-backend-2, Port:  3306\
Name: mysql-3, Index:  2, Host: mysql-backend-3, Port:  3306
