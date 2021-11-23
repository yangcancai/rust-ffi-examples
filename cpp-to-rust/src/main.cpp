#include <iostream>
#include "example.h"

using namespace std;

int main() {
   int input = 10;
  // int output = double_input(input);
  // cout<<input<<" * 2 = "<<output<<"\n";
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
