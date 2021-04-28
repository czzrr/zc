#ifndef NODE_H
#define NODE_H

typedef struct node {
    int data;
    struct node *prev;
    struct node *next;
} Node;

// Insert 'new_node' into the doubly linked list as the next node from 'node'
void insert(Node *node, Node *new_node) {
    // We can only perform insertion if both nodes are non-null 
    if (!(node && new_node))
        return;

    // Save a temporary pointer to the next node from 'node'
    Node *t = node->next;
    // Set 'new_node' as the next node from 'node'
    node->next = new_node;
    // Update previous and next nodes from 'new_node'
    new_node->prev = node;
    new_node->next = t;
    // Update previous node from 't' only if it's non-null
    if (t)
        t->prev = new_node;
}

// -- Helper functions for printing --

void print_list_forward(Node* node) {
    while(node) {
        printf("%d ", node->data);
        node = node->next;
    }
    printf("\n");
}

void print_list_backward(Node* node) {
    while(node) {
        printf("%d ", node->data);
        node = node->prev;
    }
    printf("\n");
}

#endif