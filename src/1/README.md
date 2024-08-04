
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
<button class="subtab tablinks file-source file-added active" onclick="switchSubTab(event, 'Cargo.toml')" data-id="Cargo.toml">Cargo.toml</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'rustfmt.toml')" data-id="rustfmt.toml">rustfmt.toml</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'src/impls.rs')" data-id="src/impls.rs">src/impls.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'src/lib.rs')" data-id="src/lib.rs">src/lib.rs</button>
</div>
<div id="source/Cargo.toml" class="subtab tabcontent active" data-id="Cargo.toml">

```toml
{{#include ./source/Cargo.toml}}
```

</div>

<div id="source/rustfmt.toml" class="subtab tabcontent" data-id="rustfmt.toml">

```toml
{{#include ./source/rustfmt.toml}}
```

</div>

<div id="source/src/impls.rs" class="subtab tabcontent" data-id="src/impls.rs">

```rust
{{#include ./source/src/impls.rs}}
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