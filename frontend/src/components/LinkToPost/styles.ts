import { Link } from 'react-router-dom'
import styled from 'styled-components'

export const LinkContainer = styled(Link)`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  text-decoration: none;

  background: ${props => props.theme['secondary-900']};
  border-top: 2px solid transparent;
  border-bottom: 2px solid transparent;
  margin-bottom: 1rem;
  padding: 2rem;

  h3 {
    color: ${props => props.theme['text']};
    text-decoration: none;
  }

  &:hover {
    cursor: pointer;
    font-weight: bold;
    background: ${props => props.theme['secondary-800']};

    border-bottom: 2px solid ${props => props.theme['primary-600']};
  }

`
