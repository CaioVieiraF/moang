import { useState } from "react";
import { HomeContainer } from "./styles";
import { LinkToPost } from "../../components/LinkToPost";

interface Post {
	title: string,
	content: string,
}

export function Home() {
	const [posts,] = useState<Post[]>([
		{
			title: "#1 Post",
			content: "Post test",
		}, {
			title: "#2 Post",
			content: "Post test",
		}, {
			title: "#3 Post",
			content: "Post test",
		},

	])
	return (
		<HomeContainer>
			{posts.map(post => <LinkToPost title={post.title} content={post.content} />)}
		</HomeContainer>
	);
}
