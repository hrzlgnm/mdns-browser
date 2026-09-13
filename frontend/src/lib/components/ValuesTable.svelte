<script lang="ts">
  import ClipboardButton from './ClipboardButton.svelte'

  let {
    values,
    title,
    copyValues,
  }: {
    values: Array<string>
    title: string
    copyValues?: Array<string>
  } = $props()

  const rows = $derived(values.map((value, i) => ({ value, copy: copyValues?.[i] ?? value })))
</script>

{#if values.length > 0}
  <table>
    <thead>
      <tr><th>{title}</th></tr>
    </thead>
    <tbody>
      {#each rows as row, i (i)}
        <tr>
          <td>
            <ClipboardButton text={row.value} copyText={row.copy} buttonText={row.value} iconOnly />
            {row.value}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
{:else}
  <div class="hidden"></div>
{/if}
