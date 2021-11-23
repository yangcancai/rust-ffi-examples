#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
typedef struct Player
{
    char* name;
}Player;
typedef struct Opaque Opaque;
typedef struct A A;
extern int32_t double_input(int32_t input);
extern Player create_player(char* name);
extern Player* create_player_pointer(char* name);
extern void free_player_pointer(Player* p);
extern void free_player(Player p);
extern char* check_char();
extern void free_char();
extern Player default_player();
// extern Opaque* create_opaque();
extern A* create_opaque();
char* opaque_name(A* op);
extern void free_opaque(Opaque* op);
int main() {
    int input = 4;
    Player p;
    Player* p1;
    Player def;
    // Opaque* op;
    A* op;
    char* op_name;
    int output = double_input(input);
    printf("%d * 2 = %d\n", input, output);
    p = create_player("hello");
    printf("player name = %s\n", p.name);
    p1 = create_player_pointer("hello1");
    printf("player name = %s\n", p1->name);
    // free_player(p);
    // free_player_pointer(p1);
    // or
    free(p1);
    char* c = check_char();
    printf("c=%s",c);
    // free_char(c);
    // or
    free(c);
    def= default_player();
    printf("player name = %s\n", def.name);
    free_player(def);
    //free_char(def.name);
    // or
    // free(def.name);

    op = create_opaque();
    op_name = opaque_name(op);
    printf("opaque name = %s\n", op_name);
    free(op_name);
    free_opaque(op);
    return 0;
}
