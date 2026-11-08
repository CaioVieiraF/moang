import { Link } from 'react-router-dom'
import styled from 'styled-components'

export const PostContainer = styled.main`
  max-width: 76rem;
  height: 100%;
  width: 100%;
  margin: 2rem auto;
  flex-grow: 1;

  article {
    margin: 2rem;
    font-size: 18px;
    line-height: 2;
    padding: 1rem;
    white-space: pre-line;
    overflow-wrap: anywhere;
  }

  @media (max-width: 600px) {
    article {
      margin: 1rem;
      padding: 0.5rem;
      font-size: 1rem;
      line-height: 1.8;
    }
  }
`

export const PostTitle = styled.header`
  display: flex;
  flex-direction: column;
  align-items: left;
  justify-content: center;

  padding: 2.5rem;
  gap: 1rem;
  color: ${props => props.theme['accent-100']};

  border-bottom: 2px solid ${props => props.theme['secondary']};

  h1 {
    overflow-wrap: anywhere;
  }

  small {
    color: ${props => props.theme['secondary-500']};
  }

  @media (max-width: 600px) {
    padding: 1.5rem 1.25rem;
  }
`

export const PostMeta = styled.div`
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.6rem;

  font-size: 0.85rem;
  color: ${props => props.theme['secondary-300']};

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

  .dot {
    color: ${props => props.theme['secondary-500']};
  }

  .edited {
    font-style: italic;
    color: ${props => props.theme['secondary-500']};
  }
`

export const PostStats = styled.div`
  display: flex;
  align-items: center;
  gap: 1.75rem;
  flex-wrap: wrap;

  margin: 0 2rem;
  padding: 1rem 1.5rem;

  border-top: 1px solid ${props => props.theme['secondary-400']}40;
  border-bottom: 1px solid ${props => props.theme['secondary-400']}40;

  span {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;

    font-size: 0.9rem;
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

  @media (max-width: 600px) {
    margin: 0 1.25rem;
    padding: 1rem 0.5rem;
    gap: 1rem;
  }
`

export const PostReplies = styled.section`
  margin: 2rem;
  padding: 2rem 1rem 0;
  border-top: 2px solid ${props => props.theme['secondary']};

  @media (max-width: 600px) {
    margin: 1.5rem 1rem;
    padding: 1.5rem 0 0;
  }
`

export const PostRepliesHeader = styled.header`
  display: flex;
  align-items: baseline;
  gap: 0.75rem;

  margin-bottom: 1.5rem;

  h2 {
    color: ${props => props.theme['accent-100']};
  }

  small {
    color: ${props => props.theme['secondary-500']};
  }
`

export const PostRepliesList = styled.ul`
  display: flex;
  flex-direction: column;
  gap: 1rem;

  list-style: none;
`

export const PostState = styled.p`
  padding: 2rem 1.25rem;
  text-align: center;
  color: ${props => props.theme['secondary-300']};

  button {
    border: 0;
    background: transparent;
    color: ${props => props.theme['accent-200']};
    font-weight: bold;
    text-decoration: underline;
    cursor: pointer;
  }
`

export const PostTitleBackLink = styled(Link)`
  margin-right: auto;
  text-decoration: none;
  color: ${props => props.theme['accent']};

  &:hover {
    color: ${props => props.theme['primary-900']};
    background: ${props => props.theme['accent-200']};
    text-decoration: underline;
  }
`
