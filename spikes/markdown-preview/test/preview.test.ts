import { describe, expect, it } from "vitest";
import { renderPreview } from "../src/preview";

describe("Markdown preview", () => {
  it("renders readable HTML without changing the source text", () => {
    const source = "# 标题\n\n- 第一项\n- **第二项**\n";

    const preview = renderPreview(source);

    expect(preview).toContain("<h1>标题</h1>");
    expect(preview).toContain("<strong>第二项</strong>");
    expect(source).toBe("# 标题\n\n- 第一项\n- **第二项**\n");
  });

  it("removes executable content from raw Markdown HTML", () => {
    const source = [
      '<img src="x" onerror="alert(1)">',
      '<script>alert("owned")</script>',
      '[unsafe](javascript:alert(1))',
    ].join("\n\n");

    const preview = renderPreview(source);

    expect(preview).not.toMatch(/<script/i);
    expect(preview).not.toMatch(/onerror/i);
    expect(preview).not.toMatch(/javascript:/i);
    expect(source).toContain('<script>alert("owned")</script>');
  });
});
