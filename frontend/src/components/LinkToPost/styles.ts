import { Link } from 'react-router-dom'
import styled from 'styled-components'

export const LinkContainer = styled(Link)`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  text-decoration: none;

  color: ${props => props.theme['text']};
  color: ${props => props.theme['accent-100']};
  background: ${props => props.theme['secondary-400']}05;
  border: 1px solid ${props => props.theme['secondary-400']}70;
  border-bottom: 1px solid ${props => props.theme['secondary-400']}40;
  border-right: 1px solid ${props => props.theme['secondary-400']}40;
  box-shadow: 0 25px 45px rgba(0, 0, 0, .2);
  border-radius: 3px;
  margin-bottom: 1rem;
  padding: 2rem;

  h3 {
    text-decoration: none;
  }

  &:hover {
    cursor: pointer;
    font-weight: bold;
    color: ${props => props.theme['accent-200']};
    background: ${props => props.theme['secondary-400']}10;
    box-shadow: 0 25px 45px rgba(0, 0, 0, .8);

    transition: box-shadow 0.2s;

    h3 {
      text-decoration: underline;
    }
  }

`
