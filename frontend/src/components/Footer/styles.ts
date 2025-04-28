import styled from "styled-components";

export const FooterContainer = styled.footer`
	display: flex;
	align-items: center;
	justify-content: center;

	padding: 2rem;
	border-top: 2px solid ${props => props.theme['primary-700']};

	a {
		text-decoration: none;
		color: ${props => props.theme['text']};
		
		&:hover {
			color: ${props => props.theme['background-900']};
			background: ${props => props.theme['accent-200']};
		}
	}
`;
