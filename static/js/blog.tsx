import * as React from 'react';
import {format as dateFormat} from 'date-fns';
import {Link} from 'react-router-dom';
import {Post, listPosts} from './api_client';

interface Props { }

interface State {
    posts: Array<Post>;
}

export class Blog extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props);
        this.state = {posts: []};
    }

    public componentDidMount() {
        listPosts().then(posts => this.setState({posts}));
    }

    public render() {
        const {posts} = this.state;
        return (
            <div className="blog">
                {posts.map((post, index) => (
                    <div key={index} className="post post--short">
                        <h1 className="post__title"><Link to={`/post/${post.slug}`}>{post.title}</Link></h1>
                        <h2 className="post__date">{dateFormat(post.date, 'MMM D, YYYY')}</h2>
                        { post.summary !== undefined ? (
                        <p className="post__summary">{post.summary}</p>
                        ) : null}
                    </div>
                ))}
            </div>
        );
    }
}
