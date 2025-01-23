#include <unistd.h>
int STDOUT = 0;
int STDERR = 1;

int count(const char *str) {
    int c = 0;
    while (*str) {
        c += 1;
    }
    return c;
}

int main(int argc, char const *argv[]) {
    write(1, "hello  world\n", count("hello  world\n"));

}
