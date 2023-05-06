# Zigzag

This is a crate to document my approach for matrixless zigzag conversion.

## Prompt

The string `"PAYPALISHIRING"` is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

```
P   A   H   N
A P L S I I G
Y   I   R
```

And then read line by line: `"PAHNAPLSIIGYIR"`

Write the code that will take a string and make this conversion given a number of rows:

```
string convert(string s, int numRows);
```

 

#### Example 1:
```
Input: s = "PAYPALISHIRING", numRows = 3
Output: "PAHNAPLSIIGYIR"
```

#### Example 2:
```
Input: s = "PAYPALISHIRING", numRows = 4
Output: "PINALSIGYAHRPI"
Explanation:
P     I    N
A   L S  I G
Y A   H R
P     I
```

#### Example 3:
```
Input: s = "A", numRows = 1
Output: "A"
```
 

#### Constraints:

* `1 <= s.length <= 1000`
* `s` consists of English letters (lower-case and upper-case), `','` and `'.'`.
* `1 <= numRows <= 1000`


## Explanation

The sawtooth pattern means y axis placement is dictated my the modulo of $2r-2$, where $r$ is the number of desired rows. When operated with, the remainder determines its effective placement in the y dimension (without a matrix). For $a \bmod b = c$ where $a$ is the index of the current char and $b$ is the below divisor, where $c < r$, where $r$ is the number of desired rows (or $(b+2)/2$)...

$c$ is the same as its effective y axis position, and its x axis position is equal to $(a/b)\times (b/2)$ using integer division, truncating down. Note that $b/2$ will never round as it will always be even.

Else where $r \le c$, the effective y axis position is equal to $r-(c \bmod r + 2)$ and its x axis position is equal to $((a/b)\times (b/2))+((c+1) \bmod b)$.
