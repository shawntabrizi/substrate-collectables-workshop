window.$docsify.plugins.push(
    function (hook, vm) {
        hook.afterEach(function (html) {
            var custom = [
                '<div class="row">',
                '<div class="lesson column">', html, '</div>',
                '<div class="code column"><div id="editor">SomeText</div></div>',
                '</div>'
            ].join('');

            return custom;
        });

        hook.doneEach(function () {
            var editor = ace.edit("editor");
            editor.setTheme("ace/theme/monokai");

            var RustMode = ace.require("ace/mode/rust").Mode;
            editor.session.setMode(new RustMode());

            var rust_code_element = document.getElementsByClassName("lang-embed")[0];
            if (rust_code_element) {
                editor.session.setValue(rust_code_element.innerText);
            
                document.querySelectorAll('[data-lang="embed"]')[0].hidden = true;
            }
        })
    })