const term = new Terminal();
term.open(document.getElementById('terminal'));

const scheme = location.protocol == "http:" ? "ws" : "wss";
const socket = new WebSocket(`${scheme}://localhost:5252/api/terminal`);
const attachAddon = new AttachAddon.AttachAddon(socket);

term.loadAddon(attachAddon);