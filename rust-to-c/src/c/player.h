#ifndef __GEO_IP_H_
#define __GEO_IP_H_

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

	typedef struct player
	{
		char* name;
		uint32_t id;
	}player;
	player* player_instance();

#ifdef __cplusplus
}
#endif
#endif
