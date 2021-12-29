#include "player.h"
player *player_instance()
{
	player *ret = (player*)malloc(sizeof(player));
	return ret;
}
void init(player *p){
	p->name = "hello";
	p->id = 1;
}
uint32_t id(player *p){
	return p-> id;
}
char* name(player *p){
	return p-> name;
}