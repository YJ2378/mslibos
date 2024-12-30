#include <sys/time.h>
#include <stdio.h>
#include <iostream>
#include <vector>
#include <cstring>
#include <stdlib.h>
#include <cctype>
#include <fstream>
#include <algorithm>
#include <unordered_map>

using namespace std;

#define MAX_BUFFER_SIZE 1024 * 64


__attribute__((import_module("env"), import_name("buffer_register"))) void buffer_register(void *slot_name, int name_size, void *buffer, int buffer_size);
__attribute__((import_module("env"), import_name("access_buffer"))) void access_buffer(void *slot_name, int name_size, void *buffer, int buffer_size);


void get_time() {
    timeval tv{};
    gettimeofday(&tv, nullptr);
    printf("%lld.%06lld\n", tv.tv_sec, tv.tv_usec);
}

int main()  {
    get_time();
    string slot_name = "tmp";
    // string buffer(MAX_BUFFER_SIZE, '0');
    vector<char> buffer(MAX_BUFFER_SIZE, 0);
    buffer_register((void*)slot_name.c_str(), slot_name.length(), (void*)buffer.data(), MAX_BUFFER_SIZE);

    get_time();
    access_buffer((void*)slot_name.c_str(), slot_name.length(), (void*)buffer.data(), MAX_BUFFER_SIZE);
    get_time();
    
    return 0;
}