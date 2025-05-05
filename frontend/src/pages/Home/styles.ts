import styled from 'styled-components'

export const HomeContainer = styled.main`
	max-width: 76rem;
	height: 100%;
	width: 100%;
	margin: 5rem auto;
	border-radius: 3px;
`
export const HomePostsContainer = styled.div`
  margin-top: 2rem;
	padding: 2.5rem;
	border: 1px solid ${props => props.theme['primary-800']};
	background: ${props => props.theme['primary-900']};

ul:last-child {
	border-top: 1px solid ${props => props.theme['secondary-800']};
}

ul {
  list-style-type: none;
}
`
