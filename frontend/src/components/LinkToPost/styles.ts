import { Link } from 'react-router-dom'
import styled from 'styled-components'

export const LinkContainer = styled(Link)`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  gap: 1.25rem;
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

  .post {
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    min-width: 0;
  }

  h3 {
    text-decoration: none;
    overflow-wrap: anywhere;
  }

  .meta {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .kind {
    font-size: 0.7rem;
    font-weight: bold;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    padding: 0.15rem 0.5rem;
    border-radius: 999px;
    color: ${props => props.theme['accent-200']};
    border: 1px solid ${props => props.theme['accent-400']}70;
    background: ${props => props.theme['accent-400']}10;
  }

  .stats {
    display: flex;
    align-items: center;
    gap: 1.25rem;
    flex-shrink: 0;

    span {
      display: inline-flex;
      align-items: center;
      gap: 0.35rem;

      font-size: 0.8rem;
      color: ${props => props.theme['secondary-300']};
    }

    span:nth-child(1) svg {
      color: ${props => props.theme['accent-400']};
    }

    span:nth-child(2) svg {
      color: ${props => props.theme['text-300']};
    }

    span:nth-child(3) svg {
      color: ${props => props.theme['secondary-300']};
    }
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

  @media (max-width: 600px) {
    flex-direction: column;
    align-items: flex-start;
    padding: 1.25rem;

    .stats {
      flex-wrap: wrap;
      gap: 0.85rem;
    }
  }
`
