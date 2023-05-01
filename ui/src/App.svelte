<script lang="ts">
  import type { Podcasts } from "./model/input";
  import { Card as CardModel } from "./model/card";
  import Card from "./lib/card.svelte";
  import podcast_data from "../sample.json";
  let data = podcast_data as Podcasts;
  let cards = data.podcasts.map((x) => {
    return CardModel.from(x);
  });
</script>

<main>
  <Card {cards} />
</main>

<style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</style>
