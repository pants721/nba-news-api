import './style.css'

interface ArticleProps {
	title: string,
	subtitle: string,
	author: string,
	date: string,
	base_url: string,
	url: string,
	source: string,
}

export default function Article(props: ArticleProps) {
	return (
		<div className="article-container" href={props.url}>
			<a className="article-link" href={props.url}>
				<p className="article-title">{props.title} (<a className="article-source-link" href={props.base_url}>{props.base_url.replace("https://", "").replace("www.", "")}</a>)</p>
			</a>
			<small className="article-subtitle">{props.subtitle}</small>
		</div>
	)
}

