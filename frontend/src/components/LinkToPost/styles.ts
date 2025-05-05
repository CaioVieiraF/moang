import styled from 'styled-components'

export const LinkContainer = styled.li`
border-bottom: 1px solid ${props => props.theme['secondary-800']};
padding: 2rem;


a {
  text-decoration: none;
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
