/**
 * Definition for a Node.
 * struct Node {
 *     int val;
 *     struct Node *left;
 *     struct Node *right;
 *     struct Node *next;
 * };
 */

/* 2^12 - 1 == 1 + 2 + 4 + ... + 2048 */
#define MAX_VECTOR_SIZE 2048
struct heapless_vec {
    struct Node *data[MAX_VECTOR_SIZE];
    int vec_size;
};

void vec_init(struct heapless_vec *v)
{
    v->vec_size = 0;
}

bool vec_is_empty(struct heapless_vec *v)
{
    return v->vec_size == 0;
}

void vec_push(struct heapless_vec *v, struct Node *n)
{
    // WARNING: no prevention on overflow here
    if (n == NULL) {
        return;
    }
    v->data[v->vec_size++] = n;
}

void process(struct heapless_vec *a, struct heapless_vec *b)
{
    struct Node *prev = NULL;
    for (int i = 0; i < a->vec_size; i++) {
        struct Node *n = a->data[i];
        if (prev != NULL) {
            prev->next = n;
        }
        prev = n;
        vec_push(b, n->left);
        vec_push(b, n->right);
    }
    vec_init(a);
}

struct Node *connect(struct Node *root)
{
    if (root == NULL) {
        return root;
    }
    struct heapless_vec vec1, vec2;
    vec_init(&vec1);
    vec_init(&vec2);
    vec_push(&vec1, root);

    while (!vec_is_empty(&vec1) || !vec_is_empty(&vec2)) {
        process(&vec1, &vec2);
        process(&vec2, &vec1);
    }

    return root;
}
