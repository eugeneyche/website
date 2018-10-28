export interface Post {
    slug: string;
    title: string;
    date: Date;
    body: string;
}

interface RawPost {
    slug: string;
    title: string;
    date: string;
    body: string;
}

let postCache: Promise<Array<Post>> | undefined;

export async function listPosts(): Promise<Array<Post>> {
    if (postCache !== undefined) {
        try {
            const posts = await postCache;
            return Promise.resolve(posts);
        } catch { }
    }
    postCache = fetch('/api/list_posts', {method: 'POST'})
        .then(resp => resp.json())
        .then(json => {
            return json.posts.map((raw_post: RawPost) => ({
                slug: raw_post.slug,
                title: raw_post.title,
                date: new Date(raw_post.date),
                body: raw_post.body,
            }));
        });
    return postCache
}
