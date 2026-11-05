# Quadra

A quadtree-based image approximation project written in **Rust**.

Quadra uses recursive image decomposition and Mean Squared Error (MSE) to simplify images into smaller rectangular regions, each represented by its average color.

The goal is to explore how **Quadtree Decomposition** can be used for image approximation and compression.

## Features

- **Quadtree Decomposition** — Recursively divides images into four smaller regions.
- **Average Color Calculation** — Calculates the average RGB color of each region.
- **MSE-Based Splitting** — Uses Mean Squared Error to determine whether a region should be subdivided.
- **Configurable Threshold** — Controls the level of image detail.
- **Minimum Chunk Size** — Prevents excessive subdivision.
- **Image Approximation** — Reconstructs the image using the average colors of leaf nodes.

## ⚙️ How It Works

Quadra follows a recursive image decomposition process:

1. Load an input image.
2. Resize it to the configured dimensions.
3. Calculate the average RGB color of the current region.
4. Calculate the region's Mean Squared Error (MSE).
5. Compare the MSE against a configurable threshold.
6. If the MSE exceeds the threshold and the region is large enough, divide it into four smaller regions.
7. Otherwise, treat the region as a leaf node.
8. Traverse the resulting tree and render each leaf using its average color.

This process continues recursively until the image has been sufficiently simplified.

## Mean Squared Error (MSE)

MSE measures how much the individual pixel colors differ from the average color of a region.

Quadra calculates it using the following formula:

```math
MSE = \frac{1}{3N} \sum_{i=1}^{N} \left[ (R_i - R_{avg})^2 + (G_i - G_{avg})^2 + (B_i - B_{avg})^2 \right]
```

Where:

- `N` = Number of pixels in the region.
- `R_i`, `G_i`, `B_i` = RGB values of an individual pixel.
- `R_avg`, `G_avg`, `B_avg` = Average RGB values of the region.

A higher MSE indicates greater color variation within a region.

The MSE threshold determines whether a region should be subdivided:

- **Lower threshold** → More subdivisions and greater detail.
- **Higher threshold** → Fewer subdivisions and greater simplification.

## 🛠️ Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)
- The [`image`](https://crates.io/crates/image) Rust crate
  

## 🚀 Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/YOUR_USERNAME/quadra.git
cd quadra
```

### 2. Add an input image

Place your source image inside the `images/` directory.

### 3. Configure the project

All the main configuration parameters are located at the **top of `src/main.rs`**.

The default configuration is:

```rust
const SIZE: (u32, u32) = (512, 512);
const THRESHOLD: f32 = 1000.0;

const SOURCE_IMAGE: &str = "<image path>";
const OUTPUT_IMAGE: &str = "<image path>";
```

You can modify these parameters according to your requirements:

| Parameter | Description |
|---|---|
| `SIZE` | Width and height to which the source image is resized. |
| `THRESHOLD` | MSE threshold used to determine when a region should be subdivided. |
| `SOURCE_IMAGE` | Path to the input image. |
| `OUTPUT_IMAGE` | Path where the generated image will be saved. |

**Tip:** Experiment with different threshold values to observe how they affect image detail and simplification.

### 4. Run the project

```bash
cargo run
```

The generated image will be saved to the configured output path.

## 📁 Project Structure

```text
quadra/
│
├── images/
│   ├── source_image.png
│   └── OUTPUT.png
│
├── src/
│   └── main.rs
│
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── README.md
```

## 🧠 Concepts Explored

This project explores several programming and computer science concepts:

- Recursive algorithms
- Tree data structures
- Quadtree decomposition
- Image processing
- Pixel manipulation
- Color averaging
- Mean Squared Error
- Memory management and ownership in Rust

## 📌 Project Status

**Under Development**

Quadra is an experimental project built to explore recursive image decomposition and image approximation using Rust.

More improvements and optimizations may be introduced as development progresses.

## 📄 License

This project is licensed under the [MIT License](LICENSE).

If you use or redistribute this project, please retain the original copyright and license notice.

---

Made with 🦀 and Rust.
