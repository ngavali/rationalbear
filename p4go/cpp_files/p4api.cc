#include <cstdio>
#include <iostream>

# include "myclient.h"
# include "p4libs.h"

using namespace std;

extern "C" int RunCmd(int , char **, char **);

class p4lib
{
    MyClientUser ui;
    ClientApi client;
    StrBuf msg;
    Error e;
    int drop = 0;
    public: 
    void init();
    void connect();
    void disconnect();
    void run(int, char**);
    StrBuf message();
    Error error();
    void final();
    int dropped();
    void clear();
};

int p4lib::dropped() {
    drop++;
    int is_dropped = client.Dropped();
    return is_dropped;
}

void p4lib::init() 
{
    P4Libraries::Initialize( P4LIBRARIES_INIT_ALL, &e );
    if( e.Test() )
    {
        e.Fmt( &msg );
        fprintf( stderr, "0001 : %s\n", msg.Text() );
    }
}

void p4lib::connect() 
{
    //dropped();
    client.Init( &e );
    if( e.Test() )
    {
        e.Fmt( &msg );
        fprintf( stderr, "0002 %s\n", msg.Text() );
    }
}

void p4lib::run(int argc, char** argv)
{
    dropped();
    client.SetArgv( argc - 1, argv + 1);
    client.Run(argv[0], &ui);
    //Capture command output
    msg = ui.MyData();
}

StrBuf p4lib::message() 
{
    StrBuf msg = ui.MyData();
    //ui.Clear();
    return msg;
}

Error p4lib::error() 
{
    return e;
}

void p4lib::final() 
{
    dropped();
    client.Final(&e);
    if( e.Test() )
    {
        e.Fmt( &msg );
        fprintf( stderr, "0003 %s\n", msg.Text() );
    }
}

void p4lib::disconnect() 
{
    dropped();
    P4Libraries::Shutdown( P4LIBRARIES_INIT_ALL, &e );
    if( e.Test() )
    {
        e.Fmt( &msg );
        fprintf( stderr, "0004 %s\n", msg.Text() );
    }
}

void p4lib::clear()
{
    ui.Clear();
    message();
}

/*
 * Wrapper for C
 */


typedef void* P4Client;

extern "C" P4Client NewP4Client();
extern "C" void Initialize(P4Client);
extern "C" void Connect(P4Client);
extern "C" void Run(P4Client, int, char**, char**);
extern "C" void Message(P4Client);
extern "C" void Final(P4Client);
extern "C" void Disconnect(P4Client ) ;
extern "C" void Clear(P4Client ) ;
extern "C" int  Dropped(P4Client ) ;

P4Client NewP4Client() 
{
    p4lib* P4Lib = new p4lib;
    return P4Lib;
}

void Initialize(P4Client p4client) 
{
    static_cast<p4lib*>(p4client)->init();
}

void Connect(P4Client p4client)
{
    static_cast<p4lib*>(p4client)->connect();
}

void Run(P4Client p4client, int argc, char **message, char **argv) 
{
    // Run the command
    static_cast<p4lib*>(p4client)->run(argc, argv);
    //Capture command output
    StrBuf msg = static_cast<p4lib*>(p4client)->message();
    char* tmp = (char*) malloc(strlen(msg.Text()) * sizeof(char) + 1);
    *message = strcpy(tmp, msg.Text());
}

void Message(P4Client p4client) 
{
    cout << "==Message==\n" << static_cast<p4lib*>(p4client)->message().Text() << endl;
}

void Final(P4Client p4client) 
{
    static_cast<p4lib*>(p4client)->final();
}

void Disconnect(P4Client p4client) 
{
    static_cast<p4lib*>(p4client)->disconnect();
}

void Clear(P4Client p4client) 
{
    static_cast<p4lib*>(p4client)->clear();
}

int Dropped(P4Client p4client)
{
    return static_cast<p4lib*>(p4client)->dropped();
}
