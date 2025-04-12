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
├── scripts/              # Utility scripts
│   ├── chrome_capture.rs # PDF/JPEG generation script
│   ├── stripper.rs       # HTML/CSS optimization tool
│   └── stripper.legacy   # Legacy HTML cleanup tool
├── router.rs             # URL routing
├── metadata.rs           # SEO metadata
├── main.rs               # Application entry point
├── public/               # Static assets
└── assets/               # Compiled assets
```

## Utility Scripts

This project includes several utility scripts in the `scripts/` directory:

### chrome_capture.rs

A headless Chrome automation script that generates PDF and JPEG versions of the CV in both light and dark themes.

- Automatically starts a local server when needed
- Captures high-quality PDF files optimized for printing
- Generates JPEG screenshots of the entire page
- Supports different environments (local, dev, prod)

### stripper.rs

An optimization tool that prepares the generated HTML for production by:

- Removing unnecessary hydration data and scripts
- Cleaning up Dioxus-specific attributes
- Minifying the final HTML output
- Finding and optimizing CSS assets

### stripper.legacy

A legacy version of the HTML cleanup tool that uses regex patterns to remove specific tags from the generated HTML output.

## Technology Stack

- **Rust**: Fast and reliable systems programming language
- **Dioxus**: React-inspired UI library for Rust
- **Tailwind CSS**: Utility-first CSS framework
- **hmziq-dioxus-free-icons**: Icon component library
- **Static Site Generation**: Pre-rendered HTML for better performance

## Deployment

This project uses GitHub Actions for automated deployment. The deployment pipeline builds the site, optimizes the output, generates PDF/JPEG versions of the CV, and deploys to multiple platforms.

### Deployment Infrastructure

- **GitHub Pages**: Primary hosting for the static site
- **Cloudflare Pages**: Secondary deployment location for enhanced performance
- **GitHub Actions**: CI/CD pipeline automation

### Deployment Workflow

The deployment process is managed through several GitHub Actions workflows:

1. **Deploy (`deploy.yml`)**: Main deployment workflow that:
   - Builds the site with Dioxus and Tailwind CSS
   - Downloads the latest utility binaries (stripper and chrome_capture)
   - Optimizes HTML output with the stripper tool
   - Generates PDF and JPEG versions of the CV
   - Deploys to both GitHub Pages and Cloudflare Pages
   - Uploads PDF/JPEG artifacts for download

2. **Build Tools**: Separate workflows that build and release the utility scripts:
   - `build-stripper.yml`: Builds the HTML optimization tool for multiple platforms
   - `build-chrome-capture.yml`: Builds the PDF/JPEG generation tool for multiple platforms

### Running Deployments Manually

You can trigger a manual deployment through the GitHub Actions interface by selecting the "Deploy Rust CV to Pages" workflow and clicking "Run workflow".

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

2.a Serve the built files (windows):
   ```bash
   simple-http-server.exe --ip 127.0.0.1 .\target\dx\cv\release\web\public -i
   ```

## Customization

- Edit content in component files to update CV information
- Modify styles in `input.css` and component classes for design changes
- Add new sections by creating components and adding them to the main layout

## License

This project is available as open source under the terms of the [MIT License](LICENSE).

## Author

Hamza Iqbal - [Portfolio](https://hmziq.rs) - [GitHub](https://github.com/hmziqrs)
