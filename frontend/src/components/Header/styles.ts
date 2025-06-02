import { Link } from 'react-router-dom'
import styled from 'styled-components'

export const HeaderContainer = styled.header`
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 2rem;
  background: radial-gradient(at 50% 100%, ${props => props.theme['primary-900']}, ${props => props.theme.background});
  background: ${props => props.theme['secondary-950']};
  border-bottom: 1px solid ${props => props.theme.secondary};

  nav {
    display: flex;
    gap: 1rem;

    a {
      text-decoration: none;
      color: ${props => props.theme['text']};

      &:hover {
        color: ${props => props.theme['background-900']};
        background: ${props => props.theme['accent-200']};
        text-decoration: underline;
      }
    }
  }
`

export const Logo = styled(Link)`
  text-shadow: 0 0 15px ${props => props.theme['secondary']};
  text-decoration: none;
  color: ${props => props.theme['accent-100']};
  font-weight: bold;
  font-size: 2rem;

  &:hover {
    text-decoration: underline;
    text-shadow: 0 0 25px ${props => props.theme['secondary-400']};
  }
`
