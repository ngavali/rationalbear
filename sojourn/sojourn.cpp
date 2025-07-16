#include <iostream>
#include <sys/types.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>
#include <string.h>
#include <string>

#include <thread>
#include <shared_mutex>
#include <map>
#include <vector>
#include <csignal>

static int MAX_CONNECTIONS = 2;
static bool forever = true;
static int listening = -1;

class ThreadNg {
    int thread_id;
    bool busy;
    std::thread thread_handle;
    public:

    ThreadNg() {
        busy = false;
        printf("Worker created...\n");
    }

    ThreadNg(int id) {
        thread_id = id;
        busy = false;
        printf("Worker created...\n");
    }

    ThreadNg(ThreadNg &) = default;
    ThreadNg(ThreadNg &&) = default;

    void SetId(int id) {
        thread_id = id;
    }

    int Id() {
        return thread_id;
    }

    bool Status() {
        return busy;
    }

    template<class F, class... Args>
        void thread_wrapper(F&& user_function, Args&&... args) {
            printf("Worker=%d function started...\n", thread_id);
            user_function(std::ref(args)...);
            printf("Worker=%d function ended...\n", thread_id);
            busy = false;
        }

    template<class F, class... Args>
        void Run(F&& f, Args&&... args) {
            busy = true;
            thread_handle = std::thread( &ThreadNg::thread_wrapper<typename std::decay<F>::type,
                    typename std::decay<Args>::type...>,  
                    this,
                    std::forward<F>(f),
                    std::forward<Args>(args)... );
        }

    void Join() {
        if ( thread_handle.joinable() )
            thread_handle.join();
        else
            printf("Nothing to wait for...\n");
    }

    ~ThreadNg() {
        printf("In the destructor...\n");
        if ( thread_handle.joinable() )
            thread_handle.join();
        else
            printf("Nothing to wait for...\n");
        printf("Worker=%d destroyed...\n", thread_id);
    }
};

std::shared_mutex KeyValueStoreRWLock;

struct cmp_str {
    bool operator()(char const *a, char const *b) const {
        return strcmp(a, b) < 0;
    }
};

static std::map<const char *, const char *, cmp_str> keyValueStore;

const char *get_from_keyValueStore(const char *key) {
    std::shared_lock<std::shared_mutex> shared_read(KeyValueStoreRWLock);
    return keyValueStore.find(key)->second;
}

bool erase_from_keyValueStore(const char *key) {
    std::lock_guard<std::shared_mutex> exclusive_write(KeyValueStoreRWLock);
    if (keyValueStore.find(key)->second != NULL) {
        const char* k = keyValueStore.find(key)->first;
        const char* v = keyValueStore.find(key)->second;
        printf("Memory address: %p %p\n", k, v);
        keyValueStore.erase(key);
        fflush(stdout);
        delete k;
        delete v;
        return true;
    }
    return false;
}

bool add_to_keyValueStore(const char *key, const char *val) {
    std::lock_guard<std::shared_mutex> exclusive_write(KeyValueStoreRWLock);
    if (keyValueStore.find(key)->second == NULL) {
        char *insertKey, *insertVal;
        insertKey = new char[strlen(key + 1)];
        insertVal = new char[strlen(val + 1)];
        strcpy(insertKey, key);
        strcpy(insertVal, val);
        printf("Memory address: %p %p\n", insertKey, insertVal);
        fflush(stdout);
        keyValueStore.insert({insertKey, insertVal});
        return true;
    }
    return false;
}

void handle_connection(int &clientSocket) {
    char buf[4096];
    char command[256], key[256], val[256];
    char message[264];
    int pk = 0, k = 0;
    int iteration = 0;

    bool repl = true;

    while (repl) {
        //memset(buf, 0, 4096);
        //memset(command, 0, 256);
        //memset(key, 0x00, 256);
        //memset(val, 0x00, 256);
        //memset(message, 0x00, 264);

        int bytesRecv = recv(clientSocket, buf, 4096, 0);
        if (bytesRecv == -1) {
            std::cerr << "Connection issue." << std::endl;
            break;
        }
        if (bytesRecv == 0) {
            std::cout << "The client disconnected" << std::endl;
            break;
        }

        pk = 0;
        k = 0;
        iteration = 0;

        for (int i = 0; i < bytesRecv && iteration < 3; i++) {
            if (k > 255) {
                printf("Key size exceeded...");
                break;
            }
            if (buf[i] == ' ' || buf[i] == '\r') {
                switch (iteration) {
                    case 0:
                        strncpy(command, (const char *)(buf + pk), k);
                        command[k]='\0';
                        break;
                    case 1:
                        strncpy(key, (const char *)(buf + pk), k);
                        key[k]='\0';
                        break;
                    case 2:
                        strncpy(val, (const char *)(buf + pk), k);
                        val[k]='\0';
                        break;
                }
                iteration++;
                pk += k + 1;
                k = 0;
            } else {
                k++;
            }
        }

        if (strcmp(command, "exit\0") == 0 || strcmp(command, "quit\0") == 0) {
            repl = false;
            break;
        }

        if (strcmp(command, "DEL\0") == 0 || strcmp(command, "del\0") == 0) {
            if (erase_from_keyValueStore(key)) {
                strcpy(message, "Del Ok\r\n");
                send(clientSocket, message, strlen(message) + 1, 0);
            }
        }

        if (strcmp(command, "GET\0") == 0 || strcmp(command, "get\0") == 0) {
            const char *value = get_from_keyValueStore(key);
            if (value != NULL) {
                strcpy(message, "Value=");
                strcat(message, value);
                strcat(message, "\r\n");
                send(clientSocket, message, strlen(message) + 1, 0);
                printf("Bytes sent : %zd message = %s\n", strlen(message), message);
            }
        }

        if (strcmp(command, "SET\0") == 0 || strcmp(command, "set\0") == 0) {
            if (add_to_keyValueStore(key, val)) {
                strcpy(message, "Set Ok\r\n");
                send(clientSocket, message, strlen(message) + 1, 0);
                printf("Bytes added : %zd\n", strlen(val));
            }
        }
    }

    close(clientSocket);

    delete &clientSocket;

    std::cout << "---Exited  thread" << std::endl;
}

int main(int argc, char * argv[]) {


    if ( argc == 2 )
        MAX_CONNECTIONS = atoi(argv[1]);

    ThreadNg ThreadNgCollection[MAX_CONNECTIONS];

    for( auto& t: ThreadNgCollection ) {
        t.SetId(argc--);
    }

    int activeThreads = 0;

    listening = socket(AF_INET, SOCK_STREAM, 0);
    if (listening == -1) {
        std::cerr << "Can't create a socket" << std::endl;
        return -1;
    }

    sockaddr_in hint;
    hint.sin_family = AF_INET;
    hint.sin_port = htons(54000);

    inet_pton(AF_INET, "0.0.0.0", &hint.sin_addr);

    if (bind(listening, (const sockaddr *)&hint, sizeof(hint)) == -1) {
        std::cerr << "Can't bind to IP/Port" << std::endl;
        return -2;
    }

    if (listen(listening, SOMAXCONN) == -1) {
        std::cerr << "Can't listen!" << std::endl;
    }

    sockaddr_in client;
    socklen_t clientSize = sizeof(client);
    char host[NI_MAXHOST];
    char svc[NI_MAXSERV];

    int signal_num;
    std::thread signal_handler([](int signal_num) {
            signal(SIGINT|SIGTERM, [](int signal_num) {
                    std::cout << "The interrupt signal is (" << signal_num << "). \n";
                    close(listening);
                    forever = false;
                    });
            }, signal_num
    );

    size_t connections = 0;

    int thread_num = 0;
    while (forever) {

        int *clientSocket = new int;
        *clientSocket = accept(listening, (sockaddr *)&client, &clientSize);

        if (*clientSocket == -1) {
            std::cerr << "Problem with client connecting!" << std::endl;
            break;
        } else {
            printf("CONNECTED at socket=%p.\n", clientSocket);
            connections++;
        }

        memset(host, 0, NI_MAXHOST);
        memset(svc, 0, NI_MAXSERV);

        int result = getnameinfo((sockaddr *)&client, clientSize, host, NI_MAXHOST,
                svc, NI_MAXSERV, 0);

        if (result)
            std::cout << host << " -> connected on " << svc << std::endl;
        else {
            inet_ntop(AF_INET, &client.sin_addr, host, NI_MAXHOST);
            std::cout << host << " : connected on " << ntohs(client.sin_port)
                << std::endl;
        }

        int handled = false;
        int attempts = 0;

        while ( attempts<2*MAX_CONNECTIONS ) {
            if ( !ThreadNgCollection[thread_num].Status() ) {
                ThreadNgCollection[thread_num].Join();
                printf("Assigning job to thread : %d\n", thread_num);
                ThreadNgCollection[thread_num].Run(handle_connection, std::ref(*clientSocket));
                handled = true;
                break;
            }
            thread_num=(thread_num+1)%MAX_CONNECTIONS;
            attempts++;
        }

        if (!handled) {
            printf("Too many client connections...\n");
            close(*clientSocket);
            delete clientSocket;
        }

        activeThreads=0;
        for( auto& i : ThreadNgCollection ) {
            if (i.Status()) 
                activeThreads++;
        }
        std::cout << "Total  connections : " << connections << std::endl;
        std::cout << "Active connections : " << activeThreads << std::endl;

    }

    printf("Shutting down sojourn.\n");

    signal_handler.join();

    close(listening);

    return 0;

}
