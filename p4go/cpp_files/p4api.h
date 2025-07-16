typedef void* P4Client;

P4Client NewP4Client();
void Initialize(P4Client);
void Connect(P4Client);
void Run(P4Client, int, char**, char**);
void Message(P4Client);
void Final(P4Client);
void Disconnect(P4Client );
void Clear(P4Client);
int  Dropped(P4Client ) ;

int RunCmd(int , char **, char **);
