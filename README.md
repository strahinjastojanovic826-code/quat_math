# Quat Math (`quat_math`)

`quat_math` is a high-performance, memory-efficient Rust library designed for quaternary logic and arithmetic (base-4). By packing 4 quaternary digits (each taking 2 bits) into a single standard byte (`u8`), it achieves an optimal **4-to-1 byte packing ratio**[cite: 5, 7], making it ideal for memory-constrained environments and high-speed vectorized operations.

---

## Features

* **`Quat`**: Represents a single quaternary digit (quat) packed into 2 bits, with values ranging from `0` to `3` (binary: `00`, `01`, `10`, `11`)[cite: 2].
* **Arithmetic Operations**: Implements `Add`, `Sub`, and `Mul` utilizing modulo 4 arithmetic and Euclidean remainder (`rem_euclid`)[cite: 1].
* **Logic Operations**: Custom bitwise and logical implementations for `NOT`, `BitAnd`, `BitOr`, and `BitXor`[cite: 4].
* **`QuatWord`**: Packs 4 quaternary digits into a single standard `u8` following the layout `[Q3: 7..6 | Q2: 5..4 | Q1: 3..2 | Q0: 1..0]`[cite: 5].
* **`QuatVec`**: A high-performance dynamic vector storing quaternary values with automatic memory management, bounds checking, and zero-copy raw buffer access[cite: 7, 8].
* **FFI Support**: Production-ready C-compatible FFI layer with panic safety (`catch_unwind`) and bulk processing capabilities.

---

## Licensing

This project is dual-licensed:

* **Open Source (AGPLv3):** Free to use for open-source projects. If you use or modify this library in your software or network service, your source code must also be open-source under AGPLv3.
* **Commercial License:** Required if you want to use `quat_math` in closed-source or proprietary commercial software without AGPLv3 restrictions.

For commercial licensing, contact: **strahinjastojanovic826@gmail.com**

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
quat_math = "0.3.0"
```

To build as a shared library for FFI consumers (C, C++, Python, etc.), configure the library type:

```toml
[lib]
name = "quat_math"
crate-type = ["cdylib", "rlib"]
```

---

## Quick Start Guide

### Basic Quat Arithmetic
```rust
use quat_math::quat::Quat;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let q1 = Quat::ONE; // binary 01
    let q2 = Quat::TWO; // binary 10

    // Modular addition (1 + 2 = 3)
    let sum = q1 + q2;
    assert_eq!(sum.value(), 3);

    Ok(())
}
```

### Using Packed Vectors (`QuatVec`)
```rust
use quat_math::vector::QuatVec;
use quat_math::quat::Quat;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vec = QuatVec::new();
    
    // Push quats safely into the vector
    vec.push(Quat::ZERO)?;
    vec.push(Quat::THREE)?;

    // Access raw 4-to-1 packed byte buffer via zero-copy
    let raw_bytes: &[u8] = vec.raw_data();
    println!("Packed data buffer length: {} bytes", raw_bytes.len());

    Ok(())
}
```

# FFI Import Examples for `quat_math`

Here are code examples demonstrating how to import and use your compiled `quat_math` shared library (`.dll`, `.so`, or `.dylib`) in other popular programming languages using the C-compatible FFI interface.

---

## 1. Python (Using `ctypes` and NumPy)

Python can load the compiled shared library and map the zero-copy raw memory buffer directly into a NumPy array[cite: 7, 8], allowing high-speed vectorized operations.

```python
import ctypes
import numpy as np

# Load the shared library (e.g., quat_math.dll on Windows, libquat_math.so on Linux)
lib = ctypes.CDLL("./libquat_math.so")

# Define argument and return types for safety
lib.quat_vec_new.restype = ctypes.c_void_p
lib.quat_vec_free.argtypes = [ctypes.c_void_p]
lib.quat_vec_push_bulk.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_ubyte), ctypes.c_size_t]
lib.quat_vec_get_data_buffer.argtypes = [
    ctypes.c_void_p, 
    ctypes.POINTER(ctypes.c_void_p), 
    ctypes.POINTER(ctypes.c_size_t)
]

# 1. Create a new QuatVec instance
vec_ptr = lib.quat_vec_new()

# 2. Push bulk data (quaternary values ranging from 0 to 3)[cite: 2]
data = (ctypes.c_ubyte * 4)(0, 1, 2, 3)
lib.quat_vec_push_bulk(vec_ptr, data, 4)

# 3. Access the raw 4-to-1 packed buffer via zero-copy[cite: 5, 7, 8]
buf_ptr = ctypes.c_void_p()
buf_size = ctypes.c_size_t()
lib.quat_vec_get_data_buffer(vec_ptr, ctypes.byref(buf_ptr), ctypes.byref(buf_size))

# Map the pointer into a NumPy array without copying memory
raw_array = np.frombuffer(
    (ctypes.c_ubyte * buf_size.value).from_address(buf_ptr.value), 
    dtype=np.uint8
)
print("Packed raw buffer in Python (NumPy):", raw_array)

# 4. Clean up allocated memory
lib.quat_vec_free(vec_ptr)
```

---

## 2. C / C++ (Native FFI Integration)

Using the library natively in C or C++ by linking against the shared library and calling the exported `extern "C"` functions.

```c
#include <stdio.h>
#include <stdint.h>

// Forward declarations of FFI functions
void* quat_vec_new(void);
void quat_vec_free(void* ptr);
int quat_vec_push_bulk(void* ptr, const uint8_t* data, size_t len);
int quat_vec_get_data_buffer(const void* ptr, const uint8_t** out_data, size_t* out_size);

int main() {
    // 1. Initialize vector
    void* vec = quat_vec_new();
    if (!vec) {
        fprintf(stderr, "Failed to create QuatVec\n");
        return 1;
    }

    // 2. Push elements in bulk
    uint8_t values[4] = {0, 1, 2, 3};
    quat_vec_push_bulk(vec, values, 4);

    // 3. Get zero-copy buffer pointer[cite: 7, 8]
    const uint8_t* buffer = NULL;
    size_t size = 0;
    if (quat_vec_get_data_buffer(vec, &buffer, &size) == 0) {
        printf("Packed buffer size in bytes: %zu\n", size);
        if (size > 0) {
            printf("First packed byte: 0x%02X\n", buffer[0]);
        }
    }

    // 4. Free memory
    quat_vec_free(vec);
    return 0;
}
```

---

## 3. C# (.NET / Unity via P/Invoke)

Importing and calling the library inside C# applications or Unity games using Platform Invoke (`DllImport`).

```csharp
using System;
using System.Runtime.InteropServices;

class Program
{
    const string LibraryName = "quat_math";

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr quat_vec_new();

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void quat_vec_free(IntPtr ptr);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    public static extern int quat_vec_push_bulk(IntPtr ptr, byte[] data, IntPtr len);

    static void Main()
    {
        // 1. Create vector
        IntPtr vecPtr = quat_vec_new();
        if (vecPtr != IntPtr.Zero)
        {
            // 2. Push data array
            byte[] values = new byte[] { 0, 1, 2, 3 };
            quat_vec_push_bulk(vecPtr, values, (IntPtr)values.Length);
            
            Console.WriteLine("Successfully populated QuatVec via C# P/Invoke.");

            // 3. Free resources
            quat_vec_free(vecPtr);
        }
    }
}
```
