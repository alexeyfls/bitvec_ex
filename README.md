# Bitvec

**Elixir NIF for Bit Manipulation, powered by the Rust bitvec crate**

Bitvec provides a **mutable**, **resizable**, and **highly efficient** bit-vector (or _bit array_) for Elixir. It is implemented as a Rust NIF (Native Implemented Function) using the powerful [bitvec](https://docs.rs/bitvec) crate.

---

## 🚀 Features

- **Fast:** All operations are native Rust code, running as a NIF.
- **Efficient:** Uses Rust's bitvec for densely packed bit storage.
- **Comprehensive API:** Exposes core bitvec functionality, including `push`, `pop`, `append`, `from_vec`, `capacity`, `reserve`, and more.
- **Concurrency Safe:** Each `Bitvec` resource is protected by a Rust `Mutex`, ensuring thread safety across Elixir processes.

---

## 💾 Installation

Add **bitvec** to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:bitvec, "~> 0.1.0"}
  ]
end
```

Since this is a NIF library, you must have a Rust compiler installed on your system. **Rustler** will automatically compile the native code when you run:

```bash
mix deps.get
```

---

## 🏃 Quick Usage

The `Bitvec` module provides a clean, Elixir-native API. Functions return an unwrapped value on success or an error tuple on failure — perfect for use with `with` or `case`.

### Basic Example

```elixir
# 1. Create a new bit-vector
case Bitvec.new() do
  {:error, reason} ->
    IO.puts("Failed to create: #{reason}")

  bv ->
    # 2. Push some bits (mutating functions return the resource on success)
    bv = Bitvec.push(bv, true)
    bv = Bitvec.push(bv, false)
    bv = Bitvec.push(bv, true)

    # 3. Check its state
    IO.inspect(Bitvec.len(bv))
    #=> 3

    # 4. Pop a bit (returns true, false, or nil)
    IO.inspect(Bitvec.pop(bv))
    #=> true

    IO.inspect(Bitvec.len(bv))
    #=> 2
end
```

### Chaining with `with`

```elixir
with bv <- Bitvec.from_vec([1, 0, 1, 1, 0]),
     bv <- Bitvec.push(bv, true),
     len <- Bitvec.len(bv) do
  IO.puts("Final length: #{len}")
else
  {:error, reason} ->
    IO.puts("Operation failed: #{reason}")
end
```

---

## ⚠️ Error Handling

All NIF-backed functions return either a direct value or a common error tuple.

```elixir
@type common_errors ::
        {:error, :bad_reference} |
        {:error, :lock_fail} |
        {:error, :unsupported_type} |
        {:error, :index_out_of_bounds} |
        {:error, :ordering_mismatch}

@type unwrapped_result(t) :: t | common_errors()
```

**Error Types:**

- `{:error, :bad_reference}` — The NIF resource was invalid or garbage-collected.
- `{:error, :lock_fail}` — Mutex lock was poisoned.
- `{:error, :index_out_of_bounds}` — Index operation out of range.
- `{:error, :ordering_mismatch}` — Append failed due to mismatched bit orderings or storage types.

Always handle these cases using `case` or `with`.

---

## 🧠 Memory & Concurrency

### Concurrency

Each `Bitvec.t()` is protected by a Rust `Mutex`. All operations (e.g., `push`, `pop`) acquire and release this lock safely, making concurrent access from multiple Elixir processes **thread-safe**.

### Memory

The library integrates **JemallocInfo** via `JemallocInfo.RustlerMixin`, allowing inspection of the NIF's memory usage:

```elixir
Bitvec.jemalloc_allocation_info()
```

---

## 📄 License

This project is licensed under the terms of the **MIT License**.
See the [LICENSE](./LICENSE) file for full text.

Portions of this software are derived from the [bitvec](https://github.com/ferrilab/bitvec) crate,
which is also licensed under the **MIT License**.
