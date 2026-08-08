<script lang="ts">
  let identifier = '';
</script>

<h1>Create account</h1>
<label>Email <input type="email" bind:value={identifier} /></label>
<button disabled={!identifier}>Continue</button>
