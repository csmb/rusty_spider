# Image Crawler

A Rust-based web crawler that downloads JPG and GIF images from a website and its subpages within the same domain.

## Features

- Recursively crawls websites while staying within the same domain
- Downloads JPG and GIF images
- Handles both relative and absolute URLs
- Concurrent processing for better performance
- Rate limiting to be respectful to servers
- Deduplicates URLs and images
- Shows progress and summary statistics

## Requirements

- Rust 2021 edition or later
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/image_crawler.git
cd image_crawler
```

2. Build the project:
```bash
cargo build --release
```

## Usage

Run the crawler with a URL as an argument:

```bash
cargo run -- https://example.com
```

Images will be downloaded to the `downloads` directory.

## License

MIT License 