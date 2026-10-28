// A small Markdown renderer: headings, paragraphs, fenced code, inline code,
// bold, italic, links, bullet and numbered lists, blockquotes, rules.
// Enough for lesson text; nothing fancy on purpose.
(function () {
  function escapeHtml(s) {
    return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  }

  function inline(text) {
    // Protect inline code first so its contents are not styled.
    const codes = [];
    text = text.replace(/`([^`]+)`/g, (_, c) => {
      codes.push('<code>' + escapeHtml(c) + '</code>');
      return '\u0000' + (codes.length - 1) + '\u0000';
    });
    text = escapeHtml(text);
    text = text.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
    text = text.replace(/(^|[^*\w])\*([^*\n]+)\*(?!\w)/g, '$1<em>$2</em>');
    text = text.replace(/\[([^\]]+)\]\(([^)\s]+)\)/g, (_, label, href) => {
      const safe = /^(https?:\/\/|\/|#)/.test(href) ? href : '#';
      return '<a href="' + safe + '" target="_blank" rel="noopener">' + label + '</a>';
    });
    text = text.replace(/\u0000(\d+)\u0000/g, (_, i) => codes[Number(i)]);
    return text;
  }

  function render(src) {
    const lines = (src || '').replace(/\r\n/g, '\n').split('\n');
    const out = [];
    let i = 0;
    let para = [];

    function flushPara() {
      if (para.length) {
        out.push('<p>' + inline(para.join(' ')) + '</p>');
        para = [];
      }
    }

    while (i < lines.length) {
      const line = lines[i];
      const trimmed = line.trim();

      if (trimmed.startsWith('```')) {
        flushPara();
        const lang = trimmed.slice(3).trim() || 'text';
        const buf = [];
        i++;
        while (i < lines.length && !lines[i].trim().startsWith('```')) {
          buf.push(lines[i]);
          i++;
        }
        i++; // closing fence
        const code = buf.join('\n');
        const body = lang === 'rust' && window.hl ? window.hl.rust(code) : escapeHtml(code);
        out.push('<pre class="code lang-' + escapeHtml(lang) + '"><code>' + body + '</code></pre>');
        continue;
      }

      if (trimmed === '') {
        flushPara();
        i++;
        continue;
      }

      const h = /^(#{1,6})\s+(.*)$/.exec(trimmed);
      if (h) {
        flushPara();
        const level = Math.min(h[1].length + 1, 6); // card title is h2; demote
        out.push('<h' + level + '>' + inline(h[2]) + '</h' + level + '>');
        i++;
        continue;
      }

      if (/^(-{3,}|\*{3,})$/.test(trimmed)) {
        flushPara();
        out.push('<hr>');
        i++;
        continue;
      }

      if (trimmed.startsWith('>')) {
        flushPara();
        const buf = [];
        while (i < lines.length && lines[i].trim().startsWith('>')) {
          buf.push(lines[i].trim().replace(/^>\s?/, ''));
          i++;
        }
        const isCompiler = /^(error|warning)(\[E\d+\])?:/.test(buf[0] || '');
        out.push('<blockquote class="' + (isCompiler ? 'compiler' : '') + '">' + render(buf.join('\n')) + '</blockquote>');
        continue;
      }

      const ul = /^[-*]\s+(.*)$/.exec(trimmed);
      const ol = /^\d+[.)]\s+(.*)$/.exec(trimmed);
      if (ul || ol) {
        flushPara();
        const tag = ul ? 'ul' : 'ol';
        const re = ul ? /^[-*]\s+(.*)$/ : /^\d+[.)]\s+(.*)$/;
        const items = [];
        while (i < lines.length) {
          const m = re.exec(lines[i].trim());
          if (m) {
            items.push(m[1]);
            i++;
          } else if (lines[i].startsWith('  ') && lines[i].trim() !== '' && items.length) {
            items[items.length - 1] += ' ' + lines[i].trim();
            i++;
          } else {
            break;
          }
        }
        out.push('<' + tag + '>' + items.map((it) => '<li>' + inline(it) + '</li>').join('') + '</' + tag + '>');
        continue;
      }

      para.push(trimmed);
      i++;
    }
    flushPara();
    return out.join('\n');
  }

  window.md = { render, inline, escapeHtml };
})();
