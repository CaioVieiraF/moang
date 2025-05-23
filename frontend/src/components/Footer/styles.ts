import styled from 'styled-components'

export const FooterContainer = styled.footer`
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;

  background: radial-gradient(at 50% 0%, ${props => props.theme['secondary-950']}, ${props => props.theme.background});
  display: flex;
  align-items: center;
  gap: 0.5rem;

  justify-content: center;
  padding: 2rem;
  border-top: 1px solid ${props => props.theme.secondary};

  a {
    text-decoration: none;
    color: ${props => props.theme['text-600']};
    font-size: 20px;

    &:hover {
      color: ${props => props.theme['background-900']};
      background: ${props => props.theme['accent-200']};
    }
  }
`
