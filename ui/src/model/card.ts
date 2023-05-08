import type { Podcast, Podcasts } from "./input";
export interface Card {
    image: URL,
    title: string,
    short?: string,
    content: string
}

export class Card implements Card {
    image: URL;
    title: string;
    content: string;
    short?: string;

    constructor(image: URL, title: string, content: string, short: string | null = null) {
        this.image = image
        this.title = title
        this.content = content
        if (short !== null) {
            this.short = short
        }

    }

    static from(source: Podcast): Card {
        return new Card(new URL(source.image), source.title, source.text)
    }

}
