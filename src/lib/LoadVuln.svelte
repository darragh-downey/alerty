<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";

    let vulnerabilities: string[] = [];

    const load = () => {
    invoke('load_feed', {
    number: 42,
    })
    .then((res: any) => console.log(`Message: ${res.message}, Other val: ${res.other_val}`)
    )
    .catch((e) => console.error(e));
    }
</script>

<button on:click={load}>
    new vulnerabilities
</button>
