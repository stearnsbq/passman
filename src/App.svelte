<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import Login from "./routes/login.svelte";
  import Setup from "./routes/setup.svelte";
</script>

<Router>
  <Route path="/">
    <Login />
  </Route>
  <Route path="setup">
    <Setup />
  </Route>
</Router>

<style>
</style>
