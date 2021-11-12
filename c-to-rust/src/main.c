#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
typedef struct Player
{
    char* name;
}Player;

extern int32_t double_input(int32_t input);
extern Player create_player(char* name);
extern Player* create_player_pointer(char* name);
extern void free_player_pointer(Player* p);
extern void free_player(Player p);
extern char* check_char();
extern void free_char();
extern Player default_player();
int main() {
    int input = 4;
    Player p;
    Player* p1;
    Player def;
    int output = double_input(input);
    printf("%d * 2 = %d\n", input, output);
    p = create_player("hello");
    printf("player name = %s\n", p.name);
    p1 = create_player_pointer("hello1");
    printf("player name = %s\n", p1->name);
    free_player(p);
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
    free_char(def.name);
    // or
    // free(def.name);
    return 0;
}
