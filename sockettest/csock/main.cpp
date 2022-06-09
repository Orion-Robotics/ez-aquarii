#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <sys/types.h>
#include <msgpack.hpp>
#include <string>
#include <iostream>
#include <sstream>

static const char* socket_path = "/home/pythomancer/Documents/socket";
static const unsigned int nIncomingConnections = 5;

struct packet {
	int leInt;
	float leFloat;
	std::string leString;
	MSGPACK_DEFINE (leInt, leFloat, leString);
};

int main()
{
	//create server side
	int s = 0;
	int s2 = 0;
	struct sockaddr_un local, remote;
	int len = 0;

	s = socket(AF_UNIX, SOCK_STREAM, 0);
	if( -1 == s )
	{
		printf("Error on socket() call \n");
		return 1;
	}

	local.sun_family = AF_UNIX;
	strcpy( local.sun_path, socket_path );
	unlink(local.sun_path);
	len = strlen(local.sun_path) + sizeof(local.sun_family);
	if( bind(s, (struct sockaddr*)&local, len) != 0)
	{
		printf("Error on binding socket \n");
		return 1;
	}

	if( listen(s, nIncomingConnections) != 0 )
	{
		printf("Error on listen call \n");
	}

	bool bWaiting = true;
	while (bWaiting)
	{
		unsigned int sock_len = 0;
		printf("Waiting for connection.... \n");
		if( (s2 = accept(s, (struct sockaddr*)&remote, &sock_len)) == -1 )
		{
			printf("Error on accept() call \n");
			return 1;
		}

		printf("Server connected \n");

		packet src;
		src.leFloat = 0.675;
		src.leInt = 1;
		src.leString = "sus";
		std::stringstream send_buf;
		msgpack::pack(send_buf, src);
		unsigned long len_of_transmission = strlen(send_buf.str().c_str())*sizeof(char);
		const char* str_len_of_transmission = std::to_string(len_of_transmission).c_str();

		while (true){
			if( send(s2, str_len_of_transmission, strlen(str_len_of_transmission)*sizeof(char), 0) == -1 ) {
				printf("Error on send() call \n");
			}
			if( send(s2, send_buf.str().c_str(), len_of_transmission, 0) == -1 ) {
				printf("Error on send() call \n");
			}
			std::cout << "amogus" << std::endl;
		}
		close(s2);
	}


	return 0;
}
