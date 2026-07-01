async function startTerm2() {
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

  term.writeln('\x1b[90mConnecting to Term2...\x1b[0m');

  const response = await fetch('/api/v1/sessions', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ command: '/bin/sh' }),
  });

  if (!response.ok) {
    term.writeln('\x1b[31mFailed to create session.\x1b[0m');
    return;
  }

  const { ws_url } = await response.json();
  const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
  const ws = new WebSocket(`${proto}//${location.host}${ws_url}`);
  ws.binaryType = 'arraybuffer';

  ws.onopen = () => {
    term.writeln('\x1b[32mSession connected.\x1b[0m');
  };

  ws.onmessage = (event) => {
    term.write(new Uint8Array(event.data));
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
