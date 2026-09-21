import createDOMPurify from "dompurify";
import { JSDOM } from "jsdom";
import { marked } from "marked";

const window = new JSDOM("").window;
const purifier = createDOMPurify(window);

export function renderPreview(source: string): string {
  const rendered = marked.parse(source, { async: false });
  return purifier.sanitize(rendered, {
    USE_PROFILES: { html: true },
  });
}
