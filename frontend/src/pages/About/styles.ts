import styled from 'styled-components'

export const AboutContainer = styled.main`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  margin: 2rem;

  a {
    text-decoration: none;
    color: ${props => props.theme['accent-200']};
  }
`
