import styled from 'styled-components'

export const LinkContainer = styled.div`
border-bottom: 1px solid ${props => props.theme['primary-800']};
padding: 2rem;

&:first-child {
	border-top: 1px solid ${props => props.theme['primary-800']};
}

a {
  text-decoration: none;
}

p {
	color: ${props => props.theme['primary-800']};
}

h3 {
	text-decoration-line: underline;
	color: ${props => props.theme['accent-200']};
}

&:hover {
	background: ${props => props.theme['background-800']};
	cursor: pointer;
	
	h3 {
		color: ${props => props.theme['accent-100']};
	}
	
	p {
		color: ${props => props.theme['text-600']};
	}
}

`
