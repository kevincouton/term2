async function startTerm2() {
  const params = new URLSearchParams(window.location.search);
  const sessionId = params.get('id');

  const term = new Terminal({
    cursorBlink: true,
    fontFamily: 'JetBrains Mono, Fira Code, monospace',
    theme: { background: '#0c0c0c', foreground: '#cccccc' },
  });

  const fitAddon = new FitAddon.FitAddon();
  term.loadAddon(fitAddon);
  term.open(document.getElementById('terminal'));
  fitAddon.fit();

  window.addEventListener('resize', () => fitAddon.fit());

  if (!sessionId) {
    term.writeln('\x1b[31mNo session id provided.\x1b[0m');
    term.writeln('\x1b[90mReturn to the portal to create or select a session.\x1b[0m');
    return;
  }

  term.writeln('\x1b[90mConnecting to session…\x1b[0m');

  // Hidden log of raw terminal output for E2E assertions.
  const e2eLog = document.createElement('pre');
  e2eLog.id = 'term2-e2e-log';
  e2eLog.style.display = 'none';
  e2eLog.setAttribute('aria-hidden', 'true');
  document.body.appendChild(e2eLog);

  const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
  const ws = new WebSocket(`${proto}//${location.host}/api/v1/sessions/${encodeURIComponent(sessionId)}/ws`);
  ws.binaryType = 'arraybuffer';

  ws.onopen = () => {
    term.writeln('\x1b[32mSession connected.\x1b[0m');
    e2eLog.textContent += '\n[connected]\n';
  };

  ws.onmessage = (event) => {
    const data = new Uint8Array(event.data);
    term.write(data);
    // Strip common ANSI escape sequences for easier text assertions.
    e2eLog.textContent += new TextDecoder()
      .decode(data)
      .replace(/\x1b\[[0-9;]*[a-zA-Z]/g, '');
  };

  ws.onclose = () => {
    term.writeln('\x1b[31mSession closed.\x1b[0m');
  };

  ws.onerror = (err) => {
    term.writeln(`\x1b[31mWebSocket error: ${err.type}\x1b[0m`);
  };

  term.onData((data) => {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(data);
    }
  });
}

startTerm2().catch(console.error);
