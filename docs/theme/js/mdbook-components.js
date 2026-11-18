/* Component behaviour: File Tree, Tabs and enhanced code blocks.
   The icon library is installed separately and must load before this script. */
if (!window.MDCIcons) {
  console.error('[mdbook-components] Missing icon library: load mdbook-components-icons.js first');
}

/* Lightweight, dependency-free file tree icons. No network requests. */
(() => {
  'use strict';
  let nextTreeId = 0;

  // Build one label row per node. Folder buttons contain the folder label,
  // making the entire label keyboard- and pointer-operable.
  function labelRow(li) {
    const existing = li.querySelector(':scope > p');
    if (existing) return existing;
    const p = document.createElement('p');
    li.insertBefore(p, li.firstChild);
    for (const child of Array.from(li.childNodes)) {
      if (child === p) continue;
      if (child.nodeType === Node.ELEMENT_NODE && ['UL', 'OL'].includes(child.tagName)) continue;
      p.appendChild(child);
    }
    return p;
  }

  function decorate(tree) {
    const showIcons = tree.dataset.iconSet !== 'none';
    const collapsible = tree.dataset.collapsible !== 'false';
    for (const li of tree.querySelectorAll('li')) {
      if (li.dataset.filetreeIconReady === 'true') continue;
      const row = labelRow(li);
      const children = li.querySelector(':scope > ul');
      const label = row.textContent.trim();
      if (!label) continue;
      const kind = window.MDCIcons.resolve(li.dataset.icon) || window.MDCIcons.classify(label, !!children);
      const holder = document.createElement('span');
      holder.className = `workbook-filetree__icon workbook-filetree__icon--${kind}`;
      holder.setAttribute('aria-hidden', 'true');
      if (showIcons) {
        // Fall back gracefully when a filename has no registered icon.
        const artwork = window.MDCIcons.create(kind) || window.MDCIcons.create('file');
        if (artwork) holder.appendChild(artwork);
      }
      li.classList.add('workbook-filetree__item');

      // Links must not be nested inside buttons. For linked folder names,
      // the icon is the disclosure button and the existing link remains intact.
      const hasLink = !!row.querySelector('a, button, input, select, textarea');
      if (children && collapsible) {
        const control = document.createElement('button');
        control.type = 'button';
        control.className = 'workbook-filetree__toggle';
        control.setAttribute('aria-label', `Toggle ${label}`);
        control.setAttribute('aria-expanded', 'true');
        if (!children.id) children.id = `workbook-filetree-branch-${++nextTreeId}`;
        control.setAttribute('aria-controls', children.id);
        const chevron = document.createElement('span');
        chevron.className = 'workbook-filetree__chevron';
        chevron.setAttribute('aria-hidden', 'true');
        control.appendChild(chevron);
        control.appendChild(holder);
        if (!hasLink) {
          while (row.firstChild) control.appendChild(row.firstChild);
          row.appendChild(control);
        } else {
          row.insertBefore(control, row.firstChild);
        }
        control.addEventListener('click', () => {
          children.hidden = !children.hidden;
          control.setAttribute('aria-expanded', String(!children.hidden));
        });
      } else {
        const spacer = document.createElement('span');
        spacer.className = 'workbook-filetree__spacer';
        spacer.setAttribute('aria-hidden', 'true');
        row.insertBefore(holder, row.firstChild);
        row.insertBefore(spacer, holder);
      }
      li.dataset.filetreeIconReady = 'true';
    }
  }
  function init() {
    document.querySelectorAll('.workbook-filetree[data-icon-set]').forEach(decorate);
  }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', init, {once:true});
  else init();
})();


/* Starlight-style tabs: progressive enhancement, ARIA, keyboard support and syncKey. */
(() => {
  'use strict';
  let nextGroup = 0;
  const groups = new Set();
  function directPanels(group) {
    return Array.from(group.children).filter(el => el.classList.contains('workbook-tab-panel'));
  }
  function directButtons(group) {
    return Array.from(group.querySelector(':scope > .workbook-tab-list')?.children || []);
  }
  function storedLabel(key) {
    if (!key) return null;
    try { return localStorage.getItem('mdbook-components:tabs:' + key); }
    catch { return null; }
  }
  function storeLabel(key, label) {
    if (!key) return;
    try { localStorage.setItem('mdbook-components:tabs:' + key, label); }
    catch { /* Restricted storage must never break interaction. */ }
  }
  function activate(group, index, { focus = false, broadcast = true } = {}) {
    const panels = directPanels(group);
    if (index < 0 || index >= panels.length) return;
    const buttons = directButtons(group);
    if (buttons.length !== panels.length) return;
    panels.forEach((panel, i) => {
      const selected = i === index;
      panel.hidden = !selected;
      buttons[i].setAttribute('aria-selected', String(selected));
      buttons[i].tabIndex = selected ? 0 : -1;
    });
    if (focus) buttons[index].focus();
    if (broadcast && group.dataset.syncKey) {
      const key = group.dataset.syncKey;
      const label = panels[index].dataset.label;
      storeLabel(key, label);
      groups.forEach(other => {
        if (other !== group && other.dataset.syncKey === key) {
          const target = directPanels(other).findIndex(p => p.dataset.label === label);
          if (target !== -1) activate(other, target, { broadcast: false });
        }
      });
    }
  }
  function initGroup(group) {
    if (group.dataset.tabsReady === 'true') return;
    const panels = directPanels(group);
    if (!panels.length) return;
    const groupId = 'workbook-tabs-' + (++nextGroup);
    const list = document.createElement('div');
    list.className = 'workbook-tab-list';
    list.setAttribute('role', 'tablist');
    list.setAttribute('aria-label', 'Content options');
    panels.forEach((panel, i) => {
      const tab = document.createElement('button');
      tab.type = 'button';
      tab.className = 'workbook-tab';
      tab.id = groupId + '-tab-' + i;
      tab.setAttribute('role', 'tab');
      tab.setAttribute('aria-controls', groupId + '-panel-' + i);
      tab.setAttribute('aria-selected', 'false');
      tab.tabIndex = -1;
      const svg = window.MDCIcons.create(panel.dataset.icon || '', 'workbook-tab-icon');
      if (svg) tab.append(svg);
      const name = document.createElement('span');
      name.textContent = panel.dataset.label || `Tab ${i + 1}`;
      tab.append(name);
      panel.id = groupId + '-panel-' + i;
      panel.setAttribute('role', 'tabpanel');
      panel.setAttribute('aria-labelledby', tab.id);
      panel.tabIndex = 0;
      tab.addEventListener('click', () => activate(group, i));
      tab.addEventListener('keydown', (event) => {
        let next;
        if (event.key === 'ArrowRight') next = (i + 1) % panels.length;
        else if (event.key === 'ArrowLeft') next = (i - 1 + panels.length) % panels.length;
        else if (event.key === 'Home') next = 0;
        else if (event.key === 'End') next = panels.length - 1;
        else return;
        event.preventDefault();
        activate(group, next, { focus: true });
      });
      list.append(tab);
    });
    group.prepend(list);
    group.dataset.tabsReady = 'true';
    groups.add(group);
    const preferred = storedLabel(group.dataset.syncKey);
    const index = panels.findIndex(p => p.dataset.label === preferred);
    activate(group, index >= 0 ? index : 0, { broadcast: false });
  }
  function init() {
    // Each group only manages direct children; nested tab groups remain independent.
    document.querySelectorAll('.workbook-tabs').forEach(initGroup);
  }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', init, { once: true });
  else init();
})();


/* mdbook-components: line-oriented code enhancement.
 * Syntax colouring remains owned by mdBook/Highlight.js.
 * Numbers are CSS generated content, never copied as code text.
 */
(function () {
  'use strict';

  function getMeta(code, key) {
    if (code.dataset && Object.hasOwn(code.dataset, key)) return code.dataset[key];
    const name = key.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const cls = code.getAttribute('class') || '';
    const braced = cls.match(new RegExp('\\{' + name + '=([^}]*)\\}'));
    if (braced) return braced[1];
    // mdBook may retain fence attributes in the class list as
    // `collapse={1-60}` rather than emitting a data-collapse attribute.
    const wrappedValue = cls.match(new RegExp('(?:^|\\s)' + name + '=\\{([^}]*)\\}'));
    if (wrappedValue) return `{${wrappedValue[1]}}`;
    const plain = cls.match(new RegExp('(?:^|\\s)' + name + '=([^\\s{}]+)'));
    return plain ? plain[1] : null;
  }

  function ranges(spec, limit, { min = 1 } = {}) {
    const set = new Set();
    if (!spec || typeof spec !== 'string') return set;
    const cleaned = spec.replace(/^\{/, '').replace(/\}$/, '');
    for (const part of cleaned.split(/[,\s]+/)) {
      const m = /^(\d+)(?:-(\d+))?$/.exec(part);
      if (!m) continue;
      const a = Number(m[1]);
      const b = m[2] ? Number(m[2]) : a;
      for (let n = Math.max(min, Math.min(a, b)); n <= Math.min(limit, Math.max(a, b)); n++) {
        set.add(n);
      }
    }
    return set;
  }

  // Split highlighted DOM without destroying span nesting or token classes.
  function splitNode(node) {
    if (node.nodeType === Node.TEXT_NODE) {
      return node.textContent.replace(/\r\n?/g, '\n').split('\n')
        .map(text => document.createTextNode(text));
    }
    if (node.nodeType !== Node.ELEMENT_NODE) return [node.cloneNode(true)];
    const out = [node.cloneNode(false)];
    for (const child of node.childNodes) {
      const pieces = splitNode(child);
      out[out.length - 1].append(pieces[0]);
      for (let i = 1; i < pieces.length; i++) {
        const copy = node.cloneNode(false);
        copy.append(pieces[i]);
        out.push(copy);
      }
    }
    return out;
  }

  function splitLines(code) {
    const out = [document.createDocumentFragment()];
    for (const child of code.childNodes) {
      const pieces = splitNode(child);
      out[out.length - 1].append(pieces[0]);
      for (let i = 1; i < pieces.length; i++) {
        const frag = document.createDocumentFragment();
        frag.append(pieces[i]);
        out.push(frag);
      }
    }
    // mdBook may inject a blank line immediately after the opening <code>.
    // Strip that formatting artefact, but preserve later intentional blank lines.
    while (out.length > 1 && out[0].textContent.trim() === '') out.shift();
    // Strip the customary final newline generated by Markdown fences.
    while (out.length > 1 && out[out.length - 1].textContent.trim() === '') out.pop();
    return out;
  }

  function parseConfig(code, count) {
    const rawStart = getMeta(code, 'startLineNumber') ?? getMeta(code, 'start');
    const first = rawStart !== null && /^\d+$/.test(rawStart) ? Number(rawStart) : 1;
    const ln = getMeta(code, 'ln');
    const named = getMeta(code, 'showLineNumbers');
    const numbered = ln === 'false' || named === 'false' || code.classList.contains('no-ln')
      ? false : ln === 'true' || named === 'true' || rawStart !== null || code.classList.contains('showLineNumbers');
    // hl uses displayed numbers for backward compatibility with previous component.js.
    const highlights = ranges(getMeta(code, 'hl'), first + count - 1, { min: first });
    // collapse refers to 1-based source positions, regardless of startLineNumber.
    const collapsed = ranges(getMeta(code, 'collapse'), count);
    const style = getMeta(code, 'collapseStyle') || 'start';
    const wrap = getMeta(code, 'wrap') === 'true';
    return { first, numbered, highlights, collapsed, style, wrap };
  }

  // Rust playground hides its injected setup lines inside `.boring` spans.
  // They remain in the DOM for mdBook's Show hidden lines button, but should
  // not affect the numbering or highlight coordinates of the visible example.
  function isPlaygroundHiddenLine(fragment, hideBoring) {
    if (!hideBoring || !fragment.querySelector('.boring')) return false;
    const visible = fragment.cloneNode(true);
    visible.querySelectorAll('.boring').forEach(node => node.remove());
    // An ordinary blank line should still occupy a visible source line.
    return visible.textContent.trim() === '' && fragment.textContent.trim() !== '';
  }

  function makeLine(fragment, sourceIndex, displayIndex, cfg) {
    const line = document.createElement('span');
    line.className = 'ec-line hljs-code-line';
    line.dataset.sourceLine = String(sourceIndex + 1);
    if (displayIndex !== null) {
      const number = cfg.first + displayIndex;
      line.dataset.lineNumber = String(number);
      if (cfg.highlights.has(number)) line.classList.add('ec-highlight', 'hljs-code-hl');
    } else {
      line.classList.add('ec-playground-hidden');
    }
    const source = document.createElement('span');
    source.className = 'ec-source';
    source.append(fragment);
    line.append(source);
    return line;
  }

  function appendCollapsed(code, run, cfg) {
    const details = document.createElement('details');
    details.className = 'ec-collapse';
    // 'start' is the default; only 'end' explicitly moves the disclosure.
    details.dataset.mode = cfg.style === 'end' ? 'end' : 'start';
    const summary = document.createElement('summary');
    summary.className = 'ec-collapse-summary';
    summary.textContent = `${run.length} collapsed line${run.length === 1 ? '' : 's'} — show code`;
    const content = document.createElement('span');
    content.className = 'ec-collapse-content';
    content.append(...run);
    details.append(summary, content);
    code.append(details);
    details.addEventListener('toggle', () => {
      summary.textContent = details.open
        ? `${run.length} lines — hide code` : `${run.length} collapsed line${run.length === 1 ? '' : 's'} — show code`;
    });
  }

  function enhance(code) {
    if (code.dataset.ecProcessed === 'true' || code.closest('.hljs-ln-container')) return;
    const pre = code.parentElement;
    if (!pre || pre.tagName !== 'PRE') return;
    // Do not consume raw source until browser-side highlighting has completed.
    if (typeof hljs !== 'undefined' && !code.classList.contains('hljs') && !code.classList.contains('nohighlight')) {
      try { hljs.highlightElement(code); } catch (_) { /* keep readable plain text */ }
    }
    const rawText = code.textContent.replace(/\r\n?/g, '\n');
    const sourceText = rawText.replace(/^(?:[ \t]*\n)+/, '').replace(/(?:\n[ \t]*)+$/, '');
    let fragments = splitLines(code);
    const hideBoring = code.classList.contains('hide-boring');
    // Preserve the displayed source separately from mdBook's runnable source.
    // Playground boilerplate is marked with .boring and hidden by default.
    const visibleSourceText = fragments
      .filter(fragment => !isPlaygroundHiddenLine(fragment, true))
      .map(fragment => fragment.textContent).join('\n');
    // The original fence text, not Highlight.js's generated spans, defines
    // source-line coordinates. Highlighting can introduce extra empty DOM
    // fragments; in that case use exact source rows rather than leaving
    // lines outside a requested collapse={1-N} range.
    if (!hideBoring) {
      const sourceLines = sourceText.split('\n');
      if (fragments.length !== sourceLines.length ||
          fragments.some((fragment, index) => fragment.textContent !== sourceLines[index])) {
        fragments = sourceLines.map(text => {
          const fragment = document.createDocumentFragment();
          fragment.append(document.createTextNode(text));
          return fragment;
        });
      }
    }
    const visibleCount = fragments.filter(fragment => !isPlaygroundHiddenLine(fragment, hideBoring)).length;
    const cfg = parseConfig(code, visibleCount);
    cfg.lineCount = visibleCount;
    // Use exactly the same code layout with or without line numbers.
    code.dataset.ecProcessed = 'true';
    code.classList.add('ec-code');
    if (cfg.numbered) code.classList.add('ec-numbered');
    if (cfg.wrap) pre.classList.add('ec-wrap');
    code.replaceChildren();
    let visibleIndex = 0;
    const lines = fragments.map((fragment, i) => {
      const hidden = isPlaygroundHiddenLine(fragment, hideBoring);
      return makeLine(fragment, i, hidden ? null : visibleIndex++, cfg);
    });
    // Row elements already create visual line breaks. Keep a hidden separator
    // for DOM consumers (e.g. the mdBook Rust Playground) that read textContent.
    // Unlike bare newline text nodes it does not create extra rendered rows.
    function appendSeparator() {
      const separator = document.createElement('span');
      separator.className = 'ec-line-separator';
      separator.textContent = '\n';
      code.append(separator);
    }
    // collapse coordinates refer to visible lines, not Highlight.js fragments
    // or Rust Playground setup lines hidden with #.
    const isCollapsed = line => line.dataset.lineNumber &&
      cfg.collapsed.has(Number(line.dataset.lineNumber) - cfg.first + 1);
    for (let i = 0; i < lines.length;) {
      if (!isCollapsed(lines[i])) {
        code.append(lines[i]);
        if (i < lines.length - 1) appendSeparator();
        i++;
        continue;
      }
      const run = [];
      while (i < lines.length && isCollapsed(lines[i])) run.push(lines[i++]);
      appendCollapsed(code, run, cfg);
      if (i < lines.length) appendSeparator();
    }
    pre.classList.add('ec-pre');
    // mdBook's clipboard button copies code.textContent. Intercept only this
    // enhanced block so the copied source never includes disclosure labels.
    pre.dataset.ecSource = sourceText;
    pre.dataset.ecVisibleSource = visibleSourceText;
    // mdBook's default copy handler may target <code>. Preserve the original
    // source independently of highlight.js line wrappers and collapsed labels.
    code.dataset.ecSource = sourceText;
  }

  function copyHandler(event) {
    const button = event.target.closest('.clip-button');
    if (!button) return;
    const pre = button.closest('pre.ec-pre');
    if (!pre) return;
    event.preventDefault();
    event.stopImmediatePropagation();
    const code = pre.querySelector('code');
    const hideBoring = code?.classList.contains('hide-boring');
    const source = hideBoring
      ? (pre.dataset.ecVisibleSource ?? '')
      : (pre.dataset.ecSource ?? '');
    // The original fence text is stored before individual line <span>s are
    // created: textContent of the enhanced DOM would concatenate the lines.
    if (navigator.clipboard?.writeText) {
      navigator.clipboard.writeText(source).catch(() => fallbackCopy(source));
    } else {
      fallbackCopy(source);
    }
  }

  function fallbackCopy(source) {
    const field = document.createElement('textarea');
    field.value = source;
    field.style.position = 'fixed';
    field.style.opacity = '0';
    document.body.appendChild(field);
    field.select();
    try { document.execCommand('copy'); } finally { field.remove(); }
  }

  // Version marker to distinguish fresh builds from stale hashed assets.
  window.__mdbookComponentsCodeVersion = 'source-preserving-v6-collapse-alignment';

  function initialise() {
    document.querySelectorAll('pre > code').forEach(enhance);
  }

  document.addEventListener('click', copyHandler, true);
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initialise, { once: true });
  } else {
    initialise();
  }
})();


/* Responsive tables — standard Markdown syntax, no new directive.
   Wrap only content tables. Never modify their rows or cells. */
(() => {
  'use strict';

  function initialiseTables() {
    document.querySelectorAll('.content main table').forEach(table => {
      if (table.closest('.mdc-table-scroll')) return;
      // Nested HTML tables are uncommon, but may have distinct scroll needs.
      const wrapper = document.createElement('div');
      wrapper.className = 'mdc-table-scroll';
      wrapper.setAttribute('role', 'region');
      wrapper.setAttribute('aria-label', 'Scrollable table');
      wrapper.tabIndex = 0;
      table.parentNode.insertBefore(wrapper, table);
      wrapper.appendChild(table);
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initialiseTables, { once: true });
  } else {
    initialiseTables();
  }
})();
