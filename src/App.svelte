<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import Login from "./routes/login/login.svelte";
  import Setup from "./routes/setup/setup.svelte";
</script>

<Router>
  <Route path="/">
    <Setup />
  </Route>
  <Route path="setup">
    <Setup />
  </Route>
</Router>

<style>
</style>
