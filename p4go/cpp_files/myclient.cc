# include <clientapi.h>
# include <clientprog.h>

# include <diff.h>
# include <enviro.h>
# include <echoctl.h>
# include <signaler.h>
# include <strops.h>
# include <runcmd.h>
# include <i18napi.h>
# include <charcvt.h>

# include <msgclient.h>
# include <msgserver.h>

# include <iostream>
using namespace std;

class MyClientUser : public ClientUser 
{
    StrBuf my_output;

    public:
    void OutputText( const char *, int  );
    StrBuf MyData();
    void OutputInfo( char , const char * );
    void Message( Error *);
};

void MyClientUser::OutputText( const char *data, int length )
{
    my_output.Set( data, length );
}

StrBuf MyClientUser::MyData() 
{
    return my_output;
}

void MyClientUser::Message( Error *err )
{
    StrBuf buf;
    err->Fmt( buf, EF_NEWLINE );
    my_output.Append( buf.Text() );
}
