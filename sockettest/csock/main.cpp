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

		// char send_buf[200];			
		// memset(send_buf, 0, 200*sizeof(char));
		msgpack::type::tuple<int, float, std::string> src(1, 0.675, "sus");
		std::stringstream send_buf;
		msgpack::pack(send_buf, src);
		std::stringstream len_buf;
		len_buf.put(strlen(send_buf.str().c_str())*sizeof(char));
		
		while (true){
			if( send(s2, len_buf.str().c_str(), strlen(len_buf.str().c_str())*sizeof(char), 0) == -1 ) {
				printf("Error on send() call \n");
			}
			if( send(s2, send_buf.str().c_str(), strlen(send_buf.str().c_str())*sizeof(char), 0) == -1 ) {
				printf("Error on send() call \n");
			}
		}
		close(s2);
	}


	return 0;
}
