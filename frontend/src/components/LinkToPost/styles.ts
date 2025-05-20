import { Link } from 'react-router-dom'
import styled from 'styled-components'

export const LinkContainer = styled(Link)`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  text-decoration: none;

  color: ${props => props.theme['text']};
  background: radial-gradient(at 50% 200%, ${props => props.theme['accent-900']} 0%, transparent 100%);
  border-top: 2px solid ${props => props.theme.background};
  border-bottom: 2px solid ${props => props.theme['accent-600']};
  border-radius: 3px;
  margin-bottom: 1rem;
  padding: 2rem;

  h3 {
    text-decoration: none;
  }

  &:hover {
    cursor: pointer;
    font-weight: bold;
    background: ${props => props.theme['secondary-900']};
    background: radial-gradient(at 50% 200%, ${props => props.theme['secondary-800']} 0%, transparent 100%);
    color: ${props => props.theme['accent-200']};
    /*box-shadow: 0 0 10px 2px ${props => props.theme['primary-900']};*/

    border-bottom: 2px solid ${props => props.theme['secondary-800']};
    transition: 0.1s;

    h3 {
      text-decoration: underline;
    }
  }

`
