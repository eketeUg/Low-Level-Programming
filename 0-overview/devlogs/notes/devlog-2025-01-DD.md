# Devlog – January DD, 2026

## Date & Total time spent today

- 2026-01-DD
- ~9.5 hours (theory 2h, exercises 4h, project 3.5h)

## What I worked on today (concrete achievements)

- Finished K&R Ch. 5 pointer exercises
- Implemented doubly-linked list with sentinel node in C
- Ported the list to Rust using Box + Rc<RefCell> (very painful borrow dance)
- Fixed 3 memory leaks in C version using Valgrind

## Technical deep-dive / most interesting thing I learned

(Choose 1–2 things. Explain like you're teaching a smart junior engineer)

Today the biggest revelation was understanding why Rust forces us to use interior mutability for doubly-linked lists while C lets us mutate freely.

In C:

```c
struct Node {
    int data;
    struct Node *prev, *next;
};

// Mutate without drama
node->next->data = 42;
```

### How to Make This Sustainable During an Intensive Month

- **Write at the end** of the day — never during. Brain needs distance.
- **Keep it honest** — bad days, confusion, wasted time → all valuable.
- **Length**: 300–800 words. Quality > quantity.
- **Where to store**:
  - Private Git repo (recommended — future portfolio material)
  - Or just a folder on your machine + weekly git commit
  - Bonus: After January, you can clean it up and publish selected entries on GitHub / personal blog / DEV.to
- **Review cadence**:
  - Every Sunday (~30 min): skim previous week → spot recurring mistakes
  - End of month: write a summary post "What I Learned About Low-Level Programming in 31 Days"

### Bonus Level-Up Variants (Pick 1–2 if you have extra energy)

1. **Add code diffs/screenshots** — show before/after of a nasty bug fix
2. **One-paragraph "explainer"** each day on a core concept (great technical writing practice)
3. **Benchmark section** — when relevant (C vs Rust string handling, allocation cost…)
4. **Question log** — collect things you're still unclear about (great for later research)

Stick to this format for the whole month and you'll end up with:

- Dramatically better technical articulation
- Clear record of your growth trajectory
- Material you can reuse for job applications, blog posts, or even conference talks later

Now go crush Day 1 — and write that first devlog tonight.  
You've got this! 🚀
