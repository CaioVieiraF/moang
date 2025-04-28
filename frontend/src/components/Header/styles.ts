import styled from "styled-components";

export const HeaderContainer = styled.header`
	display: flex;
	align-items: center;
	justify-content: center;
	padding: 2rem;
	border-bottom: 2px solid ${props => props.theme['primary-700']};
	
	nav {
		display: flex;
		gap: 0.5rem;

		a {
			color: ${props => props.theme['text']};
			font-weight: bold;
			font-size: 2rem;
			
			&:hover {
				color: ${props => props.theme['background-900']};
				background: ${props => props.theme['accent-200']};
				text-decoration: none;
			}
		}

	}
`;
