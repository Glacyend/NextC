# Examples

> [!Warning]
>
> NextC is currently in the early design and development phase; syntax and semantic rules may be added, removed, or modified at any time. Until NextC 1.0 is released, this document is subject to significant change.
>
> All programs below are design prototypes; they cannot be compiled or run yet. Before NextC 1.0, the programs below are likely to change.

## Hello, World!

```nextc
import io;

void main() {
    io::println!("Hello, World!");
}
```

## Ask for Name and Greet

```nextc
import io;

void main() {
    io::print!("Please enter your name: ");
    string name = "";
    io::read_line(&name);  // returns bool indicating success (no error).
    io::println!("Hello, {}!", name);
    delete name;  // remember to manage memory manually.
}
```

## Score Grading

```nextc
import io;
import strings;

void main() {
    io::print!("Please enter your score: ");
    uint score = 0;

    while (!io::read_uint(&score)) {
        io::print!("Invalid score! Try again: ");
    }

    if (score > 100) {
        io::println!("You can't get that score!");
        return;
    }

    char level;
    if (score == 100) {
        level = 'S';
    } else if (score >= 90) {
        level = 'A';
    } else if (score >= 80) {
        level = 'B';
    } else if (score >= 60) {
        level = 'C';
    } else {
        level = 'F';
    }
    // All paths initialise the variable level, so reading level later is safe.

    io::println!("Your level is: {}!", level);
}
```

## Integer Bubble Sort

```ebnf
import io;
import process;
import slices;
import strings;
import util;

void main() {
    io::print!("Enter the length of the array: ");
    int length = read_int();
    slice[int] arr = slices::new_slice!(int, length);
    io::println!("Enter the array below:");
    // foreach obtains a pointer to each element of the array or slice.
    foreach (int* ele : arr) {
        *ele = read_int();
    }
    io::println!("Sorting...");
    bubble_sort(arr);
    io::println!("Finished!");
    io::println!("The sorted array is:");
    // const int* prevents accidental modification of slice elements.
    foreach (const int* ele : arr) {
        io::print!("{} ", *ele);
    }
    io::println!();
}

// NextC uses a multi‑pass compiler, so dependency order does not matter.
int read_int() {
    int result = 0;
    if (!io::read_int(&result)) {
        io::err_println("Invalid integer.");
        process::exit(1);
    }
    return result;
}

// slice[int] is a fat pointer.
// `const` here means `arr` itself is immutable, but the slice elements are readable and writable.
// A slice[const int] would have immutable elements.
void bubble_sort(const slice[int] arr) {
    usize length = slices::len!(arr);
    for (usize i = 0; i < length; i++) {
        for (usize j = i + 1; j < length; j++) {
            if (arr[j - 1] > arr[j]) {
                util::swap!(&arr[j - 1], &arr[j]);
            }
        }
    }
}
```
