
<div class="content-row">
<div class="content-col">

{{#include ./source/README.md}}

</div>
<div class="content-col">

<div class="tab">
  <button class="maintab tablinks active" onclick="switchMainTab(event, 'Source')">Source</button>
  <button class="maintab tablinks" onclick="switchMainTab(event, 'Diff')">Diff</button>
</div>

<div id="Source" class="maintab tabcontent active">

<div class="tab">
<button class="subtab tablinks file-source file-added active" onclick="switchSubTab(event, '.gitorial/images/frame-runtime.png')" data-id=".gitorial/images/frame-runtime.png">.gitorial/images/frame-runtime.png</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, '.gitorial/metadata.json')" data-id=".gitorial/metadata.json">.gitorial/metadata.json</button>
<button class="subtab tablinks file-source file-modified" onclick="switchSubTab(event, 'src/lib.rs')" data-id="src/lib.rs">src/lib.rs</button>
</div>
<div id="source/.gitorial/images/frame-runtime.png" class="subtab tabcontent active" data-id=".gitorial/images/frame-runtime.png">

```text
{{#include ./source/.gitorial/images/frame-runtime.png}}
```

</div>

<div id="source/.gitorial/metadata.json" class="subtab tabcontent" data-id=".gitorial/metadata.json">

```text
{{#include ./source/.gitorial/metadata.json}}
```

</div>

<div id="source/src/lib.rs" class="subtab tabcontent" data-id="src/lib.rs">

```rust
{{#include ./source/src/lib.rs}}
```

</div>



</div>

<div id="Diff" class="maintab tabcontent">


<div class="tab">
	<button class="difftab tablinks active" onclick="switchDiff(event, 'changes.diff')" data-id="changes.diff">changes.diff</button>
</div>
<div id="changes.diff" class="difftab tabcontent active" data-id="changes.diff">

```diff
{{#include ./source/changes.diff}}
```

</div>

</div>

</div>
</div>
