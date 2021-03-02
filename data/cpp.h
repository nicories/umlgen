
#ifndef DEVICE_H

#define DEVICE_H

#include "gpio.h"

#include "piezo.h"

#include "display.h"

#include "game.h"

#include <fstream>

#include <string>

#include <iostream>

#include <vector>

#include <unistd.h> 

#include <stdio.h> 

#include <sys/socket.h> 

#include <stdlib.h> 

#include <netinet/in.h> 

#include <string.h> 

#include <netdb.h>

#include <arpa/inet.h>

#include <thread>

  

#define PORT     5678 

#define MAX_LINE 1024 

using namespace std;

class Device

{

    public:

        Device();

        static Display display;

        static char rcv_msg[MAX_LINE];

        char host_ip[15];

        string ip_to_number(char a, char b, char c);

        void enter_ip();

        void exit();

        void get_ip();

        bool single(Game *game, int player=1);

        int menu(vector<string> texts, bool music=false);

        static void error(std::string s);

        static void server(Game *game, int &server_socket, bool &connected, bool &leave, int player);

        bool host(Game *game, int player=1);

        bool join(Game *game, int player=2);

};

#endif
