import { useEffect, useState } from "preact/hooks";
import Article from "./article";

export default function TopStories() {
	const [stories, setStories] = useState([]);
	const [isLoading, setIsLoading] = useState(false);

	useEffect(() => {
		setIsLoading(true);
		fetch("http://localhost:8080/top")	
			.then((response) => response.json())
			.then((data) => {
					setStories(data);
					setIsLoading(false);
			})
			.catch((err) => console.error(err.message));
	}, []);

	return (
	<div className="top-stories-container">
		{isLoading ? <p>Fetching stories...</p> : null}
		<ol className="top-stories-list">
			{stories.map((story) => {
					return (
							<li>
								<Article 
									title={story.title}
									subtitle={story.subtitle}
									author={story.author}
									date={story.date}
									base_url={story.base_url}
									url={story.url}
									source={story.source}
								/>
							</li>
						   )
			})}
		</ol>
	</div>
	)
}
