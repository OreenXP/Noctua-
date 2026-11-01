const invoke = window.__TAURI__.core.invoke;

const state = { vault: null, books: [], filtered: [], view: "list", sort: "recent", selected: null, contextBook: null };
const $ = (id) => document.getElementById(id);

function setScreen(hasVault) {
  $("welcome").classList.toggle("hidden", hasVault);
  $("library").classList.toggle("hidden", !hasVault);
}

function formatCount(count) { return `${count} ${count === 1 ? "libro" : "libros"}`; }

function applyFilters() {
  const query = $("search").value.trim().toLocaleLowerCase();
  state.filtered = state.books.filter((book) => book.name.toLocaleLowerCase().includes(query));
  if (state.sort === "name") state.filtered.sort((a, b) => a.name.localeCompare(b.name));
  if (state.sort === "size") state.filtered.sort((a, b) => sizeValue(b.size) - sizeValue(a.size));
  if (state.sort === "recent") state.filtered.sort((a, b) => b.modified.localeCompare(a.modified));
  render();
}

function sizeValue(size) {
  const [amount, unit] = size.split(" ");
  return Number(amount) * ({ B: 1, KB: 1024, MB: 1048576, GB: 1073741824 }[unit] || 1);
}

function render() {
  $("book-count").textContent = formatCount(state.filtered.length);
  $("vault-path").textContent = state.vault || "";
  $("empty").classList.toggle("hidden", state.filtered.length !== 0);
  $("list-content").classList.toggle("hidden", state.view !== "list" || state.filtered.length === 0);
  $("grid-content").classList.toggle("hidden", state.view !== "grid" || state.filtered.length === 0);
  $("list-view").classList.toggle("active", state.view === "list");
  $("grid-view").classList.toggle("active", state.view === "grid");
  $("books-list").replaceChildren(...state.filtered.map(renderListRow));
  $("grid-content").replaceChildren(...state.filtered.map(renderCard));
}

function renderListRow(book) {
  const row = document.createElement("article");
  row.className = `book-row ${state.selected === book.path ? "selected" : ""}`;
  row.addEventListener("click", () => { state.selected = book.path; render(); });
  row.addEventListener("dblclick", () => openBook(book.path));
  row.addEventListener("contextmenu", (event) => showContextMenu(event, book));
  row.innerHTML = `<div class="pdf-thumb"><img src="./assets/icons/file-text.svg" alt="" /><b>PDF</b></div><div class="book-name"><strong title="${escapeHtml(book.name)}">${escapeHtml(book.name)}</strong><span>${escapeHtml(book.relativePath)}</span></div><span class="book-size">${book.size}</span><span class="book-date">${book.modified}</span><button class="row-open">Abrir</button>`;
  row.querySelector(".row-open").addEventListener("click", (event) => { event.stopPropagation(); openBook(book.path); });
  return row;
}

function showContextMenu(event, book) {
  event.preventDefault();
  state.contextBook = book;
  const menu = $("context-menu");
  menu.classList.remove("hidden");
  menu.style.left = `${Math.min(event.clientX, window.innerWidth - 190)}px`;
  menu.style.top = `${Math.min(event.clientY, window.innerHeight - 90)}px`;
}

function renderCard(book) {
  const card = document.createElement("article");
  card.className = `book-card ${state.selected === book.path ? "selected" : ""}`;
  card.addEventListener("click", () => { state.selected = book.path; render(); });
  card.addEventListener("dblclick", () => openBook(book.path));
  card.innerHTML = `<div class="card-cover"><img src="./assets/icons/file-text.svg" alt="" /><span>PDF</span></div><strong title="${escapeHtml(book.name)}">${escapeHtml(book.name)}</strong><span>${escapeHtml(book.size)} · ${escapeHtml(book.modified)}</span><button class="card-open">${state.selected === book.path ? "Abrir" : "Seleccionar"}</button>`;
  card.querySelector(".card-open").addEventListener("click", (event) => { event.stopPropagation(); state.selected === book.path ? openBook(book.path) : (state.selected = book.path, render()); });
  return card;
}

function escapeHtml(value) { return value.replace(/[&<>'"]/g, (character) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", "'": "&#39;", '"': "&quot;" }[character])); }

async function openBook(path) {
  try { await invoke("open_file", { path }); $("status").textContent = `Abierto · ${path.split(/[\\/]/).pop()}`; }
  catch (error) { $("status").textContent = `No se pudo abrir: ${error}`; }
}

async function load(payloadPromise) {
  try {
    const payload = await payloadPromise;
    state.vault = payload.vault;
    state.books = payload.books;
    setScreen(Boolean(state.vault));
    if (state.vault) { $("status").textContent = `Vault listo · ${state.books.length} libros encontrados`; applyFilters(); }
  } catch (error) { $("status").textContent = String(error); }
}

$("welcome-choose").addEventListener("click", () => load(invoke("choose_vault")));
$("change-folder").addEventListener("click", () => load(invoke("choose_vault")));
$("refresh").addEventListener("click", () => { $("status").textContent = "Actualizando..."; load(invoke("refresh", { vault: state.vault })); });
$("open-pdf").addEventListener("click", async () => { const path = await invoke("pick_pdf"); if (path) openBook(path); });
$("search").addEventListener("input", applyFilters);
$("sort").addEventListener("change", (event) => { state.sort = event.target.value; applyFilters(); });
$("list-view").addEventListener("click", () => { state.view = "list"; render(); });
$("grid-view").addEventListener("click", () => { state.view = "grid"; render(); });
$("context-open").addEventListener("click", () => { if (state.contextBook) openBook(state.contextBook.path); $("context-menu").classList.add("hidden"); });
$("context-folder").addEventListener("click", async () => { if (state.contextBook) await invoke("open_folder", { path: state.contextBook.path }); $("context-menu").classList.add("hidden"); });
document.addEventListener("click", (event) => { if (!event.target.closest(".context-menu")) $("context-menu").classList.add("hidden"); });
document.addEventListener("keydown", (event) => { if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "k") { event.preventDefault(); $("search").focus(); } });

load(invoke("load_library"));
