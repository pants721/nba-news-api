import { render } from 'preact';
import './style.css'
import TopStories from './top_stories';

export function App() {
	return (
		<div>
			<h1 class="title">NBA News Aggregator</h1>
			<TopStories/>
		</div>
	);
}

render(<App />, document.getElementById('app'));
