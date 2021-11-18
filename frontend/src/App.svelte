<script lang="ts">
    import {Router, Route} from 'svelte-routing'
    import Home from './routes/Home.svelte'
    export let url = ''
</script>

<main>
    <Router url={url}>
        <Route path="/" component={Home} />
    </Router>
</main>
