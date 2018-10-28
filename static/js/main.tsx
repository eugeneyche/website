import * as React from 'react';
import * as ReactDOM from 'react-dom';
import {BrowserRouter as Router, Route} from 'react-router-dom';
import {Blog} from './blog';
import {PostSingle} from './post_single';

const AppRouter: React.SFC<{}> = () => {
    return (
        <Router>
            <div>
                <Route path="/" exact component={Blog} />
                <Route path="/post/:slug" exact component={PostSingle} />
            </div>
        </Router>
    );
};

ReactDOM.render(<AppRouter />, document.getElementById('root'));
