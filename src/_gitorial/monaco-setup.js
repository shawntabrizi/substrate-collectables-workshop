
(function () {
  const MONACO_BASE = 'https://cdn.jsdelivr.net/npm/monaco-editor@0.47.0/min/vs';
  const MONACO_TIMEOUT_MS = 3500;

  function loadJson(url) {
    return fetch(url).then((res) => {
      if (!res.ok) {
        throw new Error('Failed to load ' + url);
      }
      return res.json();
    });
  }

  function detectMode(filePath) {
    const ext = filePath.split('.').pop();
    switch (ext) {
      case 'rs':
        return 'rust';
      case 'toml':
        return 'toml';
      case 'js':
        return 'javascript';
      case 'json':
        return 'json';
      case 'ts':
        return 'typescript';
      case 'patch':
      case 'diff':
        return 'diff';
      default:
        return 'plaintext';
    }
  }

  function setButtonState(button, isPrimary) {
    if (isPrimary) {
      button.classList.remove('secondary');
    } else {
      button.classList.add('secondary');
    }
  }

  function escapeHtml(value) {
    return String(value)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
  }

  function escapeAttribute(value) {
    return String(value).replace(/"/g, '&quot;');
  }

  function mapToHighlightLanguage(language) {
    switch (language) {
      case 'rust':
        return 'rust';
      case 'toml':
        return 'toml';
      case 'javascript':
        return 'javascript';
      case 'json':
        return 'json';
      case 'typescript':
        return 'typescript';
      case 'diff':
        return 'diff';
      case 'markdown':
        return 'markdown';
      default:
        return '';
    }
  }

  function runHighlight(root) {
    if (!window.hljs) {
      return false;
    }
    const codeBlocks = root.querySelectorAll('code[data-gitorial-lang]');
    if (!codeBlocks.length) {
      return true;
    }
    const highlightElement = typeof window.hljs.highlightElement === 'function';
    const highlightBlock = typeof window.hljs.highlightBlock === 'function';
    if (!highlightElement && !highlightBlock) {
      return false;
    }
    codeBlocks.forEach((codeBlock) => {
      try {
        if (highlightElement) {
          window.hljs.highlightElement(codeBlock);
        } else {
          window.hljs.highlightBlock(codeBlock);
        }
      } catch (_error) {
        // Keep plain fallback text if highlighting fails.
      }
    });
    return true;
  }

  function applyFallbackHighlight(root) {
    if (runHighlight(root)) {
      return;
    }
    let attempts = 0;
    const maxAttempts = 30;
    const retryMs = 120;
    const timer = setInterval(() => {
      attempts += 1;
      if (runHighlight(root) || attempts >= maxAttempts) {
        clearInterval(timer);
      }
    }, retryMs);
  }

  function renderFallback(editorNode, payload) {
    let fallback = editorNode.querySelector('.gitorial-monaco-fallback');
    if (!fallback) {
      fallback = document.createElement('div');
      fallback.className = 'gitorial-monaco-fallback';
      editorNode.appendChild(fallback);
    }

    const highlightLanguage = mapToHighlightLanguage(payload.language || '');
    const codeClass = highlightLanguage ? 'language-' + highlightLanguage : 'language-plaintext';
    const codeMeta = 'data-gitorial-lang="' + escapeAttribute(highlightLanguage || 'plaintext') + '"';

    if (payload.type === 'diff') {
      fallback.innerHTML =
        '<div class="fallback-diff">' +
        '<section class="fallback-pane">' +
        '<h4>Template</h4>' +
        '<pre><code class="' +
        codeClass +
        '" ' +
        codeMeta +
        '>' +
        escapeHtml(payload.original || '') +
        '</code></pre>' +
        '</section>' +
        '<section class="fallback-pane">' +
        '<h4>Solution</h4>' +
        '<pre><code class="' +
        codeClass +
        '" ' +
        codeMeta +
        '>' +
        escapeHtml(payload.modified || '') +
        '</code></pre>' +
        '</section>' +
        '</div>';
      applyFallbackHighlight(fallback);
      fallback.classList.remove('hidden');
      return;
    }

    fallback.innerHTML =
      '<pre><code class="' +
      codeClass +
      '" ' +
      codeMeta +
      '>' +
      escapeHtml(payload.content || '') +
      '</code></pre>';
    applyFallbackHighlight(fallback);
    fallback.classList.remove('hidden');
  }

  function createMonacoSession(editorNode, onReady, onFailure, onEdit) {
    const iframe = document.createElement('iframe');
    iframe.setAttribute('title', 'Gitorial Editor');
    iframe.className = 'gitorial-monaco-frame';
    iframe.style.display = 'none';

    const iframeDocument = [
      '<!doctype html>',
      '<html>',
      '<head>',
      '<meta charset="utf-8" />',
      '<style>html, body { margin: 0; height: 100%; background: #0f0f10; } #editor { height: 100%; } .monaco-editor .gitorial-todo-line { background: rgba(255, 196, 0, 0.12); } .monaco-editor .margin .gitorial-todo-glyph { border-left: 3px solid #ffc400; box-sizing: border-box; }</style>',
      '</head>',
      '<body>',
      '<div id="editor"></div>',
      '<script>',
      '(function () {',
      "  var MONACO_BASE = '" + MONACO_BASE + "';",
      '  var editor = null;',
      '  var diffEditor = null;',
      '  var changeSubscription = null;',
      '  var todoDecorations = [];',
      '  var highlightTodos = false;',
      "  var currentMode = 'single';",
      '  var pendingPayload = null;',
      '  var isReady = false;',
      '  var suppressChange = false;',
      '  function post(type, detail) {',
      "    parent.postMessage({ source: 'gitorial-monaco', type: type, detail: detail }, '*');",
      '  }',
      '  function getContainer() { return document.getElementById(\'editor\'); }',
      '  function resetContainer() {',
      '    var container = getContainer();',
      '    container.innerHTML = "";',
      '  }',
      '  function attachChangeHandler() {',
      '    if (!editor) { return; }',
      '    if (changeSubscription) {',
      '      changeSubscription.dispose();',
      '      changeSubscription = null;',
      '    }',
      '    changeSubscription = editor.onDidChangeModelContent(function () {',
      '      if (suppressChange) { return; }',
      '      syncTodoDecorations();',
      "      post('change', editor.getValue());",
      '    });',
      '  }',
      '  function clearTodoDecorations() {',
      '    if (!editor) { return; }',
      '    todoDecorations = editor.deltaDecorations(todoDecorations, []);',
      '  }',
      '  function syncTodoDecorations() {',
      '    if (!editor) { return; }',
      '    if (!highlightTodos) {',
      '      clearTodoDecorations();',
      '      return;',
      '    }',
      '    var model = editor.getModel();',
      '    if (!model) {',
      '      clearTodoDecorations();',
      '      return;',
      '    }',
      '    var matches = model.findMatches("TODO", true, false, false, null, true);',
      '    var next = matches.map(function (match) {',
      '      return {',
      '        range: match.range,',
      '        options: {',
      '          isWholeLine: true,',
      '          className: "gitorial-todo-line",',
      '          linesDecorationsClassName: "gitorial-todo-glyph"',
      '        }',
      '      };',
      '    });',
      '    todoDecorations = editor.deltaDecorations(todoDecorations, next);',
      '  }',
      '  function disposeEditorModel() {',
      '    if (!editor) { return; }',
      '    var model = editor.getModel();',
      '    if (model) { model.dispose(); }',
      '  }',
      '  function disposeDiffModels() {',
      '    if (!diffEditor) { return; }',
      '    var models = diffEditor.getModel();',
      '    if (!models) { return; }',
      '    if (models.original) { models.original.dispose(); }',
      '    if (models.modified) { models.modified.dispose(); }',
      '  }',
      '  function disposeEditor() {',
      '    if (!editor) { return; }',
      '    if (changeSubscription) {',
      '      changeSubscription.dispose();',
      '      changeSubscription = null;',
      '    }',
      '    disposeEditorModel();',
      '    todoDecorations = [];',
      '    editor.dispose();',
      '    editor = null;',
      '  }',
      '  function disposeDiffEditor() {',
      '    if (!diffEditor) { return; }',
      '    disposeDiffModels();',
      '    diffEditor.dispose();',
      '    diffEditor = null;',
      '  }',
      '  function ensureEditor(content, language, readOnly) {',
      "    if (currentMode !== 'single') {",
      '      disposeDiffEditor();',
      '      resetContainer();',
      "      currentMode = 'single';",
      '    }',
      '    if (!editor) {',
      '      editor = monaco.editor.create(getContainer(), {',
      '        value: content,',
      '        language: language,',
      "        theme: 'vs-dark',",
      '        automaticLayout: true,',
      '        readOnly: !!readOnly',
      '      });',
      '      highlightTodos = !readOnly;',
      '      syncTodoDecorations();',
      '      attachChangeHandler();',
      '      return;',
      '    }',
      '    editor.updateOptions({ readOnly: !!readOnly });',
      '    highlightTodos = !readOnly;',
      '    suppressChange = true;',
      '    disposeEditorModel();',
      '    editor.setModel(monaco.editor.createModel(content, language));',
      '    suppressChange = false;',
      '    syncTodoDecorations();',
      '  }',
      '  function ensureDiffEditor(original, modified, language) {',
      "    if (currentMode !== 'diff') {",
      '      disposeEditor();',
      '      resetContainer();',
      "      currentMode = 'diff';",
      '    }',
      '    if (!diffEditor) {',
      '      diffEditor = monaco.editor.createDiffEditor(getContainer(), {',
      "        theme: 'vs-dark',",
      '        automaticLayout: true,',
      '        readOnly: true,',
      '        renderSideBySide: true',
      '      });',
      '    }',
      '    disposeDiffModels();',
      '    var originalModel = monaco.editor.createModel(original, language);',
      '    var modifiedModel = monaco.editor.createModel(modified, language);',
      '    diffEditor.setModel({ original: originalModel, modified: modifiedModel });',
      '  }',
      '  function renderPayload(payload) {',
      "    if (payload.type === 'diff') {",
      "      ensureDiffEditor(payload.original || '', payload.modified || '', payload.language || 'plaintext');",
      '      return;',
      '    }',
      "    ensureEditor(payload.content || '', payload.language || 'plaintext', !!payload.readOnly);",
      '  }',
      '  window.addEventListener(\'message\', function (event) {',
      '    var data = event.data || {};',
      "    if (data.source !== 'gitorial-monaco-host' || !data.type) { return; }",
      '    if (!isReady) {',
      '      pendingPayload = data;',
      '      return;',
      '    }',
      '    renderPayload(data);',
      '  });',
      '  function bootMonaco() {',
      "    if (typeof window.require !== 'function') {",
      "      post('error', 'require_missing');",
      '      return;',
      '    }',
      '    window.require.config({ paths: { vs: MONACO_BASE } });',
      "    window.require(['vs/editor/editor.main'], function () {",
      '      isReady = true;',
      "      post('ready');",
      '      if (pendingPayload) {',
      '        renderPayload(pendingPayload);',
      '        pendingPayload = null;',
      '      }',
      '    }, function (err) {',
      "      post('error', String(err || 'require_failed'));",
      '    });',
      '  }',
      "  var loader = document.createElement('script');",
      "  loader.src = MONACO_BASE + '/loader.js';",
      '  loader.async = true;',
      "  loader.onload = bootMonaco;",
      "  loader.onerror = function () { post('error', 'loader_failed'); };",
      '  document.head.appendChild(loader);',
      '})();',
      '</script>',
      '</body>',
      '</html>',
    ].join('');

    let settled = false;
    let timeoutId = null;

    function clearTimer() {
      if (timeoutId) {
        clearTimeout(timeoutId);
        timeoutId = null;
      }
    }

    function cleanUp() {
      clearTimer();
      window.removeEventListener('message', onMessage);
    }

    function onMessage(event) {
      if (event.source !== iframe.contentWindow) {
        return;
      }
      const data = event.data || {};
      if (data.source !== 'gitorial-monaco') {
        return;
      }
      if (data.type === 'change') {
        onEdit(data.detail || '');
        return;
      }
      if (data.type === 'ready') {
        if (settled) {
          return;
        }
        settled = true;
        clearTimer();
        onReady(iframe);
        return;
      }
      if (data.type === 'error') {
        if (settled) {
          return;
        }
        settled = true;
        cleanUp();
        onFailure(data.detail || 'load_error');
      }
    }

    timeoutId = setTimeout(() => {
      if (settled) {
        return;
      }
      settled = true;
      cleanUp();
      onFailure('timeout');
    }, MONACO_TIMEOUT_MS);

    window.addEventListener('message', onMessage);
    editorNode.appendChild(iframe);
    iframe.srcdoc = iframeDocument;

    return {
      iframe,
      post(payload) {
        if (!iframe.contentWindow) {
          return;
        }
        iframe.contentWindow.postMessage({ source: 'gitorial-monaco-host', ...payload }, '*');
      },
      destroy() {
        cleanUp();
        if (iframe.parentNode) {
          iframe.parentNode.removeChild(iframe);
        }
      },
    };
  }

  async function initMonaco(container) {
    const manifestUrl = container.dataset.manifest;
    const config = await loadJson(manifestUrl);

    const toolbar = container.querySelector('[data-gitorial-toolbar]');
    const select = container.querySelector('[data-gitorial-files]');
    const toggle = container.querySelector('[data-gitorial-toggle]');
    const diffToggle = container.querySelector('[data-gitorial-diff]');
    const copyToggle = container.querySelector('[data-gitorial-copy]');
    const footer = container.querySelector('[data-gitorial-footer]');
    const editorNode = container.querySelector('[data-gitorial-editor]');
    const retryToggle = document.createElement('button');
    retryToggle.type = 'button';
    retryToggle.className = 'retry-toggle';
    retryToggle.textContent = 'Retry editor';
    toolbar.appendChild(retryToggle);
    copyToggle.disabled = true;

    let currentMode = 'template';
    let previousMode = 'template';
    let selectedLabel = null;
    const templateFiles = config.template || [];
    const solutionFiles = config.solution || [];
    const fileContentCache = new Map();
    const templateEdits = new Map();
    let monacoSession = null;
    let monacoReady = false;
    let lastPayload = null;
    let copyState = { text: '', label: '' };

    if (!solutionFiles.length) {
      toggle.style.display = 'none';
      footer.textContent = 'Template view only.';
      diffToggle.style.display = 'none';
    } else {
      footer.textContent = 'Template view. Click View solution to compare.';
    }

    function formatOptionLabel(file) {
      if (!file) {
        return '';
      }
      let marker = '';
      if (file.status === 'A') {
        marker = '[+]';
      } else if (file.status === 'M') {
        marker = '[~]';
      } else if (file.status === 'D') {
        marker = '[-]';
      } else {
        marker = '[ ]';
      }
      return marker + ' ' + file.label;
    }

    function updateFileOptions() {
      let list = currentMode === 'template' ? templateFiles : solutionFiles;
      if (currentMode === 'diff') {
        const byLabel = new Map();
        templateFiles.forEach((file) => byLabel.set(file.label, file));
        solutionFiles.forEach((file) => {
          if (byLabel.has(file.label)) {
            const existing = byLabel.get(file.label);
            byLabel.set(file.label, { ...existing, status: file.status || existing.status });
          } else {
            byLabel.set(file.label, file);
          }
        });
        list = Array.from(byLabel.values());
      }
      select.innerHTML = '';
      list.forEach((file, index) => {
        const option = document.createElement('option');
        option.value = file.label;
        option.textContent = formatOptionLabel(file);
        if (selectedLabel && selectedLabel === file.label) {
          option.selected = true;
        } else if (!selectedLabel && index === 0) {
          option.selected = true;
        }
        select.appendChild(option);
      });
      if (!selectedLabel && list.length) {
        selectedLabel = list[0].label;
      }
    }

    function getFileContent(filePath) {
      if (fileContentCache.has(filePath)) {
        return Promise.resolve(fileContentCache.get(filePath));
      }
      return fetch(filePath)
        .then((res) => res.text())
        .then((text) => {
          fileContentCache.set(filePath, text);
          return text;
        });
    }

    function findFile(list, label) {
      return list.find((file) => file.label === label) || null;
    }

    async function copyTextToClipboard(text) {
      if (!text) {
        return false;
      }
      if (navigator.clipboard && typeof navigator.clipboard.writeText === 'function') {
        try {
          await navigator.clipboard.writeText(text);
          return true;
        } catch (_error) {}
      }
      const textarea = document.createElement('textarea');
      textarea.value = text;
      textarea.setAttribute('readonly', 'readonly');
      textarea.style.position = 'absolute';
      textarea.style.left = '-9999px';
      document.body.appendChild(textarea);
      textarea.select();
      let copied = false;
      try {
        copied = document.execCommand('copy');
      } catch (_error) {
        copied = false;
      }
      document.body.removeChild(textarea);
      return copied;
    }

    function setCopyState(text, label) {
      copyState = { text: text || '', label: label || '' };
      copyToggle.disabled = !copyState.text;
    }

    function flashCopyLabel(label) {
      copyToggle.textContent = label;
      setTimeout(() => {
        copyToggle.textContent = 'Copy code';
      }, 1300);
    }

    function setRetryVisible(visible) {
      retryToggle.style.display = visible ? 'inline-block' : 'none';
    }

    function startMonacoSession() {
      if (monacoSession) {
        monacoSession.destroy();
        monacoSession = null;
      }
      monacoReady = false;
      setRetryVisible(false);
      monacoSession = createMonacoSession(
        editorNode,
        (iframe) => {
          monacoReady = true;
          iframe.style.display = 'block';
          const fallback = editorNode.querySelector('.gitorial-monaco-fallback');
          if (fallback) {
            fallback.classList.add('hidden');
          }
          if (lastPayload) {
            monacoSession.post(lastPayload);
          }
        },
        (reason) => {
          monacoReady = false;
          if (monacoSession) {
            monacoSession.destroy();
            monacoSession = null;
          }
          setRetryVisible(true);
          footer.textContent = 'Monaco is unavailable (' + reason + '). Using fallback renderer.';
        },
        (content) => {
          if (currentMode !== 'template' || !selectedLabel) {
            return;
          }
          templateEdits.set(selectedLabel, content);
          setCopyState(content, selectedLabel + ' (template)');
        }
      );
    }

    function renderPayload(payload) {
      lastPayload = payload;
      if (monacoReady && monacoSession) {
        const fallback = editorNode.querySelector('.gitorial-monaco-fallback');
        if (fallback) {
          fallback.classList.add('hidden');
        }
        monacoSession.post(payload);
        return;
      }
      renderFallback(editorNode, payload);
    }

    async function setEditorFile(label) {
      if (!label) {
        return;
      }
      if (currentMode === 'diff') {
        const templateFile = findFile(templateFiles, label);
        const solutionFile = findFile(solutionFiles, label);
        let original = '';
        if (templateEdits.has(label)) {
          original = templateEdits.get(label);
        } else if (templateFile) {
          original = await getFileContent(templateFile.path);
        }
        const modified = solutionFile ? await getFileContent(solutionFile.path) : '';
        const language = detectMode((solutionFile || templateFile || { path: '' }).path);
        renderPayload({ type: 'diff', original, modified, language });
        if (solutionFile) {
          setCopyState(modified, label + ' (solution)');
        } else {
          setCopyState(original, label + ' (template)');
        }
        return;
      }

      const list = currentMode === 'template' ? templateFiles : solutionFiles;
      const file = findFile(list, label);
      if (!file) {
        return;
      }
      let content = '';
      if (currentMode === 'template' && templateEdits.has(label)) {
        content = templateEdits.get(label);
      } else {
        content = await getFileContent(file.path);
      }
      const language = detectMode(file.path);
      renderPayload({ type: 'set', content, language, readOnly: currentMode !== 'template' });
      setCopyState(content, label + ' (' + currentMode + ')');
    }

    updateFileOptions();
    if (!select.value) {
      footer.textContent = 'No files available for this step.';
      toggle.style.display = 'none';
      return;
    }
    selectedLabel = select.value;
    await setEditorFile(selectedLabel);
    startMonacoSession();

    select.addEventListener('change', async () => {
      selectedLabel = select.value;
      await setEditorFile(selectedLabel);
    });

    toggle.addEventListener('click', async () => {
      if (!solutionFiles.length) {
        return;
      }
      currentMode = currentMode === 'template' ? 'solution' : 'template';
      updateFileOptions();
      setButtonState(toggle, currentMode === 'template');
      toggle.textContent = currentMode === 'template' ? 'View solution' : 'Back to template';
      toggle.disabled = currentMode === 'diff';
      footer.textContent =
        currentMode === 'template'
          ? 'Template view. Click View solution to compare.'
          : 'Solution view. Click Back to template to continue.';
      if (select.value) {
        selectedLabel = select.value;
        await setEditorFile(selectedLabel);
      }
    });

    diffToggle.addEventListener('click', async () => {
      if (!solutionFiles.length) {
        return;
      }
      if (currentMode === 'diff') {
        currentMode = previousMode;
        diffToggle.textContent = 'View diff';
      } else {
        previousMode = currentMode;
        currentMode = 'diff';
        diffToggle.textContent = 'Back to code';
      }
      updateFileOptions();
      toggle.disabled = currentMode === 'diff';
      if (monacoSession) {
        startMonacoSession();
      }
      if (select.value) {
        selectedLabel = select.value;
        await setEditorFile(selectedLabel);
      }
    });

    copyToggle.addEventListener('click', async () => {
      if (!copyState.text) {
        return;
      }
      const ok = await copyTextToClipboard(copyState.text);
      if (ok) {
        flashCopyLabel('Copied');
        return;
      }
      footer.textContent = 'Copy failed for ' + copyState.label + '.';
      flashCopyLabel('Copy failed');
    });

    retryToggle.addEventListener('click', () => {
      startMonacoSession();
      if (!solutionFiles.length) {
        footer.textContent = 'Template view only.';
      } else if (currentMode === 'template') {
        footer.textContent = 'Template view. Click View solution to compare.';
      } else {
        footer.textContent = 'Solution view. Click Back to template to continue.';
      }
    });
  }

  function boot() {
    const containers = document.querySelectorAll('[data-gitorial-monaco]');
    if (!containers.length) {
      return;
    }
    if (document.body) {
      document.body.classList.add('gitorial-has-editor');
    }
    containers.forEach((container) => {
      if (container.dataset.gitorialInitialized === 'true') {
        return;
      }
      container.dataset.gitorialInitialized = 'true';
      initMonaco(container).catch((error) => {
        console.error(error);
        delete container.dataset.gitorialInitialized;
      });
    });
  }

  window.__gitorialBoot = boot;
})();
