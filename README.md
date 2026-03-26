# matlab
A Rust CLI tool intending to incrementally clone functionality of matlab.

## Documentation

### Types
 - Number: ASCII numbers (eg `1`, `1.0`, `.0`)
 - Matrix: Denoted with `[]` with values separated with commas (`,`) or spaces (` `) and rows separated by semicolons (`;`) (eg `[ 1, 2, 3 ; 4, 5, 6 ]`)
 - Variable: Must be consecutive alphabetic ASCII characters.

### Operations
 - Addition (`+`)
```
>> 1 + 1
2
>> 1 + [ 1 2 3 ]
[
  2, 3, 4
]
>> [ 1 2 3 ] + 1
[
  2, 3, 4
]
>> [ 1 2 3 ] +  [ 1 1 1 ]
[
  2 3 4
]
```
 - Subtraction (`-`)
```
>> 2 - 1
1
>> [ 3 2 1 ] - 1
[
  2, 1, 0
]
```
 - Multiplication (`*`)
```
>> 2 * 3
6
2 * [ 1 2 3 ]
[
  2, 4, 6
]
>> [ 1 2 3 ] * 2
[
  2, 4, 6
]
>> [ 1 2 ; 3 4 ; 5 6 ] * [ 6 5 4 ; 3 2 1 ]
[
  12 9 6
  30 23 16
  48 37 26
]
```
 - Division (`/`)
 ```
>> 6 / 2
3
>> [ 2 4 6 ] / 2
[
  1 2 3
]
 ```
 - Equality (`==`)
```
>> 2 == 1
0
>> 2 == 2
1
```
 - Inequality (`!=`)
```
>> 2 != 1
1
>> 2 != 2
0
```
 - Less than (`<`)
```
>> 2 < 1
0
>> 2 < 2
0
>> 2 < 3
1
```
 - Less than or equal to (`<=`)
```
>> 2 <= 1
0
>> 2 <= 2
1
>> 2 <= 3
1
```
 - Greater than (`>`)
```
>> 2 > 1
1
>> 2 > 2
0 
>> 2 > 3
0
```
 - Greater than or equal to (`>=`)
```
>> 2 >= 1
1
>> 2 >= 2
1 
>> 2 >= 3
0
```
 - Assign (`=`)
```
>> A = 1
1
>> A
A = 1
>> A = [ 1 2 3 ; 4 5 6 ]
[
  1 2 3
  4 5 6
]
>> A
A = [
  1 2 3
  4 5 6
]
```
