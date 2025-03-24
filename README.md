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
- Organizes downloads by format, domain, and size categories


## Installation

1. Clone the repository:
```bash
git clone https://github.com/csmb/rusty_spider.git
cd rusty_spider
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

Images will be downloaded to the `downloads` directory with the following organization:

```
downloads/
├── jpg/                    # All JPG images
│   ├── example.com/        # Grouped by domain
│   │   ├── small/          # < 100KB
│   │   ├── medium/         # 100KB - 1MB
│   │   └── large/          # > 1MB
│   └── another-site.com/
└── gif/                    # All GIF images
    └── example.com/
        ├── small/
        ├── medium/
        └── large/
```

The crawler will:
1. Create all necessary directories automatically
2. Save only the highest quality version of each image
3. Organize images by format (jpg/gif), domain, and size category
4. Show progress as it downloads and organizes images

## License
MIT License 