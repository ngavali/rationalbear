# Sojourn

Simple In-memory key value store.

(Still under developement)

Supported commands:

1. add - adds key value to the key value store

    add key value
2. get - returns key's value if key exists in the key value store

    get key
3. del - deletes key and it's value from the key value store

    del key

Max length 255 for key and value

Build instructions

g++ -std=c++11 -pthread sojourn.cpp -o sojourn
