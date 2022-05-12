p4go: libs/libp4go.so p4go.go
	go build
libs/libp4go.so: cpp_files/p4api.o cpp_files/myclient.o
	gcc -fPIC -shared -o libs/libp4go.so cpp_files/p4api.o cpp_files/myclient.o ${perforce_src_path}/lib/libclient.a ${perforce_src_path}/lib/libp4api.a ${perforce_src_path}/lib/libp4script.a ${perforce_src_path}/lib/libp4script_c.a ${perforce_src_path}/lib/libp4script_curl.a ${perforce_src_path}/lib/libp4script_sqlite.a ${perforce_src_path}/lib/librpc.a ${perforce_src_path}/lib/libsupp.a -lssl -lcrypto -lpthread -lcurl -lsqlite3 -ldl -lstdc++ -lm -lrt
cpp_files/myclient.o cpp_files/p4api.o: cpp_files/myclient.cc cpp_files/p4api.cc
	g++ -fPIC -c cpp_files/myclient.cc -o cpp_files/myclient.o -I${perforce_src_path}/include/p4
	g++ -fPIC -c cpp_files/p4api.cc -o cpp_files/p4api.o -I${perforce_src_path}/include/p4
clean:
	rm -rf libs/libp4go.so
	rm -rf cpp_files/p4api.o
	rm -rf cpp_files/myclient.o
