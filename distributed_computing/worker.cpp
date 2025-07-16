#include<unistd.h>
#include<stdio.h>
#include<sys/wait.h>
#include<sys/resource.h>
#include<thread>

int main(int argc, char * argv[]) {

  const char * command = argv[1];
  int retries = 3;

  while ( retries-- ) {
    if ( command ) {
      printf("Running the command = %s\n", command);

      int pid = fork();

      if ( pid == 0 ) {
        execvp(command, (char* const *)(argv+1));
      } else {
        printf("PID of child is : %d\n", pid);
      }

      std::thread t1([](int pid){
          int child_status = -1;
          struct rusage rusage = {};
          wait4(pid, &child_status, 0, &rusage);
          printf("Result:\nMaxRss=%ld\nTime=%lds%ldus\n", rusage.ru_maxrss, rusage.ru_utime.tv_sec+rusage.ru_stime.tv_sec, rusage.ru_utime.tv_usec+rusage.ru_stime.tv_usec);
          }, pid);

      t1.join();
    }
  }

  printf("Main completed...\n");

}
