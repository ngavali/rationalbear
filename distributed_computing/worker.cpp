/* The MIT License

   Copyright (c) 2008, 2009 by gavali.nilesh80186 <gavali.nilesh80186@gmail.com>

   Permission is hereby granted, free of charge, to any person obtaining
   a copy of this software and associated documentation files (the
   "Software"), to deal in the Software without restriction, including
   without limitation the rights to use, copy, modify, merge, publish,
   distribute, sublicense, and/or sell copies of the Software, and to
   permit persons to whom the Software is furnished to do so, subject to
   the following conditions:

   The above copyright notice and this permission notice shall be
   included in all copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
   EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
   NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
   BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
   ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
   CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
   SOFTWARE.
*/

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
