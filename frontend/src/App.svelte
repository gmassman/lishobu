<script lang="ts">
    import axios from 'axios'

    let api_host: string = import.meta.env.VITE_API_HOST as string

    let data: string = ""
    const getData = async () => {
        const resp = await axios.get(api_host)
        debugger
        data = resp.data
    }
</script>

<main>
    API at {api_host}

    <button on:click={getData}>Backend says</button>
    {data}
</main>

<style>
    :root {
        font-family: Helvetica, Verdana, Arial, Geneva, sans-serif;
    }

    button {
        margin-top: 0.5em;
        margin-bottom: 0.5em;
        display: block;
        font-size: 1.5em;
        border-radius: 0.25em;
    }
</style>
