#include <unistd.h>
int STDOUT = 0;
int STDERR = 1;
int main(int argc, char const *argv[])
{
    write(STDOUT,"Hello world",12);
    return 0;
}
