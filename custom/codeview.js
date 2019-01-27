var template = initializeAceSession();
var final = initializeAceSession();

function initializeAceSession() {
    var session = new ace.EditSession("");
    var RustMode = ace.require("ace/mode/rust").Mode;
    session.setMode(new RustMode());
    session.setUseWrapMode(true);
    session.setWrapLimitRange(0, 80);

    return session;
}

window.$docsify.plugins.push(
    function (hook, vm) {
        hook.afterEach(function (html) {
            var parser = new DOMParser();
            var htmlDoc = parser.parseFromString(html, 'text/html');
            if (htmlDoc.getElementsByClassName("lang-embed-template")[0] ||
                htmlDoc.getElementsByClassName("lang-embed-final")[0])
            {
                var two_col = [
                    '<div class="row">',
                        '<div class="lesson column">', html, '</div>',
                        '<div class="code column">',
                            '<div id="editor">Failed to load rust code...</div>',
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
            if (document.getElementById("editor")) {
                var editor = ace.edit("editor");
                editor.setTheme("ace/theme/vibrant_ink");

                var rust_final = document.getElementsByClassName("lang-embed-final")[0]
                if (rust_final) {
                    final.setValue(rust_final.innerText);
                    editor.setSession(final);

                    document.querySelectorAll('[data-lang="embed-final"]')[0].hidden = true;
                }

                var rust_template = document.getElementsByClassName("lang-embed-template")[0];
                if (rust_template) {
                    template.setValue(rust_template.innerText);
                    editor.setSession(template);

                    document.querySelectorAll('[data-lang="embed-template"]')[0].hidden = true;
                }
            }
        })
    })

function showHint() {
    var editor = ace.edit("editor");
    var scroll = template.getScrollTop();
    final.setScrollTop(scroll);
    editor.setSession(final);
}

function hideHint() {
    var editor = ace.edit("editor");
    editor.setSession(template);
}