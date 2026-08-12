# 示例程序

> [!Warning]
>
> 目前 NextC 处于初期设计和开发阶段，随时可能会加入、删除或修改一些语法规则和语义规则。直到 NextC 1.0 发布之前，此文档都有很大可能进行改动。
>
> 下面所有程序为设计构想，暂不能编译运行。NextC 1.0 发布之前，下面的程序随时可能进行改动。

## Hello, World!

```nextc
import io;

void main() {
    io::println!("Hello, World!");
}
```

## 询问名字并问好

```nextc
import io;

void main() {
    io::print!("Please enter your name: ");
    string name = "";
    io::read_line(&name);  // 事实上返回 bool 表示是否正常返回（是否没有错误）。
    io::println!("Hello, {}!", name);
    delete name;  // 记得手动管理内存。
}
```

## 分数等级评定

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
    // 所有路径均初始化了变量 level，则后面可以正常读取 level。

    io::println!("Your level is: {}!", level);
}
```

## 整数冒泡排序

```nextc
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
    // foreach 语句获取数组或切片的每个元素的指针。
    foreach (int* ele : arr) {
        *ele = read_int();
    }
    io::println!("Sorting...");
    bubble_sort(arr);
    io::println!("Finished!");
    io::println!("The sorted array is:");
    // const int* 可保证不意外修改切片的元素
    foreach (const int* ele : arr) {
        io::print!("{} ", *ele);
    }
    io::println!();
}

// NextC 使用多遍编译器，不需要考虑依赖顺序的问题。
int read_int() {
    int result = 0;
    if (!io::read_int(&result)) {
        io::err_println("Invalid integer.");
        process::exit(1);
    }
    return result;
}

// slice[int] 是胖指针。
// 此处的 const 表示 arr 是不可变变量，但切片内部元素可读可写。
// slice[const int] 的元素就是不可变的。
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
