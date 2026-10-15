// The course frontend. Vanilla JS, no build step. Talks to the local server
// in course/src/main.rs.
(function () {
  'use strict';

  const state = {
    course: null,
    progress: null,
    chapterId: null,
    showAll: false,
    // Per-exercise UI memory that survives re-renders.
    exercise: {}, // id -> { outcome, hints: [], solution, running, autoRun, lastMtime, pollTimer }
  };

  const $ = (sel, root) => (root || document).querySelector(sel);
  const el = (tag, attrs, ...children) => {
    const node = document.createElement(tag);
    if (attrs) {
      for (const [k, v] of Object.entries(attrs)) {
        if (k === 'class') node.className = v;
        else if (k === 'html') node.innerHTML = v;
        else if (k.startsWith('on') && typeof v === 'function') node.addEventListener(k.slice(2), v);
        else if (v !== null && v !== undefined && v !== false) node.setAttribute(k, v === true ? '' : v);
      }
    }
    for (const c of children.flat()) {
      if (c === null || c === undefined || c === false) continue;
      node.append(c instanceof Node ? c : document.createTextNode(String(c)));
    }
    return node;
  };

  // ------------------------------------------------------------------
  // API
  // ------------------------------------------------------------------

  async function api(method, path, body) {
    const res = await fetch(path, {
      method,
      headers: body !== undefined ? { 'Content-Type': 'application/json' } : {},
      body: body !== undefined ? JSON.stringify(body) : undefined,
    });
    let data = null;
    try { data = await res.json(); } catch (_) { /* no body */ }
    if (!res.ok) {
      throw new Error((data && data.error) || (res.status + ' ' + res.statusText));
    }
    return data;
  }

  const exPath = (id) => '/api/exercises/' + id; // id is "chNN/exNN"

  // ------------------------------------------------------------------
  // Progress helpers
  // ------------------------------------------------------------------

  function cardDone(card) {
    if (card.type === 'exercise') {
      const ex = state.progress.exercises[card.ref];
      return !!(ex && ex.passed);
    }
    const c = state.progress.cards[card.id];
    return !!(c && c.done);
  }

  function chapterStats(ch) {
    const total = ch.cards.length;
    const done = ch.cards.filter(cardDone).length;
    const exercises = ch.cards.filter((c) => c.type === 'exercise');
    const exDone = exercises.filter(cardDone).length;
    return { total, done, exTotal: exercises.length, exDone, complete: total > 0 && done === total };
  }

  function suggestedChapter() {
    return state.course.chapters.find((ch) => ch.cards.length && !chapterStats(ch).complete) || null;
  }

  function levelInfo() {
    const levels = state.course.levels;
    const xp = state.progress.xp;
    let idx = 0;
    levels.forEach((l, i) => { if (l.xp <= xp) idx = i; });
    const cur = levels[idx];
    const next = levels[idx + 1] || null;
    const pct = next ? Math.min(100, Math.round(((xp - cur.xp) / (next.xp - cur.xp)) * 100)) : 100;
    return { idx, cur, next, pct, xp };
  }

  const storage = {
    get(k, d) { try { const v = localStorage.getItem(k); return v === null ? d : JSON.parse(v); } catch (_) { return d; } },
    set(k, v) { try { localStorage.setItem(k, JSON.stringify(v)); } catch (_) { /* ignore */ } },
  };

  // ------------------------------------------------------------------
  // Toasts & dialogs
  // ------------------------------------------------------------------

  function toast(text, cls, ms) {
    const node = el('div', { class: 'toast ' + (cls || '') }, text);
    $('#toasts').append(node);
    setTimeout(() => node.remove(), ms || 2600);
  }

  function handleAward(award) {
    if (!award) return;
    state.progress.xp = award.xp_total;
    if (award.xp_gained > 0) toast('+' + award.xp_gained + ' XP', 'xp');
    if (award.level_after > award.level_before) {
      const lvl = state.course.levels[award.level_after];
      setTimeout(() => toast('Level up: ' + lvl.title, 'level', 4000), 350);
    }
    renderSidebar();
  }

  function confirmDialog(title, text, okLabel) {
    return new Promise((resolve) => {
      const root = $('#dialog-root');
      const close = (v) => { root.innerHTML = ''; resolve(v); };
      const box = el('div', { class: 'dialog', role: 'dialog', 'aria-modal': 'true' },
        el('h3', null, title),
        el('p', null, text),
        el('div', { class: 'row' },
          el('button', { class: 'btn', onclick: () => close(false) }, 'Cancel'),
          el('button', { class: 'btn primary', onclick: () => close(true) }, okLabel || 'OK')));
      const backdrop = el('div', { class: 'dialog-backdrop', onclick: (e) => { if (e.target === backdrop) close(false); } }, box);
      root.append(backdrop);
    });
  }

  // ------------------------------------------------------------------
  // Sidebar
  // ------------------------------------------------------------------

  function ring(done, total) {
    const r = 13, c = 2 * Math.PI * r;
    const frac = total ? done / total : 0;
    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.setAttribute('viewBox', '0 0 32 32');
    svg.setAttribute('class', 'ring' + (total && done === total ? ' done' : ''));
    svg.innerHTML =
      '<circle class="track" cx="16" cy="16" r="' + r + '" fill="none" stroke-width="3"></circle>' +
      '<circle class="fill" cx="16" cy="16" r="' + r + '" fill="none" stroke-width="3" stroke-linecap="round" ' +
      'stroke-dasharray="' + c.toFixed(2) + '" stroke-dashoffset="' + (c * (1 - frac)).toFixed(2) + '" transform="rotate(-90 16 16)"></circle>' +
      (total && done === total
        ? '<text x="16" y="20.5" text-anchor="middle">✓</text>'
        : '<text x="16" y="20" text-anchor="middle">' + Math.round(frac * 100) + '</text>');
    return svg;
  }

  function renderSidebar() {
    const info = levelInfo();
    $('#level-title').textContent = info.cur.title;
    $('#xp-fill').style.width = info.pct + '%';
    $('#xp-text').textContent = info.next
      ? info.xp + ' XP · ' + (info.next.xp - info.xp) + ' to ' + info.next.title
      : info.xp + ' XP · max level';

    const list = $('#chapter-list');
    list.innerHTML = '';
    const next = suggestedChapter();
    for (const ch of state.course.chapters) {
      const s = chapterStats(ch);
      const btn = el('button', {
        class: 'chapter-link' + (ch.id === state.chapterId ? ' active' : '') + (ch.cards.length ? '' : ' empty'),
        onclick: () => { location.hash = '#/' + ch.id; $('#sidebar').classList.remove('open'); },
      },
        ring(s.done, s.total),
        el('span', { class: 'chapter-link-text' },
          el('span', { class: 'chapter-link-title' }, ch.number + '. ' + ch.title),
          el('span', { class: 'chapter-link-sub' }, ch.cards.length ? s.done + '/' + s.total + ' done' : 'coming soon')),
        next && next.id === ch.id ? el('span', { class: 'next-pill' }, 'next') : null);
      list.append(btn);
    }
  }

  // ------------------------------------------------------------------
  // Home
  // ------------------------------------------------------------------

  function renderHome() {
    const main = $('#content');
    main.innerHTML = '';
    const next = suggestedChapter();
    const totalDone = state.course.chapters.reduce((n, ch) => n + chapterStats(ch).done, 0);
    const total = state.course.chapters.reduce((n, ch) => n + ch.cards.length, 0);
    main.append(
      el('div', { class: 'chapter-header' },
        el('div', { class: 'chapter-kicker' }, 'Interactive edition'),
        el('h1', null, state.course.title),
        el('p', { class: 'chapter-subtitle' },
          'Lessons, predictions and quizzes here; real code in the exercises/ folder, checked by cargo test.'),
        el('div', { class: 'chapter-stats' },
          el('span', null, el('strong', null, state.progress.xp), ' XP earned of ', state.course.total_xp),
          el('span', null, el('strong', null, totalDone), ' of ', total, ' cards done'))),
      next ? el('div', { class: 'row' },
        el('button', { class: 'btn primary', onclick: () => { location.hash = '#/' + next.id; } },
          'Continue: ' + next.number + '. ' + next.title)) : null,
      el('div', { class: 'home-grid' },
        state.course.chapters.map((ch) => {
          const s = chapterStats(ch);
          return el('button', {
            class: 'home-card' + (ch.cards.length ? '' : ' soon'),
            onclick: () => { if (ch.cards.length) location.hash = '#/' + ch.id; },
          },
            el('div', { class: 'num' }, 'Chapter ' + ch.number + (s.complete ? ' · complete' : '')),
            el('div', { class: 't' }, ch.title),
            el('div', { class: 's' }, ch.cards.length ? ch.subtitle + ' · ' + s.exTotal + ' exercises' : 'Coming soon'));
        })));
  }

  // ------------------------------------------------------------------
  // Chapter
  // ------------------------------------------------------------------

  function unlockedKey(chId) { return 'unlocked.' + chId; }

  function renderChapter(ch) {
    const main = $('#content');
    main.innerHTML = '';
    const s = chapterStats(ch);

    const firstUndone = ch.cards.findIndex((c) => !cardDone(c));
    const manuallyUnlocked = storage.get(unlockedKey(ch.id), -1);
    const frontier = state.showAll ? Infinity : Math.max(firstUndone === -1 ? ch.cards.length : firstUndone, manuallyUnlocked);

    const showAllToggle = el('label', { class: 'toggle' },
      el('input', { type: 'checkbox', checked: state.showAll, onchange: (e) => { state.showAll = e.target.checked; renderChapter(ch); } }),
      'Show every card');

    main.append(
      el('div', { class: 'chapter-header' },
        el('div', { class: 'chapter-kicker' }, 'Chapter ' + ch.number + (ch.book ? ' · Book ' + ch.book : '')),
        el('h1', null, ch.title),
        el('p', { class: 'chapter-subtitle' }, ch.subtitle),
        el('div', { class: 'chapter-stats' },
          el('span', null, el('strong', null, s.done), ' / ', s.total, ' cards'),
          el('span', null, el('strong', null, s.exDone), ' / ', s.exTotal, ' exercises'),
          el('span', null, el('strong', null, ch.cards.reduce((n, c) => n + (c.xp || 0), 0) +
            ch.cards.filter((c) => c.type === 'exercise').reduce((n, c) => n + (exerciseById(c.ref).xp || 0), 0)), ' XP available'))),
      el('div', { class: 'toolbar' }, showAllToggle));

    ch.cards.forEach((card, idx) => {
      const done = cardDone(card);
      const locked = idx > frontier;
      const expanded = !locked && (idx === frontier || (!done && idx <= frontier));
      main.append(renderCard(ch, card, idx, { done, locked, expanded }));
    });

    if (s.complete) {
      const nextCh = state.course.chapters.find((c) => c.number > ch.number && c.cards.length);
      main.append(el('div', { class: 'chapter-end' },
        el('div', null, '🎉'),
        el('h2', null, 'Chapter ' + ch.number + ' complete'),
        el('p', null, 'Every card answered and every exercise green.'),
        nextCh ? el('button', { class: 'btn primary', onclick: () => { location.hash = '#/' + nextCh.id; } },
          'On to ' + nextCh.number + '. ' + nextCh.title) : null));
    }
  }

  function exerciseById(id) {
    return state.course.exercises.find((e) => e.id === id) || { id, title: id, xp: 0, kind: '?', description: '', file: '', hint_count: 0 };
  }

  function typeLabel(card) {
    return { read: 'Read', code: 'Try it', predict: 'Predict', quiz: 'Quiz', exercise: 'Exercise' }[card.type] || card.type;
  }

  function cardTitle(card) {
    if (card.type === 'exercise') return exerciseById(card.ref).title;
    return card.title || typeLabel(card);
  }

  function cardXp(card) {
    return card.type === 'exercise' ? exerciseById(card.ref).xp : card.xp;
  }

  function renderCard(ch, card, idx, opts) {
    const node = el('div', {
      class: 'card' + (opts.done ? ' done' : '') + (opts.locked ? ' locked' : '') + (opts.expanded ? '' : ' collapsed'),
      id: 'card-' + card.id,
    });
    const head = el('div', { class: 'card-head', onclick: () => { if (!opts.locked) node.classList.toggle('collapsed'); } },
      el('span', { class: 'card-type ' + card.type }, typeLabel(card)),
      el('span', { class: 'card-title' }, cardTitle(card)),
      el('span', { class: 'card-meta' }, cardXp(card) + ' XP'),
      el('span', { class: 'card-check' }, '✓'));
    node.append(head);

    const body = el('div', { class: 'card-body' });
    node.append(body);
    if (opts.locked) return node;

    switch (card.type) {
      case 'read': renderRead(ch, card, body, opts); break;
      case 'code': renderCode(ch, card, body, opts); break;
      case 'predict': renderPredict(ch, card, body, opts); break;
      case 'quiz': renderQuiz(ch, card, body, opts); break;
      case 'exercise': renderExercise(ch, card, body, opts); break;
    }
    return node;
  }

  async function completeCard(ch, card, correct) {
    try {
      const reply = await api('POST', '/api/cards/' + encodeURIComponent(card.id) + '/answer', { correct });
      state.progress.cards[card.id] = reply.card;
      handleAward(reply.award);
      return reply;
    } catch (e) {
      toast('Could not save: ' + e.message, 'error', 4000);
      return null;
    }
  }

  function unlockNext(ch, card) {
    const idx = ch.cards.indexOf(card);
    const cur = storage.get(unlockedKey(ch.id), -1);
    if (idx + 1 > cur) storage.set(unlockedKey(ch.id), idx + 1);
  }

  function advance(ch, card) {
    unlockNext(ch, card);
    renderChapter(ch);
    const idx = ch.cards.indexOf(card);
    const next = ch.cards[idx + 1];
    const target = next ? $('#card-' + CSS.escape(next.id)) : null;
    if (target) target.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }

  function continueButton(ch, card, label, onDone) {
    return el('div', { class: 'continue-row' },
      el('button', {
        class: 'btn primary',
        onclick: async (e) => {
          e.target.disabled = true;
          if (onDone) await onDone();
          advance(ch, card);
        },
      }, label));
  }

  // --- read -------------------------------------------------------------

  function renderRead(ch, card, body, opts) {
    body.append(el('div', { html: md.render(card.body) }));
    if (!opts.done) {
      body.append(continueButton(ch, card, 'Got it', () => completeCard(ch, card, true)));
    }
  }

  // --- code -------------------------------------------------------------

  function playgroundUrl(code) {
    return 'https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=' + encodeURIComponent(code);
  }

  function renderCode(ch, card, body, opts) {
    const term = el('div', { class: 'term' });
    const runBtn = el('button', {
      class: 'btn primary small',
      onclick: async () => {
        runBtn.disabled = true;
        term.innerHTML = '<span class="dim">Compiling on play.rust-lang.org…</span>';
        try {
          const reply = await api('POST', '/api/playground', { code: card.code });
          const r = reply.result;
          let html = '';
          if (r.stderr.trim()) html += '<span class="' + (r.success ? 'warn' : 'err') + '">' + md.escapeHtml(stripPlaygroundNoise(r.stderr)) + '</span>\n';
          html += md.escapeHtml(r.stdout);
          if (!r.stdout.trim() && r.success) html += '<span class="dim">(no output)</span>';
          term.innerHTML = html.trim() || '<span class="dim">(no output)</span>';
        } catch (e) {
          term.innerHTML = '<span class="err">' + md.escapeHtml(e.message) + '</span>\n<span class="dim">The Playground needs an internet connection. You can still read the code and move on.</span>';
        }
        runBtn.disabled = false;
      },
    }, '▶ Run');
    body.append(
      el('pre', { class: 'code lang-rust', html: '<code>' + hl.rust(card.code) + '</code>' }),
      el('div', { class: 'row' },
        runBtn,
        el('a', { class: 'btn small', href: playgroundUrl(card.code), target: '_blank', rel: 'noopener' }, 'Open in Playground ↗'),
        el('span', { class: 'muted' }, 'Runs on play.rust-lang.org')),
      term,
      card.body ? el('div', { html: md.render(card.body) }) : null);
    if (!opts.done) {
      body.append(continueButton(ch, card, 'Continue', () => completeCard(ch, card, true)));
    }
  }

  function stripPlaygroundNoise(stderr) {
    return stderr
      .split('\n')
      .filter((l) => !/^\s*(Compiling|Finished|Running|Standard Error|Standard Output)/.test(l))
      .join('\n')
      .trim();
  }

  // --- predict ----------------------------------------------------------

  function renderPredict(ch, card, body, opts) {
    body.append(el('pre', { class: 'code lang-rust', html: '<code>' + hl.rust(card.code) + '</code>' }));
    body.append(el('div', { class: 'question' }, card.question || 'What happens?'));
    renderChoice(ch, card, body, opts, false);
  }

  // --- quiz -------------------------------------------------------------

  function renderQuiz(ch, card, body, opts) {
    if (card.body) body.append(el('div', { html: md.render(card.body) }));
    body.append(el('div', { class: 'question' }, card.question));
    if (card.answers && card.answers.length) renderFill(ch, card, body, opts);
    else renderChoice(ch, card, body, opts, card.multi);
  }

  function renderChoice(ch, card, body, opts, multi) {
    const selected = new Set();
    const buttons = card.options.map((opt, i) => el('button', {
      class: 'option' + (multi ? ' multi' : ''),
      onclick: () => {
        if (opts.done) return;
        if (multi) {
          if (selected.has(i)) selected.delete(i); else selected.add(i);
          buttons[i].classList.toggle('selected', selected.has(i));
          checkBtn.disabled = selected.size === 0;
        } else {
          selected.clear(); selected.add(i);
          submit();
        }
      },
    }, el('span', { class: 'bullet' }), el('span', { html: md.inline(opt.text) })));

    const feedback = el('div');
    const checkBtn = multi ? el('button', { class: 'btn primary', disabled: true, onclick: () => submit() }, 'Check') : null;

    function reveal(correct) {
      card.options.forEach((opt, i) => {
        const b = buttons[i];
        b.disabled = true;
        b.classList.remove('selected');
        if (selected.has(i) && opt.correct) b.classList.add('correct');
        else if (selected.has(i) && !opt.correct) b.classList.add('wrong');
        else if (!selected.has(i) && opt.correct && correct) b.classList.add('correct');
      });
      feedback.innerHTML = '';
      feedback.append(el('div', { class: 'feedback ' + (correct ? 'good' : 'bad') },
        el('div', { class: 'verdict' }, correct ? 'Correct' : 'Not quite'),
        correct ? el('div', { html: md.render(card.explanation) }) : el('p', null, multi ? 'Some of your picks are off. Try a different combination.' : 'Try another option.')));
    }

    async function submit() {
      const correct = card.options.every((opt, i) => opt.correct === selected.has(i));
      reveal(correct);
      if (checkBtn) checkBtn.disabled = true;
      await completeCard(ch, card, correct);
      if (correct) {
        $('#card-' + CSS.escape(card.id)).classList.add('done', 'flash');
        feedback.append(continueButton(ch, card, 'Continue'));
      } else {
        // Let them try again.
        setTimeout(() => {
          card.options.forEach((_, i) => {
            buttons[i].disabled = false;
            buttons[i].classList.remove('wrong', 'correct');
          });
          selected.clear();
          if (checkBtn) checkBtn.disabled = true;
        }, 900);
      }
    }

    body.append(el('div', { class: 'options' }, buttons));
    if (checkBtn) body.append(el('div', { class: 'row' }, checkBtn));
    body.append(feedback);

    if (opts.done) {
      card.options.forEach((opt, i) => { buttons[i].disabled = true; if (opt.correct) buttons[i].classList.add('correct'); });
      if (checkBtn) checkBtn.remove();
      feedback.append(el('div', { class: 'feedback good' },
        el('div', { class: 'verdict' }, 'Answered'),
        el('div', { html: md.render(card.explanation) })));
    }
  }

  function renderFill(ch, card, body, opts) {
    const input = el('input', { class: 'fill-input', type: 'text', placeholder: 'Type your answer', autocomplete: 'off', spellcheck: 'false' });
    const feedback = el('div');
    const btn = el('button', { class: 'btn primary', onclick: () => submit() }, 'Check');
    input.addEventListener('keydown', (e) => { if (e.key === 'Enter') submit(); });

    const normalise = (s) => s.trim().toLowerCase().replace(/^`|`$/g, '').replace(/\s+/g, ' ');

    async function submit() {
      const given = normalise(input.value);
      if (!given) return;
      const correct = card.answers.some((a) => normalise(a) === given);
      feedback.innerHTML = '';
      feedback.append(el('div', { class: 'feedback ' + (correct ? 'good' : 'bad') },
        el('div', { class: 'verdict' }, correct ? 'Correct' : 'Not quite'),
        correct ? el('div', { html: md.render(card.explanation) }) : el('p', null, 'Try again.')));
      await completeCard(ch, card, correct);
      if (correct) {
        input.disabled = true; btn.disabled = true;
        $('#card-' + CSS.escape(card.id)).classList.add('done', 'flash');
        feedback.append(continueButton(ch, card, 'Continue'));
      }
    }

    body.append(el('div', { class: 'row' }, input, btn), feedback);
    if (opts.done) {
      input.value = card.answers[0]; input.disabled = true; btn.disabled = true;
      feedback.append(el('div', { class: 'feedback good' },
        el('div', { class: 'verdict' }, 'Answered'),
        el('div', { html: md.render(card.explanation) })));
    }
  }

  // --- exercise -----------------------------------------------------------

  function exState(id) {
    if (!state.exercise[id]) {
      state.exercise[id] = { outcome: null, hints: [], solution: null, running: false, autoRun: storage.get('autorun.' + id, false), lastMtime: null, pollTimer: null };
    }
    return state.exercise[id];
  }

  function linkifyErrors(text) {
    return md.escapeHtml(text)
      .replace(/error\[(E\d{4})\]/g, (m, code) =>
        '<a href="https://doc.rust-lang.org/error_codes/' + code + '.html" target="_blank" rel="noopener">' + m + '</a>');
  }

  function formatOutput(outcome) {
    const lines = (outcome.stderr + '\n' + outcome.stdout).split('\n');
    const out = [];
    for (const raw of lines) {
      const line = raw.replace(/\r$/, '');
      if (/^\s*(Compiling|Finished|Running|Blocking|Downloading|Downloaded|Locking|Updating|Adding)\b/.test(line)) continue;
      if (line.trim() === '') { out.push(''); continue; }
      let cls = '';
      if (/^error(\[E\d+\])?:|^error:|\bFAILED\b|^failures:|panicked at|^thread '/.test(line)) cls = 'err';
      else if (/^warning/.test(line)) cls = 'warn';
      else if (/\.\.\. ok$|^test result: ok/.test(line)) cls = 'ok';
      else if (/^\s+-->|^\s+\|/.test(line)) cls = 'dim';
      out.push(cls ? '<span class="' + cls + '">' + linkifyErrors(line) + '</span>' : linkifyErrors(line));
    }
    return out.join('\n').replace(/\n{3,}/g, '\n\n').trim();
  }

  const STATUS_TEXT = {
    passed: 'All tests pass',
    failed: 'Tests failed',
    compile_error: 'Does not compile',
    timeout: 'Timed out',
    requirements_unmet: 'Tests pass, requirements not met',
  };

  function renderExercise(ch, card, body, opts) {
    const ex = exerciseById(card.ref);
    const ui = exState(ex.id);
    const prog = state.progress.exercises[ex.id] || { attempts: 0, hints_revealed: 0, solution_revealed: false, passed: false, last_status: '' };

    body.append(
      el('div', { class: 'row', style: 'margin-top:14px' },
        el('span', { class: 'exercise-kind' }, { fix: 'Fix the code', implement: 'Implement', write: 'Write it', project: 'Project stage' }[ex.kind] || ex.kind),
        el('span', { class: 'muted' }, 'Attempts: ' + prog.attempts)),
      el('div', { html: md.render(ex.description) }));

    // File row
    const fileRow = el('div', { class: 'file-row' },
      el('span', { class: 'muted' }, 'Edit'),
      el('code', null, ex.file),
      el('button', {
        class: 'btn small', onclick: async (e) => {
          try { await api('POST', exPath(ex.id) + '/open'); } catch (err) { toast(err.message, 'error', 4000); }
        },
      }, 'Open file'),
      el('button', {
        class: 'btn small', onclick: async () => {
          try { await navigator.clipboard.writeText(ex.file); toast('Path copied'); } catch (_) { toast('Could not copy', 'error'); }
        },
      }, 'Copy path'),
      el('span', { class: 'spacer' }),
      el('code', { class: 'muted' }, 'cargo test -p ' + ex.crate + ' --test ' + ex.target));
    body.append(fileRow);

    // Run row
    const statusPill = el('span', { class: 'status-pill ' + (prog.last_status || '') },
      STATUS_TEXT[prog.last_status] || (prog.passed ? 'Passed' : 'Not run yet'));
    const term = el('div', { class: 'term' });
    const checks = el('ul', { class: 'checks' });
    const runBtn = el('button', { class: 'btn primary', onclick: () => run(false) }, '▶ Run tests');
    const autoToggle = el('label', { class: 'toggle' },
      el('input', { type: 'checkbox', checked: ui.autoRun, onchange: (e) => { ui.autoRun = e.target.checked; storage.set('autorun.' + ex.id, ui.autoRun); setupPolling(); } }),
      'Run on save');
    body.append(el('div', { class: 'row' }, runBtn, autoToggle, el('span', { class: 'spacer' }), statusPill), term, checks);

    function showOutcome(outcome) {
      term.innerHTML = formatOutput(outcome) || '<span class="dim">(no output)</span>';
      checks.innerHTML = '';
      for (const c of outcome.checks || []) checks.append(el('li', { class: c.ok ? '' : 'unmet' }, c.label));
      statusPill.className = 'status-pill ' + outcome.status;
      statusPill.textContent = STATUS_TEXT[outcome.status] || outcome.status;
    }
    if (ui.outcome) showOutcome(ui.outcome);

    async function run(fromPoll) {
      if (ui.running) return;
      ui.running = true;
      runBtn.disabled = true;
      term.innerHTML = '<span class="dim">Running ' + md.escapeHtml('cargo test -p ' + ex.crate + ' --test ' + ex.target) + (fromPoll ? ' (file changed)' : '') + '…</span>';
      try {
        const reply = await api('POST', exPath(ex.id) + '/run');
        ui.outcome = reply.outcome;
        state.progress.exercises[ex.id] = reply.exercise;
        showOutcome(reply.outcome);
        const wasDone = opts.done;
        handleAward(reply.award);
        if (reply.outcome.status === 'passed') {
          const cardNode = $('#card-' + CSS.escape(card.id));
          cardNode.classList.add('done', 'flash');
          if (!wasDone) {
            toast('Exercise complete: ' + ex.title, 'xp', 3000);
            unlockNext(ch, card);
            if (!$('.continue-row', body)) body.append(continueButton(ch, card, 'Continue'));
          }
        }
        // Reveal the solution button after two failed attempts.
        renderSolutionRow();
        $('.muted', body).textContent = 'Attempts: ' + reply.exercise.attempts;
      } catch (e) {
        term.innerHTML = '<span class="err">' + md.escapeHtml(e.message) + '</span>';
      }
      ui.running = false;
      runBtn.disabled = false;
    }

    // Auto-run: poll the file's mtime while the toggle is on and the card is visible.
    function setupPolling() {
      if (ui.pollTimer) { clearInterval(ui.pollTimer); ui.pollTimer = null; }
      if (!ui.autoRun) return;
      ui.pollTimer = setInterval(async () => {
        if (!document.body.contains(term)) { clearInterval(ui.pollTimer); ui.pollTimer = null; return; }
        try {
          const src = await api('GET', exPath(ex.id) + '/source');
          if (ui.lastMtime !== null && src.modified_ms !== ui.lastMtime && !ui.running) {
            ui.lastMtime = src.modified_ms;
            run(true);
          } else {
            ui.lastMtime = src.modified_ms;
          }
        } catch (_) { /* server busy; try again next tick */ }
      }, 1500);
    }
    setupPolling();

    // Hints
    const hintsBox = el('div', { class: 'hints' });
    const hintList = el('div');
    const hintBtn = el('button', { class: 'btn small', onclick: () => revealHint(ui.hints.length + 1) });
    function refreshHintBtn() {
      const n = ui.hints.length;
      if (ex.hint_count === 0) { hintBtn.style.display = 'none'; return; }
      if (n >= ex.hint_count) { hintBtn.disabled = true; hintBtn.textContent = 'No more hints'; }
      else hintBtn.textContent = n === 0 ? 'Show a hint (' + ex.hint_count + ' available)' : 'Next hint (' + (n + 1) + '/' + ex.hint_count + ')';
    }
    async function revealHint(n) {
      try {
        const reply = await api('GET', exPath(ex.id) + '/hint/' + n);
        ui.hints[n - 1] = reply.hint;
        state.progress.exercises[ex.id] = Object.assign({}, state.progress.exercises[ex.id] || {}, { hints_revealed: Math.max(n, (state.progress.exercises[ex.id] || {}).hints_revealed || 0) });
        renderHints();
      } catch (e) { toast(e.message, 'error', 4000); }
    }
    function renderHints() {
      hintList.innerHTML = '';
      ui.hints.forEach((h, i) => h && hintList.append(el('div', { class: 'hint' }, el('strong', null, 'Hint ' + (i + 1) + '. '), el('span', { html: md.inline(h) }))));
      refreshHintBtn();
    }
    hintsBox.append(el('div', { class: 'row' }, hintBtn), hintList);
    body.append(hintsBox);
    renderHints();
    // Hints revealed in an earlier session come back without asking again.
    if (prog.hints_revealed > ui.hints.length) {
      (async () => { for (let n = 1; n <= prog.hints_revealed; n++) if (!ui.hints[n - 1]) await revealHint(n); })();
    }

    // Solution
    const solutionBox = el('div', { class: 'solution' });
    body.append(solutionBox);
    function renderSolutionRow() {
      solutionBox.innerHTML = '';
      const p = state.progress.exercises[ex.id] || prog;
      if (ui.solution) {
        solutionBox.append(
          el('div', { class: 'muted' }, 'Solution (read it, then write your own version from memory):'),
          el('pre', { class: 'code lang-rust', html: '<code>' + hl.rust(ui.solution) + '</code>' }));
        return;
      }
      if (p.attempts >= 2 || p.solution_revealed || p.passed) {
        solutionBox.append(el('button', {
          class: 'btn small', onclick: async () => {
            const ok = p.passed || p.solution_revealed || await confirmDialog('Reveal the solution?',
              'You will still be able to earn XP for this exercise, but only half of it. Hints are free, if you have not tried them.', 'Reveal');
            if (!ok) return;
            try {
              const reply = await api('GET', exPath(ex.id) + '/solution');
              ui.solution = reply.source;
              state.progress.exercises[ex.id] = Object.assign({}, state.progress.exercises[ex.id] || {}, { solution_revealed: true });
              renderSolutionRow();
            } catch (e) { toast(e.message, 'error', 4000); }
          },
        }, p.passed ? 'Compare with the reference solution' : 'Show the solution'));
      } else {
        solutionBox.append(el('span', { class: 'muted' }, 'The solution unlocks after two attempts.'));
      }
    }
    renderSolutionRow();

    if (!opts.done) {
      body.append(el('div', { class: 'row' },
        el('span', { class: 'spacer' }),
        el('button', { class: 'link-btn', onclick: () => advance(ch, card) }, 'Skip for now →')));
    }
  }

  // ------------------------------------------------------------------
  // Routing & boot
  // ------------------------------------------------------------------

  function route() {
    const hash = location.hash.replace(/^#\/?/, '');
    const ch = state.course.chapters.find((c) => c.id === hash);
    state.chapterId = ch ? ch.id : null;
    renderSidebar();
    if (ch) renderChapter(ch); else renderHome();
    window.scrollTo({ top: 0 });
  }

  async function load() {
    const [course, progress] = await Promise.all([api('GET', '/api/course'), api('GET', '/api/progress')]);
    state.course = course;
    state.progress = progress;
  }

  async function boot() {
    try {
      await load();
    } catch (e) {
      $('#content').innerHTML = '';
      $('#content').append(el('div', { class: 'error-box' }, 'Could not load the course:\n' + e.message +
        '\n\nIs the server running? Start it with: cargo run -p course'));
      return;
    }

    // Mobile menu button.
    $('.brand').append(el('button', {
      class: 'btn small menu-btn', style: 'margin-left:auto',
      onclick: () => $('#sidebar').classList.toggle('open'),
    }, 'Chapters'));

    $('#reset-progress').addEventListener('click', async () => {
      const ok = await confirmDialog('Reset all progress?', 'XP, answered cards and exercise history will be cleared. Your code in exercises/ is not touched.', 'Reset');
      if (!ok) return;
      try {
        state.progress = await api('POST', '/api/progress/reset');
        state.exercise = {};
        for (const k of Object.keys(localStorage)) if (k.startsWith('unlocked.')) localStorage.removeItem(k);
        route();
        toast('Progress reset');
      } catch (e) { toast(e.message, 'error', 4000); }
    });

    window.addEventListener('hashchange', route);
    route();
  }

  boot();
})();
