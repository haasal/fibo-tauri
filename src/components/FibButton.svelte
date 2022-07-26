<script lang="ts">
    let n_1 = 1;
    let n_2 = 1;
    let n = 1;

    function fibbonachi() {
        let n_1_ = n_1;
        let n_2_ = n_2;
        n_1 = n_2_;
        n_2 = n_1_ + n_2_;
        n = n_2;
    }
</script>

<button
    class="rounded-md hover:bg-slate-100 bg-white py-2 px-10 text-secondary"
    on:click={fibbonachi}
>
    {n}
</button>
