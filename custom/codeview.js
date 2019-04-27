require.config({ paths: { 'vs': 'https://unpkg.com/monaco-editor@0.15.6/min/vs' } });

// Before loading vs/editor/editor.main, define a global MonacoEnvironment that overwrites
// the default worker url location (used when creating WebWorkers). The problem here is that
// HTML5 does not allow cross-domain web workers, so we need to proxy the instantiation of
// a web worker through a same-domain script
window.MonacoEnvironment = {
    getWorkerUrl: function (workerId, label) {
        return `data:text/javascript;charset=utf-8,${encodeURIComponent(`
            self.MonacoEnvironment = {
            baseUrl: 'https://unpkg.com/monaco-editor@0.15.6/min/'
            };
            importScripts('https://unpkg.com/monaco-editor@0.15.6/min/vs/base/worker/workerMain.js');`
        )}`;
    }
};

window.$docsify.plugins.push(
    function (hook, vm) {
        console.log("Docsify Window")
        hook.afterEach(function (html) {
            var parser = new DOMParser();
            var htmlDoc = parser.parseFromString(html, 'text/html');
            var template_element = htmlDoc.getElementsByClassName("lang-embed-template")[0];
            var final_element = htmlDoc.getElementsByClassName("lang-embed-final")[0];
            var previous_element = htmlDoc.getElementsByClassName("lang-embed-previous")[0];

            window.code_template = template_element ? template_element.innerText : null;
            window.code_final = final_element ? final_element.innerText : null;
            window.code_previous = previous_element ? previous_element.innerText: null;

            if (template_element || final_element) {
                var two_col = [
                    '<div class="row">',
                    '<div class="lesson column">', html, '</div>',
                    '<div class="code column">',
                    '<div id="editor" class="editor"></div>',
                    '<div id="editor_bar" class="editor-bar"></div>',
                    '</div>',
                    '</div>'
                ].join('');

                return two_col;
            } else {
                var one_col = [
                    '<div class="fullpage">',
                    html,
                    '</div>'
                ].join('')

                return one_col;
            }
        });

        hook.doneEach(function () {
            hintRevealed = false;
            if (document.getElementById("editor")) {
                var editor_bar = document.getElementById("editor_bar");

                if (window.code_previous && window.code_template) {
                    var previous_button = document.createElement("button");
                    previous_button.innerHTML = "&#8656; &#x1D321;";
                    previous_button.classList += "editor-button";
                    previous_button.onclick = function () { loadDiffEditor(window.code_previous, window.code_template); };
                    editor_bar.appendChild(previous_button);
                }

                if (window.code_template) {
                    var template_button = document.createElement("button");
                    template_button.innerHTML = "&#x1F6E0; Starting Point";
                    template_button.classList += "editor-button";
                    template_button.onclick = function () { loadEditor(window.code_template, false, true) };
                    editor_bar.appendChild(template_button);
                    loadEditor(window.code_template, false, true);
                }

                if (window.code_final) {
                    var final_button = document.createElement("button");
                    final_button.innerHTML = "&#x2705; Potential Solution";
                    final_button.classList += "editor-button";
                    final_button.onclick = function () { loadEditor(window.code_final, true, false); };
                    editor_bar.appendChild(final_button);

                    if (!window.code_template) {
                        loadEditor(window.code_final, true, false);
                    }
                }

                if (window.code_template && window.code_final) {
                    var diff_button = document.createElement("button");
                    diff_button.innerHTML = "&#x1D321; Diff View";
                    diff_button.classList += "editor-button";
                    diff_button.onclick = function () { loadDiffEditor(window.code_template, window.code_final); };
                    editor_bar.appendChild(diff_button);
                }
            }
        })
    });

function loadEditor(editor_text, read_only, template_update) {
    require(['vs/editor/editor.main'], function () {
        if (window.monaco_editor) {
            window.view_state = window.monaco_editor.saveViewState();
            window.monaco_editor.dispose();
        }

        window.monaco_editor = monaco.editor.create(document.getElementById('editor'), {
            language: "rust",
            theme: "vs-dark",
            readOnly: read_only,
            automaticLayout: true,
            minimap: {
                enabled: false
            }
        });

        window.monaco_editor.setValue(editor_text)
        if (template_update) {
            window.monaco_editor.onDidChangeModelContent(function () {
                window.code_template = window.monaco_editor.getValue();
            });
        }

        window.monaco_editor.restoreViewState(currentView());

        removeElement(document.getElementsByClassName("docsify-tabs")[0]);
    });
}

function loadDiffEditor(original_text, modified_text) {
    require(['vs/editor/editor.main'], function () {
        if (window.monaco_editor) {
            window.view_state = window.monaco_editor.saveViewState();
            window.monaco_editor.dispose();
        }

        window.monaco_editor = monaco.editor.createDiffEditor(document.getElementById('editor'), {
            language: "rust",
            theme: "vs-dark",
            enableSplitViewResizing: false,
            renderSideBySide: false,
            readOnly: true,
            automaticLayout: true
        });

        var originalModel = monaco.editor.createModel(original_text, "rust");
        var modifiedModel = monaco.editor.createModel(modified_text, "rust");

        window.monaco_editor.setModel({
            original: originalModel,
            modified: modifiedModel
        });

        window.monaco_editor.restoreViewState({ original: currentView() });

        removeElement(document.getElementsByClassName("docsify-tabs")[0]);
    });
}

function removeElement(element) {
    if (element) {
        element.parentNode.removeChild(element);
    }
}

function currentView() {
    if (window.view_state) {
        if (window.view_state.modified) {
            return window.view_state.modified;
        }

        return window.view_state;
    }
}