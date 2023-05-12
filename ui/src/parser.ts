import type { Podcast, Podcasts } from "./model/input";
import { SUMMARIZER_URL } from "./default";
export interface api_response {
    content: string,
    created_at: Date,
    image_id: string | null,
    link: string,
    title: string | null
}



export function parse_api(data: api_response): Podcast {
    let image_link: string
    if (data.image_id === null) {
        image_link = `${SUMMARIZER_URL}/thumbnails/notfound.jpg`
    } else {
        image_link = `${SUMMARIZER_URL}/thumbnails/${data.image_id}.jpg`
    }
    return {
        title: data.title || "no-title",
        image: image_link,
        url: data.link,
        text: data.content
    }
};
export function parse_api_array(data: Array<api_response>): Podcasts {
    return {
        version: "0.1",
        podcasts: data.map((x) => parse_api(x))
    }
};
