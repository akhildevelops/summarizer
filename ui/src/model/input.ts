export interface Podcasts {
    version: string,
    podcasts: Array<Podcast>
}

export interface Podcast {
    title: string,
    image: string,
    url: string,
    text: string
}