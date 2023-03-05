use direct_c_lang::c_lang;
use std::{arch::global_asm, ffi::c_void};

fn main() {
    println!("Hello, world! from Rust.");
    println!(
        "{:?}",
        (0..10)
            .into_iter()
            .map(|n| unsafe { fib(n) })
            .collect::<Vec<_>>()
    );
    let linked_list: *mut c_void = unsafe { linked_list_new() };
    assert_eq!(unsafe { linked_list_pop(linked_list) }, 0);
    for i in 1..=10 {
        unsafe { linked_list_push(linked_list, i) };
    }
    let mut sum = 0;
    loop {
        let popped = unsafe { linked_list_pop(linked_list) };
        println!("{} popped", popped);
        if popped == 0 {
            break;
        }
        sum += popped;
    }
    assert_eq!(sum, 55);
    println!("{}", sum);
}

c_lang! {
    #include<stdio.h>
    #include<stdlib.h>

    typedef struct Node Node;
    typedef struct LinkedList LinkedList;
    struct Node {
        int value;
        struct Node* next;
    };

    struct LinkedList {
        struct Node* head;
    };

    void *linked_list_new(void) {
        LinkedList* ptr = calloc(1, sizeof(LinkedList));
        return ptr;
    }

    void *node_new(int value) {
        Node* ptr = calloc(1, sizeof(Node));
        ptr->value = value;
        return ptr;
    }

    void linked_list_push(void *_list, int value) {
        LinkedList* list = _list;
        Node* head = list->head;
        if (head == NULL) {
            list->head = node_new(value);
            return;
        }
        Node *node = head;
        while (node->next != NULL) {
            node = node->next;
        }
        node->next = node_new(value);
    }

    int linked_list_pop(void *_list) {
        LinkedList* list = _list;
        Node* head = list->head;
        if (head == NULL) {
            return 0;
        }
        if (head->next == NULL) {
            int popped = head->value;
            list->head = NULL;
            return popped;
        }
        Node *prev = NULL;
        Node *node = head;
        while (node->next != NULL) {
            prev = node;
            node = node->next;
        }
        prev->next = NULL;
        return node->value;
    }


    int fib(int n) {
        if (n < -1) {
            return -1;
        }
        if (n == 0 || n == 1) {
            return 1;
        }

        int n1 = fib(n - 1);
        int n2 = fib(n - 2);
        return n1 + n2;
    }
}
