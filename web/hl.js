// A tiny Rust syntax highlighter. Tokenises comments, strings, chars,
// lifetimes, numbers, keywords, types, macros and attributes.
(function () {
  const KEYWORDS = new Set((
    'as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref ' +
    'return self Self static struct super trait true type unsafe use where while async await dyn'
  ).split(' '));
  const TYPES = new Set((
    'i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 bool char str String Vec Option Some None Result Ok Err ' +
    'Box Rc Arc RefCell Cell HashMap HashSet BTreeMap Ordering'
  ).split(' '));

  function esc(s) {
    return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
  }
  function span(cls, text) {
    return '<span class="tok-' + cls + '">' + esc(text) + '</span>';
  }

  function rust(code) {
    let out = '';
    let i = 0;
    const n = code.length;
    while (i < n) {
      const ch = code[i];
      const rest = code.slice(i);
      let m;

      if ((m = /^\/\/[^\n]*/.exec(rest))) { out += span('comment', m[0]); i += m[0].length; continue; }
      if ((m = /^\/\*[\s\S]*?\*\//.exec(rest))) { out += span('comment', m[0]); i += m[0].length; continue; }
      if ((m = /^#!?\[[^\]\n]*\]/.exec(rest))) { out += span('attr', m[0]); i += m[0].length; continue; }
      if ((m = /^b?"(?:[^"\\]|\\[\s\S])*"/.exec(rest))) { out += span('string', m[0]); i += m[0].length; continue; }
      if ((m = /^b?r(#*)"[\s\S]*?"\1/.exec(rest))) { out += span('string', m[0]); i += m[0].length; continue; }
      if ((m = /^b?'(?:[^'\\\n]|\\[\s\S])'/.exec(rest))) { out += span('string', m[0]); i += m[0].length; continue; }
      if ((m = /^'[A-Za-z_][A-Za-z0-9_]*\b(?!')/.exec(rest))) { out += span('lifetime', m[0]); i += m[0].length; continue; }
      if ((m = /^(?:0x[0-9a-fA-F_]+|0b[01_]+|0o[0-7_]+|\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d+)?)(?:[iu](?:8|16|32|64|128|size)|f32|f64)?/.exec(rest))) {
        out += span('number', m[0]); i += m[0].length; continue;
      }
      if ((m = /^[A-Za-z_][A-Za-z0-9_]*!/.exec(rest))) { out += span('macro', m[0]); i += m[0].length; continue; }
      if ((m = /^[A-Za-z_][A-Za-z0-9_]*/.exec(rest))) {
        const w = m[0];
        if (KEYWORDS.has(w)) out += span('kw', w);
        else if (TYPES.has(w) || /^[A-Z]/.test(w)) out += span('type', w);
        else if (/^\s*\(/.test(code.slice(i + w.length)) ) out += span('fn', w);
        else out += esc(w);
        i += w.length;
        continue;
      }
      out += esc(ch);
      i++;
    }
    return out;
  }

  window.hl = { rust };
})();
