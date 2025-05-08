import styled from 'styled-components'

export const LinkContainer = styled.li`
  background: ${props => props.theme['secondary-900']};
  border-radius: 3px;
  margin-bottom: 1rem;
  padding: 2rem;

  a {
    text-decoration: none;
  }

  h3 {
    color: ${props => props.theme['accent-200']};
  }

  h3:hover {
    cursor: pointer;
    color: ${props => props.theme['text-100']};
    text-decoration-line: underline;
  }

`
