import { Link } from 'react-router-dom'
import styled from 'styled-components'

export const HeaderContainer = styled.header`
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 2rem;
  background: ${props => props.theme['primary-600']};

  nav {
    display: flex;
    gap: 0.5rem;

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
  text-decoration: none;
  color: ${props => props.theme['accent-100']};
  font-weight: bold;
  font-size: 2rem;

  &:hover {
    color: ${props => props.theme['background-900']};
    background: ${props => props.theme['accent-200']};
    text-decoration: underline;
  }
`
