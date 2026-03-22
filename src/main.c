#include <stdio.h>

int player_x = 10;
int goal_x = 20;

int hello() {
    return 5;
}

void update_logic() {
    if (player_x < goal_x) {
        player_x++;
        printf("Player moved to %d\n", player_x);
    } else {
        printf("Goal reached!\n");
    }
}

int main() {
    for(int i = 0; i < 15; i++) {
        update_logic();
    }
    return 0;
}