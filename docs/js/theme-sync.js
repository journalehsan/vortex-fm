// Theme configurations
const themes = {
    'tokyo-night': {
        window: '#14141F',
        view: '#1F1F2E', 
        accent: '#3E9BFF',
        foreground: '#D8D8D0'
    },
    'catppuccin': {
        window: '#1E1E2E',
        view: '#24273A',
        accent: '#3E9BFF', 
        foreground: '#E8E4E0'
    },
    'dracula': {
        window: '#0B0B0F',
        view: '#282A36',
        accent: '#E49BF8',
        foreground: '#F2F2F2'
    },
    'everforest': {
        window: '#323D43',
        view: '#3F4944',
        accent: '#66FF99',
        foreground: '#D8D8D0'
    },
    'gruvbox': {
        window: '#262626',
        view: '#32302F',
        accent: '#D79921',
        foreground: '#D5C4A1'
    },
    'kanagawa': {
        window: '#16161D',
        view: '#1F1F28',
        accent: '#3E9BFF',
        foreground: '#D8D8D0'
    },
    'matte-black': {
        window: '#0D0D0D',
        view: '#1A1A1A',
        accent: '#3E9BFF',
        foreground: '#E6E6E6'
    },
    'nord': {
        window: '#242933',
        view: '#2E3440',
        accent: '#3E9BFF',
        foreground: '#D8D8D0'
    },
    'osaka-jade': {
        window: '#0D1A14',
        view: '#14261F',
        accent: '#33CC66',
        foreground: '#D9E6D9'
    },
    'ristretto': {
        window: '#1A1414',
        view: '#261F1F',
        accent: '#CC6666',
        foreground: '#D9CCCC'
    },
    'catppuccin-latte': {
        window: '#F2F2F2',
        view: '#FFFFFF',
        accent: '#3E9BFF',
        foreground: '#333333'
    },
    'rose-pine': {
        window: '#F2F2F2',
        view: '#FFFFFF',
        accent: '#996699',
        foreground: '#333333'
    }
};

// Create app previews
function createAppPreview(containerId, theme, appType) {
    const container = document.getElementById(containerId);
    container.innerHTML = '';
    
    const preview = document.createElement('div');
    preview.className = 'w-full h-full relative';
    
    // Different preview layouts for different apps
    switch(appType) {
        case 'filemanager':
            preview.innerHTML = `
                <div class="absolute inset-0" style="background: ${theme.window}"></div>
                <div class="absolute left-0 top-0 w-1/4 h-full" style="background: ${theme.view}"></div>
                <div class="absolute left-1/4 top-0 w-3/4 h-full" style="background: ${theme.window}"></div>
                <div class="absolute bottom-0 left-0 right-0 h-1" style="background: ${theme.accent}"></div>
            `;
            break;
        case 'editor':
            preview.innerHTML = `
                <div class="absolute inset-0" style="background: ${theme.window}"></div>
                <div class="absolute top-2 left-2 right-2 h-4 rounded" style="background: ${theme.view}"></div>
                <div class="absolute top-8 left-2 right-2 h-20 rounded" style="background: ${theme.view}"></div>
                <div class="absolute bottom-2 left-2 right-2 h-1" style="background: ${theme.accent}"></div>
            `;
            break;
        case 'terminal':
            preview.innerHTML = `
                <div class="absolute inset-0" style="background: ${theme.window}"></div>
                <div class="absolute top-2 left-2 right-2 h-4 rounded" style="background: ${theme.foreground}; opacity: 0.1"></div>
                <div class="absolute top-8 left-2 right-2 h-4 rounded" style="background: ${theme.foreground}; opacity: 0.2"></div>
                <div class="absolute top-14 left-2 right-2 h-4 rounded" style="background: ${theme.foreground}; opacity: 0.3"></div>
                <div class="absolute bottom-2 left-2 right-2 h-1" style="background: ${theme.accent}"></div>
            `;
            break;
        case 'settings':
            preview.innerHTML = `
                <div class="absolute inset-0" style="background: ${theme.window}"></div>
                <div class="absolute left-0 top-0 w-1/3 h-full" style="background: ${theme.view}"></div>
                <div class="absolute left-1/3 top-0 w-2/3 h-full" style="background: ${theme.window}"></div>
                <div class="absolute bottom-0 left-0 right-0 h-1" style="background: ${theme.accent}"></div>
            `;
            break;
    }
    
    container.appendChild(preview);
}

// Apply theme to all apps
function applySystemTheme(themeName) {
    const theme = themes[themeName];
    if (!theme) return;

    // Add animation to all app cards
    document.querySelectorAll('.app-card').forEach(card => {
        card.classList.add('animate-theme-switch');
    });

    // Update all app previews
    createAppPreview('vortexPreview', theme, 'filemanager');
    createAppPreview('editorPreview', theme, 'editor');
    createAppPreview('terminalPreview', theme, 'terminal');
    createAppPreview('settingsPreview', theme, 'settings');

    // Remove animation after completion
    setTimeout(() => {
        document.querySelectorAll('.app-card').forEach(card => {
            card.classList.remove('animate-theme-switch');
        });
    }, 500);
}

// Theme selector event
document.addEventListener('DOMContentLoaded', function() {
    const themeSelector = document.getElementById('themeSelector');
    if (themeSelector) {
        themeSelector.addEventListener('change', (e) => {
            applySystemTheme(e.target.value);
        });
    }

    // Theme card clicks
    document.querySelectorAll('.theme-card').forEach(card => {
        card.addEventListener('click', () => {
            const themeName = card.dataset.theme;
            if (themeSelector) {
                themeSelector.value = themeName;
            }
            applySystemTheme(themeName);
        });
    });

    // Dark mode toggle
    const themeToggle = document.getElementById('themeToggle');
    if (themeToggle) {
        themeToggle.addEventListener('click', () => {
            document.documentElement.classList.toggle('dark');
            const icon = themeToggle.querySelector('i');
            if (document.documentElement.classList.contains('dark')) {
                icon.classList.replace('fa-moon', 'fa-sun');
            } else {
                icon.classList.replace('fa-sun', 'fa-moon');
            }
        });
    }

    // Initialize with Tokyo Night theme
    applySystemTheme('tokyo-night');
});
