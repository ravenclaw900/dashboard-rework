const term = new Terminal();
term.open(document.getElementById('terminal'));

const fitAddon = new FitAddon.FitAddon();
term.loadAddon(fitAddon);
fitAddon.fit();

const origin = location.origin.replace("http", "ws");
const socket = new WebSocket(`${origin}/api/terminal`);
const attachAddon = new AttachAddon.AttachAddon(socket);

term.loadAddon(attachAddon);

addEventListener("resize", () => fitAddon.fit());

// Terminal onResize event only fires when the actual number of columns/rows changes
term.onResize(() => socket.send(`size${term.cols},${term.rows}`));