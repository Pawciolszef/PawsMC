document.addEventListener('DOMContentLoaded', async () => {
    try {
        const res = await fetch('https://api.github.com/repos/Pawciolszef/PawsMC/releases/latest');
        if (!res.ok) return;

        const data = await res.json();
        
        // Update tag badge
        const badge = document.getElementById('versionBadge');
        if (badge && data.tag_name) {
            badge.textContent = `${data.tag_name} Release`;
        }

        // Update download links
        if (data.assets && Array.isArray(data.assets)) {
            const setup = data.assets.find(a => a.name.endsWith('.exe') && !a.name.endsWith('.sig') && a.name.includes('Setup'));
            const zip = data.assets.find(a => a.name.endsWith('.zip'));

            if (setup) {
                const btn = document.getElementById('downloadBtn');
                if (btn) btn.href = setup.browser_download_url;
            }

            if (zip) {
                const portable = document.getElementById('portableBtn');
                if (portable) portable.href = zip.browser_download_url;
            }
        }
    } catch (e) {
        console.warn('Could not fetch latest release:', e);
    }
});
