// Error tracking (GlitchTip, Sentry-compatible). Loads @sentry/browser from
// the CDN only when the server reports a DSN via /api/v1/config; otherwise
// this is a no-op so dev stays clean.
(async () => {
  try {
    const res = await fetch('/api/v1/config');
    if (!res.ok) return;
    const { sentryDsn } = await res.json();
    if (!sentryDsn) return;

    const script = document.createElement('script');
    script.src = 'https://cdn.jsdelivr.net/npm/@sentry/browser@8/build/bundle.min.js';
    script.crossOrigin = 'anonymous';
    script.onload = () => {
      if (window.Sentry) {
        window.Sentry.init({ dsn: sentryDsn });
      }
    };
    document.head.appendChild(script);
  } catch {
    // Error tracking must never break the app.
  }
})();
