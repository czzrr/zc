#include <stdio.h>
#include "Node.h"

void print_forwards_backwards(Node*, Node*);

// Create some nodes to insert and print the list after each insertion
int main() {
    Node node1 = { .data = 19 };
    printf("Initial list: ");
    print_list_forward(&node1);
    printf("\n");

    printf("Insert 43 as next value from 19:\n");
    Node node2 = { .data = 43 };
    insert(&node1, &node2);
    print_forwards_backwards(&node1, &node2);

    printf("Insert 31 as next value from 19:\n");
    Node node3 = { .data = 31 };
    insert(&node1, &node3);
    print_forwards_backwards(&node1, &node2);


    printf("Insert 41 as next value from 31:\n");
    Node node4 = { .data = 41 };
    insert(&node3, &node4);
    print_forwards_backwards(&node1, &node2);

    return 0;
}

// Helper function for printing
void print_forwards_backwards(Node *node1, Node *node2) {
    printf("Forwards: ");
    print_list_forward(node1);
    printf("Backwards: ");
    print_list_backward(node2);
    printf("\n");
}