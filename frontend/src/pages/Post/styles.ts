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
  border-bottom: 2px solid ${props => props.theme['secondary-800']};
  color: ${props => props.theme['accent-100']};

  small {
    color: ${props => props.theme['secondary-500']};
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
