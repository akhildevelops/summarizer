<script lang="ts">
    import { text } from "svelte/internal";
    import { Card } from "../model/card";
    export let cards: Array<Card> = [];
    let pop_card: Card;
    let visible: boolean = false;
</script>

<div class="cards">
    {#each cards as card}
        <div
            class="card"
            on:click={() => {
                pop_card = card;
                visible = !visible;
            }}
            on:keypress={() => {
                console.log("Does nothing"); // To ignore the ay11 warning
            }}
        >
            <p>{card.title}</p>
        </div>
    {/each}
    {#if visible}
        <div class="pop-box">
            <div class="header">
                <div class="title">
                    {pop_card.title}
                </div>
                <div
                    class="cross"
                    on:click={() => (visible = !visible)}
                    on:keypress={() => {
                        console.log("Does Nothing");
                    }}
                >
                    ❌
                </div>
            </div>
            <div class="text">
                {pop_card.content}
            </div>
        </div>
    {/if}
</div>

<style>
    .cards {
        display: grid;
        grid-auto-flow: column;
    }
    .card {
        border: 0.1em solid white;
        margin: 2em;
    }
    .pop-box {
        position: fixed;
        border: 0.1em solid white;
        background-color: black;
        height: 20em;
        width: 78em;
        text-align: justify;
    }
    .pop-box > .text {
        position: relative;
        top: 2em;
        padding: 1em;
    }
    .pop-box > .header {
        position: relative;
        text-align: center;
        top: 1em;
    }
    .pop-box > .header > * {
        display: inline;
    }
    .pop-box > .header > .cross {
        position: fixed;
        margin-left: 34em;
    }
    .pop-box > .header > .cross:hover {
        cursor: pointer;
    }
</style>
