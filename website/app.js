/* ==========================================================================
   PawsMC — Website JavaScript Logic
   ========================================================================== */

document.addEventListener('DOMContentLoaded', () => {
    initThemeSwitcher();
    initDownloadDropdown();
    initFaqAccordion();
    initNavbarScroll();
    fetchGitHubReleaseData();
});

/* 1. Theme Switcher (Updates CSS variables and mockup) */
function initThemeSwitcher() {
    const themePills = document.querySelectorAll('.theme-pill');
    const html = document.documentElement;

    themePills.forEach(pill => {
        pill.addEventListener('click', () => {
            const theme = pill.getAttribute('data-set-theme');
            
            // Set active state
            themePills.forEach(p => p.classList.remove('active'));
            pill.classList.add('active');

            // Apply theme attribute
            html.setAttribute('data-theme', theme);
        });
    });
}

/* 2. Download Options Dropdown */
function initDownloadDropdown() {
    const dropdownBtn = document.getElementById('downloadDropdownBtn');
    const dropdownMenu = document.getElementById('downloadDropdownMenu');

    if (!dropdownBtn || !dropdownMenu) return;

    dropdownBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        dropdownMenu.classList.toggle('open');
    });

    document.addEventListener('click', (e) => {
        if (!dropdownMenu.contains(e.target) && e.target !== dropdownBtn) {
            dropdownMenu.classList.remove('open');
        }
    });
}

/* 3. FAQ Accordion */
function initFaqAccordion() {
    const faqItems = document.querySelectorAll('.faq-item');

    faqItems.forEach(item => {
        const questionBtn = item.querySelector('.faq-question');
        questionBtn.addEventListener('click', () => {
            const isActive = item.classList.contains('active');
            
            // Close other items
            faqItems.forEach(other => other.classList.remove('active'));

            if (!isActive) {
                item.classList.add('active');
            }
        });
    });
}

/* 4. Navbar Scroll Effect & Mobile Toggle */
function initNavbarScroll() {
    const navbar = document.getElementById('navbar');
    const mobileToggle = document.getElementById('mobileToggle');
    const navLinks = document.getElementById('navLinks');

    window.addEventListener('scroll', () => {
        if (window.scrollY > 40) {
            navbar.classList.add('scrolled');
        } else {
            navbar.classList.remove('scrolled');
        }
    });

    if (mobileToggle && navLinks) {
        mobileToggle.addEventListener('click', () => {
            navLinks.classList.toggle('mobile-open');
        });
    }
}

/* 5. Fetch Latest GitHub Release Info */
async function fetchGitHubReleaseData() {
    const repoOwner = 'Pawciolszef';
    const repoName = 'PawsMC';
    const apiUrl = `https://api.github.com/repos/${repoOwner}/${repoName}/releases/latest`;

    try {
        const response = await fetch(apiUrl);
        if (!response.ok) return;

        const data = await response.json();
        const tagName = data.tag_name || 'v1.0.0';

        // Update tag badge
        const badge = document.getElementById('releaseTagBadge');
        if (badge) {
            badge.textContent = `${tagName} is now live!`;
        }

        // Find assets in release
        if (data.assets && Array.isArray(data.assets)) {
            const setupAsset = data.assets.find(a => a.name.endsWith('.exe') || a.name.includes('Setup'));
            const zipAsset = data.assets.find(a => a.name.endsWith('.zip'));

            if (setupAsset) {
                const primaryBtn = document.getElementById('primaryDownloadBtn');
                if (primaryBtn) primaryBtn.href = setupAsset.browser_download_url;

                const subtext = document.getElementById('installerSubtext');
                const sizeMb = (setupAsset.size / (1024 * 1024)).toFixed(1);
                if (subtext) subtext.textContent = `Installer (.exe) • ${sizeMb} MB`;
            }

            if (zipAsset) {
                const portableLink = document.getElementById('portableDownloadLink');
                if (portableLink) portableLink.href = zipAsset.browser_download_url;
            }
        }
    } catch (err) {
        console.warn('Could not fetch latest release from GitHub API:', err);
    }
}
