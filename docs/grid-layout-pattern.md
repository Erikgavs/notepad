# Grid Layout Pattern with Chunks

## Problem

When displaying multiple items (like notes) in a GUI, you may want them distributed in a grid pattern:

```
[item] [item] [item] [item]
[item] [item] [item] [item]
[item] [item]
```

However, if your UI framework doesn't support `flex-wrap` or a native `Grid` component, items will either:
- Stack vertically (one below another)
- Overflow horizontally (go off-screen)

## Solution

Use Rust's `.chunks(n)` method to manually create rows.

## How `.chunks()` Works

```rust
let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

items.chunks(4)
// Returns an iterator of slices:
// [1, 2, 3, 4]
// [5, 6, 7, 8]
// [9, 10]
```

The last chunk may have fewer elements if the total isn't evenly divisible.

## Implementation Pattern

### Before (single loop - items overflow)

```rust
for item in items.iter() {
    rect { /* render item */ }
}
```

### After (nested loops - grid layout)

```rust
// Outer container: stacks rows vertically
rect {
    direction: "vertical",

    // First loop: iterate over chunks (rows)
    for chunk in items.chunks(4) {

        // Each chunk becomes a horizontal row
        rect {
            direction: "horizontal",
            spacing: "10",

            // Second loop: iterate over items in this row
            for item in chunk {
                rect { /* render item */ }
            }
        }
    }
}
```

## Visual Representation

```
rect (vertical)
├── rect (horizontal) ─── [item0] [item1] [item2] [item3]
├── rect (horizontal) ─── [item4] [item5] [item6] [item7]
└── rect (horizontal) ─── [item8] [item9]
```

## Key Concepts

1. **Outer loop** creates rows (horizontal containers)
2. **Inner loop** fills each row with items
3. **Chunk size** determines items per row (e.g., `chunks(4)` = 4 items per row)

## Adapting for Different Grid Sizes

```rust
// 2 items per row
items.chunks(2)

// 3 items per row
items.chunks(3)

// 5 items per row
items.chunks(5)
```

## Notes for Freya

In Freya, use these properties for the row container:

```rust
rect {
    direction: "horizontal",  // items side by side
    spacing: "10",            // gap between items
    // ...
}
```

## Handling Indexes

If you need the position of each item (e.g., for delete buttons), use `.enumerate()`:

```rust
for (chunk_index, chunk) in items.chunks(4).enumerate() {
    rect {
        direction: "horizontal",

        for (item_index, item) in chunk.iter().enumerate() {
            // Calculate global position
            let position = chunk_index * 4 + item_index;

            rect { /* render item with position */ }
        }
    }
}
```
