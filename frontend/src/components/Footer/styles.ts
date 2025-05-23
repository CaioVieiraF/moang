import styled from 'styled-components'

export const FooterContainer = styled.footer`
  display: flex;
  align-items: center;
  gap: 0.5rem;
  justify-content: center;
  padding: 2rem;
  width: 100%;

  background: ${props => props.theme['secondary-950']};
  border-top: 1px solid ${props => props.theme.secondary};

  a {
    text-decoration: none;
    color: ${props => props.theme['text-300']};
    font-size: 20px;

    &:hover {
      color: ${props => props.theme['secondary-900']};
      background: ${props => props.theme['text-300']};
    }
  }
`
