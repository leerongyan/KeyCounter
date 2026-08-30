const KEY_LABELS = {
  backspace: "Backspace",
  capslock: "Caps",
  meta: "Win",
  alt_gr: "AltGr",
  menu: "Menu",
  right: "→",
  left: "←",
  up: "↑",
  down: "↓",
  insert: "Ins",
  home: "Home",
  end: "End",
  delete: "Del",
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
    { id: "esc", label: "Esc", width: 1 }, { id: "f1", label: "F1", width: 1 },
    { id: "f2", label: "F2", width: 1 }, { id: "f3", label: "F3", width: 1 },
    { id: "f4", label: "F4", width: 1 }, { id: "f5", label: "F5", width: 1 },
    { id: "f6", label: "F6", width: 1 }, { id: "f7", label: "F7", width: 1 },
    { id: "f8", label: "F8", width: 1 }, { id: "f9", label: "F9", width: 1 },
    { id: "f10", label: "F10", width: 1 }, { id: "f11", label: "F11", width: 1 },
    { id: "f12", label: "F12", width: 1 },
  ],
  [
    { id: "`", label: "`", width: 1 }, { id: "1", label: "1", width: 1 },
    { id: "2", label: "2", width: 1 }, { id: "3", label: "3", width: 1 },
    { id: "4", label: "4", width: 1 }, { id: "5", label: "5", width: 1 },
    { id: "6", label: "6", width: 1 }, { id: "7", label: "7", width: 1 },
    { id: "8", label: "8", width: 1 }, { id: "9", label: "9", width: 1 },
    { id: "0", label: "0", width: 1 }, { id: "-", label: "-", width: 1 },
    { id: "=", label: "=", width: 1 }, { id: "backspace", label: "Backspace", width: 2 },
  ],
  [
    { id: "tab", label: "Tab", width: 1.5 }, { id: "q", label: "Q", width: 1 },
    { id: "w", label: "W", width: 1 }, { id: "e", label: "E", width: 1 },
    { id: "r", label: "R", width: 1 }, { id: "t", label: "T", width: 1 },
    { id: "y", label: "Y", width: 1 }, { id: "u", label: "U", width: 1 },
    { id: "i", label: "I", width: 1 }, { id: "o", label: "O", width: 1 },
    { id: "p", label: "P", width: 1 }, { id: "[", label: "[", width: 1 },
    { id: "]", label: "]", width: 1 }, { id: "\\", label: "\\", width: 1.5 },
  ],
  [
    { id: "capslock", label: "Caps", width: 1.75 }, { id: "a", label: "A", width: 1 },
    { id: "s", label: "S", width: 1 }, { id: "d", label: "D", width: 1 },
    { id: "f", label: "F", width: 1 }, { id: "g", label: "G", width: 1 },
    { id: "h", label: "H", width: 1 }, { id: "j", label: "J", width: 1 },
    { id: "k", label: "K", width: 1 }, { id: "l", label: "L", width: 1 },
    { id: ";", label: ";", width: 1 }, { id: "'", label: "'", width: 1 },
    { id: "enter", label: "Enter", width: 2.25 },
  ],
  [
    { id: "shift", label: "Shift", width: 2.25 }, { id: "z", label: "Z", width: 1 },
    { id: "x", label: "X", width: 1 }, { id: "c", label: "C", width: 1 },
    { id: "v", label: "V", width: 1 }, { id: "b", label: "B", width: 1 },
    { id: "n", label: "N", width: 1 }, { id: "m", label: "M", width: 1 },
    { id: ",", label: ",", width: 1 }, { id: ".", label: ".", width: 1 },
    { id: "/", label: "/", width: 1 }, { id: "shift", label: "Shift", width: 2.75 },
  ],
  [
    { id: "ctrl", label: "Ctrl", width: 1.5 }, { id: "meta", label: "Win", width: 1.25 },
    { id: "alt", label: "Alt", width: 1.25 }, { id: "space", label: "Space", width: 6.25 },
    { id: "alt_gr", label: "AltGr", width: 1.25 }, { id: "menu", label: "Menu", width: 1.25 },
    { id: "ctrl", label: "Ctrl", width: 1.5 },
  ],
];

const NUMPAD_KEYS = [
  { id: "num_lock", label: "NumLk", row: 1, col: 1 },
  { id: "numpad_divide", label: "/", row: 1, col: 2 },
  { id: "numpad_multiply", label: "*", row: 1, col: 3 },
  { id: "numpad_subtract", label: "-", row: 1, col: 4 },
  { id: "numpad7", label: "7", row: 2, col: 1 },
  { id: "numpad8", label: "8", row: 2, col: 2 },
  { id: "numpad9", label: "9", row: 2, col: 3 },
  { id: "numpad_add", label: "+", row: 2, col: 4, rowSpan: 2 },
  { id: "numpad4", label: "4", row: 3, col: 1 },
  { id: "numpad5", label: "5", row: 3, col: 2 },
  { id: "numpad6", label: "6", row: 3, col: 3 },
  { id: "numpad1", label: "1", row: 4, col: 1 },
  { id: "numpad2", label: "2", row: 4, col: 2 },
  { id: "numpad3", label: "3", row: 4, col: 3 },
  { id: "enter", label: "Enter", row: 4, col: 4, rowSpan: 2 },
  { id: "numpad0", label: "0", row: 5, col: 1, colSpan: 2 },
  { id: "numpad_decimal", label: ".", row: 5, col: 3 },
];

const NAV_KEYS = [
  { id: "insert", label: "Ins", row: 1, col: 1 },
  { id: "home", label: "Home", row: 1, col: 2 },
  { id: "page_up", label: "PgUp", row: 1, col: 3 },
  { id: "delete", label: "Del", row: 2, col: 1 },
  { id: "end", label: "End", row: 2, col: 2 },
  { id: "page_down", label: "PgDn", row: 2, col: 3 },
  { id: "up", label: "↑", row: 4, col: 2 },
  { id: "left", label: "←", row: 5, col: 1 },
  { id: "down", label: "↓", row: 5, col: 2 },
  { id: "right", label: "→", row: 5, col: 3 },
];

const state = {
  paused: false,
  summary: null,
  unit: localStorage.getItem("distance-unit") || "auto",
  range: { mode: "today", start: "", end: "" },
};

const statusText = document.getElementById("status-text");
const liveIndicator = document.getElementById("live-indicator");
const pauseButton = document.getElementById("pause-button");
const autostartButton = document.getElementById("autostart-button");
const unitSelect = document.getElementById("distance-unit");
const rangeSelect = document.getElementById("range-select");
const rangeStart = document.getElementById("range-start");
const rangeEnd = document.getElementById("range-end");
const closeActionSelect = document.getElementById("close-action");
const keyboardEl = document.getElementById("keyboard");
const keyboardHolder = document.getElementById("keyboard-holder");
const keyboardStage = document.querySelector(".keyboard-stage");
const mainEl = document.querySelector("main");
const workspaceEl = document.querySelector(".workspace");
const statStripEl = document.querySelector(".stat-strip");
const trendPanelEl = document.querySelector(".trend-panel");
const heatmapPanelEl = document.querySelector(".heatmap-panel");
const mousePanelEl = document.querySelector(".mouse-panel");
// 与 style.css 中 .keyboard-stage 的 aspect-ratio 保持一致（键盘自然宽高比）
const KEYBOARD_RATIO = 1160 / 270;
// 单列紧凑布局断点，与 style.css 的 @media (max-width: 900px) 保持一致
const COMPACT_BREAKPOINT = 900;

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
  const main = document.createElement("div");
  main.className = "kb-main";
  for (const row of KEYBOARD_ROWS) {
    const rowEl = document.createElement("div");
    rowEl.className = "kb-row";
    for (const key of row) rowEl.appendChild(createKeyButton(key));
    main.appendChild(rowEl);
  }
  const numpad = document.createElement("div");
  numpad.className = "numpad";
  for (const key of NUMPAD_KEYS) {
    const el = createKeyButton(key);
    el.style.flex = "";
    el.style.gridRowStart = key.row;
    el.style.gridColumnStart = key.col;
    if (key.rowSpan) el.style.gridRowEnd = `span ${key.rowSpan}`;
    if (key.colSpan) el.style.gridColumnEnd = `span ${key.colSpan}`;
    numpad.appendChild(el);
  }
  const nav = document.createElement("div");
  nav.className = "nav-cluster";
  for (const key of NAV_KEYS) {
    const el = createKeyButton(key);
    el.style.flex = "";
    el.style.gridRowStart = key.row;
    el.style.gridColumnStart = key.col;
    nav.appendChild(el);
  }
  keyboardEl.append(main, nav, numpad);
}

function fitKeyboard() {
  if (!keyboardEl || !keyboardHolder || !keyboardStage) return;
  const naturalW = keyboardEl.scrollWidth || 1160;
  const naturalH = keyboardEl.scrollHeight || 270;
  const availableW = keyboardStage.clientWidth || 620;
  const availableH = keyboardStage.clientHeight || 260;
  const scale = Math.max(0.2, Math.min(availableW / naturalW, availableH / naturalH, 3.2));
  keyboardHolder.style.setProperty("--kb-scale", scale.toFixed(3));
}

let lastWorkspaceHeight = 0;

function fitWorkspace() {
  if (!workspaceEl || !heatmapPanelEl || !mainEl) return;
  if (window.innerWidth <= COMPACT_BREAKPOINT) {
    if (lastWorkspaceHeight !== 0) {
      workspaceEl.style.height = "";
      lastWorkspaceHeight = 0;
      if (statStripEl) requestAnimationFrame(fitKeyboard);
    }
    return;
  }
  const mainStyles = getComputedStyle(mainEl);
  const mainInner = mainEl.clientHeight
    - parseFloat(mainStyles.paddingTop || 0)
    - parseFloat(mainStyles.paddingBottom || 0);
  const gap = 10;
  const trendReserve = 130;
  const available = Math.max(
    mainInner - statStripEl.offsetHeight - 2 * gap - trendReserve,
    260,
  );
  // 键盘按宽度自适应时热力图面板的理想高度 = 键盘高度 + 面板头尾开销
  const chrome = heatmapPanelEl.offsetHeight - keyboardStage.offsetHeight;
  const ideal = heatmapPanelEl.clientWidth / KEYBOARD_RATIO + chrome;
  // 下方留白优先分给趋势图，但保证右栏（鼠标面板 + 最常用按键面板）可用
  const floor = (mousePanelEl ? mousePanelEl.offsetHeight : 170) + 240;
  const height = Math.round(
    Math.min(available, Math.max(ideal, Math.min(floor, ideal + 120))),
  );
  if (Math.abs(height - lastWorkspaceHeight) > 0.5) {
    workspaceEl.style.height = `${height}px`;
    lastWorkspaceHeight = height;
  }
}

function requestRelayout() {
  requestAnimationFrame(() => {
    fitWorkspace();
    fitKeyboard();
  });
}

function formatNumber(value) {
  return new Intl.NumberFormat("zh-CN").format(value || 0);
}

const DISTANCE_UNITS = {
  auto: { auto: true },
  mm: { factor: 25.4 / 96, suffix: "mm" },
  cm: { factor: 2.54 / 96, suffix: "cm" },
  m: { factor: 1 / 3779.527, suffix: "m" },
  km: { factor: 1 / 3779527.559, suffix: "km" },
  inch: { factor: 1 / 96, suffix: "in" },
  ft: { factor: 1 / 1152, suffix: "ft" },
  px: { factor: 1, suffix: "px" },
};

function formatDistance(px) {
  if (state.unit === "auto") {
    const meters = (px || 0) / 3779.527;
    return meters >= 1 ? `${meters.toFixed(2)} m` : `${Math.round(px || 0).toLocaleString("zh-CN")} px`;
  }
  const unit = DISTANCE_UNITS[state.unit] || DISTANCE_UNITS.m;
  const value = (px || 0) * unit.factor;
  if (unit.suffix === "px") return `${Math.round(value).toLocaleString("zh-CN")} px`;
  const abs = Math.abs(value);
  const digits = abs >= 100 ? 1 : abs >= 1 ? 2 : 3;
  return `${value.toFixed(digits)} ${unit.suffix}`;
}

function escapeHtml(value) {
  return String(value).replace(/[&<>"'/]/g, (char) => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;"
  }[char]));
}

async function fetchJSON(url, options) {
  const response = await fetch(url, options);
  if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
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

function renderDistance() {
  if (!state.summary) return;
  const text = formatDistance(state.summary.distance_px);
  document.getElementById("stat-distance").textContent = text;
  document.getElementById("mouse-distance").textContent = text;
}

function renderSummary(summary) {
  state.summary = summary;
  document.getElementById("stat-keys").textContent = formatNumber(summary.keys);
  document.getElementById("stat-clicks").textContent = formatNumber(summary.clicks);
  document.getElementById("stat-scrolls").textContent = formatNumber(summary.scrolls);
  const mouse = {};
  for (const item of summary.mouse) mouse[item.button || item.event_type] = item.count;
  const sideCount = (mouse.x1 || 0) + (mouse.x2 || 0) + (mouse.side || 0);
  document.getElementById("mouse-left").textContent = formatNumber(mouse.left);
  document.getElementById("mouse-right").textContent = formatNumber(mouse.right);
  document.getElementById("mouse-middle").textContent = formatNumber(mouse.middle);
  document.getElementById("mouse-side").textContent = formatNumber(sideCount);
  document.getElementById("mouse-up").textContent = formatNumber(mouse.up);
  document.getElementById("mouse-down").textContent = formatNumber(mouse.down);
  renderDistance();

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
            <span class="top-key-name">${escapeHtml(labelFor(item.key_name, item.key_name))}</span>
            <span class="top-key-bar" style="width: ${width}%;"></span>
            <span class="top-key-count">${formatNumber(item.count)}</span>
          </li>
        `;
      })
      .join("");
  }

  renderPause(summary.paused);
  renderAutostart(summary.autostart_enabled);
  statusText.textContent = summary.paused ? "统计已暂停" : `统计中 · ${summary.date} · ${new Date().toLocaleTimeString("zh-CN")}`;
}

function renderHeatmap(payload) {
  const counts = {};
  let total = 0;
  for (const item of payload.keys) {
    counts[item.key_name] = item.count;
    total += item.count;
  }
  const maxCount = Math.max(1, ...Object.values(counts));
  for (const el of document.querySelectorAll(".key")) {
    const keyName = el.dataset.key;
    const count = counts[keyName] || counts[keyName.toUpperCase()] || 0;
    const heat = count > 0 ? 0.08 + 0.9 * (Math.log(count) / Math.log(maxCount)) : 0;
    el.style.setProperty("--heat", heat.toFixed(3));
    el.title = count > 0 ? `${labelFor(keyName, keyName)}: ${formatNumber(count)} 次` : "";
  }
  document.getElementById("heatmap-info").textContent = `${payload.date} · 总计 ${formatNumber(total)} 次按键`;
  requestAnimationFrame(fitKeyboard);
}

function renderTrend(payload) {
  const isDay = payload.period === "day";
  const items = isDay ? (payload.days || []) : (payload.hours || []);
  const totalKeys = items.reduce((sum, item) => sum + (item.keys || 0), 0);
  const totalClicks = items.reduce((sum, item) => sum + (item.clicks || 0), 0);
  const active = items.reduce((best, item) => {
    return (item.keys || 0) + (item.clicks || 0) > (best.keys || 0) + (best.clicks || 0) ? item : best;
  }, items[0] || { keys: 0, clicks: 0, hour: 0, day: "" });
  const activeLabel = isDay ? String(active.day || "").slice(5) : `${String(active.hour).padStart(2, "0")}:00`;
  const summaryEl = document.getElementById("trend-summary");
  if (summaryEl) {
    summaryEl.textContent = `${isDay ? "范围内" : "今日"}按键 ${formatNumber(totalKeys)} · 点击 ${formatNumber(totalClicks)} · 最活跃 ${activeLabel}`;
  }

  const maxValue = Math.max(1, ...items.map((item) => Math.max(item.keys || 0, item.clicks || 0)));
  const chart = document.getElementById("trend-chart");
  const columns = items.map((item) => {
    const keysHeight = Math.max((item.keys || 0) > 0 ? 2 : 0, ((item.keys || 0) / maxValue) * 100);
    const clicksHeight = Math.max((item.clicks || 0) > 0 ? 2 : 0, ((item.clicks || 0) / maxValue) * 100);
    const label = isDay ? String(item.day || "").slice(5) : String(item.hour).padStart(2, "0");
    const title = isDay ? `${item.day}  按键 ${item.keys || 0} 次，点击 ${item.clicks || 0} 次` : `${item.hour}:00  按键 ${item.keys || 0} 次，点击 ${item.clicks || 0} 次`;
    return `
      <div class="trend-col" title="${title}">
        <div class="trend-bars">
          <div class="trend-bar keys" style="height:${keysHeight}%"></div>
          <div class="trend-bar clicks" style="height:${clicksHeight}%"></div>
        </div>
        <span class="trend-hour">${label}</span>
      </div>
    `;
  }).join("");

  chart.innerHTML = `
    <div class="trend-yaxis">
      <span>${formatNumber(maxValue)}</span>
      <span>${formatNumber(Math.ceil(maxValue / 2))}</span>
      <span>0</span>
    </div>
    <div class="trend-columns">${columns}</div>
  `;
}

function formatDateInput(value) {
  const d = new Date(value.getFullYear(), value.getMonth(), value.getDate());
  const year = d.getFullYear();
  const month = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

function getDateRange() {
  const today = formatDateInput(new Date());
  const mode = state.range.mode;
  if (mode === "today") return { start: today, end: today };
  if (mode === "yesterday") {
    const d = new Date();
    d.setDate(d.getDate() - 1);
    const day = formatDateInput(d);
    return { start: day, end: day };
  }
  if (mode === "7d") {
    const d = new Date();
    d.setDate(d.getDate() - 6);
    return { start: formatDateInput(d), end: today };
  }
  if (mode === "30d") {
    const d = new Date();
    d.setDate(d.getDate() - 29);
    return { start: formatDateInput(d), end: today };
  }
  if (mode === "month") {
    const now = new Date();
    const first = new Date(now.getFullYear(), now.getMonth(), 1);
    return { start: formatDateInput(first), end: today };
  }
  if (mode === "all") return { start: "all", end: "all" };
  return { start: rangeStart.value || today, end: rangeEnd.value || today };
}

function buildApiUrl(path) {
  const range = getDateRange();
  const params = [];
  if (range.start) params.push(`start=${encodeURIComponent(range.start)}`);
  if (range.end) params.push(`end=${encodeURIComponent(range.end)}`);
  return params.length ? `${path}?${params.join("&")}` : path;
}

function updateRangeInputs() {
  const custom = state.range.mode === "custom";
  rangeStart.hidden = !custom;
  rangeEnd.hidden = !custom;
  if (custom) {
    const today = formatDateInput(new Date());
    if (!rangeStart.value) rangeStart.value = today;
    if (!rangeEnd.value) rangeEnd.value = today;
  }
}



async function loadSettings() {
  try {
    const settings = await fetchJSON("/api/settings");
    renderSettings(settings);
  } catch (error) {
    // Settings can retry on the next refresh; it should not mark the app offline.
  }
}

function saveCloseAction() {
  fetchJSON("/api/settings", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ close_action: closeActionSelect.value }),
  })
    .then(() => {
      statusText.textContent = "关闭行为已保存";
    })
    .catch(() => {
      statusText.textContent = "关闭行为保存失败";
    });
}

async function refresh() {
  const results = await Promise.allSettled([
    fetchJSON(buildApiUrl("/api/summary")),
    fetchJSON(buildApiUrl("/api/heatmap")),
    fetchJSON(buildApiUrl("/api/trend")),
  ]);
  let coreSuccess = 0;
  if (results[0].status === "fulfilled") {
    renderSummary(results[0].value);
    coreSuccess += 1;
  }
  if (results[1].status === "fulfilled") {
    renderHeatmap(results[1].value);
    coreSuccess += 1;
  }
  if (results[2].status === "fulfilled") {
    renderTrend(results[2].value);
    coreSuccess += 1;
  }
  if (coreSuccess === 0) {
    statusText.textContent = "无法连接统计服务";
    liveIndicator.classList.add("paused");
    liveIndicator.textContent = "离线";
  } else {
    liveIndicator.classList.remove("paused");
    liveIndicator.textContent = "运行中";
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
    if (result.enabled !== undefined) renderAutostart(result.enabled);
    else throw new Error(result.error || "设置失败");
  } catch (error) {
    statusText.textContent = `开机自启设置失败：${error.message}`;
  }
});

unitSelect.value = state.unit;
unitSelect.addEventListener("change", () => {
  state.unit = unitSelect.value;
  localStorage.setItem("distance-unit", state.unit);
  renderDistance();
});

closeActionSelect.addEventListener("change", saveCloseAction);

window.addEventListener("resize", requestRelayout);
if (typeof ResizeObserver !== "undefined") {
  const layoutObserver = new ResizeObserver(requestRelayout);
  layoutObserver.observe(keyboardStage);
  if (mainEl) layoutObserver.observe(mainEl);
  if (statStripEl) layoutObserver.observe(statStripEl);
  if (trendPanelEl) layoutObserver.observe(trendPanelEl);
  if (heatmapPanelEl) layoutObserver.observe(heatmapPanelEl);
}

rangeSelect.value = state.range.mode;
updateRangeInputs();
rangeSelect.addEventListener("change", () => {
  state.range.mode = rangeSelect.value;
  updateRangeInputs();
  refresh();
});
rangeStart.addEventListener("change", () => {
  state.range.mode = "custom";
  rangeSelect.value = "custom";
  updateRangeInputs();
  refresh();
});
rangeEnd.addEventListener("change", () => {
  state.range.mode = "custom";
  rangeSelect.value = "custom";
  updateRangeInputs();
  refresh();
});


function setupPanelLayout() {
  let draggedPanel = null;
  const panels = document.querySelectorAll(".panel");
  panels.forEach((panel) => {
    panel.draggable = true;
    panel.addEventListener("dragstart", (event) => {
      draggedPanel = panel;
      event.dataTransfer.effectAllowed = "move";
    });
    panel.addEventListener("dragover", (event) => {
      event.preventDefault();
      panel.classList.add("drag-over");
    });
    panel.addEventListener("dragleave", () => panel.classList.remove("drag-over"));
    panel.addEventListener("drop", (event) => {
      event.preventDefault();
      panel.classList.remove("drag-over");
      if (!draggedPanel || draggedPanel === panel) return;
      const targetParent = panel.parentNode;
      targetParent.insertBefore(draggedPanel, panel.nextSibling);
      draggedPanel = null;
    });
    panel.addEventListener("dragend", () => {
      panels.forEach((item) => item.classList.remove("drag-over"));
      draggedPanel = null;
    });
  });
}

setupPanelLayout();
loadSettings();
buildKeyboard();
requestRelayout();
refresh();
setInterval(refresh, 1000);

