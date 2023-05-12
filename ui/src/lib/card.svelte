<script lang="ts">
    import { Card } from "../model/card";
    export let cards: Array<Card> = [];
    let pop_card: Card;
    let visible: boolean = false;
</script>

<div class="cards">
    {#each cards as card}
        {#if card.title != "no-title"}
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
                <img src={card.image.toString()} alt="Podcast Summary" />
                <p>{card.title}</p>
            </div>
        {/if}
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
        grid-template-columns: 1fr 1fr;
    }
    .card {
        border: 0.1em solid white;
        margin: 2em;
        /* width: 20em; */
    }

    .card > img {
        /* width: 20em; */
        height: 20em;
        cursor: pointer;
    }
    .pop-box {
        left: 50%;
        transform: translate(-50%, 0);
        position: fixed;
        border: 0.1em solid white;
        background-color: black;
        text-align: justify;
    }
    .pop-box > .text {
        position: relative;
        top: 1em;
        padding: 2em;
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
        /* position: fixed; */
        transform: translate(100%, 0);
        cursor: pointer;
    }
</style>
