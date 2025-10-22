# Vortex FM Website

This directory contains the source code for the Vortex File Manager website, built with [Zola](https://www.getzola.org/) and deployed to GitHub Pages.

## Features

- **Dynamic Theme System**: Interactive theme switching with system-wide synchronization demo
- **Blog**: Development updates and technical insights
- **Wiki**: Comprehensive documentation
- **Dev Journey**: Development story and progress tracking
- **Responsive Design**: Works on desktop and mobile
- **Fast Loading**: Optimized with Tailwind CSS and modern web standards

## Structure

```
├── config.toml              # Zola configuration
├── content/                 # Content files
│   ├── _index.md           # Homepage content
│   ├── blog/               # Blog posts
│   ├── wiki/               # Documentation
│   └── dev-journey/        # Development journey
├── templates/              # HTML templates
│   └── index.html          # Main template
├── static/                 # Static assets
│   ├── css/                # Custom styles
│   └── js/                 # JavaScript files
└── docs/                   # Screenshots and assets
    └── assets/img/         # Images
```

## Development

### Prerequisites

- [Zola](https://www.getzola.org/) static site generator
- Git for version control

### Local Development

1. **Install Zola:**
   ```bash
   curl -sSL https://get.zola.rs | sh -s - --locked
   ```

2. **Start development server:**
   ```bash
   zola serve
   ```

3. **Open in browser:**
   Navigate to `http://127.0.0.1:1111`

### Building

1. **Build the site:**
   ```bash
   ./build-site.sh
   ```

2. **Or manually:**
   ```bash
   zola build
   ```

## Deployment

The site is automatically deployed to GitHub Pages when changes are pushed to the `master` branch.

### Manual Deployment

1. **Build the site:**
   ```bash
   zola build
   ```

2. **Deploy to GitHub Pages:**
   ```bash
   # Copy public/ contents to docs/ folder
   cp -r public/* docs/
   
   # Commit and push
   git add docs/
   git commit -m "Update website"
   git push origin master
   ```

## Content Management

### Adding Blog Posts

1. Create a new markdown file in `content/blog/`
2. Add frontmatter with title, date, and description
3. Write your content in markdown
4. The post will appear automatically on the blog page

### Adding Wiki Pages

1. Create markdown files in `content/wiki/`
2. Update the wiki index to include links
3. Use relative links for internal navigation

### Adding Dev Journey Entries

1. Create markdown files in `content/dev-journey/`
2. Document your development progress
3. Include screenshots and technical details

## Theme System

The website features an interactive theme system that demonstrates the Omarchy theme synchronization:

- **Live Theme Switching**: Users can switch between different themes
- **System Sync Demo**: Shows how themes sync across applications
- **Responsive Design**: Works on all screen sizes
- **Dark Mode**: Automatic dark/light mode switching

## Customization

### Adding New Themes

1. Update the theme configuration in `templates/index.html`
2. Add theme colors to the Tailwind config
3. Update the JavaScript theme definitions
4. Test the theme switching functionality

### Styling

- Uses Tailwind CSS for styling
- Custom CSS in `static/css/`
- Responsive design with mobile-first approach
- Dark mode support with automatic switching

## Deployment

The website is automatically deployed to GitHub Pages using GitHub Actions:

1. **Trigger**: Push to `master` branch
2. **Build**: Zola builds the static site
3. **Deploy**: Automatically deployed to GitHub Pages
4. **URL**: `https://journalehsan.github.io/vortex-fm`

### Manual Deployment

If you need to deploy manually:

1. Build the site: `zola build`
2. Copy contents to `docs/` folder
3. Commit and push to trigger GitHub Pages

## Contributing

To contribute to the website:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally with `zola serve`
5. Submit a pull request

## License

The website content is licensed under the same GPLv3 license as the main project.
