# Rust Syntax Basics

[cn] Rust语法基础，参考视频教程 [链接](https://www.bilibili.com/video/BV1hp4y1k7SV/)

[en] Rust Syntax Basics, Reference Video Tutorial [link](https://www.bilibili.com/video/BV1hp4y1k7SV/)

## 01 Base

### Hello World

[cn] 了解 Rust 的第一个程序

[en] Learn about Rust's first program

### Data Type

[cn] 这个章节包括基础的变量和数据类型，如怎么定义一个变量，以及基础数据类型 i8、i16、i32、i64、i128、isize 和 u8、u16、u32、u64、u128、usize, bool, float32、float64, char, tuple, array, statement 等的学习练习。

[en] This chapter includes basic variable and data types, such as how to define a variable, and fundamental data types i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool, float32, float64, char, tuple, array, statement, etc.

### Loop Structure

[cn] 这个章节介绍了 Rust 中各种循环的使用，包括 loop、while、for 等，其中最为重要且最常用的是 for 循环，因为它使用简单，且程序比较简洁。

[en] This section introduces the use of various loops in Rust, including `loop`, `while`, and `for`. Among them, the `for` loop is the most important and commonly used because it is simple to use and makes the program more concise.

### Function

[cn] 这个章节介绍了函数的定义、函数的调用、函数参数和函数返回值。在 Rust 中，函数的位置不一定需要在 main 函数或者调用函数之前，只要在文件能够被访问到的地方即可。

[en] This section covers function definitions, function calls, function parameters, and return values. In Rust, the position of a function does not necessarily need to be before the `main` function or the calling function; it just needs to be accessible within the file.

## 02 Ownership

[cn] Rust 的所有权，包括在各个使用场景中生命周期和所有权的变化，移动、销毁等等。

[en] The ownership of Rust, including the changes in lifecycle and ownership across various usage scenarios, such as moving, destroying, etc.

### String

[cn] String 类型的使用介绍，以及它的生命周期。

[en] An introduction to the use of the `String` type and its lifecycle.

### Scope

[cn] 这里介绍了生命周期和作用域。

[en] This section introduces lifetimes and scopes.

### Move Feature

[cn] 这里介绍了所有权移动的问题，以及深拷贝和浅拷贝。

[en] This section covers the issue of ownership transfer, as well as deep copy and shallow copy.

### Function Parameters

[cn] 这个章节重点介绍了函数参数、函数返回值的各种使用场景对所有权变化的问题，字符串的引用和借用，字符串的字面值，以及字符串的切片，其他类型的切片。

[en] This section focuses on the various use cases of function parameters and return values in relation to ownership changes, string references and borrowing, string literals, string slices, and slices of other types.

## 03 struct
[cn] 本章节主要介绍结构体, 包括结构体定义，初始化赋值，实例更新，以及结构体方法，和tuple struct，这种结构成员变量没有名字，只有类型。
[en] This section primarily introduces structures, including structure definitions, initialization and assignment, instance updates, structure methods, and tuple structs, which are structures whose members have no names but only types.
