
<div class="gitorial-step">
<div class="gitorial-step-text">

{{#include ./source/README.md}}

</div>
<div class="gitorial-step-editor">


<link rel="stylesheet" href="../_gitorial/monaco-setup.css">
<script src="../_gitorial/monaco-setup.js"></script>

<div class="gitorial-monaco" data-gitorial-monaco data-manifest="./files.json">
  <div class="gitorial-monaco-toolbar" data-gitorial-toolbar>
    <span class="label">File</span>
    <select class="file-select" data-gitorial-files></select>
    <button class="toggle" data-gitorial-toggle>View solution</button>
    <button class="diff-toggle" data-gitorial-diff>View diff</button>
    <button class="copy-toggle" data-gitorial-copy>Copy code</button>
  </div>
  <div class="gitorial-monaco-editor" data-gitorial-editor></div>
  <div class="gitorial-monaco-footer" data-gitorial-footer></div>
</div>

<script>
  if (window.__gitorialBoot) {
    window.__gitorialBoot();
  }
</script>


</div>
</div>
