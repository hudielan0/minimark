import { invoke } from "@tauri-apps/api/core";
import DOMPurify from "dompurify";
import { marked } from "marked";
import "./styles.css";

type RecoveryOffer = {
  draft: string;
  diskChanged: boolean;
};

type OpenedDocument = {
  name: string;
  text: string;
  recovery: RecoveryOffer | null;
};

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <main class="app-shell">
    <header class="toolbar">
      <div>
        <strong>MiniMark</strong>
        <span id="document-name">尚未打开文件</span>
      </div>
      <nav>
        <button id="open-button" type="button">打开 Markdown</button>
        <button id="save-button" type="button" disabled>保存</button>
      </nav>
    </header>
    <section class="workspace">
      <label class="pane">
        <span>原文</span>
        <textarea id="editor" spellcheck="false" disabled></textarea>
      </label>
      <section class="pane preview-pane">
        <span>预览</span>
        <article id="preview"></article>
      </section>
    </section>
    <footer id="status">先打开一个 .md 文件。MiniMark 不会自动覆盖外部修改。</footer>
  </main>
`;

const editor = document.querySelector<HTMLTextAreaElement>("#editor")!;
const preview = document.querySelector<HTMLElement>("#preview")!;
const name = document.querySelector<HTMLElement>("#document-name")!;
const status = document.querySelector<HTMLElement>("#status")!;
const openButton = document.querySelector<HTMLButtonElement>("#open-button")!;
const saveButton = document.querySelector<HTMLButtonElement>("#save-button")!;
let recoveryTimer: number | undefined;

function render(markdown: string) {
  const unsafeHtml = marked.parse(markdown) as string;
  preview.innerHTML = DOMPurify.sanitize(unsafeHtml, { USE_PROFILES: { html: true } });
}

function setDocument(document: OpenedDocument) {
  let text = document.text;
  if (document.recovery) {
    const warning = document.recovery.diskChanged
      ? "上次未保存的草稿和磁盘文件都发生过变化。要载入草稿吗？磁盘内容不会被覆盖。"
      : "发现上次未保存的草稿，要继续编辑吗？";
    if (window.confirm(warning)) text = document.recovery.draft;
  }
  name.textContent = document.name;
  editor.value = text;
  editor.disabled = false;
  saveButton.disabled = false;
  render(text);
  status.textContent = "文件已打开。输入内容会保存为恢复草稿，点击“保存”才写回原文件。";
  editor.focus();
}

openButton.addEventListener("click", async () => {
  try {
    const document = await invoke<OpenedDocument | null>("open_document");
    if (document) setDocument(document);
  } catch (error) {
    status.textContent = `打开失败：${String(error)}`;
  }
});

editor.addEventListener("input", () => {
  render(editor.value);
  status.textContent = "正在编辑；尚未保存。";
  window.clearTimeout(recoveryTimer);
  recoveryTimer = window.setTimeout(async () => {
    try {
      await invoke("write_draft", { text: editor.value });
      status.textContent = "草稿已备份；尚未写回原文件。";
    } catch (error) {
      status.textContent = `草稿备份失败：${String(error)}`;
    }
  }, 400);
});

saveButton.addEventListener("click", async () => {
  try {
    await invoke("save_document", { text: editor.value });
    status.textContent = "已安全保存。";
  } catch (error) {
    status.textContent = `没有覆盖原文件：${String(error)}`;
  }
});
