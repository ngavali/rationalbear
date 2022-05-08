#include <cstdio>
#include <iostream>

//# include "clientapi.h"
# include "myclient.h"
# include "p4libs.h"

using namespace std;

extern "C" char* RunCmd(int argc, char **argv);

class p4lib
{
    MyClientUser ui;
    ClientApi client;
    StrBuf msg;
    Error e;
    public: 
    void init();
    void connect();
    void disconnect();
    void run(int, char**);
    StrBuf message();
    Error error();
    void final();
};

void p4lib::init() 
{
	P4Libraries::Initialize( P4LIBRARIES_INIT_ALL, &e );
}

void p4lib::connect() 
{
    client.Init( &e );
}

void p4lib::run(int argc, char** argv)
{
    client.SetArgv( argc - 1, argv + 1);
    client.Run(argv[0], &ui);
}

StrBuf p4lib::message() 
{
    return ui.MyData();
}

Error p4lib::error() 
{
    return e;
}

void p4lib::final() 
{
    client.Final(&e);
}

void p4lib::disconnect() 
{
	P4Libraries::Shutdown( P4LIBRARIES_INIT_ALL, &e );
}

char* RunCmd(int argc, char **argv)
{
    StrBuf msg;
    Error e;

    p4lib* p4 = new p4lib;

	p4->init();

    e = p4->error();
	if( e.Test() )
	{
	    e.Fmt( &msg );
	    fprintf( stderr, "%s\n", msg.Text() );
	    return msg.Text();
	}

	// Connect to server

	p4->connect();

    e = p4->error();
	if( e.Test() )
	{
	    e.Fmt( &msg );
	    fprintf( stderr, "%s\n", msg.Text() );
	    return msg.Text();
	}

	// Run the command
    p4->run(argc, argv);

    //Capture command output
    char * message = (char*) malloc(strlen(p4->message().Text()) * sizeof(char) + 1);
    message = strcpy(message , p4->message().Text());

	// Close connection

	p4->final();

    e = p4->error();
	if( e.Test() )
	{
	    e.Fmt( &msg );
	    fprintf( stderr, "%s\n", msg.Text() );
	    return msg.Text();
	}

    p4->disconnect();

    e = p4->error();
	if( e.Test() )
	{
	    e.Fmt( &msg );
	    fprintf( stderr, "%s\n", msg.Text() );
	    return msg.Text();
	}
	
	return message;
}
