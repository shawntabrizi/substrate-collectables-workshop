window.$docsify.plugins.push(
    function (hook, vm) {
        hook.afterEach(function (html) {
            var parser = new DOMParser();
            var htmlDoc = parser.parseFromString(html, 'text/html');
            if (htmlDoc.getElementsByClassName("lang-embed")[0]) {
                var custom = [
                    '<div class="row">',
                    '<div class="lesson column">', html, '</div>',
                    '<div class="code column"><div id="editor">Failed to load rust code...</div></div>',
                    '</div>'
                ].join('');

                return custom;
            } else {
                return html;
            }
        });

        hook.doneEach(function () {
            var editor = ace.edit("editor");
            editor.setTheme("ace/theme/vibrant_ink");

            var RustMode = ace.require("ace/mode/rust").Mode;
            editor.session.setMode(new RustMode());
            editor.getSession().setUseWrapMode(true);
            editor.session.setWrapLimitRange(0, 80);

            var rust_code_element = document.getElementsByClassName("lang-embed")[0];
            if (rust_code_element) {
                editor.session.setValue(rust_code_element.innerText);

                document.querySelectorAll('[data-lang="embed"]')[0].hidden = true;
            }
        })
    })