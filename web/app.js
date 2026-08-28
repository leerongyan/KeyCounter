const KEY_LABELS = {
  backspace: "Backspace",
  capslock: "Caps",
  meta: "Win",
  alt_gr: "AltGr",
  menu: "Menu",
  print_screen: "PrtSc",
  scroll_lock: "ScrLk",
  page_up: "PgUp",
  page_down: "PgDn",
  num_lock: "NumLk",
  divide: "/",
  multiply: "*",
  subtract: "-",
  add: "+",
  decimal: ".",
  enter: "Enter",
  escape: "Esc",
};

const KEYBOARD_ROWS = [
  [
    { id: "esc", label: "Esc", width: 1 },
    { id: "f1", label: "F1", width: 1 },
    { id: "f2", label: "F2", width: 1 },
    { id: "f3", label: "F3", width: 1 },
    { id: "f4", label: "F4", width: 1 },
    { id: "f5", label: "F5", width: 1 },
    { id: "f6", label: "F6", width: 1 },
    { id: "f7", label: "F7", width: 1 },
    { id: "f8", label: "F8", width: 1 },
    { id: "f9", label: "F9", width: 1 },
    { id: "f10", label: "F10", width: 1 },
    { id: "f11", label: "F11", width: 1 },
    { id: "f12", label: "F12", width: 1 },
  ],
  [
    { id: "`", label: "`", width: 1 },
    { id: "1", label: "1", width: 1 },
    { id: "2", label: "2", width: 1 },
    { id: "3", label: "3", width: 1 },
    { id: "4", label: "4", width: 1 },
    { id: "5", label: "5", width: 1 },
    { id: "6", label: "6", width: 1 },
    { id: "7", label: "7", width: 1 },
    { id: "8", label: "8", width: 1 },
    { id: "9", label: "9", width: 1 },
    { id: "0", label: "0", width: 1 },
    { id: "-", label: "-", width: 1 },
    { id: "=", label: "=", width: 1 },
    { id: "backspace", label: "Backspace", width: 2 },
  ],
  [
    { id: "tab", label: "Tab", width: 1.5 },
    { id: "q", label: "Q", width: 1 },
    { id: "w", label: "W", width: 1 },
    { id: "e", label: "E", width: 1 },
    { id: "r", label: "R", width: 1 },
    { id: "t", label: "T", width: 1 },
    { id: "y", label: "Y", width: 1 },
    { id: "u", label: "U", width: 1 },
    { id: "i", label: "I", width: 1 },
    { id: "o", label: "O", width: 1 },
    { id: "p", label: "P", width: 1 },
    { id: "[", label: "[", width: 1 },
    { id: "]", label: "]", width: 1 },
    { id: "\\", label: "\\", width: 1.5 },
  ],
  [
    { id: "capslock", label: "Caps", width: 1.75 },
    { id: "a", label: "A", width: 1 },
    { id: "s", label: "S", width: 1 },
    { id: "d", label: "D", width: 1 },
    { id: "f", label: "F", width: 1 },
    { id: "g", label: "G", width: 1 },
    { id: "h", label: "H", width: 1 },
    { id: "j", label: "J", width: 1 },
    { id: "k", label: "K", width: 1 },
    { id: "l", label: "L", width: 1 },
    { id: ";", label: ";", width: 1 },
    { id: "'", label: "'", width: 1 },
    { id: "enter", label: "Enter", width: 2.25 },
  ],
  [
    { id: "shift", label: "Shift", width: 2.25 },
    { id: "z", label: "Z", width: 1 },
    { id: "x", label: "X", width: 1 },
    { id: "c", label: "C", width: 1 },
    { id: "v", label: "V", width: 1 },
    { id: "b", label: "B", width: 1 },
    { id: "n", label: "N", width: 1 },
    { id: "m", label: "M", width: 1 },
    { id: ",", label: ",", width: 1 },
    { id: ".", label: ".", width: 1 },
    { id: "/", label: "/", width: 1 },
    { id: "shift", label: "Shift", width: 2.75 },
  ],
  [
    { id: "ctrl", label: "Ctrl", width: 1.5 },
    { id: "meta", label: "Win", width: 1.25 },
    { id: "alt", label: "Alt", width: 1.25 },
    { id: "space", label: "Space", width: 6.25 },
    { id: "alt_gr", label: "AltGr", width: 1.25 },
    { id: "menu", label: "Menu", width: 1.25 },
    { id: "ctrl", label: "Ctrl", width: 1.5 },
  ],
];

const NUMPAD_KEYS = [
  { id: "num_lock", label: "NumLk", row: 1, col: 1 },
  { id: "divide", label: "/", row: 1, col: 2 },
  { id: "multiply", label: "*", row: 1, col: 3 },
  { id: "subtract", label: "-", row: 1, col: 4 },
  { id: "7", label: "7", row: 2, col: 1 },
  { id: "8", label: "8", row: 2, col: 2 },
  { id: "9", label: "9", row: 2, col: 3 },
  { id: "add", label: "+", row: 2, col: 4, rowSpan: 2 },
  { id: "4", label: "4", row: 3, col: 1 },
  { id: "5", label: "5", row: 3, col: 2 },
  { id: "6", label: "6", row: 3, col: 3 },
  { id: "1", label: "1", row: 4, col: 1 },
  { id: "2", label: "2", row: 4, col: 2 },
  { id: "3", label: "3", row: 4, col: 3 },
  { id: "enter", label: "Enter", row: 4, col: 4, rowSpan: 2 },
  { id: "0", label: "0", row: 5, col: 1, colSpan: 2 },
  { id: "decimal", label: ".", row: 5, col: 3 },
];

const state = {
  paused: false,
};

const statusText = document.getElementById("status-text");
const liveIndicator = document.getElementById("live-indicator");
const pauseButton = document.getElementById("pause-button");
const autostartButton = document.getElementById("autostart-button");

function createKeyButton(key) {
  const el = document.createElement("div");
  el.className = "key";
  el.dataset.key = key.id;
  el.style.flex = `${key.width || 1} 1 0`;

  const hint = document.createElement("span");
  hint.className = "key-hint";
  hint.textContent = labelFor(key.id, key.label);
  el.appendChild(hint);
  return el;
}

function labelFor(id, fallback) {
  return KEY_LABELS[id] || fallback || id;
}

function buildKeyboard() {
  const keyboard = document.getElementById("keyboard");
  const main = document.createElement("div");
  main.className = "kb-main";

  for (const row of KEYBOARD_ROWS) {
    const rowEl = document.createElement("div");
    rowEl.className = "kb-row";
    for (const key of row) {
      rowEl.appendChild(createKeyButton(key));
    }
    main.appendChild(rowEl);
  }

  const numpad = document.createElement("div");
  numpad.className = "numpad";
  for (const key of NUMPAD_KEYS) {
    const el = createKeyButton(key);
    el.style.flex = "";
    el.style.gridRowStart = key.row;
    el.style.gridColumnStart = key.col;
    if (key.rowSpan) {
      el.style.gridRowEnd = `span ${key.rowSpan}`;
    }
    if (key.colSpan) {
      el.style.gridColumnEnd = `span ${key.colSpan}`;
    }
    numpad.appendChild(el);
  }

  keyboard.append(main, numpad);
}

function formatNumber(value) {
  return new Intl.NumberFormat("zh-CN").format(value || 0);
}

function formatDistance(px) {
  const meters = px / 3779.527;
  if (meters >= 1) {
    return `${meters.toFixed(2)} m`;
  }
  return `${Math.round(px).toLocaleString("zh-CN")} px`;
}

function escapeHtml(value) {
  return String(value).replace(/[&<>"']/g, (char) => ({
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
    "'": "&#39;",
  }[char]));
}

async function fetchJSON(url, options) {
  const response = await fetch(url, options);
  if (!response.ok) {
    throw new Error(`${response.status} ${response.statusText}`);
  }
  return response.json();
}

function renderPause(paused) {
  state.paused = paused;
  pauseButton.textContent = paused ? "继续统计" : "暂停统计";
  pauseButton.classList.toggle("paused", paused);
  liveIndicator.classList.toggle("paused", paused);
  liveIndicator.textContent = paused ? "已暂停" : "运行中";
}

function renderAutostart(enabled) {
  autostartButton.textContent = enabled ? "开机自启已开" : "开启开机自启";
  autostartButton.classList.toggle("enabled", enabled);
}

function renderSummary(summary) {
  document.getElementById("stat-keys").textContent = formatNumber(summary.keys);
  document.getElementById("stat-clicks").textContent = formatNumber(summary.clicks);
  document.getElementById("stat-scrolls").textContent = formatNumber(summary.scrolls);
  document.getElementById("stat-distance").textContent = formatDistance(summary.distance_px);

  const mouse = {};
  for (const item of summary.mouse) {
    mouse[item.button || item.event_type] = item.count;
  }

  const sideCount = (mouse.x1 || 0) + (mouse.x2 || 0);
  document.getElementById("mouse-left").textContent = formatNumber(mouse.left);
  document.getElementById("mouse-right").textContent = formatNumber(mouse.right);
  document.getElementById("mouse-middle").textContent = formatNumber(mouse.middle);
  document.getElementById("mouse-side").textContent = formatNumber(sideCount);
  document.getElementById("mouse-up").textContent = formatNumber(mouse.up);
  document.getElementById("mouse-down").textContent = formatNumber(mouse.down);
  document.getElementById("mouse-distance").textContent = formatDistance(summary.distance_px);

  const topList = document.getElementById("top-list");
  if (!summary.top_keys.length) {
    topList.innerHTML = '<li class="empty-note">还没有按键数据</li>';
  } else {
    const maxCount = Math.max(1, ...summary.top_keys.map((item) => item.count));
    topList.innerHTML = summary.top_keys
      .map((item, index) => {
        const width = Math.max(8, Math.round((item.count / maxCount) * 100));
        return `
          <li>
            <span>${index + 1}</span>
            <span class="top-key-name">${escapeHtml(item.key_name)}</span>
            <span class="top-key-bar" style="width: ${width}%;"></span>
            <span class="top-key-count">${formatNumber(item.count)}</span>
          </li>
        `;
      })
      .join("");
  }

  renderPause(summary.paused);
  renderAutostart(summary.autostart_enabled);
  statusText.textContent = summary.paused
    ? "统计已暂停"
    : `统计中 · 更新于 ${new Date().toLocaleTimeString("zh-CN")}`;
}

function renderHeatmap(payload) {
  const counts = {};
  for (const item of payload.keys) {
    counts[item.key_name] = item.count;
  }
  const maxCount = Math.max(1, ...Object.values(counts));
  const keys = document.querySelectorAll(".key");

  for (const el of keys) {
    const keyName = el.dataset.key;
    const count = counts[keyName] || counts[keyName.toUpperCase()] || 0;
    const heat = count > 0 ? 0.12 + 0.82 * (count / maxCount) : 0;
    el.style.setProperty("--heat", heat.toFixed(3));
    el.title = count > 0 ? `${labelFor(keyName, keyName)}: ${formatNumber(count)} 次` : "";
  }
}

function renderTrend(payload) {
  const hours = payload.hours || [];
  const maxKeys = Math.max(1, ...hours.map((item) => item.keys));
  const maxClicks = Math.max(1, ...hours.map((item) => item.clicks));
  const chart = document.getElementById("trend-chart");

  chart.innerHTML = hours
    .map((item) => {
      const keysHeight = Math.max(item.keys > 0 ? 2 : 0, (item.keys / maxKeys) * 100);
      const clicksHeight = Math.max(item.clicks > 0 ? 2 : 0, (item.clicks / maxClicks) * 100);
      return `
        <div class="trend-col" title="${item.hour}:00  按键 ${item.keys}  点击 ${item.clicks}">
          <div class="trend-bars">
            <div class="trend-bar keys" style="height:${keysHeight}%"></div>
            <div class="trend-bar clicks" style="height:${clicksHeight}%"></div>
          </div>
          <span class="trend-hour">${item.hour}</span>
        </div>
      `;
    })
    .join("");
}

async function refresh() {
  try {
    const [summary, heatmap, trend] = await Promise.all([
      fetchJSON("/api/summary"),
      fetchJSON("/api/heatmap"),
      fetchJSON("/api/trend"),
    ]);
    renderSummary(summary);
    renderHeatmap(heatmap);
    renderTrend(trend);
  } catch (error) {
    statusText.textContent = "无法连接统计服务";
    liveIndicator.classList.add("paused");
    liveIndicator.textContent = "离线";
  }
}

pauseButton.addEventListener("click", async () => {
  try {
    const result = await fetchJSON("/api/pause", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ paused: !state.paused }),
    });
    renderPause(result.paused);
  } catch (error) {
    statusText.textContent = "暂停操作失败";
  }
});

autostartButton.addEventListener("click", async () => {
  const enabled = autostartButton.classList.contains("enabled");
  try {
    const result = await fetchJSON("/api/autostart", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ enabled: !enabled }),
    });
    renderAutostart(result.enabled);
  } catch (error) {
    statusText.textContent = "开机自启设置失败";
  }
});

buildKeyboard();
refresh();
setInterval(refresh, 5000);
