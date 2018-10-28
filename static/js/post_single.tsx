import * as React from 'react';
import {Link, Redirect, RouteComponentProps} from 'react-router-dom';
import {format as dateFormat} from 'date-fns';
import {Post, listPosts} from './api_client';

interface Props extends RouteComponentProps<{slug: string}> { }

interface State {
    // This is a tri-state: not-loaded, loaded, invalid-load
    post?: Post | null;
}

export class PostSingle extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props);
        this.state = {};
    }

    public componentDidMount() {
        const slug = this.props.match.params.slug;
        listPosts().then(posts => {
            const validPosts = posts.filter(post => post.slug == slug);
            if (validPosts.length == 0) {
                this.setState({post: null})
            } else {
                this.setState({post: validPosts[0]})
            }
        });
    }

    public render() {
        const {post} = this.state;
        if (post === undefined) {
            return null;
        } 
        if (post === null) {
            return <Redirect to="/" />
        }
        return (
            <div className="post-single">
                <div className="post">
                    <div className="post__back-button"><Link to="/">&laquo;</Link></div>
                    <h1 className="post__title">{post.title}</h1>
                    <h2 className="post__date">{dateFormat(post.date, 'MMM D, YYYY')}</h2>
                    <div className="post__body" dangerouslySetInnerHTML={{__html: post.body}} />
                </div>
            </div>
        )
    }
}
