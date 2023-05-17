<script lang="ts">
  import type { api_response } from "./parser";
  import { parse_api_array } from "./parser";
  import { Card as CardModel } from "./model/card";
  import Card from "./lib/card.svelte";
  import { SUMMARIZER_URL } from "./default";
  import { onMount } from "svelte";
  let cards: Array<CardModel> = [];
  onMount(async () => {
    let podcast_data = await (
      await fetch(`${SUMMARIZER_URL}/summaries`)
    ).json();
    let data = parse_api_array(podcast_data as Array<api_response>);
    cards = data.podcasts.map((x) => {
      return CardModel.from(x);
    });
  });
</script>

<main>
  <Card {cards} />
</main>
