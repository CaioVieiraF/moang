import styled from 'styled-components'

export const AboutContainer = styled.main`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  margin: 2rem;
  flex-grow: 1;

  a {
    text-decoration: none;
    color: ${props => props.theme['accent-200']};
  }

  @media (max-width: 600px) {
    margin: 1.5rem 1rem;
  }
`
