#ifndef _EXAMPLE_H
#define _EXAMPLE_H

#ifdef __cplusplus
class Player{
    public:
        char* name;
};
class Opaque;
class A;

extern "C"{
#endif

int double_input(int input);
Player create_player(char* name);
Player* create_player_pointer(char* name);
void free_player_pointer(Player* p);
void free_player(Player p);
char* check_char();
void free_char();
Player default_player();
A* create_opaque();
char* opaque_name(A* op);
void free_opaque(A* op);

#ifdef __cplusplus
}
#endif
#endif
