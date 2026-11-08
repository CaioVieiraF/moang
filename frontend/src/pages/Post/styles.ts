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

  small {
    color: ${props => props.theme['secondary-500']};
  }
`

export const PostReplies = styled.section`
  margin: 2rem;
  padding: 2rem 1rem 0;
  border-top: 2px solid ${props => props.theme['secondary']};
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
