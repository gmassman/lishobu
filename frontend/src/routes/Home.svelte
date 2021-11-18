<script lang="ts">
    import axios from 'axios'

    let api_host = import.meta.env.VITE_API_HOST as string

    let data = ""
    const getData = async () => {
        const resp = await axios.get(api_host)
        data = resp.data
    }

</script>

<div>
    <h1>Home</h1>

    API at {api_host}

    <button on:click={getData}>Backend says</button>
    {data}
</div>

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
