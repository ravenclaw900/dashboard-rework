const term = new Terminal();
term.open(document.getElementById('terminal'));

const fitAddon = new FitAddon.FitAddon();
term.loadAddon(fitAddon);
fitAddon.fit();

const origin = location.origin.replace("http", "ws");
const socket = new WebSocket(`${origin}/api/terminal`);
const attachAddon = new AttachAddon.AttachAddon(socket);

term.loadAddon(attachAddon);

const sendSize = () => socket.send(`size${term.cols},${term.rows}`);

// Set initial terminal size once connected
socket.addEventListener("open", sendSize);

addEventListener("resize", () => fitAddon.fit());

// Terminal onResize event only fires when the actual number of columns/rows changes
term.onResize(sendSize);