import { LinkContainer } from "./styles";

interface LinkProps {
	title: string,
	content: string,
}

export function LinkToPost({ title, content }: LinkProps) {
	return (
		<LinkContainer>
			<h3>{title}</h3>
			<p>{content}</p>
		</LinkContainer>
	)
}
