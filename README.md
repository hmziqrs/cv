# Hamza Iqbal's CV Website

A modern, responsive CV/portfolio website built with Rust and Dioxus. This project showcases a clean and professional design, using Tailwind CSS for styling and hmziq-dioxus-free-icons for custom icon components.

## Features

- **Responsive Design**: Optimized for desktop, tablet, and mobile viewing
- **Light/Dark Mode**: Auto-detects system preference with smooth theme transitions
- **Print-Friendly**: Special layout optimizations for PDF export and printing
- **SEO Optimized**: Structured metadata for better search engine visibility
- **Component-Based Architecture**: Modular components for maintainability
- **Static Site Generation**: Fast loading, SEO-friendly static HTML output

## Project Structure

```
cv/
├── components/           # UI components
│   ├── download.rs       # PDF download section
│   ├── experience.rs     # Work experience timeline
│   ├── footer.rs         # Site footer with social links
│   ├── header.rs         # Profile header section
│   ├── project.rs        # Project showcases
│   ├── skills.rs         # Skills section
│   ├── spacer.rs         # Layout spacing component
│   └── mod.rs            # Component exports
├── screens/              # Page layouts
│   ├── home.rs           # Main CV page
│   └── mod.rs            # Screen exports
├── router.rs             # URL routing
├── metadata.rs           # SEO metadata
├── main.rs               # Application entry point
├── public/               # Static assets
└── assets/               # Compiled assets
```

## Technology Stack

- **Rust**: Fast and reliable systems programming language
- **Dioxus**: React-inspired UI library for Rust
- **Tailwind CSS**: Utility-first CSS framework
- **hmziq-dioxus-free-icons**: Icon component library
- **Static Site Generation**: Pre-rendered HTML for better performance

## Getting Started

### Prerequisites

- Rust toolchain (1.65+)
- Dioxus CLI

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/hmziqrs/cv.git
   cd cv
   ```

2. Install Dioxus CLI:

   ```bash
   cargo install dioxus-cli
   ```

3. Install Tailwind dependencies:
   ```bash
   npm install
   ```

### Development

Run the Tailwind CSS watcher:

```bash
npm run tailwind
```

Start the development server:

```bash
dx serve
```

### Building for Production

Build static site:

```bash
dx build --platform web --release --ssg
```

### Hosting

To locally host the static site:

1. Install simple-http-server:

   ```bash
   cargo install simple-http-server
   ```

2. Serve the built files:
   ```bash
   simple-http-server target/dx/cv/release/web/public -i
   ```

## Customization

- Edit content in component files to update CV information
- Modify styles in `input.css` and component classes for design changes
- Add new sections by creating components and adding them to the main layout

## License

This project is available as open source under the terms of the [MIT License](LICENSE).

## Author

Hamza Iqbal - [Portfolio](https://hmziq.rs) - [GitHub](https://github.com/hmziqrs)
