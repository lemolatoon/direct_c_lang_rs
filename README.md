# direct_c_lang_rs
マクロの入力として与えられたC言語の関数を直接Rustのコードから呼べるインターフェースを提供する。

## 例
[main.rs](https://github.com/lemolatoon/direct_c_lang_rs/blob/master/direct_c_lang_sample/src/main.rs)

## 実装
[lib.rs](https://github.com/lemolatoon/direct_c_lang_rs/blob/master/direct_c_lang/src/lib.rs)


## マクロの展開例
展開前
```rust
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

```

展開後
```rust
extern "C" {
    fn linked_list_new() -> *mut ::core::ffi::c_void;
    fn node_new(arg0: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn linked_list_push(
        arg0: *mut ::core::ffi::c_void,
        arg1: ::core::ffi::c_int,
    ) -> ::core::ffi::c_void;
    fn linked_list_pop(arg0: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn fib(arg0: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
global_asm!(".intel_syntax noprefix\n\n.global main\n.text\n.global linked_list_new\nlinked_list_new:\n  push rbp\n  mov rbp, rsp\n  sub rsp, 16\n  lea rax, [rbp-8]\n  push rax\n  push 1\n  push 8\n  pop rsi\n  pop rdi\n  sub rsp, 8\n  call calloc\n  add rsp, 8\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-8]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  jmp .Llinked_list_new_ret\n.Llinked_list_new_ret:\n  mov rsp, rbp\n  pop rbp\n  ret\n.text\n.global node_new\nnode_new:\n  push rbp\n  mov rbp, rsp\n  sub rsp, 16\n  mov rax, rbp\n  sub rax, 4\n  mov DWORD PTR [rax], edi\n  lea rax, [rbp-16]\n  push rax\n  push 1\n  push 16\n  pop rsi\n  pop rdi\n  sub rsp, 8\n  call calloc\n  add rsp, 8\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-16]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 0\n  push rax\n  lea rax, [rbp-4]\n  mov eax, DWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov DWORD PTR [rax], edi\n  push rdi\n  pop rax\n  lea rax, [rbp-16]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  jmp .Lnode_new_ret\n.Lnode_new_ret:\n  mov rsp, rbp\n  pop rbp\n  ret\n.text\n.global linked_list_push\nlinked_list_push:\n  push rbp\n  mov rbp, rsp\n  sub rsp, 48\n  mov rax, rbp\n  sub rax, 8\n  mov QWORD PTR [rax], rdi\n  mov rax, rbp\n  sub rax, 12\n  mov DWORD PTR [rax], esi\n  lea rax, [rbp-24]\n  push rax\n  lea rax, [rbp-8]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-32]\n  push rax\n  lea rax, [rbp-24]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 0\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-32]\n  mov rax, QWORD PTR [rax]\n  push rax\n  push 0\n  pop rdi\n  pop rax\n  cmp rax, rdi\n  sete al\n  movzx rax, al\n  push rax\n  pop rax\n  cmp eax, 0\n  je .Lend1\n  lea rax, [rbp-24]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 0\n  push rax\n  lea rax, [rbp-12]\n  mov eax, DWORD PTR [rax]\n  push rax\n  pop rdi\n  sub rsp, 8\n  call node_new\n  add rsp, 8\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  jmp .Llinked_list_push_ret\n.Lend1:\n  lea rax, [rbp-40]\n  push rax\n  lea rax, [rbp-32]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n.Lbegin2:\n  lea rax, [rbp-40]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 8\n  mov rax, QWORD PTR [rax]\n  push rax\n  push 0\n  pop rdi\n  pop rax\n  cmp rax, rdi\n  setne al\n  movzx rax, al\n  push rax\n  pop rax\n  cmp eax, 0\n  je .Lend2\n  lea rax, [rbp-40]\n  push rax\n  lea rax, [rbp-40]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 8\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  jmp .Lbegin2\n.Lend2:\n  lea rax, [rbp-40]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 8\n  push rax\n  lea rax, [rbp-12]\n  mov eax, DWORD PTR [rax]\n  push rax\n  pop rdi\n  sub rsp, 8\n  call node_new\n  add rsp, 8\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n.Llinked_list_push_ret:\n  mov rsp, rbp\n  pop rbp\n  ret\n.text\n.global linked_list_pop\nlinked_list_pop:\n  push rbp\n  mov rbp, rsp\n  sub rsp, 48\n  mov rax, rbp\n  sub rax, 8\n  mov QWORD PTR [rax], rdi\n  lea rax, [rbp-16]\n  push rax\n  lea rax, [rbp-8]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-24]\n  push rax\n  lea rax, [rbp-16]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 0\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-24]\n  mov rax, QWORD PTR [rax]\n  push rax\n  push 0\n  pop rdi\n  pop rax\n  cmp rax, rdi\n  sete al\n  movzx rax, al\n  push rax\n  pop rax\n  cmp eax, 0\n  je .Lend3\n  push 0\n  pop rax\n  jmp .Llinked_list_pop_ret\n.Lend3:\n  lea rax, [rbp-24]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 8\n  mov rax, QWORD PTR [rax]\n  push rax\n  push 0\n  pop rdi\n  pop rax\n  cmp rax, rdi\n  sete al\n  movzx rax, al\n  push rax\n  pop rax\n  cmp eax, 0\n  je .Lend4\n  lea rax, [rbp-28]\n  push rax\n  lea rax, [rbp-24]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 0\n  mov eax, DWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov DWORD PTR [rax], edi\n  push rdi\n  pop rax\n  lea rax, [rbp-16]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 0\n  push rax\n  push 0\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-28]\n  mov eax, DWORD PTR [rax]\n  push rax\n  pop rax\n  jmp .Llinked_list_pop_ret\n.Lend4:\n  lea rax, [rbp-32]\n  push rax\n  push 0\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-40]\n  push rax\n  lea rax, [rbp-24]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n.Lbegin5:\n  lea rax, [rbp-40]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 8\n  mov rax, QWORD PTR [rax]\n  push rax\n  push 0\n  pop rdi\n  pop rax\n  cmp rax, rdi\n  setne al\n  movzx rax, al\n  push rax\n  pop rax\n  cmp eax, 0\n  je .Lend5\n  lea rax, [rbp-32]\n  push rax\n  lea rax, [rbp-40]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-40]\n  push rax\n  lea rax, [rbp-40]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 8\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  jmp .Lbegin5\n.Lend5:\n  lea rax, [rbp-32]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 8\n  push rax\n  push 0\n  pop rdi\n  pop rax\n  mov QWORD PTR [rax], rdi\n  push rdi\n  pop rax\n  lea rax, [rbp-40]\n  mov rax, QWORD PTR [rax]\n  push rax\n  pop rax\n  add rax, 0\n  mov eax, DWORD PTR [rax]\n  push rax\n  pop rax\n  jmp .Llinked_list_pop_ret\n.Llinked_list_pop_ret:\n  mov rsp, rbp\n  pop rbp\n  ret\n.text\n.global fib\nfib:\n  push rbp\n  mov rbp, rsp\n  sub rsp, 16\n  mov rax, rbp\n  sub rax, 4\n  mov DWORD PTR [rax], edi\n  lea rax, [rbp-4]\n  mov eax, DWORD PTR [rax]\n  push rax\n  push 0\n  push 1\n  pop rdi\n  pop rax\n  sub rax, rdi\n  push rax\n  pop rdi\n  pop rax\n  cmp eax, edi\n  setl al\n  movzx rax, al\n  push rax\n  pop rax\n  cmp eax, 0\n  je .Lend6\n  push 0\n  push 1\n  pop rdi\n  pop rax\n  sub rax, rdi\n  push rax\n  pop rax\n  jmp .Lfib_ret\n.Lend6:\n  lea rax, [rbp-4]\n  mov eax, DWORD PTR [rax]\n  push rax\n  push 0\n  pop rdi\n  pop rax\n  cmp eax, edi\n  sete al\n  movzx rax, al\n  push rax\n  pop rax\n  cmp eax, 0\n  je .Lelse8\n  push 1\n  jmp .Lend8\n.Lelse8:\n  lea rax, [rbp-4]\n  mov eax, DWORD PTR [rax]\n  push rax\n  push 1\n  pop rdi\n  pop rax\n  cmp eax, edi\n  sete al\n  movzx rax, al\n  push rax\n.Lend8:\n  pop rax\n  cmp eax, 0\n  je .Lend7\n  push 1\n  pop rax\n  jmp .Lfib_ret\n.Lend7:\n  lea rax, [rbp-8]\n  push rax\n  lea rax, [rbp-4]\n  mov eax, DWORD PTR [rax]\n  push rax\n  push 1\n  pop rdi\n  pop rax\n  sub rax, rdi\n  push rax\n  pop rdi\n  sub rsp, 8\n  call fib\n  add rsp, 8\n  push rax\n  pop rdi\n  pop rax\n  mov DWORD PTR [rax], edi\n  push rdi\n  pop rax\n  lea rax, [rbp-12]\n  push rax\n  lea rax, [rbp-4]\n  mov eax, DWORD PTR [rax]\n  push rax\n  push 2\n  pop rdi\n  pop rax\n  sub rax, rdi\n  push rax\n  pop rdi\n  sub rsp, 8\n  call fib\n  add rsp, 8\n  push rax\n  pop rdi\n  pop rax\n  mov DWORD PTR [rax], edi\n  push rdi\n  pop rax\n  lea rax, [rbp-8]\n  mov eax, DWORD PTR [rax]\n  push rax\n  lea rax, [rbp-12]\n  mov eax, DWORD PTR [rax]\n  push rax\n  pop rdi\n  pop rax\n  add rax, rdi\n  push rax\n  pop rax\n  jmp .Lfib_ret\n.Lfib_ret:\n  mov rsp, rbp\n  pop rbp\n  ret\n");
```

## 参考
https://speakerdeck.com/lemolatoon/rustnoshou-sok-kimakurodehei-mo-shu-ru-men
