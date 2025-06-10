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
      color: ${props => props.theme['accent-100']};

      &:hover {
        color: ${props => props.theme['secondary-950']};
        background: ${props => props.theme['accent-200']};
        text-decoration: underline;
      }
    }
  }
`

export const Logo = styled(Link)`
  color: ${props => props.theme['accent-100']};
  padding: 0.25rem;
  text-decoration: none;
  border-radius: 3px;
  font-weight: bold;
  font-size: 2rem;

  &:hover {
    color: ${props => props.theme['secondary-950']};
    background-color: ${props => props.theme['accent-100']};

    text-decoration: underline;
    transition: text-shadow 0.1s, color 0.1s, background-color 0.1s;
  }
`
