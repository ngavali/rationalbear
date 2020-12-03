#include <iostream>
#include <sys/types.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netdb.h>
#include <arpa/inet.h>
#include <string.h>
#include <string>

#include <thread>
#include <mutex>
#include <map>

std::mutex keyValueStoreLock;

struct cmp_str {
  bool operator()(char const *a, char const *b) const {
    return strcmp(a, b) < 0;
  }
};

std::map<const char *, const char *, cmp_str> keyValueStore;

const char *get_from_keyValueStore(const char *key) {
  return keyValueStore.find(key)->second;
}

bool erase_from_keyValueStore(const char *key) {
  std::lock_guard<std::mutex> lg(keyValueStoreLock);
  if (get_from_keyValueStore(key) != NULL) {
    const char* k = keyValueStore.find(key)->first;
    const char* v = keyValueStore.find(key)->second;
    printf("Memory address: %p %p\n", k, v);
    keyValueStore.erase(key);
    delete k;
    delete v;
    return true;
  }
  return false;
}

bool add_to_keyValueStore(const char *key, const char *val) {
  std::lock_guard<std::mutex> lg(keyValueStoreLock);
  if (get_from_keyValueStore(key) == NULL) {
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
  int pk = 0, k = 0;
  int iteration = 0;

  bool repl = true;

  while (repl) {
    memset(buf, 0, 4096);
    memset(command, 0, 256);
    memset(key, 0x00, 256);
    memset(val, 0x00, 256);

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
              break;
            case 1:
              strncpy(key, (const char *)(buf + pk), k);
              break;
            case 2:
              strncpy(val, (const char *)(buf + pk), k);
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
          char message[9] = "Del Ok";
          send(clientSocket, strcat(message, "\r\n"), strlen(message) + 3, 0);
        }
      }

      if (strcmp(command, "GET\0") == 0 || strcmp(command, "get\0") == 0) {
        const char *value = get_from_keyValueStore(key);
        if (value != NULL) {
          char message[264] = "Value=";
          strcpy(message, value);
          send(clientSocket, strcat(message, "\r\n"), strlen(message) + 3, 0);
        }
      }

      if (strcmp(command, "ADD\0") == 0 || strcmp(command, "add\0") == 0) {
        if (add_to_keyValueStore(key, val)) {
          char message[9] = "Add Ok";
          send(clientSocket, strcat(message, "\r\n"), strlen(message) + 3, 0);
        }
      }
  }

  close(clientSocket);

  delete &clientSocket;

  std::cout << "---Exited  thread" << std::endl;
}

int main() {

  int listening = socket(AF_INET, SOCK_STREAM, 0);
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

  bool forever = true;
  size_t connections = 0;

  while (forever) {
    int *clientSocket = new int;
    *clientSocket = accept(listening, (sockaddr *)&client, &clientSize);

    if (*clientSocket == -1) {
      std::cerr << "Problem with client connecting!" << std::endl;
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

    std::thread(&handle_connection, std::ref(*clientSocket)).detach();
    std::cout << "Total connections : " << connections << std::endl;
  }

  close(listening);
  return 0;
}
