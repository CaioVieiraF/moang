import { Link } from 'react-router-dom'
import styled from 'styled-components'

export const LinkContainer = styled(Link)`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  text-decoration: none;

  color: ${props => props.theme['text']};
  border-top: 2px solid transparent;
  border-bottom: 2px solid ${props => props.theme['primary-600']};
  margin-bottom: 1rem;
  padding: 2rem;

  h3 {
    text-decoration: none;
  }

  &:hover {
    cursor: pointer;
    font-weight: bold;
    background: ${props => props.theme['secondary-800']};
    color: ${props => props.theme['accent-200']};

    border-bottom: 2px solid transparent;
  }

`
