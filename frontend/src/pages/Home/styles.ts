import styled from "styled-components";

export const HomeContainer = styled.main`
	max-width: 76rem;
	height: 100%;
	width: 100%;
	margin: 5rem auto;
	padding: 2.5rem;

	border-radius: 3px;
	background: ${props => props.theme['primary-900']};
`;
