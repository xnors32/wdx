pub const CSS: &str = r#"
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap');

:root {
    --bg: #080b10;
    --bg-elevated: #0d1117;
    --bg-card: #111620;
    --bg-hover: #161d2a;
    --bg-input: #0a0e15;
    --border: #1e2535;
    --border-subtle: #161c27;
    --text: #e8ecf0;
    --text-muted: #7a8899;
    --text-dim: #4a5568;
    --accent: #4f8cff;
    --accent-glow: rgba(79, 140, 255, 0.18);
    --accent-hover: #6b9fff;
    --accent-dim: rgba(79, 140, 255, 0.12);
    --success: #22c55e;
    --success-dim: rgba(34, 197, 94, 0.12);
    --danger: #f87171;
    --danger-dim: rgba(248, 113, 113, 0.12);
    --warning: #f59e0b;
    --warning-dim: rgba(245, 158, 11, 0.12);
    --radius: 10px;
    --radius-lg: 14px;
    --radius-xl: 18px;
    --sidebar-width: 210px;
    --transition: 0.18s cubic-bezier(0.4, 0, 0.2, 1);
}

.theme-light {
    --bg: #f0f2f7;
    --bg-elevated: #ffffff;
    --bg-card: #ffffff;
    --bg-hover: #f5f7fc;
    --bg-input: #f8f9fc;
    --border: #dde1ea;
    --border-subtle: #eaecf2;
    --text: #111827;
    --text-muted: #6b7280;
    --text-dim: #9ca3af;
    --accent: #2563eb;
    --accent-glow: rgba(37, 99, 235, 0.15);
    --accent-hover: #3b72f5;
    --accent-dim: rgba(37, 99, 235, 0.1);
    --success: #16a34a;
    --success-dim: rgba(22, 163, 74, 0.1);
    --danger: #dc2626;
    --danger-dim: rgba(220, 38, 38, 0.08);
}

*, *::before, *::after { box-sizing: border-box; }

html, body {
    margin: 0;
    padding: 0;
    height: 100%;
    background: var(--bg);
    color: var(--text);
    font-family: "Inter", system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    font-size: 14px;
    line-height: 1.5;
    -webkit-font-smoothing: antialiased;
}

/* ─── Layout ────────────────────────────────────────────────────────────── */

.app {
    display: flex;
    height: 100vh;
    overflow: hidden;
}

.sidebar {
    width: var(--sidebar-width);
    flex-shrink: 0;
    background: var(--bg-elevated);
    border-right: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    padding: 0;
    overflow: hidden;
}

.sidebar-header {
    padding: 1.25rem 1rem 1rem;
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    gap: 0.6rem;
}

.sidebar-logo {
    width: 28px;
    height: 28px;
    background: linear-gradient(135deg, var(--accent) 0%, #a855f7 100%);
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 15px;
    flex-shrink: 0;
    box-shadow: 0 2px 10px var(--accent-glow);
}

.sidebar-title {
    font-size: 0.9rem;
    font-weight: 600;
    letter-spacing: 0.01em;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin: 0;
}

.sidebar-nav {
    flex: 1;
    padding: 0.75rem 0.6rem;
    list-style: none;
    margin: 0;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 2px;
}

.nav-item {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    width: 100%;
    padding: 0.6rem 0.8rem;
    border: none;
    border-radius: var(--radius);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: background var(--transition), color var(--transition);
    font-size: 0.875rem;
    font-weight: 500;
    text-align: left;
}

.nav-item:hover {
    background: var(--bg-hover);
    color: var(--text);
}

.nav-item.active {
    background: var(--accent-dim);
    color: var(--accent);
}

.nav-icon {
    font-size: 1rem;
    width: 18px;
    text-align: center;
    flex-shrink: 0;
}

.sidebar-footer {
    padding: 0.75rem 0.6rem;
    border-top: 1px solid var(--border-subtle);
}

.content {
    flex: 1;
    min-width: 0;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    background: var(--bg);
}

/* ─── Page header ────────────────────────────────────────────────────────── */

.page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 1.75rem 0;
    margin-bottom: 1.25rem;
    flex-shrink: 0;
}

.page-title {
    font-size: 1.35rem;
    font-weight: 700;
    letter-spacing: -0.01em;
    color: var(--text);
    margin: 0;
}

.page-body {
    padding: 0 1.75rem 1.75rem;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    min-width: 0;
}

/* ─── Toolbar / search ───────────────────────────────────────────────────── */

.toolbar {
    display: flex;
    gap: 0.6rem;
    align-items: stretch;
}

.search-wrap {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
}

.search-icon {
    position: absolute;
    left: 0.75rem;
    color: var(--text-dim);
    font-size: 0.9rem;
    pointer-events: none;
}

.search-input {
    width: 100%;
    padding: 0.6rem 0.9rem 0.6rem 2.2rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg-input);
    color: var(--text);
    font-size: 0.875rem;
    outline: none;
    transition: border-color var(--transition), box-shadow var(--transition);
}

.search-input::placeholder { color: var(--text-dim); }

.search-input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-glow);
}

/* ─── Filters ────────────────────────────────────────────────────────────── */

.filter-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: flex-end;
    padding: 0.85rem 1rem;
    background: var(--bg-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
}

.filter-group {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    min-width: 130px;
}

.filter-label {
    font-size: 0.72rem;
    font-weight: 600;
    color: var(--text-dim);
    text-transform: uppercase;
    letter-spacing: 0.06em;
}

.filter-input,
.filter-select {
    padding: 0.45rem 0.65rem;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--bg-input);
    color: var(--text);
    font-size: 0.825rem;
    outline: none;
    transition: border-color var(--transition), box-shadow var(--transition);
    width: 100%;
}

.filter-input:focus,
.filter-select:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-glow);
}

.filter-select { appearance: none; cursor: pointer; }

.check-group {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
}

.check-label {
    display: flex;
    align-items: center;
    gap: 0.45rem;
    font-size: 0.825rem;
    color: var(--text-muted);
    cursor: pointer;
    user-select: none;
}

.check-label input { cursor: pointer; accent-color: var(--accent); }

/* ─── Buttons ────────────────────────────────────────────────────────────── */

.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    border: none;
    border-radius: var(--radius);
    padding: 0.55rem 1rem;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 500;
    transition: background var(--transition), box-shadow var(--transition), transform var(--transition), opacity var(--transition);
    white-space: nowrap;
    outline: none;
    line-height: 1;
}

.btn:focus-visible {
    box-shadow: 0 0 0 3px var(--accent-glow);
}

.btn-primary {
    background: var(--accent);
    color: #fff;
}
.btn-primary:hover {
    background: var(--accent-hover);
    box-shadow: 0 4px 14px var(--accent-glow);
    transform: translateY(-1px);
}

.btn-secondary {
    background: var(--bg-card);
    color: var(--text);
    border: 1px solid var(--border);
}
.btn-secondary:hover {
    background: var(--bg-hover);
    border-color: var(--accent);
    color: var(--accent);
}

.btn-ghost {
    background: transparent;
    color: var(--text-muted);
    border: 1px solid transparent;
}
.btn-ghost:hover {
    background: var(--bg-hover);
    color: var(--text);
    border-color: var(--border);
}

.btn-danger {
    background: var(--danger-dim);
    color: var(--danger);
    border: 1px solid transparent;
}
.btn-danger:hover {
    background: var(--danger);
    color: #fff;
}

.btn-success {
    background: var(--success-dim);
    color: var(--success);
    border: 1px solid transparent;
}

.btn-sm {
    padding: 0.38rem 0.75rem;
    font-size: 0.78rem;
    border-radius: 8px;
}

.btn-icon {
    padding: 0.5rem;
    border-radius: 8px;
}

.btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    transform: none !important;
    box-shadow: none !important;
}

/* ─── Cards ──────────────────────────────────────────────────────────────── */

.card-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 0.85rem;
}

.card {
    background: var(--bg-card);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    overflow: hidden;
    cursor: pointer;
    transition: transform var(--transition), box-shadow var(--transition), border-color var(--transition);
    position: relative;
}

.card:hover {
    transform: translateY(-4px);
    border-color: var(--border);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.35);
}

.card img {
    width: 100%;
    display: block;
    aspect-ratio: 16 / 10;
    object-fit: cover;
    background: var(--bg-input);
}

.card-meta {
    padding: 0.55rem 0.7rem;
    font-size: 0.75rem;
    color: var(--text-dim);
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.card-id {
    font-weight: 600;
    color: var(--text-muted);
}

.card-res {
    color: var(--text-dim);
}

.card-placeholder {
    width: 100%;
    aspect-ratio: 16 / 10;
    background: var(--bg-input);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-dim);
    font-size: 1.5rem;
}

/* ─── Detail panel ────────────────────────────────────────────────────────── */

.detail-panel {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius-xl);
    overflow: hidden;
    margin-bottom: 0.5rem;
}

.detail-top-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.85rem 1rem;
    border-bottom: 1px solid var(--border-subtle);
}

.detail-top-title {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text);
    margin: 0;
}

.detail-body {
    display: grid;
    grid-template-columns: 340px 1fr;
    gap: 0;
}

@media (max-width: 860px) {
    .detail-body { grid-template-columns: 1fr; }
}

.detail-image-wrap {
    position: relative;
    background: var(--bg-input);
}

.detail-image-wrap img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
    max-height: 320px;
}

.detail-image-failed {
    width: 100%;
    height: 220px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-dim);
    font-size: 0.875rem;
    gap: 0.5rem;
}

.detail-info {
    padding: 1.1rem 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.detail-meta-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 0.55rem;
}

.meta-chip {
    background: var(--bg-card);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    padding: 0.45rem 0.65rem;
}

.meta-chip-label {
    font-size: 0.68rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-dim);
    margin-bottom: 0.15rem;
}

.meta-chip-value {
    font-size: 0.82rem;
    color: var(--text);
    font-weight: 500;
    word-break: break-all;
}

.detail-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
}

.tag-pill {
    background: var(--bg-card);
    border: 1px solid var(--border-subtle);
    border-radius: 20px;
    padding: 0.22rem 0.6rem;
    font-size: 0.72rem;
    color: var(--text-muted);
    transition: background var(--transition), color var(--transition);
}

.tag-pill:hover {
    background: var(--accent-dim);
    color: var(--accent);
    border-color: var(--accent);
}

.detail-colors {
    display: flex;
    gap: 0.4rem;
    flex-wrap: wrap;
    align-items: center;
}

.color-dot {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 2px solid rgba(255,255,255,0.1);
    flex-shrink: 0;
}

.detail-actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    margin-top: auto;
    padding-top: 0.5rem;
    border-top: 1px solid var(--border-subtle);
}

/* ─── Download / Library items ────────────────────────────────────────────── */

.item-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.download-item {
    background: var(--bg-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    padding: 0.85rem 1rem;
    display: flex;
    align-items: center;
    gap: 0.85rem;
    transition: border-color var(--transition);
}

.download-item:hover {
    border-color: var(--border);
}

.download-item-icon {
    width: 38px;
    height: 38px;
    border-radius: 10px;
    background: var(--bg-card);
    border: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.1rem;
    flex-shrink: 0;
}

.download-item-body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
}

.download-item-name {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.download-item-meta {
    font-size: 0.775rem;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.download-item-actions {
    display: flex;
    gap: 0.4rem;
    flex-shrink: 0;
}

/* ─── Progress bar ────────────────────────────────────────────────────────── */

.progress-wrap {
    height: 4px;
    background: var(--bg-card);
    border-radius: 99px;
    overflow: hidden;
    margin-top: 0.35rem;
}

.progress-bar {
    height: 100%;
    border-radius: 99px;
    background: linear-gradient(90deg, var(--accent), #a855f7);
    transition: width 0.3s ease;
}

/* ─── Status badge ────────────────────────────────────────────────────────── */

.badge {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    padding: 0.18rem 0.55rem;
    border-radius: 99px;
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 0.02em;
}

.badge-queued { background: var(--bg-card); color: var(--text-dim); border: 1px solid var(--border-subtle); }
.badge-downloading { background: var(--accent-dim); color: var(--accent); }
.badge-completed { background: var(--success-dim); color: var(--success); }
.badge-failed { background: var(--danger-dim); color: var(--danger); }
.badge-cancelled { background: var(--warning-dim); color: var(--warning); }

/* ─── Favorites ────────────────────────────────────────────────────────────── */

.fav-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(230px, 1fr));
    gap: 0.85rem;
}

.fav-card {
    background: var(--bg-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    overflow: hidden;
    transition: transform var(--transition), box-shadow var(--transition), border-color var(--transition);
}

.fav-card:hover {
    transform: translateY(-3px);
    box-shadow: 0 10px 28px rgba(0,0,0,0.3);
    border-color: var(--border);
}

.fav-card-thumb {
    width: 100%;
    aspect-ratio: 16/9;
    object-fit: cover;
    display: block;
    background: var(--bg-input);
}

.fav-card-body {
    padding: 0.65rem 0.8rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
}

.fav-card-info {
    min-width: 0;
}

.fav-card-id {
    font-size: 0.825rem;
    font-weight: 600;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.fav-card-res {
    font-size: 0.73rem;
    color: var(--text-dim);
}

/* ─── Pagination ──────────────────────────────────────────────────────────── */

.pager {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    padding: 1rem 0 0.5rem;
}

.pager-info {
    font-size: 0.8rem;
    color: var(--text-muted);
    min-width: 110px;
    text-align: center;
}

/* ─── States ─────────────────────────────────────────────────────────────── */

.state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    padding: 4rem 2rem;
    color: var(--text-muted);
    text-align: center;
}

.state-icon {
    font-size: 2.5rem;
    opacity: 0.6;
}

.state-title {
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--text-muted);
}

.state-desc {
    font-size: 0.825rem;
    color: var(--text-dim);
    max-width: 300px;
}

/* ─── Settings ────────────────────────────────────────────────────────────── */

.settings-layout {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    max-width: 720px;
}

.settings-section {
    background: var(--bg-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-xl);
    overflow: hidden;
}

.settings-section-header {
    padding: 0.9rem 1.25rem;
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    gap: 0.6rem;
}

.settings-section-icon {
    font-size: 1rem;
    width: 24px;
    text-align: center;
    color: var(--accent);
}

.settings-section-title {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text);
    margin: 0;
}

.settings-section-body {
    padding: 1rem 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
}

.settings-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
}

.settings-field {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    flex: 1;
}

.field-label {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--text-muted);
}

.field-hint {
    font-size: 0.73rem;
    color: var(--text-dim);
}

.text-input {
    padding: 0.6rem 0.85rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg-input);
    color: var(--text);
    font-size: 0.875rem;
    outline: none;
    width: 100%;
    transition: border-color var(--transition), box-shadow var(--transition);
}

.text-input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-glow);
}

.text-input::placeholder { color: var(--text-dim); }

.key-display {
    display: flex;
    align-items: center;
    gap: 0.65rem;
    padding: 0.55rem 0.85rem;
    background: var(--bg-card);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius);
    font-size: 0.825rem;
    color: var(--text-muted);
    font-family: monospace;
}

.key-dot { color: var(--success); font-size: 0.7rem; }

.inline-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
}

.check-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
    gap: 0.5rem;
}

.check-pill {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.45rem 0.75rem;
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius);
    cursor: pointer;
    font-size: 0.825rem;
    color: var(--text-muted);
    transition: background var(--transition), border-color var(--transition), color var(--transition);
    user-select: none;
    background: var(--bg-card);
}

.check-pill:has(input:checked) {
    background: var(--accent-dim);
    border-color: var(--accent);
    color: var(--accent);
}

.check-pill input { accent-color: var(--accent); cursor: pointer; }

.settings-message {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.65rem 1rem;
    background: var(--success-dim);
    border: 1px solid var(--success);
    border-radius: var(--radius);
    font-size: 0.825rem;
    color: var(--success);
}

.settings-message.error {
    background: var(--danger-dim);
    border-color: var(--danger);
    color: var(--danger);
}

/* ─── Modal ────────────────────────────────────────────────────────────────── */

.modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius-xl);
    padding: 1.5rem;
    max-width: 440px;
    width: calc(100% - 2rem);
    box-shadow: 0 24px 64px rgba(0, 0, 0, 0.5);
}

.modal-title {
    font-size: 1rem;
    font-weight: 700;
    color: var(--text);
    margin: 0 0 0.5rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.modal-body {
    font-size: 0.875rem;
    color: var(--text-muted);
    margin-bottom: 1.25rem;
    line-height: 1.6;
}

.modal-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
}

/* ─── Detail modal (popup wallpaper) ────────────────────────────────────────── */

.detail-overlay {
    align-items: center;
    justify-content: center;
    padding: 1.5rem;
    overflow-y: auto;
}

.detail-modal {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius-xl);
    box-shadow: 0 32px 80px rgba(0, 0, 0, 0.6);
    max-width: 900px;
    width: 100%;
    max-height: calc(100vh - 3rem);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    padding: 0;
    /* Override generic .modal sizing */
    padding: 0 !important;
    max-width: 900px !important;
}

.detail-modal .detail-top-bar {
    padding: 0.85rem 1.25rem;
    background: var(--bg-elevated);
    border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
}

.detail-modal .detail-body {
    grid-template-columns: 380px 1fr;
    overflow: hidden;
    flex: 1;
    min-height: 0;
}

@media (max-width: 860px) {
    .detail-modal .detail-body {
        grid-template-columns: 1fr;
        overflow-y: auto;
    }
}

.detail-modal .detail-image-wrap {
    background: #000;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 260px;
    max-height: 560px;
    overflow: hidden;
}

.detail-modal .detail-image-wrap img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    max-height: 560px;
    display: block;
}

.detail-modal .detail-info {
    overflow-y: auto;
    padding: 1.25rem;
    max-height: 560px;
}

.detail-modal .state {
    padding: 3rem 2rem;
    min-height: 200px;
}

/* ─── Radio pill (filter resolusi, rasio, warna) ─────────────────────────── */

.radio-group {
    display: flex;
    flex-wrap: wrap;
    gap: 0.3rem;
}

.radio-pill {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.3rem 0.65rem;
    border: 1px solid var(--border);
    border-radius: 99px;
    cursor: pointer;
    font-size: 0.78rem;
    color: var(--text-muted);
    background: var(--bg-card);
    transition: background var(--transition), border-color var(--transition), color var(--transition);
    user-select: none;
    white-space: nowrap;
}

.radio-pill:has(input:checked) {
    background: var(--accent-dim);
    border-color: var(--accent);
    color: var(--accent);
}

.radio-pill input[type="radio"] {
    display: none;
}

/* Kelompok warna — pill berbentuk lingkaran kecil */

.color-radio-group {
    flex-wrap: wrap;
    gap: 0.35rem;
    align-items: center;
}

.color-radio-pill {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    border: 2px solid transparent;
    border-radius: 50%;
    cursor: pointer;
    transition: border-color var(--transition), transform var(--transition);
}

.color-radio-pill:hover {
    transform: scale(1.15);
}

.color-radio-pill--selected {
    border-color: var(--accent);
    transform: scale(1.18);
    box-shadow: 0 0 0 2px var(--accent-glow);
}

.color-radio-pill input[type="radio"] {
    display: none;
}

.color-swatch {
    display: block;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 1px solid rgba(255,255,255,0.12);
}

/* Tombol aksi filter (Terapkan + Reset) */

.filter-actions {
    display: flex;
    align-items: flex-end;
    gap: 0.4rem;
    margin-left: auto;
}



::-webkit-scrollbar { width: 6px; height: 6px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb { background: var(--border); border-radius: 99px; }
::-webkit-scrollbar-thumb:hover { background: var(--text-dim); }

/* ─── Divider ─────────────────────────────────────────────────────────────── */

.divider {
    height: 1px;
    background: var(--border-subtle);
    margin: 0.25rem 0;
}

/* ─── Loading spinner ─────────────────────────────────────────────────────── */

@keyframes spin { to { transform: rotate(360deg); } }

.spinner {
    width: 28px;
    height: 28px;
    border: 3px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
}

/* ─── Utility ─────────────────────────────────────────────────────────────── */

.truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.status-ok  { color: var(--success); }
.status-error { color: var(--danger); }
"#;
