# Raybit!

The official (*non-official*) **Brainfuck** bindings for **Raylib**!

# What is Raybit? or... BitBit?

**BitBit** is a modified version of the esoteric programming language, **Brainfuck**. More specifically, it is a super set of **Brainfuck**, meaning, it can run all normal **Brainfuck** code but also includes some extra functionally. It has all the bells and whistles of normal Brainfuck, plus 4 very special extra compound commands; `+-`, `-+`, `<>`, `><`. These commands allow **BitBit** to call functions from external libraries with a predefined function map. **BitBit** uses the native word size that the interpreter was built in, meaning a typical 64-bit target allows **BitBit** to access a maximum of `2^66 - 4` possible external functions!

**Raybit** is simply the bindings for **Raylib** in **BitBit**.

This interpreter also includes 8 additional commands for debugging purposes: `?`, `#`, `$`, `!`, `{`, `}`, `;`, `|`.

### Some Specifics
* The memory strip is grown dynamically, thus you must "*explore*" a cell before it can be accessed.
* This interpreter implements looping memory values: Decrementing from `0` to `255`, or incrementing from `255` to `0`.
* This interpreter includes single-line comments with `;`.
* This interpreter includes multi-line comments with `{}`.
* <code>&nbsp;</code> (*space*), `|`, and `\n` characters can be used to explicitly separate commands for the interpreter.
* `Panic!` occurs when trying to access invalid memory or an invalid function.
* The default word size in **BitBit** is `64` bits. However, the interpreter can be built with `16` or `32` bit word sizes.

## Why is Raybit?
umm... idk.

In all seriousness, this is an experimental hobby project to test interpretation of base-256 numerical computation.

This project focuses on modeling the basic principles of computer architecture and organization within the confines of **Brainfuck**.

## Commands
| Command | Functionality                                                               |
| :-----: | :-------------------------------------------------------------------------- |
|   `>`   | Increment the memory pointer right 1 cell.                                  |
|   `<`   | Decrement the memory pointer left 1 cell.                                   |
|   `+`   | Increment the value stored at the current cell.                             |
|   `-`   | Decrement the value stored at the current cell.                             |
|   `[`   | If the current cell value is zero, then jump forward to the matching `]`.   |
|   `]`   | If the current cell value is nonzero, then jump back to the matching `[`.   |
|   `,`   | Accept one character of input, storing its ASCII value in the current cell. |
|   `.`   | Output the character corresponding to the value at the current cell.        |
|  `+-`   | Flip up. Calls a function with the value at current cell as the ID.         |
|  `-+`   | Flip down. Calls a function with the value at current cell as the ID.       |
|  `<>`   | Flip left. Calls a function with the value at current cell as the ID.       |
|  `><`   | Flip right. Calls a function with the value at current cell as the ID.      |
|   `?`   | Prints the pointer and value at current cell to the console.                |
|   `#`   | Prints the entire current memory layout horizontally.                       |
|   `$`   | Prints the entire current memory layout vertically.                         |
|   `!`   | Immediately stops the program.                                              |
|   `;`   | Single-line comment.                                                        |
|   `{`   | Begin multi-line comment.                                                   |
|   `}`   | End multi-line comment.                                                     |
|  `\|`   | Spacer. Used to space commands for the parser.                              |

## BitBit Data Types
In strip memory, all data types must somehow be represented using only an array of unsigned 8-bit integers. So to fix this, **BitBit** uses separate models for different types of data.

### `Boolean`:
`bool` has a bit-size of `8`. Thus, you only need one cell to store it. Normally its value is either `1` or `0` to represent `true` or `false` respectively, but any number greater than `0` is also read as `true`.

Value `true` at pointer position `2`.\
Value `false` at pointer position `3`.\
Value `true` at pointer position `4`.
```brainfuck
Memory Cells
-------------
[0][0][1][0][9][0][0]
       ^
Memory Pointer
```
* *`bool` values can range from `0` to `255`, meaning that future implementations may use this quirk to create additional functionality*

### `String`:
For most values, the byte-size of a particular object is predefined based on its type. A `bool` will always have a byte-size of `1`. However, `String` has a dynamic byte-size. Meaning, it can be stored in any amount of cells depending on the length of the `String`. Thus, the ASCII value of each character in the `String` is stored in separate consecutive cells, left-to-right order, with a null terminator at the end to model C strings.

Value `"ABC"` at pointer position `2`.
```brainfuck
Memory Cells
-------------
[0][0][65][66][67][0][68]
       ^
Memory Pointer
```

### `Integer`:
`Integer` is actually a class of several types defining an `Integer` of varying byte-sizes. **BitBit** uses `base-256` to represent an `Integer`. `Signed` types use signed 2's complement. `Integer` types are stored in little-endian order, left-to-right.

#### `Unsigned Integer`:

Max Value = `(256^byte_width) - 1`

| Type  | Bit Width | Byte Width | Max Value  |
| :---- | :-------- | :--------- | :--------- |
| `u8`  | 8         | 1          | 255        |
| `u16` | 16        | 2          | 65535      |
| `u24` | 24        | 3          | 16777215   |
| `u32` | 32        | 4          | 4294967295 |

The value, `v` of an `Unsigned Integer` to a decimal base is calculated by the sum of each digit in the `Integer` multiplied with its corresponding `256^n`.

$$
\begin{aligned}
w &= \text{width of unsigned integer} \\
\vec{U} &= \left[x_0\right]\left[x_1\right]\left[x_2\right]\dots\left[x_{w-1}\right] \\
v &=\sum_{i=0}^{w - 1}\left(\vec{U}_i \cdot 256^i \right)
\end{aligned}
$$

| `base-10`  | `256^0` | `256^1` | `256^2` | `256^3` | `256^4` |
| :--------- | :------ | :------ | :------ | :------ | :------ |
| 0          | 0       | 0       | 0       | 0       | 0       |
| 1          | 1       | 0       | 0       | 0       | 0       |
| 255        | 255     | 0       | 0       | 0       | 0       |
| 256        | 0       | 1       | 0       | 0       | 0       |
| 65545      | 9       | 0       | 1       | 0       | 0       |
| 234897     | 145     | 149     | 3       | 0       | 0       |
| 4295098368 | 0       | 0       | 2       | 0       | 1       |

A `u16` value of `65545` at pointer position `2`.
```brainfuck
Memory Cells
-------------
[0][0][9][0][1][0][0]
       ^
Memory Pointer
```
#### `Signed Integer`:

Max Value = `(256^byte_width / 2) - 1`

| Type  | Bit Width | Byte Width | Max Value  |
| :---- | :-------- | :--------- | :--------- |
| `i8`  | 8         | 1          | 127        |
| `i16` | 16        | 2          | 32767      |
| `i24` | 24        | 3          | 8388607    |
| `i32` | 32        | 4          | 2147483647 |

The value of a negative `Signed Integer` to a decimal base is calculated by adding it to the corresponding `Unsigned` maximum value.

$$
\begin{aligned}
w &= \text{width of signed integer} \\
\vec{T} &= \left[x_0\right]\left[x_1\right]\left[x_2\right]\dots\left[x_{w-1}\right] \\
v &= \sum_{i=0}^{w-2}{\left(\vec{T}_i \cdot 256^i\right)} - \left( \vec{T}_{w-1} \cdot 256^{w-1} \right)
\end{aligned}
$$

| `base-10`   | `256^0` | `256^1` | `256^2` | `256^3` | `256^4` |
| :---------- | :------ | :------ | :------ | :------ | :------ |
| 0           | 0       | 0       | 0       | 0       | 0       |
| 1           | 1       | 0       | 0       | 0       | 0       |
| -1          | 255     | 255     | 255     | 255     | 255     |
| -256        | 0       | 255     | 255     | 255     | 255     |
| -65545      | 247     | 255     | 254     | 255     | 255     |
| -234897     | 111     | 106     | 252     | 255     | 255     |
| -4295098368 | 0       | 0       | 254     | 255     | 254     |

A `i16` value of `-65545` at pointer position `1`.
```brainfuck
Memory Cells
-------------
[0][247][255][254][255][255][0]
     ^
Memory Pointer
```
### `Float`:
`Float` is actually a class of several types defining a `Float` of varying byte-sizes and precisions. Floating-point representation encodes rational numbers of the form `v = x * 2^y`. `Float` is useful for performing computations involving very large numbers, `|v| >> 0`, numbers very close to zero, `0 < |v| << 1`, and more generally as an approximation to real arithmetic. `Float` models the `IEEE 754` standard.

##### Numerical Form: `(-1)^s * M * 2^E`  
* Sign byte `s` determines whether number is negative or positive
* Mantissa `M` a fractional value `[1.0, 2.0)`
* Exponent `E` weights value by power of `2`

##### Encoding:  
* `s` is sign byte `s`
* `exp` field encodes `E` where `exp != E`
* `frac` field encodes `M` where `frac != M` 

Total Bit Size: 24
```
#### `f32`
```brainfuck
Bit Size: s = 1, exp = 8, frac = 23, bias = 127
-------------
[s][exp][frac          ]
[    ][    ][    ][    ]
-------------
Total Bit Size: 32
```

#### `f64`
```brainfuck
Bit Size: s = 1, exp = 11, frac = 52, bias = 1023
-------------
[s][exp       ][frac                           ]
[    ][    ][    ][    ][    ][    ][    ][    ]
-------------
Total Bit Size: 64
```

### Calculation of `Float`:

`v256` = value of floating-point number in `base-256`

`v10` = value of floating-point number in `base-10`

#### `Float32`:

Since `Float` values in **BitBit** must be represented as a byte array, the base 10 values are calculated directly using the raw bits.

**Brainfuck** representation

```brainfuck
Bit Size: s = 8, exp = 8, frac = 16, bias = 127
-------------
[64      ][176     ][0       ][0       ]
[01000000][10110000][00000000][00000000]
Bits = 01000000101100000000000000000000
```

Once the raw bits are extracted, the `sign`, `exponent`, and `fraction` fields can be parsed.

```rust
s: 0
exp: 10000001
frac: 01100000000000000000000
```
Once the raw bits are parsed, the Sign, Exponent, and Significand can be calculated.
#### Normalized:

##### Sign: `S`

```math
S = (-1)^s
```

##### Exponent: `E`

```math
k    = |exp|         = 8
Exp  = unsigned(exp) = 129
Bias = 2^(k-1) - 1   = 127
E    = Exp - Bias    = 2
```

##### Significand: `M`

```math
M = 1 + sum_{i=0}^{|frac| - 1}{frac_i / 2^{i + 1}}
M = 1.375
```

![alt text](mantissa_equation.png "Mantissa Equation")

Once the Sign, Exponent, and Significand are calculated, the base-10 value of the `Float` can be calculated.

```math
v256 = [64][176][0][0]

v10  = S * M * 2^E
     = 1 * 1.375 * 2^2
     = +5.5
```

### `Structs`:
A `Struct` is simply defined as a collection of fields. It consists of an array of bytes containing its fields data in left-to-right order.

### Pointers
A `Pointer` in **BitBit** points to the index of the first cell where the data is held. The default word-size for **BitBit** is `64` bits. So **BitBit** pointers in a 64-bit target actually require `8` cells to hold the value of the pointer that points to the first cell where specific data is stored at.

* This interpreter can be compiled to execute with different word-sizes.

## Raylib Function Map
All **Raylib** functions are mapped in alphabetical order. Because one flip call can access `2^64 - 1` unique functions in a typical 64-bit target and there only `552` defined **Raylib** functions, all **Raylib** functions are mapped to a single flip call: `+-`.

* The only **Raylib** functions included are those listed in the [Raylib Cheat Sheet Website](https://www.raylib.com/cheatsheet/cheatsheet.html).

So calling flip up, `+-`, with the current cell value at `3` will call the `BeginDrawing()` function.
```brainfuck
+++ // cell #0
+-  // flip up
```
```
Memory Cells
-------------
[3][0][0][0][0][0][0][0]
 ^
Memory Pointer
```

***Flip commands are awesome!***

## Function Output
In **BitBit**, functions that return values, will return those values at a specified index in the memory. For example, the `WindowShouldClose` function returns a `bool`. So in **Raybit**, calling the  `WindowShouldClose` function stores the corresponding `Boolean` value, (`1` or `0`), into memory at the pointer value direct to the right of the function ID cell. For all other data types, such as `String` or `Color`, their values are output in little-endian order, left-to-right.

First we will store `551` in base-256 for the function ID, since that value corresponds to `WindowShouldClose`.
```brainfuck
>++++++[-<++++++>]<+++> ++ >>>>>>>
```
```brainfuck
Memory Cells
-------------
[39][2][0][0][0][0][0][0][0]
                          ^
Memory Pointer
```

Then we need to specify the output index. Output pointer is specified at index `17`.
```brainfuck
>++++[-<++++>]<+> >>>>>>>
```
```brainfuck
Memory Cells
-------------
[39][2][0][0][0][0][0][0][17][0][0][0][0][0][0][0][0] 
                                                   ^
Memory Pointer
```

If `WindowShouldClose` is triggered and returns `true`, then a `1` will be stored at the specified output index. Our memory layout will look like this:
```brainfuck
Memory Cells
-------------
[39][2][0][0][0][0][0][0][17][0][0][0][0][0][0][0][1]
                                                   ^
Memory Pointer
```

## Function Input
Functions that require argument values, will use the values in cells to the right of the current cell as `Pointers`. For example, `InitWindow` requires 3 input values for `width`, `height`, and `title`. Therefore on a 64-bit target, calling the `InitWindow` function uses the 24 cell values to the right of the last function ID cell in left-to-right argument order as `Pointers`.

In practice the memory layout of calling `InitWindow` will look like:

`InitWindow(width: i32, height: i32, title: String)`
```brainfuck
cell #0  : ID      : [061][1][0][0][0][0][0][0] : 317
cell #8  : *width  : [032][0][0][0][0][0][0][0] : 32 
cell #16 : *height : [040][0][0][0][0][0][0][0] : 40
cell #24 : *title  : [048][0][0][0][0][0][0][0] : 48
cell #32 : width   : [086][3][0][0][0][0][0][0] : 854
cell #40 : height  : [224][1][0][0][0][0][0][0] : 480
cell #48 : title   : [065][0][0][0][0][0][0][0] : "A"
```
Once `+-` is called, it will initialize an `854x480` window with the title `A`. 

## Example

Here is a simple **Raybit** example for creating a basic window!
```brainfuck
>++++++[-<++++++++++>]<+>+>>>>>>>>++++++++[-<------------->]>>>>>>>>++++++++++[-<---------->]>>>>>
>>>++++++++[-<------------>]>>>>>>>>++++[-<----------->]<-->+>>>>>>>>++++++[-<---------->]<+>>>>>>
>>>++++++[-<++++++>]<+++>++>>>>>>>>+++++++[-<-------->]<+>>>>>>>>+++>>>>>>>>>+++++++++[-<---------
---->]>>>>>>>>+++++[-<+++++>]<+>>>>>>>>>+++++[-<+++++>]<-->>>>>>>>>+++++[-<----------->]<+>>>>>>>>
>++++++++++[-<++++++++++++>]<-->>>>>>>>>+++[-<----------------->]<+>>>>>>>>--------->>>>>>>>----->
>>>>>>>->>>>>>>>+++>+>>>>>>>>++++++++[-<++++++++++>]<++++++>+++>>>>++++[-<-------->]+>>>>++++++[<+
++++++++++++++++++>-]>++++++++[<++++++++++++>-]<+>>+++++++++++[<+++++++++++>-]>+++++++++[<++++++++
++++>-]>+++++++[<+++++++++++++++>-]>+++++++[<++++++++++++++>-]>++++[<++++++++>-]>+++++++[<++++++++
+++++>-]>+++++++++[<+++++++++++>-]>++++++++++[<+++++++++++>-]<+>>++++++[<+++++++++++++++++++>-]>++
++++++++[<++++++++++>-]<+>>+++++++[<+++++++++++++>-]<++>>++++[<++++++++>-]>++++++++++[<++++++++++>
-]<+>>++++++++++[<++++++++++++>-]>++++++++[<++++++++++++>-]<+>>++++++++++[<+++++++++++>-]<->>+++++
+++[<++++++++++++++>-]>+++++++++[<++++++++++++>-]>++++++++++[<++++++++++>-]<+>>++++[<++++++++>-]>+
++++[<+++++++++>-]>++++[<++++++++>-]>+++++++[<++++++++++++++>-]>++++++++[<++++++++++++>-]<+>>+++++
++++[<+++++++++++++>-]<-->>+++++++[<+++++++++++++++>-]>+++++++++[<+++++++++++>-]>++++[<++++++++>-]
>+++++++[<+++++++++++++++++>-]>+++++++[<+++++++++++++++>-]>++++++++++[<+++++++++++>-]>++++++++++[<
++++++++++>-]>++++++++++[<+++++++++++>-]<+>>+++++++[<+++++++++++++++++>-]>>++++++[-<++++++++++>]>>
>>----------->----------->----------->->>++++++[<+++++++++++>-]<+>>++++++++++[<+++++++++++>-]<+>>+
+++++++++[<+++++++++++>-]>++++++++[<+++++++++++++>-]<->>++++++[<+++++++++++++++++++>-]>++++++++[<+
+++++++++++>-]<+>>+++++++++[<+++++++++++++>-]<->>+++++++++[<+++++++++++++>-]<-->>+++[<+++++++++++>
-]>++++[<++++++++>-]>+++++++++[<++++++++++>-]<->>++++++++++[<+++++++++++>-]<+>>+++++++++[<++++++++
+++++>-]>++++[<++++++++>-]>+++++++++[<+++++++++++>-]>++++++[<+++++++++++++++++++>-]>++++++++++[<++
++++++++>-]<+>>++++++++[<++++++++++++>-]<+>>+++++++++[<+++++++++++++>-]<->>++++++++++[<++++++++++>
-]<+>>++++++++++[<++++++++++>-]>++++[<++++++++>-]>+++++++++++[<+++++++++++>-]>++++++++++[<++++++++
+++>-]<+>>+++++++++[<+++++++++++++>-]>++++++[<+++++++++++++++++++>-]>++++[<++++++++>-]>++++++[<+++
++++++++++++++>-]>+++++++[<+++++++++++++++>-]>++++++[<+++++++++++++++++++>-]>+++++++++[<++++++++++
+++>-]<-->>+++++++++[<+++++++++++++>-]<->>++++[<++++++++>-]>+++++++[<+++++++++++++++++>-]>+++++++[
<+++++++++++++++>-]>++++++++++[<+++++++++++>-]>++++++++++[<++++++++++>-]>++++++++++[<+++++++++++>-
]<+>>+++++++[<+++++++++++++++++>-]>+++[<+++++++++++>-]>>++++++[-<----------->]>>>>+++++++[-<------
-->]>>>>++++[-<+++++>]>>>>+++++++[-<-------->]>+++++++[-<-------->]>+++++++[-<-------->]-> <<<<<<<
<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< +- >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>[ +- >>>>>>>>>>>>>>>>>>>>>>>> +- >>>>>>>>>>>>>>>> +- <<<<<<<<<<<<<<<
<<<<<<<<<<<<<<<<< +- <<<<<<<<<<<<<<<<<<<<<<<< +- >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
>>>>>>[<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< +- [>>>>>>>>]>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>]<<<<<<<<<<<<<<
<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
<<<<<<<<<<<<<<<<<<<<<<<<<]
```

Pretty self-explanatory right?

Just kidding! Here is a break down of the code with comments :)

I'll warn you though, it's quite *vertical*! 
```brainfuck
[
    raylib [core] example - Basic window

    Welcome to raybit!

    To test examples with cargo installed, just run `cargo run -- "program.bf"`
    To text examples with the compiled binary, just run `raybit.exe "program.bf"`
    Note that `-raw` or `-r` flag will run the program as normal brainfuck code

    Enjoy using raylib with brainfuck. :)

    Example originally created with raylib 5.0, last time updated with raylib 5.0
]
[
    ARCH:
        Width: 64-bit
        Endianess: little-endian
        *Pointer { usize }
        fn { usize }
    MEMORY LAYOUT: [ call ][ data ][ heap ]
    CALL:
        0   : fn InitWindow         : usize { 317 }
        8   : *width                : usize { 152 }
        16  : *height               : usize { 156 }
        24  : *title                : usize { 160 }
        32  : fn SetTargetFPS       : usize { 466 }
        40  : *fps                  : usize { 197 }
        48  : fn WindowShouldClose  : usize { 551 }
        56  : *shouldCloseWindow    : usize { 201 }
        64  : fn BeginDrawing       : usize { 003 }
        72  : fn EndDrawing         : usize { 139 }
        80  : fn CloseWindow        : usize { 026 }
        88  : fn ClearBackground    : usize { 023 }
        96  : *backgroundColor      : usize { 202 }
        104 : fn DrawText           : usize { 118 }
        112 : *text                 : usize { 206 }
        120 : *posX                 : usize { 247 }
        128 : *posY                 : usize { 251 }
        136 : *fontSize             : usize { 255 }
        144 : *textColor            : usize { 259 }
    DATA:
        152 : screenWidth           : i32 { 854 }
        156 : screenHeight          : i32 { 480 }
        160 : title                 : c_str { "raylib [core] example - basic window\0" }
        197 : fps                   : i32 { 60 }
        201 : shouldCloseWindow     : bool { ? }
        202 : backgroundColor       : Color { r: 245, g: 245, b: 245, a: 255 }
        206 : text                  : str { "Congrats! You created your first window!\0" }
        247 : posX                  : i32 { 190 }
        251 : posY                  : i32 { 200 }
        255 : fontSize              : i32 { 20 }
        259 : textColor             : Color { r: 200, g: 200, b: 200, a: 255 }
]

;==================================================================================
; Program main entry point (Function & Argument Pointers Initialization)
;==================================================================================

; fn InitWindow        : 317 usize { 061 1 0 0 0 0 0 0 }
>++++++[-<++++++++++>]<+>
+ >>>>>>>
; *width               : 152 usize { 152 0 0 0 0 0 0 0 }
>++++++++[-<------------->]
>>>>>>>

; *height              : 156 usize { 156 0 0 0 0 0 0 0 }
>++++++++++[-<---------->]
>>>>>>>

; *title               : 160 usize { 160 0 0 0 0 0 0 0 }
>++++++++[-<------------>]
>>>>>>>

; fn SetTargetFPS      : 466 usize { 210 1 0 0 0 0 0 0 }
>++++[-<----------->]<-->
+ >>>>>>>

; *fps                 : 197 usize { 197 0 0 0 0 0 0 0 }
>++++++[-<---------->]<+>
>>>>>>>
; fn WindowShouldClose : 551 usize { 039 2 0 0 0 0 0 0 }
>++++++[-<++++++>]<+++>
++ >>>>>>>

; *shouldCloseWindow   : 201 usize { 201 0 0 0 0 0 0 0 }
>+++++++[-<-------->]<+>
>>>>>>>

; fn BeginDrawing      : 003 usize { 003 0 0 0 0 0 0 0 }
+++>
>>>>>>>

; fn EndDrawing        : 139 usize { 139 0 0 0 0 0 0 0 }
>+++++++++[-<------------->]
>>>>>>>

; fn CloseWindow       : 026 usize { 026 0 0 0 0 0 0 0 }
>+++++[-<+++++>]<+>
>>>>>>>

; fn ClearBackground   : 023 usize { 023 0 0 0 0 0 0 0 }
>+++++[-<+++++>]<-->
>>>>>>>

; *backgroundColor     : 202 usize { 202 0 0 0 0 0 0 0 }
>+++++[-<----------->]<+>
>>>>>>>

; fn DrawText          : 118 usize { 118 0 0 0 0 0 0 0 }
>++++++++++[-<++++++++++++>]<-->
>>>>>>>

; *text                : 206 usize { 206 0 0 0 0 0 0 0 }
>+++[-<----------------->]<+>
>>>>>>>

; *posX                : 247 usize { 247 0 0 0 0 0 0 0 }
--------->
>>>>>>>

; *posY                : 251 usize { 251 0 0 0 0 0 0 0 }
----->
>>>>>>>

; *fontSize            : 255 usize { 255 0 0 0 0 0 0 0 }
->
>>>>>>>

; *textColor           : 259 usize { 003 1 0 0 0 0 0 0 }
+++>
+ >>>>>>>

; Data Initialization
;==================================================================================

; screenWidth          : 854 i32 { 086 3 0 0 }
>++++++++[-<++++++++++>]<++++++>
+++ >>>

; screenHeight         : 480 i32 { 224 1 0 0 }
>++++[-<-------->]
+ >>>

; title                : c_str { "raylib [core] example - basic window\0" }
>++++++[<+++++++++++++++++++>-]
>++++++++[<++++++++++++>-]<+>
>+++++++++++[<+++++++++++>-]
>+++++++++[<++++++++++++>-]
>+++++++[<+++++++++++++++>-]
>+++++++[<++++++++++++++>-]
>++++[<++++++++>-]
>+++++++[<+++++++++++++>-]
>+++++++++[<+++++++++++>-]
>++++++++++[<+++++++++++>-]<+>
>++++++[<+++++++++++++++++++>-]
>++++++++++[<++++++++++>-]<+>
>+++++++[<+++++++++++++>-]<++>
>++++[<++++++++>-]
>++++++++++[<++++++++++>-]<+>
>++++++++++[<++++++++++++>-]
>++++++++[<++++++++++++>-]<+>
>++++++++++[<+++++++++++>-]<->
>++++++++[<++++++++++++++>-]
>+++++++++[<++++++++++++>-]
>++++++++++[<++++++++++>-]<+>
>++++[<++++++++>-]
>+++++[<+++++++++>-]
>++++[<++++++++>-]
>+++++++[<++++++++++++++>-]
>++++++++[<++++++++++++>-]<+>
>+++++++++[<+++++++++++++>-]<-->
>+++++++[<+++++++++++++++>-]
>+++++++++[<+++++++++++>-]
>++++[<++++++++>-]
>+++++++[<+++++++++++++++++>-]
>+++++++[<+++++++++++++++>-]
>++++++++++[<+++++++++++>-]
>++++++++++[<++++++++++>-]
>++++++++++[<+++++++++++>-]<+>
>+++++++[<+++++++++++++++++>-]
> null terminated string

; fps                  : 060 i32 { 060 0 0 0 }
>++++++[-<++++++++++>]
>>>

; shouldCloseWindow    : bool { 0 }
>

; backgroundColor      : Color { 245 245 245 255 }
----------->
----------->
----------->
->

; text                 : c_str { "Congrats! You created your first window!\0" }
>++++++[<+++++++++++>-]<+>
>++++++++++[<+++++++++++>-]<+>
>++++++++++[<+++++++++++>-]
>++++++++[<+++++++++++++>-]<->
>++++++[<+++++++++++++++++++>-]
>++++++++[<++++++++++++>-]<+>
>+++++++++[<+++++++++++++>-]<->
>+++++++++[<+++++++++++++>-]<-->
>+++[<+++++++++++>-]
>++++[<++++++++>-]
>+++++++++[<++++++++++>-]<->
>++++++++++[<+++++++++++>-]<+>
>+++++++++[<+++++++++++++>-]
>++++[<++++++++>-]
>+++++++++[<+++++++++++>-]
>++++++[<+++++++++++++++++++>-]
>++++++++++[<++++++++++>-]<+>
>++++++++[<++++++++++++>-]<+>
>+++++++++[<+++++++++++++>-]<->
>++++++++++[<++++++++++>-]<+>
>++++++++++[<++++++++++>-]
>++++[<++++++++>-]
>+++++++++++[<+++++++++++>-]
>++++++++++[<+++++++++++>-]<+>
>+++++++++[<+++++++++++++>-]
>++++++[<+++++++++++++++++++>-]
>++++[<++++++++>-]
>++++++[<+++++++++++++++++>-]
>+++++++[<+++++++++++++++>-]
>++++++[<+++++++++++++++++++>-]
>+++++++++[<+++++++++++++>-]<-->
>+++++++++[<+++++++++++++>-]<->
>++++[<++++++++>-]
>+++++++[<+++++++++++++++++>-]
>+++++++[<+++++++++++++++>-]
>++++++++++[<+++++++++++>-]
>++++++++++[<++++++++++>-]
>++++++++++[<+++++++++++>-]<+>
>+++++++[<+++++++++++++++++>-]
>+++[<+++++++++++>-]
> null terminated string

; posX                 : 190 i32 { 190 0 0 0 }
>++++++[-<----------->]
>>>

; posY                 : 200 i32 { 200 0 0 0 }
>+++++++[-<-------->]
>>>

; fontSize             : 020 i32 { 020 0 0 0 }
>++++[-<+++++>]
>>>

; textColor            : Color { 200 200 200 255 } 
>+++++++[-<-------->]
>+++++++[-<-------->]
>+++++++[-<-------->]
->

; Call InitWindow()
<<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<<
<<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<<
<<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<<
<<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<<
<<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<<
<<<<<<<<<< <<< +-

; Goto fn BeginDrawing
>>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>>
>>>>>>>>>> >>>>

; Main game loop
[
    ; Update
    ;==================================================================================

    ; TODO: Update your variables here
    ;==================================================================================

    ; Draw
    ;==================================================================================

    +-                                      ; Call BeginDrawing()
    >>>>>>>>>> >>>>>>>>>> >>>> +-           ; Call ClearBackground()
    >>>>>>>>>> >>>>>> +-                    ; Call DrawText()
    <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< << +-  ; Call EndDrawing() 

    ; Detect window close button or ESC key
    ;==================================================================================
    
    <<<<<<<<<< <<<<<<<<<< <<<< +-           ; Call WindowShouldClose

    ; Goto shouldCloseWindow   (0 / 1)
    >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>>
    >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>>
    >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>>
    >>>

    ; if (shouldCloseWindow != 0)
    [
        ; De-Initialization
        ;==============================================================================

        ; Call CloseWindow()
        <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<<
        <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<<
        <<<<<<<<<< <<<<<<<<<< < +-
        
        ; Goto garunteed null quadword (GNQ)
        [>>>>>>>>]

        ; Goto GNQ + fn BeginDrawing (137)
        >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>>
        >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>>>>
        >>>>>>>>>> >>>>>>>>>> >>>>>>>>>> >>>>>>>
    ]

    ; Goto BeginDrawing()
    <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<<
    <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<<<<
    <<<<<<<<<< <<<<<<<<<< <<<<<<<<<< <<<<<<<
]
```

#### Why the fuck is your README so long?

¯\\_\_(ツ)__/¯