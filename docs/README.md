# excel-to-json Documentation

[![Built with Starlight](https://astro.badg.es/v2/built-with-starlight/tiny.svg)](https://starlight.astro.build)

This folder contains the documentation site for the excel-to-json project, built with Astro and Starlight.

## 🚀 Project Structure

```
.
├── public/
│   ├── favicon.svg
│   └── rust-api-docs/       # Auto-generated Rust API documentation
├── src/
│   ├── assets/              # Images and other assets
│   ├── content/
│   │   └── docs/           # Documentation content (MDX files)
│   │       ├── getting-started/
│   │       ├── usage/
│   │       └── reference/
│   └── styles/             # Custom CSS
├── astro.config.mjs        # Astro configuration
├── package.json            # Node dependencies and scripts
└── dist/                   # Built site (generated)
```

## 🧞 Commands

All commands are run from this directory (`docs/`):

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run build:rust-docs` | Generate Rust API documentation                  |
| `npm run build:all`       | Generate Rust docs + build site                  |

## 📝 Development Workflow

### Local Development

1. **Install dependencies**:
   ```bash
   npm install
   ```

2. **Generate Rust API docs** (optional, for latest API docs):
   ```bash
   npm run build:rust-docs
   ```

3. **Start development server**:
   ```bash
   npm run dev
   ```
   Open http://localhost:4321 to view the site.

### Building for Production

```bash
# Build everything (Rust docs + site)
npm run build:all

# Or build separately:
npm run build:rust-docs  # Rust API docs only
npm run build           # Astro site only
```

### Preview Production Build

```bash
npm run preview
```

## 🚢 Deployment

The documentation is automatically deployed to GitHub Pages when pushing to the main branch.

The GitHub Actions workflow (`.github/workflows/deploy-pages.yml`) handles:
1. Building the Rust API documentation with `cargo doc`
2. Building the Astro documentation site
3. Deploying everything to GitHub Pages

### Manual Deployment

1. Go to the Actions tab in GitHub
2. Select "Deploy to GitHub Pages"
3. Click "Run workflow"

## 📚 Adding Documentation

### Regular Documentation

Add MDX files to `src/content/docs/` following the existing structure:
- `getting-started/` - Installation and quick start guides
- `usage/` - How to use the tool
- `reference/` - API reference and detailed documentation

### Rust API Documentation

The Rust API docs are auto-generated from source code comments:
1. Update doc comments in the Rust source code
2. Run `npm run build:rust-docs` to regenerate
3. The docs will be placed in `public/rust-api-docs/`

## 🔧 Troubleshooting

### Rust docs not showing in development
Rust API documentation links work in production but may show 404 in dev mode.
To test them locally:
```bash
npm run build
npm run preview
```

### Missing Rust documentation
Regenerate the Rust docs:
```bash
npm run build:rust-docs
```

### Build failures on GitHub Actions
Check that:
- The Rust code compiles without errors
- All dependencies are installed
- The `cargo doc` command runs successfully locally

## 👀 Learn More

- [Starlight Documentation](https://starlight.astro.build/)
- [Astro Documentation](https://docs.astro.build)
- [Rust Documentation](https://doc.rust-lang.org/rustdoc/)
